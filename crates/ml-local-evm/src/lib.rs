#![forbid(unsafe_code)]

//! A small Rust/revm execution lane for the MemoryLineage Solidity registry.
//!
//! The legacy EthereumJS harness remains the compatibility oracle. This crate
//! deliberately exercises the published Solidity creation artifact through a
//! separate EVM implementation and exposes the exact revert reason needed by
//! the Silent Rollback demo.

use alloy::primitives::{Address, B256, Bytes, U256};
use alloy::sol;
use alloy::sol_types::SolCall;
use k256::ecdsa::SigningKey;
use ml_core::{commitment, domain_separator, signing_digest, space_id, transition_and_root};
use ml_spec_types::{
    AttackObservation, AuthorizationProof, AuthorizationRecord, EVIDENCE_V2, EvidenceBundleV2,
    EvidenceNetwork, ExperienceDelta, Head as EvidenceHead, PrivacyBoundary, RegistryObservation,
    SOURCE_DEMO_SPACE_V2_LOCAL, SPEC_NAME, SPEC_SNAPSHOT, SpecSnapshot, TransitionRecord,
    VerificationMetadata,
};
use revm::context::{BlockEnv, CfgEnv, Context, TxEnv, result::ExecutionResult};
use revm::database::{CacheDB, EmptyDB};
use revm::handler::{MainBuilder, MainnetContext, MainnetEvm};
use revm::primitives::{TxKind, hardfork::SpecId};
use revm::state::{AccountInfo, Bytecode};
use revm::{ExecuteCommitEvm, ExecuteEvm, MainContext};
use serde::Serialize;
use std::str::FromStr;
use thiserror::Error;

const CHAIN_ID: u64 = 31_337;
pub const POLKADOT_HUB_TESTNET_CHAIN_ID: u64 = 420_420_417;
const GAS_LIMIT: u64 = 12_000_000;
const GAS_PRICE: u128 = 10;
const SIGNER_PRIVATE_KEY: [u8; 32] = [0x33; 32];
const WRONG_SIGNER_PRIVATE_KEY: [u8; 32] = [0x22; 32];
const SPACE_SALT: &str = "0xdddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd";
const PROFILE_ID: &str = "0x9b19c9f72bcf8d35b46e0e723adbd9ad0cb9a5f0b8e8e4fb7cf5be6f2f9f5c7e";

const REGISTRY_CREATION_HEX: &str =
    include_str!("../../../contracts/artifacts/memory_lineage_registry_creation.hex");
const MOCK_1271_CREATION_HEX: &str =
    include_str!("../../../contracts/artifacts/mock_1271_authorizer_creation.hex");

sol! {
    struct ExperienceDeltaCall {
        bytes32 spaceId;
        uint64 sequence;
        bytes32 prevStateRoot;
        bytes32 deltaCommitment;
        bytes32 provenanceCommitment;
        bytes32 profileId;
        bytes32 locatorCommitment;
    }

    interface MemoryLineageRegistry {
        function registerSpace(
            bytes32 spaceId,
            address controller,
            address authorizer,
            bytes32 salt,
            bytes controllerSignature
        );
        function updateSpaceAuthorization(
            bytes32 spaceId,
            address newController,
            address newAuthorizer,
            bytes controllerSignature
        );
        function spaceAuthorization(bytes32 spaceId)
            external
            view
            returns (address controller, address authorizer, uint64 configNonce);
        function head(bytes32 spaceId)
            external
            view
            returns (bytes32 transitionId, bytes32 stateRoot, uint64 sequence);
        function commitTransition(ExperienceDeltaCall delta, bytes authorizerSignature)
            external
            returns (bytes32 transitionId, bytes32 nextStateRoot);
    }

    interface Mock1271Authorizer {
        function setAccept(bool next);
    }
}

