#![forbid(unsafe_code)]

use alloy::primitives::{Address, B256, Bytes};
#[cfg(not(target_arch = "wasm32"))]
use alloy::providers::{Provider, ProviderBuilder};
#[cfg(not(target_arch = "wasm32"))]
use alloy::rpc::types::TransactionRequest;
use alloy::sol;
use alloy::sol_types::SolCall;
use std::str::FromStr;
use thiserror::Error;

sol! {
    struct ExperienceDelta {
        bytes32 spaceId;
        uint64 sequence;
        bytes32 prevStateRoot;
        bytes32 deltaCommitment;
        bytes32 provenanceCommitment;
        bytes32 profileId;
        bytes32 locatorCommitment;
    }

    interface MemoryLineageRegistry {
        function head(bytes32 spaceId)
            external
            view
            returns (bytes32 transitionId, bytes32 stateRoot, uint64 sequence);
        function spaceAuthorization(bytes32 spaceId)
            external
            view
            returns (address controller, address authorizer, uint64 configNonce);
        function commitTransition(ExperienceDelta calldata delta, bytes calldata authorizerSignature)
            external
            returns (bytes32 transitionId, bytes32 nextStateRoot);
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RegistryHead {
    pub transition_id: B256,
    pub state_root: B256,
    pub sequence: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RegistryAuthority {
    pub controller: Address,
    pub authorizer: Address,
    pub config_nonce: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LiveInspection {
    pub block_number: u64,
    pub head: RegistryHead,
    pub authority: RegistryAuthority,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RollbackSimulation {
    pub status: String,
    pub reason: String,
    pub sequence: u64,
    pub canonical_root: B256,
}

#[derive(Debug, Error)]
pub enum EthereumError {
    #[error("invalid Ethereum address: {0}")]
    InvalidAddress(String),
    #[error("invalid bytes32 value: {0}")]
    InvalidBytes32(String),
    #[error("invalid RPC URL: {0}")]
    InvalidRpcUrl(String),
    #[error("RPC error: {0}")]
    Rpc(String),
    #[error("rollback simulation unexpectedly succeeded")]
    UnexpectedSuccess,
    #[error("rollback simulation reverted without BAD_PREVIOUS_STATE: {0}")]
    UnexpectedRevert(String),
}

#[cfg(not(target_arch = "wasm32"))]
fn address(value: &str) -> Result<Address, EthereumError> {
    Address::from_str(value).map_err(|_| EthereumError::InvalidAddress(value.to_owned()))
}

fn bytes32(value: &str) -> Result<B256, EthereumError> {
    B256::from_str(value).map_err(|_| EthereumError::InvalidBytes32(value.to_owned()))
}

/// ABI-encodes the read-only `head(bytes32)` call for browser transports.
/// The browser uses this payload with `eth_call`; it does not need a wallet.
pub fn head_call_data(space_id: &str) -> Result<String, EthereumError> {
    let call = MemoryLineageRegistry::headCall {
        spaceId: bytes32(space_id)?,
    };
    Ok(format!("0x{}", hex::encode(call.abi_encode())))
}

/// ABI-encodes the deliberately stale-predecessor commit used by the hero demo.
/// The contract checks the predecessor before it checks authorization, so this
/// simulation never broadcasts a transaction or requires a signature.
pub fn silent_rollback_call_data(
    space_id: &str,
    canonical_sequence: u64,
) -> Result<String, EthereumError> {
    let call = MemoryLineageRegistry::commitTransitionCall {
        delta: ExperienceDelta {
            spaceId: bytes32(space_id)?,
            sequence: canonical_sequence
                .checked_add(1)
                .ok_or_else(|| EthereumError::Rpc("canonical sequence overflow".to_owned()))?,
            prevStateRoot: B256::ZERO,
            deltaCommitment: B256::from([0x11; 32]),
            provenanceCommitment: B256::from([0x22; 32]),
            profileId: B256::from([0x33; 32]),
            locatorCommitment: B256::from([0x44; 32]),
        },
        authorizerSignature: Bytes::new(),
    };
    Ok(format!("0x{}", hex::encode(call.abi_encode())))
}

#[cfg(not(target_arch = "wasm32"))]
fn provider(rpc_url: &str) -> Result<impl Provider + Clone, EthereumError> {
    let url = rpc_url
        .parse::<reqwest::Url>()
        .map_err(|error| EthereumError::InvalidRpcUrl(error.to_string()))?;
    Ok(ProviderBuilder::new().connect_http(url))
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn inspect_registry(
    rpc_url: &str,
    registry_address: &str,
    space_id: &str,
) -> Result<LiveInspection, EthereumError> {
    let provider = provider(rpc_url)?;
    let registry = address(registry_address)?;
    let space = bytes32(space_id)?;
    let head_call = MemoryLineageRegistry::headCall { spaceId: space };
    let head_bytes = provider
        .call(
            TransactionRequest::default()
                .to(registry)
                .input(head_call.abi_encode().into()),
        )
        .await
        .map_err(|error| EthereumError::Rpc(error.to_string()))?;
    let head = MemoryLineageRegistry::headCall::abi_decode_returns(&head_bytes)
        .map_err(|error| EthereumError::Rpc(error.to_string()))?;
    let authority_call = MemoryLineageRegistry::spaceAuthorizationCall { spaceId: space };
    let authority_bytes = provider
        .call(
            TransactionRequest::default()
                .to(registry)
                .input(authority_call.abi_encode().into()),
        )
        .await
        .map_err(|error| EthereumError::Rpc(error.to_string()))?;
    let authority =
        MemoryLineageRegistry::spaceAuthorizationCall::abi_decode_returns(&authority_bytes)
            .map_err(|error| EthereumError::Rpc(error.to_string()))?;
    let block_number = provider
        .get_block_number()
        .await
        .map_err(|error| EthereumError::Rpc(error.to_string()))?;
    Ok(LiveInspection {
        block_number,
        head: RegistryHead {
            transition_id: head.transitionId,
            state_root: head.stateRoot,
            sequence: head.sequence,
        },
        authority: RegistryAuthority {
            controller: authority.controller,
            authorizer: authority.authorizer,
            config_nonce: authority.configNonce,
        },
    })
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn simulate_silent_rollback(
    rpc_url: &str,
    registry_address: &str,
    space_id: &str,
) -> Result<RollbackSimulation, EthereumError> {
    let provider = provider(rpc_url)?;
    let registry = address(registry_address)?;
    let space = bytes32(space_id)?;
    let head_call = MemoryLineageRegistry::headCall { spaceId: space };
    let head_bytes = provider
        .call(
            TransactionRequest::default()
                .to(registry)
                .input(head_call.abi_encode().into()),
        )
        .await
        .map_err(|error| EthereumError::Rpc(error.to_string()))?;
    let head = MemoryLineageRegistry::headCall::abi_decode_returns(&head_bytes)
        .map_err(|error| EthereumError::Rpc(error.to_string()))?;
    let canonical_root = head.stateRoot;
    let canonical_sequence = head.sequence;

    let attempt = ExperienceDelta {
        spaceId: space,
        sequence: canonical_sequence
            .checked_add(1)
            .ok_or_else(|| EthereumError::Rpc("canonical sequence overflow".to_owned()))?,
        prevStateRoot: B256::ZERO,
        deltaCommitment: B256::from([0x11; 32]),
        provenanceCommitment: B256::from([0x22; 32]),
        profileId: B256::from([0x33; 32]),
        locatorCommitment: B256::from([0x44; 32]),
    };
    let call = MemoryLineageRegistry::commitTransitionCall {
        delta: attempt,
        authorizerSignature: Bytes::new(),
    };
    let transaction = TransactionRequest::default()
        .to(registry)
        .input(call.abi_encode().into());
    match provider.call(transaction).await {
        Ok(_) => Err(EthereumError::UnexpectedSuccess),
        Err(error) => {
            let message = error.to_string();
            if message.contains("BAD_PREVIOUS_STATE") {
                Ok(RollbackSimulation {
                    status: "REJECTED".to_owned(),
                    reason: "BAD_PREVIOUS_STATE".to_owned(),
                    sequence: canonical_sequence + 1,
                    canonical_root,
                })
            } else {
                Err(EthereumError::UnexpectedRevert(message))
            }
        }
    }
}