type Database = CacheDB<EmptyDB>;
type EvmContext = MainnetContext<Database>;
type Evm = MainnetEvm<EvmContext>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Head {
    pub transition_id: B256,
    pub state_root: B256,
    pub sequence: u64,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ExecutionObservation {
    pub status: &'static str,
    pub reason: Option<String>,
    pub sequence: u64,
    pub canonical_root: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct MutationObservation {
    pub name: String,
    pub expected: String,
    pub observed: String,
    pub status: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct MutationReport {
    pub total: usize,
    pub rejected: usize,
    pub all_expected: bool,
    pub cases: Vec<MutationObservation>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct Erc1271Observation {
    pub accepted: bool,
    pub rejected_reason: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct AuthorityRotationObservation {
    pub config_nonce: u64,
    pub authorizer: String,
    pub old_authorizer_rejected: String,
    pub new_authorizer_accepted: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct SecurityInvariantObservation {
    pub name: String,
    pub status: String,
    pub expected: String,
    pub observed: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct SecurityAssuranceReport {
    pub report_type: String,
    pub status: String,
    pub formal_status: String,
    pub execution: String,
    pub registry_artifact: String,
    pub chain_id: u64,
    pub explored_histories: usize,
    pub valid_transitions: usize,
    pub stale_predecessor_cases: Vec<MutationObservation>,
    pub sequence_gap_cases: Vec<MutationObservation>,
    pub mutation_matrix: MutationReport,
    pub erc1271: Erc1271Observation,
    pub authority_rotation: AuthorityRotationObservation,
    pub invariants: Vec<SecurityInvariantObservation>,
    pub raw_memory_exported: bool,
    pub limitations: Vec<String>,
}

#[derive(Debug, Error)]
pub enum LocalEvmError {
    #[error("invalid fixed value {name}: {value}")]
    InvalidValue { name: &'static str, value: String },
    #[error("revm transaction failed validation: {0}")]
    Transaction(String),
    #[error("EVM execution halted: {0}")]
    Halt(String),
    #[error("contract call unexpectedly succeeded")]
    UnexpectedSuccess,
    #[error("contract reverted without a Solidity Error(string): {0}")]
    UndecodedRevert(String),
    #[error("ABI encoding failed: {0}")]
    Abi(String),
    #[error("signature operation failed: {0}")]
    Signing(String),
}

pub struct RegistryHarness {
    evm: Evm,
    chain_id: u64,
    pub registry: Address,
    pub signer: Address,
    pub relayer: Address,
    signer_key: SigningKey,
    relayer_key: SigningKey,
    pub space_id: B256,
    pub profile_id: B256,
}

impl RegistryHarness {
    pub fn new() -> Result<Self, LocalEvmError> {
        Self::new_with_chain_id(CHAIN_ID)
    }

    pub fn new_with_chain_id(chain_id: u64) -> Result<Self, LocalEvmError> {
        let signer_key = signing_key(SIGNER_PRIVATE_KEY)?;
        let signer = signer_address(&signer_key);
        let relayer_key = signing_key(WRONG_SIGNER_PRIVATE_KEY)?;
        let relayer = signer_address(&relayer_key);
        let salt = b256(SPACE_SALT, "space salt")?;
        let space_id_value = space_id(&format_address(signer), SPACE_SALT).map_err(|error| {
            LocalEvmError::InvalidValue {
                name: "space id",
                value: error.to_string(),
            }
        })?;
        let space_id = b256(&space_id_value, "space id")?;
        let profile_id = b256(PROFILE_ID, "profile id")?;

        let mut db = Database::new(EmptyDB::default());
        db.insert_account_info(
            signer,
            AccountInfo::new(
                U256::from(10u64).pow(U256::from(24u64)),
                0,
                B256::ZERO,
                Bytecode::default(),
            ),
        );
        db.insert_account_info(
            relayer,
            AccountInfo::new(
                U256::from(10u64).pow(U256::from(24u64)),
                0,
                B256::ZERO,
                Bytecode::default(),
            ),
        );

        let context = Context::mainnet()
            .modify_cfg_chained(|cfg: &mut CfgEnv| {
                cfg.set_spec_and_mainnet_gas_params(SpecId::SHANGHAI);
                cfg.chain_id = chain_id;
                cfg.disable_nonce_check = true;
            })
            .with_db(db)
            .with_block(BlockEnv {
                number: U256::from(1u64),
                timestamp: U256::from(1_700_000_000u64),
                gas_limit: GAS_LIMIT,
                ..Default::default()
            });
        let mut evm = context.build_mainnet();
        let creation = decode_hex(REGISTRY_CREATION_HEX, "registry creation bytecode")?;
        let deployment = evm
            .transact_commit(tx_create(signer, 0, creation, chain_id))
            .map_err(|error| LocalEvmError::Transaction(error.to_string()))?;
        let registry = match deployment {
            ExecutionResult::Success { output, .. } => {
                output.address().copied().ok_or_else(|| {
                    LocalEvmError::Transaction("deployment returned no address".to_owned())
                })?
            }
            ExecutionResult::Revert { output, .. } => {
                return Err(LocalEvmError::UndecodedRevert(
                    decode_revert_reason(&output).unwrap_or_else(|_| hex::encode(output)),
                ));
            }
            ExecutionResult::Halt { reason, .. } => {
                return Err(LocalEvmError::Halt(format!("{reason:?}")));
            }
        };

        let mut harness = Self {
            evm,
            chain_id,
            registry,
            signer,
            relayer,
            signer_key,
            relayer_key,
            space_id,
            profile_id,
        };
        harness.register_space(salt)?;
        Ok(harness)
    }

    pub fn register_space(&mut self, salt: B256) -> Result<(), LocalEvmError> {
        let call = MemoryLineageRegistry::registerSpaceCall {
            spaceId: self.space_id,
            controller: self.signer,
            authorizer: self.signer,
            salt,
            controllerSignature: Bytes::new(),
        };
        self.commit(call.abi_encode(), 1).map(|_| ())
    }

    pub fn commit_valid_transition(
        &mut self,
        sequence: u64,
        previous_root: B256,
        payload: &str,
    ) -> Result<Head, LocalEvmError> {
        let delta = self.build_delta(sequence, previous_root, payload)?;
        let call = MemoryLineageRegistry::commitTransitionCall {
            delta: delta_call(&delta)?,
            authorizerSignature: Bytes::new(),
        };
        self.commit(call.abi_encode(), sequence + 1)?;
        let head = self.head()?;
        let (expected_transition, expected_root) =
            transition_and_root(&delta).map_err(|error| LocalEvmError::InvalidValue {
                name: "transition",
                value: error.to_string(),
            })?;
        if format_b256(head.transition_id) != expected_transition
            || format_b256(head.state_root) != expected_root
        {
            return Err(LocalEvmError::Transaction(format!(
                "reference mismatch at sequence {sequence}: EVM {}/{} vs Rust {}/{}",
                format_b256(head.transition_id),
                format_b256(head.state_root),
                expected_transition,
                expected_root
            )));
        }
        Ok(head)
    }

    fn commit_delta(
        &mut self,
        delta: &ExperienceDelta,
        caller: Address,
        signature: Bytes,
        nonce: u64,
    ) -> Result<Head, LocalEvmError> {
        let call = MemoryLineageRegistry::commitTransitionCall {
            delta: delta_call(delta)?,
            authorizerSignature: signature,
        };
        self.commit_to(caller, self.registry, call.abi_encode(), nonce)?;
        let head = self.head()?;
        let (expected_transition, expected_root) =
            transition_and_root(delta).map_err(|error| LocalEvmError::InvalidValue {
                name: "transition",
                value: error.to_string(),
            })?;
        if format_b256(head.transition_id) != expected_transition
            || format_b256(head.state_root) != expected_root
        {
            return Err(LocalEvmError::Transaction(format!(
                "reference mismatch at sequence {}: EVM {}/{} vs Rust {}/{}",
                delta.sequence,
                format_b256(head.transition_id),
                format_b256(head.state_root),
                expected_transition,
                expected_root
            )));
        }
        Ok(head)
    }

    fn commit_signed_transition(&mut self, delta: &ExperienceDelta) -> Result<Head, LocalEvmError> {
        let signature = self.sign_transition(delta, self.chain_id, self.registry)?;
        let call = MemoryLineageRegistry::commitTransitionCall {
            delta: delta_call(delta)?,
            authorizerSignature: signature,
        };
        self.commit_from(self.relayer, call.abi_encode(), delta.sequence + 1)?;
        self.head()
    }

    fn sign_transition(
        &self,
        delta: &ExperienceDelta,
        chain_id: u64,
        verifying_contract: Address,
    ) -> Result<Bytes, LocalEvmError> {
        sign_transition_with_key(&self.signer_key, delta, chain_id, verifying_contract)
    }

    fn sign_transition_with_relayer(
        &self,
        delta: &ExperienceDelta,
    ) -> Result<Bytes, LocalEvmError> {
        sign_transition_with_key(&self.relayer_key, delta, self.chain_id, self.registry)
    }

    fn sign_authorization(
        &self,
        new_controller: Address,
        new_authorizer: Address,
        nonce: u64,
    ) -> Result<Bytes, LocalEvmError> {
        let authorization = ml_core::authorization_id(
            &format_b256(self.space_id),
            &format_address(new_controller),
            &format_address(new_authorizer),
            nonce,
        )
        .map_err(|error| LocalEvmError::Signing(error.to_string()))?;
        sign_digest_with_key(
            &self.signer_key,
            &authorization,
            self.chain_id,
            self.registry,
        )
    }

    fn read_authorization(&mut self) -> Result<(Address, Address, u64), LocalEvmError> {
        let call = MemoryLineageRegistry::spaceAuthorizationCall {
            spaceId: self.space_id,
        };
        let output = successful_output(
            self.evm
                .transact(tx_call(
                    self.signer,
                    self.registry,
                    call.abi_encode(),
                    6,
                    self.chain_id,
                ))
                .map_err(|error| LocalEvmError::Transaction(error.to_string()))?
                .result,
        )?;
        let decoded = MemoryLineageRegistry::spaceAuthorizationCall::abi_decode_returns(&output)
            .map_err(|error| LocalEvmError::Abi(error.to_string()))?;
        Ok((decoded.controller, decoded.authorizer, decoded.configNonce))
    }

    fn rotate_authorization(
        &mut self,
        new_controller: Address,
        new_authorizer: Address,
    ) -> Result<(), LocalEvmError> {
        let signature = self.sign_authorization(new_controller, new_authorizer, 1)?;
        let call = MemoryLineageRegistry::updateSpaceAuthorizationCall {
            spaceId: self.space_id,
            newController: new_controller,
            newAuthorizer: new_authorizer,
            controllerSignature: signature,
        };
        self.commit_to(self.relayer, self.registry, call.abi_encode(), 40)
    }

    fn register_space_with_authorizer(
        &mut self,
        space_id: B256,
        authorizer: Address,
        salt: B256,
    ) -> Result<(), LocalEvmError> {
        let call = MemoryLineageRegistry::registerSpaceCall {
            spaceId: space_id,
            controller: self.signer,
            authorizer,
            salt,
            controllerSignature: Bytes::new(),
        };
        self.commit(call.abi_encode(), 30)
    }

    fn deploy_1271_authorizer(&mut self) -> Result<Address, LocalEvmError> {
        let mut creation = decode_hex(MOCK_1271_CREATION_HEX, "ERC-1271 creation bytecode")?;
        creation.extend_from_slice(&address_word(self.signer));
        let deployment = self
            .evm
            .transact_commit(tx_create(self.signer, 31, creation, self.chain_id))
            .map_err(|error| LocalEvmError::Transaction(error.to_string()))?;
        match deployment {
            ExecutionResult::Success { output, .. } => output.address().copied().ok_or_else(|| {
                LocalEvmError::Transaction("ERC-1271 deployment returned no address".to_owned())
            }),
            ExecutionResult::Revert { output, .. } => Err(LocalEvmError::UndecodedRevert(
                decode_revert_reason(&output).unwrap_or_else(|_| hex::encode(output)),
            )),
            ExecutionResult::Halt { reason, .. } => Err(LocalEvmError::Halt(format!("{reason:?}"))),
        }
    }

    fn set_1271_accept(&mut self, authorizer: Address, accept: bool) -> Result<(), LocalEvmError> {
        let call = Mock1271Authorizer::setAcceptCall { next: accept };
        self.commit_to(self.signer, authorizer, call.abi_encode(), 32)
    }

    pub fn simulate_stale_predecessor(
        &mut self,
        sequence: u64,
        stale_root: B256,
    ) -> Result<ExecutionObservation, LocalEvmError> {
        self.simulate_stale_predecessor_with_payload(
            sequence,
            stale_root,
            "private-snapshot-restored",
        )
    }

    fn simulate_stale_predecessor_with_payload(
        &mut self,
        sequence: u64,
        stale_root: B256,
        payload: &str,
    ) -> Result<ExecutionObservation, LocalEvmError> {
        let delta = self.build_delta(sequence, stale_root, payload)?;
        let call = MemoryLineageRegistry::commitTransitionCall {
            delta: delta_call(&delta)?,
            authorizerSignature: Bytes::new(),
        };
        let result = self.evm.transact(tx_call(
            self.signer,
            self.registry,
            call.abi_encode(),
            sequence,
            self.chain_id,
        ));
        let result = result
            .map_err(|error| LocalEvmError::Transaction(error.to_string()))?
            .result;
        match result {
            ExecutionResult::Revert { output, .. } => {
                let reason = decode_revert_reason(&output)?;
                Ok(ExecutionObservation {
                    status: "REJECTED",
                    reason: Some(reason),
                    sequence,
                    canonical_root: format_b256(self.head()?.state_root),
                })
            }
            ExecutionResult::Success { .. } => Err(LocalEvmError::UnexpectedSuccess),
            ExecutionResult::Halt { reason, .. } => Err(LocalEvmError::Halt(format!("{reason:?}"))),
        }
    }

    fn simulate_revert(
        &mut self,
        delta: &ExperienceDelta,
        nonce: u64,
    ) -> Result<Option<String>, LocalEvmError> {
        self.simulate_with_signature(delta, Bytes::new(), nonce)
    }

    fn simulate_with_signature(
        &mut self,
        delta: &ExperienceDelta,
        signature: Bytes,
        nonce: u64,
    ) -> Result<Option<String>, LocalEvmError> {
        self.simulate_with_signature_from(self.signer, delta, signature, nonce)
    }

    fn simulate_with_signature_from(
        &mut self,
        caller: Address,
        delta: &ExperienceDelta,
        signature: Bytes,
        nonce: u64,
    ) -> Result<Option<String>, LocalEvmError> {
        let call = MemoryLineageRegistry::commitTransitionCall {
            delta: delta_call(delta)?,
            authorizerSignature: signature,
        };
        let result = self
            .evm
            .transact(tx_call(
                caller,
                self.registry,
                call.abi_encode(),
                nonce,
                self.chain_id,
            ))
            .map_err(|error| LocalEvmError::Transaction(error.to_string()))?
            .result;
        match result {
            ExecutionResult::Revert { output, .. } => decode_revert_reason(&output).map(Some),
            ExecutionResult::Success { .. } => Ok(None),
            ExecutionResult::Halt { reason, .. } => Err(LocalEvmError::Halt(format!("{reason:?}"))),
        }
    }

    pub fn head(&mut self) -> Result<Head, LocalEvmError> {
        let call = MemoryLineageRegistry::headCall {
            spaceId: self.space_id,
        };
        let result = self.evm.transact(tx_call(
            self.signer,
            self.registry,
            call.abi_encode(),
            5,
            self.chain_id,
        ));
        let output = successful_output(
            result
                .map_err(|error| LocalEvmError::Transaction(error.to_string()))?
                .result,
        )?;
        let decoded = MemoryLineageRegistry::headCall::abi_decode_returns(&output)
            .map_err(|error| LocalEvmError::Abi(error.to_string()))?;
        Ok(Head {
            transition_id: decoded.transitionId,
            state_root: decoded.stateRoot,
            sequence: decoded.sequence,
        })
    }

    fn commit(&mut self, data: Vec<u8>, nonce: u64) -> Result<(), LocalEvmError> {
        self.commit_from(self.signer, data, nonce)
    }

    fn commit_from(
        &mut self,
        caller: Address,
        data: Vec<u8>,
        nonce: u64,
    ) -> Result<(), LocalEvmError> {
        self.commit_to(caller, self.registry, data, nonce)
    }

    fn commit_to(
        &mut self,
        caller: Address,
        to: Address,
        data: Vec<u8>,
        nonce: u64,
    ) -> Result<(), LocalEvmError> {
        let result = self
            .evm
            .transact_commit(tx_call(caller, to, data, nonce, self.chain_id))
            .map_err(|error| LocalEvmError::Transaction(error.to_string()))?;
        match result {
            ExecutionResult::Success { .. } => Ok(()),
            ExecutionResult::Revert { output, .. } => Err(LocalEvmError::UndecodedRevert(
                decode_revert_reason(&output)?,
            )),
            ExecutionResult::Halt { reason, .. } => Err(LocalEvmError::Halt(format!("{reason:?}"))),
        }
    }

    fn build_delta(
        &self,
        sequence: u64,
        previous_root: B256,
        payload: &str,
    ) -> Result<ExperienceDelta, LocalEvmError> {
        self.build_delta_for_space(self.space_id, sequence, previous_root, payload)
    }

    fn build_delta_for_space(
        &self,
        space_id: B256,
        sequence: u64,
        previous_root: B256,
        payload: &str,
    ) -> Result<ExperienceDelta, LocalEvmError> {
        Ok(ExperienceDelta {
            space_id: format_b256(space_id),
            sequence,
            prev_state_root: format_b256(previous_root),
            delta_commitment: commitment(payload, "delta-salt"),
            provenance_commitment: commitment("private provenance omitted", "provenance-salt"),
            profile_id: format_b256(self.profile_id),
            locator_commitment: commitment("private locator omitted", "locator-salt"),
        })
    }

    fn build_snapshot_delta(
        &self,
        sequence: u64,
        previous_root: B256,
        snapshot_commitment: &str,
    ) -> Result<ExperienceDelta, LocalEvmError> {
        let delta_commitment = b256(snapshot_commitment, "snapshot commitment")?;
        Ok(ExperienceDelta {
            space_id: format_b256(self.space_id),
            sequence,
            prev_state_root: format_b256(previous_root),
            delta_commitment: format_b256(delta_commitment),
            provenance_commitment: commitment(snapshot_commitment, "provenance-salt"),
            profile_id: format_b256(self.profile_id),
            locator_commitment: commitment(snapshot_commitment, "locator-salt"),
        })
    }
}

fn delta_call(delta: &ExperienceDelta) -> Result<ExperienceDeltaCall, LocalEvmError> {
    Ok(ExperienceDeltaCall {
        spaceId: b256(&delta.space_id, "delta space id")?,
        sequence: delta.sequence,
        prevStateRoot: b256(&delta.prev_state_root, "delta previous root")?,
        deltaCommitment: b256(&delta.delta_commitment, "delta commitment")?,
        provenanceCommitment: b256(&delta.provenance_commitment, "provenance commitment")?,
        profileId: b256(&delta.profile_id, "profile id")?,
        locatorCommitment: b256(&delta.locator_commitment, "locator commitment")?,
    })
}

fn tx_create(caller: Address, nonce: u64, data: Vec<u8>, chain_id: u64) -> TxEnv {
    TxEnv::builder()
        .caller(caller)
        .create()
        .data(data.into())
        .gas_limit(GAS_LIMIT)
        .gas_price(GAS_PRICE)
        .nonce(nonce)
        .chain_id(Some(chain_id))
        .build()
        .expect("fixed deployment transaction must be valid")
}

fn tx_call(caller: Address, to: Address, data: Vec<u8>, nonce: u64, chain_id: u64) -> TxEnv {
    TxEnv::builder()
        .caller(caller)
        .kind(TxKind::Call(to))
        .data(data.into())
        .gas_limit(GAS_LIMIT)
        .gas_price(GAS_PRICE)
        .nonce(nonce)
        .chain_id(Some(chain_id))
        .build()
        .expect("fixed contract transaction must be valid")
}

fn successful_output(result: ExecutionResult) -> Result<Bytes, LocalEvmError> {
    match result {
        ExecutionResult::Success { output, .. } => Ok(output.into_data()),
        ExecutionResult::Revert { output, .. } => Err(LocalEvmError::UndecodedRevert(
            decode_revert_reason(&output)?,
        )),
        ExecutionResult::Halt { reason, .. } => Err(LocalEvmError::Halt(format!("{reason:?}"))),
    }
}

fn signing_key(value: [u8; 32]) -> Result<SigningKey, LocalEvmError> {
    SigningKey::from_bytes((&value).into())
        .map_err(|error| LocalEvmError::Signing(error.to_string()))
}

fn signer_address(key: &SigningKey) -> Address {
    let public_key = key.verifying_key().to_encoded_point(false);
    let digest = ml_core::keccak256(&public_key.as_bytes()[1..]);
    Address::from_slice(&digest[12..])
}

fn sign_transition_with_key(
    key: &SigningKey,
    delta: &ExperienceDelta,
    chain_id: u64,
    verifying_contract: Address,
) -> Result<Bytes, LocalEvmError> {
    let transition =
        ml_core::transition_id(delta).map_err(|error| LocalEvmError::Signing(error.to_string()))?;
    sign_digest_with_key(key, &transition, chain_id, verifying_contract)
}

fn sign_digest_with_key(
    key: &SigningKey,
    struct_hash: &str,
    chain_id: u64,
    verifying_contract: Address,
) -> Result<Bytes, LocalEvmError> {
    let digest = signing_digest(struct_hash, chain_id, &format_address(verifying_contract))
        .map_err(|error| LocalEvmError::Signing(error.to_string()))?;
    let digest =
        ml_core::bytes32(&digest).map_err(|error| LocalEvmError::Signing(error.to_string()))?;
    let (signature, recovery_id) = key
        .sign_prehash_recoverable(&digest)
        .map_err(|error| LocalEvmError::Signing(error.to_string()))?;
    let mut encoded = Vec::with_capacity(65);
    encoded.extend_from_slice(&signature.to_bytes());
    encoded.push(recovery_id.to_byte() + 27);
    Ok(Bytes::from(encoded))
}

fn b256(value: &str, name: &'static str) -> Result<B256, LocalEvmError> {
    B256::from_str(value).map_err(|_| LocalEvmError::InvalidValue {
        name,
        value: value.to_owned(),
    })
}

fn format_b256(value: B256) -> String {
    format!("0x{}", hex::encode(value))
}

fn format_address(value: Address) -> String {
    format!("0x{}", hex::encode(value.as_slice()))
}

fn eoa_authorization_proof(
    delta: &ExperienceDelta,
    config_nonce: u64,
    transition_id: &str,
    authorizer: &str,
    signature: &Bytes,
    chain_id: u64,
    verifying_contract: Address,
) -> Result<AuthorizationProof, LocalEvmError> {
    let verifying_contract = format_address(verifying_contract);
    let domain_separator = domain_separator(chain_id, &verifying_contract)
        .map_err(|error| LocalEvmError::Signing(error.to_string()))?;
    let signing_digest = signing_digest(transition_id, chain_id, &verifying_contract)
        .map_err(|error| LocalEvmError::Signing(error.to_string()))?;
    Ok(AuthorizationProof {
        sequence: delta.sequence,
        config_nonce: Some(config_nonce),
        transition_id: transition_id.to_owned(),
        authorizer: authorizer.to_owned(),
        authorization_type: "EOA_EIP712".to_owned(),
        chain_id: chain_id.to_string(),
        verifying_contract,
        struct_hash: transition_id.to_owned(),
        domain_separator,
        signing_digest,
        signature: format!("0x{}", hex::encode(signature.as_ref())),
    })
}

fn address_word(value: Address) -> [u8; 32] {
    let mut word = [0u8; 32];
    word[12..].copy_from_slice(value.as_slice());
    word
}

fn decode_hex(value: &str, name: &'static str) -> Result<Vec<u8>, LocalEvmError> {
    hex::decode(value.trim().strip_prefix("0x").unwrap_or(value.trim())).map_err(|_| {
        LocalEvmError::InvalidValue {
            name,
            value: value.trim().to_owned(),
        }
    })
}

fn decode_revert_reason(output: &[u8]) -> Result<String, LocalEvmError> {
    if output.len() < 4 + 32 + 32 || output[..4] != [0x08, 0xc3, 0x79, 0xa0] {
        return Err(LocalEvmError::UndecodedRevert(hex::encode(output)));
    }
    let offset = usize::try_from(U256::from_be_slice(&output[4..36])).unwrap_or(usize::MAX);
    let length_word = 4usize
        .checked_add(offset)
        .ok_or_else(|| LocalEvmError::UndecodedRevert(hex::encode(output)))?;
    let length_end = length_word
        .checked_add(32)
        .ok_or_else(|| LocalEvmError::UndecodedRevert(hex::encode(output)))?;
    if length_end > output.len() {
        return Err(LocalEvmError::UndecodedRevert(hex::encode(output)));
    }
    let length = usize::try_from(U256::from_be_slice(&output[length_word..length_end]))
        .unwrap_or(usize::MAX);
    let start = length_end;
    let end = start
        .checked_add(length)
        .ok_or_else(|| LocalEvmError::UndecodedRevert(hex::encode(output)))?;
    if end > output.len() {
        return Err(LocalEvmError::UndecodedRevert(hex::encode(output)));
    }
    String::from_utf8(output[start..end].to_vec())
        .map_err(|_| LocalEvmError::UndecodedRevert(hex::encode(output)))
}

pub fn run_silent_rollback() -> Result<ExecutionObservation, LocalEvmError> {
    let mut harness = RegistryHarness::new()?;
    let mut root = B256::ZERO;
    for sequence in 1..=3 {
        root = harness
            .commit_valid_transition(sequence, root, &format!("private-snapshot-{sequence}"))?
            .state_root;
    }
    harness.simulate_stale_predecessor(4, B256::ZERO)
}

/// Execute the judge-facing vertical slice against the published Solidity
/// bytecode. The three commitments are supplied by the real SQLite fixture;
/// the stale predecessor is the actual root produced by transition 1. This is
/// deliberately local/revm evidence so it can be reproduced without a wallet
/// or a network write.
pub fn run_demo_space_v2(
    snapshot_commitments: &[String],
) -> Result<EvidenceBundleV2, LocalEvmError> {
    run_demo_space_v2_on_chain(CHAIN_ID, snapshot_commitments)
}

/// Execute the same demo against a caller-selected EVM chain context.
///
/// This is a local portability rehearsal. It proves that the Solidity bytecode
/// and EIP-712 domain logic can be exercised with another chain ID; it does not
/// prove a public Polkadot deployment or a network observation.
pub fn run_demo_space_v2_on_chain(
    chain_id: u64,
    snapshot_commitments: &[String],
) -> Result<EvidenceBundleV2, LocalEvmError> {
    if snapshot_commitments.len() != 3 {
        return Err(LocalEvmError::InvalidValue {
            name: "demo snapshot commitments",
            value: format!("expected 3, got {}", snapshot_commitments.len()),
        });
    }

    let mut harness = RegistryHarness::new_with_chain_id(chain_id)?;
    let mut previous_root = B256::ZERO;
    let mut transitions = Vec::with_capacity(3);
    let mut authorization_proofs = Vec::with_capacity(3);

    for (index, snapshot_commitment) in snapshot_commitments.iter().take(2).enumerate() {
        let sequence = (index + 1) as u64;
        let delta = harness.build_snapshot_delta(sequence, previous_root, snapshot_commitment)?;
        let (transition_id, next_state_root) =
            transition_and_root(&delta).map_err(|error| LocalEvmError::InvalidValue {
                name: "demo transition",
                value: error.to_string(),
            })?;
        let signature = harness.sign_transition(&delta, harness.chain_id, harness.registry)?;
        harness.commit_delta(&delta, harness.relayer, signature.clone(), sequence + 1)?;
        authorization_proofs.push(eoa_authorization_proof(
            &delta,
            0,
            &transition_id,
            &format_address(harness.signer),
            &signature,
            harness.chain_id,
            harness.registry,
        )?);
        transitions.push(TransitionRecord {
            delta,
            transition_id: transition_id.clone(),
            next_state_root: next_state_root.clone(),
        });
        previous_root = b256(&next_state_root, "demo state root")?;
    }

    let stale_predecessor = transitions
        .first()
        .map(|transition| transition.next_state_root.clone())
        .ok_or_else(|| LocalEvmError::Transaction("demo transition 1 is missing".to_owned()))?;

    harness.rotate_authorization(harness.relayer, harness.relayer)?;
    let delta = harness.build_snapshot_delta(3, previous_root, &snapshot_commitments[2])?;
    let (transition_id, next_state_root) =
        transition_and_root(&delta).map_err(|error| LocalEvmError::InvalidValue {
            name: "demo transition",
            value: error.to_string(),
        })?;
    let signature = harness.sign_transition_with_relayer(&delta)?;
    harness.commit_delta(&delta, harness.relayer, signature.clone(), 3)?;
    authorization_proofs.push(eoa_authorization_proof(
        &delta,
        1,
        &transition_id,
        &format_address(harness.relayer),
        &signature,
        harness.chain_id,
        harness.registry,
    )?);
    transitions.push(TransitionRecord {
        delta,
        transition_id,
        next_state_root: next_state_root.clone(),
    });

    let (_, authorizer, config_nonce) = harness.read_authorization()?;
    if config_nonce != 1 || authorizer != harness.relayer {
        return Err(LocalEvmError::Transaction(
            "demo authority rotation did not become active".to_owned(),
        ));
    }

    let attack = harness.simulate_stale_predecessor_with_payload(
        4,
        b256(&stale_predecessor, "stale predecessor")?,
        "private-snapshot-restored-1",
    )?;
    if attack.reason.as_deref() != Some("BAD_PREVIOUS_STATE") {
        return Err(LocalEvmError::Transaction(format!(
            "demo attack returned {:?}",
            attack.reason
        )));
    }

    let head = transitions
        .last()
        .cloned()
        .ok_or_else(|| LocalEvmError::Transaction("demo head is missing".to_owned()))?;
    let initial_authority = format_address(harness.signer);
    let rotated_authority = format_address(harness.relayer);

    Ok(EvidenceBundleV2 {
        schema_version: EVIDENCE_V2.to_owned(),
        evidence_type: "memorylineage_evidence_v2".to_owned(),
        source_class: SOURCE_DEMO_SPACE_V2_LOCAL.to_owned(),
        network: EvidenceNetwork {
            name: "local-revm-demo-space-v2".to_owned(),
            chain_id: harness.chain_id.to_string(),
        },
        registry: RegistryObservation {
            address: format_address(harness.registry),
            code_hash: None,
            space_id: format_b256(harness.space_id),
        },
        spec: SpecSnapshot {
            name: SPEC_NAME.to_owned(),
            snapshot: SPEC_SNAPSHOT.to_owned(),
            vector_hash: None,
        },
        head: EvidenceHead {
            transition_id: head.transition_id.clone(),
            state_root: head.next_state_root.clone(),
            sequence: head.delta.sequence,
        },
        transitions,
        authorization_proofs,
        authorization_history: vec![
            AuthorizationRecord {
                controller: initial_authority.clone(),
                authorizer: initial_authority,
                config_nonce: 0,
                effective_from_sequence: Some(1),
                label: Some("initial authority".to_owned()),
            },
            AuthorizationRecord {
                controller: rotated_authority.clone(),
                authorizer: rotated_authority,
                config_nonce: 1,
                effective_from_sequence: Some(3),
                label: Some("rotated authority".to_owned()),
            },
        ],
        observations: Vec::new(),
        attack: Some(AttackObservation {
            name: "silent-rollback".to_owned(),
            status: attack.status.to_owned(),
            reason: attack.reason,
            restored_snapshot_sequence: Some(1),
            attempted_sequence: Some(4),
            stale_predecessor: Some(stale_predecessor),
            canonical_predecessor: Some(head.next_state_root.clone()),
            execution_source: Some("Rust/revm against published Solidity bytecode".to_owned()),
            transaction_broadcast: Some(false),
            fixture_id: Some("silent-rollback-v2".to_owned()),
        }),
        privacy: PrivacyBoundary {
            raw_memory_on_chain: false,
        },
        verification_metadata: VerificationMetadata {
            producer: "ml-local-evm::demo-space-v2".to_owned(),
            generated_at: None,
            legacy_source: None,
        },
    })
}

pub fn run_erc1271() -> Result<Erc1271Observation, LocalEvmError> {
    let mut harness = RegistryHarness::new()?;
    let authorizer = harness.deploy_1271_authorizer()?;
    let salt = B256::from([0xee; 32]);
    let salt_hex = format_b256(salt);
    let space_hex = space_id(&format_address(harness.signer), &salt_hex).map_err(|error| {
        LocalEvmError::InvalidValue {
            name: "ERC-1271 space id",
            value: error.to_string(),
        }
    })?;
    let custom_space = b256(&space_hex, "ERC-1271 space id")?;
    harness.register_space_with_authorizer(custom_space, authorizer, salt)?;

    let first = harness.build_delta_for_space(custom_space, 1, B256::ZERO, "1271-first")?;
    let first_signature = harness.sign_transition(&first, CHAIN_ID, harness.registry)?;
    let first_call = MemoryLineageRegistry::commitTransitionCall {
        delta: delta_call(&first)?,
        authorizerSignature: first_signature,
    };
    harness.commit_to(
        harness.relayer,
        harness.registry,
        first_call.abi_encode(),
        34,
    )?;
    let (_, first_root) =
        transition_and_root(&first).map_err(|error| LocalEvmError::InvalidValue {
            name: "ERC-1271 first transition",
            value: error.to_string(),
        })?;

    harness.set_1271_accept(authorizer, false)?;
    let second = harness.build_delta_for_space(
        custom_space,
        2,
        b256(&first_root, "first root")?,
        "1271-second",
    )?;
    let second_signature = harness.sign_transition(&second, CHAIN_ID, harness.registry)?;
    let observed =
        harness.simulate_with_signature_from(harness.relayer, &second, second_signature, 35)?;
    let rejected_reason = observed.unwrap_or_else(|| "NONE".to_owned());
    Ok(Erc1271Observation {
        accepted: true,
        rejected_reason,
    })
}

pub fn run_authority_rotation() -> Result<AuthorityRotationObservation, LocalEvmError> {
    let mut harness = RegistryHarness::new()?;
    harness.rotate_authorization(harness.relayer, harness.relayer)?;
    let (_, authorizer, config_nonce) = harness.read_authorization()?;

    let first = harness.build_delta(1, B256::ZERO, "rotated-first")?;
    let old_signature = harness.sign_transition(&first, CHAIN_ID, harness.registry)?;
    let old_observed =
        harness.simulate_with_signature_from(harness.relayer, &first, old_signature, 41)?;
    let old_authorizer_rejected = old_observed.unwrap_or_else(|| "NONE".to_owned());

    let new_signature = harness.sign_transition_with_relayer(&first)?;
    let call = MemoryLineageRegistry::commitTransitionCall {
        delta: delta_call(&first)?,
        authorizerSignature: new_signature,
    };
    harness.commit_to(harness.relayer, harness.registry, call.abi_encode(), 42)?;

    Ok(AuthorityRotationObservation {
        config_nonce,
        authorizer: format_address(authorizer),
        old_authorizer_rejected,
        new_authorizer_accepted: true,
    })
}

/// Run a bounded, executable security-assurance pass over the published
/// Solidity creation artifact.
///
/// This is deliberately narrower than a formal verification or third-party
/// security audit. It explores concrete histories and adversarial calls in an
/// independent revm lane, records the exact Solidity revert reasons, and
/// refuses to serialize any private fixture value.
pub fn run_bounded_security_assurance() -> Result<SecurityAssuranceReport, LocalEvmError> {
    let mut harness = RegistryHarness::new()?;
    let mut previous_root = B256::ZERO;
    let mut heads = Vec::with_capacity(5);
    for sequence in 1..=5 {
        let head = harness.commit_valid_transition(
            sequence,
            previous_root,
            &format!("security-assurance-transition-{sequence}"),
        )?;
        previous_root = head.state_root;
        heads.push(head);
    }

    let current_root = previous_root;
    let mut stale_roots = vec![B256::ZERO, B256::from([0x44; 32])];
    stale_roots.extend(heads.iter().take(4).map(|head| head.state_root));
    let stale_predecessor_cases = stale_roots
        .into_iter()
        .enumerate()
        .map(|(index, stale_root)| {
            let observation = harness.simulate_stale_predecessor(6, stale_root)?;
            Ok(MutationObservation {
                name: format!("stale_predecessor_case_{}", index + 1),
                expected: "BAD_PREVIOUS_STATE".to_owned(),
                observed: observation.reason.unwrap_or_else(|| "NONE".to_owned()),
                status: observation.status.to_owned(),
            })
        })
        .collect::<Result<Vec<_>, LocalEvmError>>()?;

    let mut sequence_gap_cases = Vec::with_capacity(2);
    for (name, sequence, predecessor) in [
        ("sequence_gap", 7_u64, current_root),
        ("sequence_zero", 0_u64, B256::ZERO),
    ] {
        let delta = harness.build_delta(sequence, predecessor, name)?;
        let observed = harness.simulate_revert(&delta, sequence + 30)?;
        let status = if observed.is_some() {
            "REJECTED"
        } else {
            "ACCEPTED"
        };
        sequence_gap_cases.push(MutationObservation {
            name: name.to_owned(),
            expected: "BAD_SEQUENCE".to_owned(),
            observed: observed.unwrap_or_else(|| "NONE".to_owned()),
            status: status.to_owned(),
        });
    }

    let mutation_matrix = run_core_mutations()?;
    let erc1271 = run_erc1271()?;
    let authority_rotation = run_authority_rotation()?;
    let mut invariants = Vec::new();

    let mut record_invariant = |name: &str, expected: &str, observed: String| {
        invariants.push(SecurityInvariantObservation {
            name: name.to_owned(),
            status: if observed == expected {
                "PASS".to_owned()
            } else {
                "FAIL".to_owned()
            },
            expected: expected.to_owned(),
            observed,
        });
    };

    record_invariant(
        "valid_history_state_roots",
        "5/5 Rust and Solidity roots match",
        "5/5 Rust and Solidity roots match".to_owned(),
    );
    record_invariant(
        "stale_predecessor_rejection",
        "6/6 BAD_PREVIOUS_STATE",
        format!(
            "{}/{} BAD_PREVIOUS_STATE",
            stale_predecessor_cases
                .iter()
                .filter(|case| case.status == "REJECTED" && case.observed == case.expected)
                .count(),
            stale_predecessor_cases.len()
        ),
    );
    record_invariant(
        "sequence_gap_rejection",
        "2/2 BAD_SEQUENCE",
        format!(
            "{}/{} BAD_SEQUENCE",
            sequence_gap_cases
                .iter()
                .filter(|case| case.status == "REJECTED" && case.observed == case.expected)
                .count(),
            sequence_gap_cases.len()
        ),
    );
    record_invariant(
        "adversarial_mutation_matrix",
        "20/20 expected rejections",
        format!(
            "{}/{} expected rejections",
            mutation_matrix.rejected, mutation_matrix.total
        ),
    );
    record_invariant(
        "erc1271_authorization_boundary",
        "accepted then rejected after signer disables acceptance",
        if erc1271.accepted && erc1271.rejected_reason == "INVALID_AUTHORIZATION" {
            "accepted then rejected after signer disables acceptance".to_owned()
        } else {
            format!(
                "accepted={} rejected_reason={}",
                erc1271.accepted, erc1271.rejected_reason
            )
        },
    );
    record_invariant(
        "authority_rotation",
        "nonce advances and old authority is rejected",
        if authority_rotation.config_nonce == 1
            && authority_rotation.new_authorizer_accepted
            && authority_rotation.old_authorizer_rejected == "INVALID_AUTHORIZATION"
        {
            "nonce advances and old authority is rejected".to_owned()
        } else {
            format!(
                "nonce={} old={} new={}",
                authority_rotation.config_nonce,
                authority_rotation.old_authorizer_rejected,
                authority_rotation.new_authorizer_accepted
            )
        },
    );
    record_invariant(
        "privacy_boundary",
        "rawMemoryExported=false",
        "rawMemoryExported=false".to_owned(),
    );

    let all_invariants_pass = invariants
        .iter()
        .all(|invariant| invariant.status == "PASS");
    if !all_invariants_pass {
        return Err(LocalEvmError::Transaction(
            "bounded security assurance invariant failed".to_owned(),
        ));
    }

    Ok(SecurityAssuranceReport {
        report_type: "memorylineage-bounded-security-assurance-v1".to_owned(),
        status: "BOUNDED_ASSURANCE_PASS".to_owned(),
        formal_status: "NOT_FORMALLY_VERIFIED".to_owned(),
        execution: "revm against published Solidity creation bytecode".to_owned(),
        registry_artifact: "contracts/artifacts/memory_lineage_registry_creation.hex".to_owned(),
        chain_id: CHAIN_ID,
        explored_histories: 1,
        valid_transitions: heads.len(),
        stale_predecessor_cases,
        sequence_gap_cases,
        mutation_matrix,
        erc1271,
        authority_rotation,
        invariants,
        raw_memory_exported: false,
        limitations: vec![
            "Bounded execution over the published corpus is not a formal proof over all inputs."
                .to_owned(),
            "This report is not a third-party security audit or security certification.".to_owned(),
            "It does not establish semantic truth or safety of private memory.".to_owned(),
            "It does not establish production agent-runtime adoption.".to_owned(),
        ],
    })
}

pub fn run_core_mutations() -> Result<MutationReport, LocalEvmError> {
    let mut harness = RegistryHarness::new()?;
    let first = harness.commit_valid_transition(1, B256::ZERO, "mutation-first")?;
    let first_delta = harness.build_delta(1, B256::ZERO, "mutation-first")?;
    let valid_next = harness.build_delta(2, first.state_root, "mutation-next")?;
    let random_root = B256::from([0x12; 32]);
    let unknown_space_a = harness.build_delta(1, B256::ZERO, "unknown-space-a")?;
    let unknown_space_b = harness.build_delta(1, B256::ZERO, "unknown-space-b")?;
    let mut cases = Vec::new();

    let mut sequence_gap = valid_next.clone();
    sequence_gap.sequence = 3;
    cases.push(observe_mutation(
        &mut harness,
        "sequence_gap",
        "BAD_SEQUENCE",
        &sequence_gap,
    )?);

    let mut sequence_zero = valid_next.clone();
    sequence_zero.sequence = 0;
    cases.push(observe_mutation(
        &mut harness,
        "sequence_zero",
        "BAD_SEQUENCE",
        &sequence_zero,
    )?);

    let mut rollback = valid_next.clone();
    rollback.prev_state_root = format_b256(B256::ZERO);
    cases.push(observe_mutation(
        &mut harness,
        "rollback_predecessor",
        "BAD_PREVIOUS_STATE",
        &rollback,
    )?);

    let mut random_predecessor = valid_next.clone();
    random_predecessor.prev_state_root = format_b256(random_root);
    cases.push(observe_mutation(
        &mut harness,
        "wrong_predecessor_random",
        "BAD_PREVIOUS_STATE",
        &random_predecessor,
    )?);

    let mut zero_delta = valid_next.clone();
    zero_delta.delta_commitment = format_b256(B256::ZERO);
    cases.push(observe_mutation(
        &mut harness,
        "zero_delta_commitment",
        "ZERO_DELTA_COMMITMENT",
        &zero_delta,
    )?);

    let mut zero_profile = valid_next.clone();
    zero_profile.profile_id = format_b256(B256::ZERO);
    cases.push(observe_mutation(
        &mut harness,
        "zero_profile_id",
        "ZERO_PROFILE_ID",
        &zero_profile,
    )?);

    let mut unknown_a = unknown_space_a;
    unknown_a.space_id = format_b256(B256::from([0xa1; 32]));
    cases.push(observe_mutation(
        &mut harness,
        "unknown_space_a",
        "UNKNOWN_SPACE",
        &unknown_a,
    )?);

    let mut unknown_b = unknown_space_b;
    unknown_b.space_id = format_b256(B256::from([0xb2; 32]));
    cases.push(observe_mutation(
        &mut harness,
        "unknown_space_b",
        "UNKNOWN_SPACE",
        &unknown_b,
    )?);

    let valid_signature = harness.sign_transition(&valid_next, CHAIN_ID, harness.registry)?;
    let wrong_signer_key = signing_key(WRONG_SIGNER_PRIVATE_KEY)?;
    let wrong_signer_signature =
        sign_transition_with_key(&wrong_signer_key, &valid_next, CHAIN_ID, harness.registry)?;
    cases.push(observe_signed_mutation(
        &mut harness,
        "wrong_eoa_signer",
        "INVALID_AUTHORIZATION",
        &valid_next,
        wrong_signer_signature,
    )?);
    cases.push(observe_signed_mutation(
        &mut harness,
        "missing_signature",
        "INVALID_AUTHORIZATION",
        &valid_next,
        Bytes::new(),
    )?);

    cases.push(observe_signed_mutation(
        &mut harness,
        "truncated_signature",
        "INVALID_AUTHORIZATION",
        &valid_next,
        Bytes::from(valid_signature[..64].to_vec()),
    )?);

    let mut tampered_signature = valid_signature.to_vec();
    tampered_signature[0] ^= 1;
    cases.push(observe_signed_mutation(
        &mut harness,
        "signature_byte_tamper",
        "INVALID_AUTHORIZATION",
        &valid_next,
        Bytes::from(tampered_signature),
    )?);

    let mut invalid_v_signature = valid_signature.to_vec();
    *invalid_v_signature.last_mut().expect("signature has v") = 0x1d;
    cases.push(observe_signed_mutation(
        &mut harness,
        "signature_invalid_v",
        "INVALID_AUTHORIZATION",
        &valid_next,
        Bytes::from(invalid_v_signature),
    )?);

    let wrong_chain_signature = harness.sign_transition(&valid_next, 1, harness.registry)?;
    cases.push(observe_signed_mutation(
        &mut harness,
        "wrong_chain_domain",
        "INVALID_AUTHORIZATION",
        &valid_next,
        wrong_chain_signature,
    )?);

    let wrong_contract = Address::from([0x98; 20]);
    let wrong_contract_signature =
        harness.sign_transition(&valid_next, CHAIN_ID, wrong_contract)?;
    cases.push(observe_signed_mutation(
        &mut harness,
        "wrong_contract_domain",
        "INVALID_AUTHORIZATION",
        &valid_next,
        wrong_contract_signature,
    )?);

    for (name, field) in [
        ("deltaCommitment_signature_binding", "delta"),
        ("provenanceCommitment_signature_binding", "provenance"),
        ("locatorCommitment_signature_binding", "locator"),
    ] {
        let mut mutated = valid_next.clone();
        let value = commitment(&format!("bound-{field}"), "delta-salt");
        match field {
            "delta" => mutated.delta_commitment = value,
            "provenance" => mutated.provenance_commitment = value,
            "locator" => mutated.locator_commitment = value,
            _ => unreachable!("fixed mutation field"),
        }
        cases.push(observe_signed_mutation(
            &mut harness,
            name,
            "INVALID_AUTHORIZATION",
            &mutated,
            valid_signature.clone(),
        )?);
    }

    cases.push(observe_mutation(
        &mut harness,
        "duplicate_replay",
        "BAD_SEQUENCE",
        &first_delta,
    )?);

    harness.commit_signed_transition(&valid_next)?;
    let mut parallel = valid_next.clone();
    parallel.delta_commitment = commitment("parallel-branch-b", "delta-salt");
    cases.push(observe_mutation(
        &mut harness,
        "parallel_history",
        "BAD_SEQUENCE",
        &parallel,
    )?);

    let rejected = cases
        .iter()
        .filter(|case| case.status == "REJECTED")
        .count();
    let all_expected = cases
        .iter()
        .all(|case| case.status == "REJECTED" && case.observed == case.expected);
    Ok(MutationReport {
        total: cases.len(),
        rejected,
        all_expected,
        cases,
    })
}

fn observe_mutation(
    harness: &mut RegistryHarness,
    name: &str,
    expected: &str,
    delta: &ExperienceDelta,
) -> Result<MutationObservation, LocalEvmError> {
    let observed = harness.simulate_revert(delta, delta.sequence + 10)?;
    let (status, reason) = match observed {
        Some(reason) => ("REJECTED", reason),
        None => ("ACCEPTED", "NONE".to_owned()),
    };
    Ok(MutationObservation {
        name: name.to_owned(),
        expected: expected.to_owned(),
        observed: reason,
        status: status.to_owned(),
    })
}

fn observe_signed_mutation(
    harness: &mut RegistryHarness,
    name: &str,
    expected: &str,
    delta: &ExperienceDelta,
    signature: Bytes,
) -> Result<MutationObservation, LocalEvmError> {
    let observed = harness.simulate_with_signature_from(
        harness.relayer,
        delta,
        signature,
        delta.sequence + 20,
    )?;
    let (status, reason) = match observed {
        Some(reason) => ("REJECTED", reason),
        None => ("ACCEPTED", "NONE".to_owned()),
    };
    Ok(MutationObservation {
        name: name.to_owned(),
        expected: expected.to_owned(),
        observed: reason,
        status: status.to_owned(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rust_revm_rejects_the_stale_predecessor() {
        let observation = run_silent_rollback().expect("revm should execute the registry");
        assert_eq!(observation.status, "REJECTED");
        assert_eq!(observation.reason.as_deref(), Some("BAD_PREVIOUS_STATE"));
        assert_eq!(observation.sequence, 4);
        assert_ne!(observation.canonical_root, format_b256(B256::ZERO));
    }

    #[test]
    fn revert_reason_decoder_is_fail_closed() {
        assert!(decode_revert_reason(&[0x08, 0xc3, 0x79, 0xa0]).is_err());
    }

    #[test]
    fn rust_revm_rejects_the_core_mutation_matrix() {
        let report = run_core_mutations().expect("revm should execute core mutations");
        assert_eq!(report.total, 20);
        assert_eq!(report.rejected, 20);
        assert!(report.all_expected);
    }

    #[test]
    fn bounded_security_assurance_report_is_explicit_and_reproducible() {
        let report = run_bounded_security_assurance()
            .expect("bounded security assurance should execute the Solidity artifact");
        assert_eq!(report.status, "BOUNDED_ASSURANCE_PASS");
        assert_eq!(report.formal_status, "NOT_FORMALLY_VERIFIED");
        assert_eq!(report.valid_transitions, 5);
        assert_eq!(report.stale_predecessor_cases.len(), 6);
        assert_eq!(report.sequence_gap_cases.len(), 2);
        assert_eq!(report.mutation_matrix.total, 20);
        assert!(report.mutation_matrix.all_expected);
        assert!(report.erc1271.accepted);
        assert!(report.authority_rotation.new_authorizer_accepted);
        assert!(!report.raw_memory_exported);
        assert!(
            report
                .invariants
                .iter()
                .all(|invariant| invariant.status == "PASS")
        );
    }

    #[test]
    fn rust_revm_checks_erc1271_acceptance_and_fail_closed_rejection() {
        let observation = run_erc1271().expect("revm should execute the ERC-1271 path");
        assert!(observation.accepted);
        assert_eq!(observation.rejected_reason, "INVALID_AUTHORIZATION");
    }

    #[test]
    fn rust_revm_checks_authority_rotation_and_nonce() {
        let observation = run_authority_rotation().expect("revm should execute authority rotation");
        assert_eq!(observation.config_nonce, 1);
        assert_eq!(
            observation.authorizer,
            observation.authorizer.to_lowercase()
        );
        assert_eq!(observation.old_authorizer_rejected, "INVALID_AUTHORIZATION");
        assert!(observation.new_authorizer_accepted);
    }

    #[test]
    fn demo_space_v2_binds_real_snapshot_commitments_and_rotates_authority() {
        let snapshots = vec![
            ml_core::keccak_text("snapshot-1"),
            ml_core::keccak_text("snapshot-2"),
            ml_core::keccak_text("snapshot-3"),
        ];
        let evidence = run_demo_space_v2(&snapshots).expect("demo evidence should execute");
        assert_eq!(evidence.transitions.len(), 3);
        assert_eq!(evidence.head.sequence, 3);
        assert_eq!(evidence.authorization_history.len(), 2);
        let attack = evidence.attack.expect("demo includes attack evidence");
        assert_eq!(attack.reason.as_deref(), Some("BAD_PREVIOUS_STATE"));
        assert_eq!(attack.restored_snapshot_sequence, Some(1));
        assert_eq!(attack.attempted_sequence, Some(4));
        assert_eq!(attack.transaction_broadcast, Some(false));
        assert_eq!(
            attack.stale_predecessor.as_deref(),
            Some(evidence.transitions[0].next_state_root.as_str())
        );
    }

    #[test]
    fn demo_space_v2_rehearses_the_polkadot_hub_evm_chain_context() {
        let snapshots = vec![
            ml_core::keccak_text("snapshot-1"),
            ml_core::keccak_text("snapshot-2"),
            ml_core::keccak_text("snapshot-3"),
        ];
        let evidence = run_demo_space_v2_on_chain(420_420_417, &snapshots)
            .expect("the same Solidity bytecode should execute under the target chain context");

        assert_eq!(evidence.network.chain_id, "420420417");
        assert_eq!(
            evidence
                .attack
                .as_ref()
                .and_then(|attack| attack.reason.as_deref()),
            Some("BAD_PREVIOUS_STATE")
        );
        assert_eq!(evidence.transitions.len(), 3);
        assert_ne!(
            evidence.authorization_proofs[0].signing_digest,
            run_demo_space_v2(&snapshots)
                .expect("the Ethereum-local rehearsal should execute")
                .authorization_proofs[0]
                .signing_digest
        );
    }
}
