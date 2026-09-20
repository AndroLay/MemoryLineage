#![forbid(unsafe_code)]

use k256::ecdsa::{RecoveryId, Signature, VerifyingKey};
use ml_spec_types::{
    AuthorizationProof, EVIDENCE_V2, EvidenceBundleV2, PortabilityRehearsalReport,
    PublicReplayBundle, RECOVERY_BLOCK_UNVERIFIED, RECOVERY_CURRENT_HEAD,
    RECOVERY_HISTORICAL_CHECKPOINT, RECOVERY_HOLD_FOR_REVIEW,
    RECOVERY_POLICY_STRICT_CURRENT_HEAD_V1, RECOVERY_RECEIPT_V1, RECOVERY_REHEARSE_ONLY,
    RECOVERY_RESUME_ALLOWED, RECOVERY_UNKNOWN_OR_DIVERGED, RECOVERY_UNVERIFIED, RecoveryAssurance,
    RecoveryBlockContext, RecoveryCandidate, RecoveryDecision, RecoveryDecisionReceipt,
    RecoveryEvidence, SNAPSHOT_PROFILE_V1, SNAPSHOT_PROFILE_V2, SOURCE_DEMO_SPACE_V2_LOCAL,
    SOURCE_LEGACY_UNDECLARED, SOURCE_PROTOCOL_CORPUS_LOCAL, SOURCE_SEPOLIA_REFERENCE_OBSERVATION,
    SPEC_NAME, SPEC_SNAPSHOT, TransitionRecord,
};
use serde::Serialize;
use std::path::Path;
use thiserror::Error;
use tiny_keccak::{Hasher, Keccak};

const PUBLIC_REPLAY_BUNDLE: &str = "public_commitment_replay_bundle";
const ZERO_ROOT: &str = "0x0000000000000000000000000000000000000000000000000000000000000000";
const EXPERIENCE_DELTA_TYPE: &str = "ExperienceDelta(bytes32 spaceId,uint64 sequence,bytes32 prevStateRoot,bytes32 deltaCommitment,bytes32 provenanceCommitment,bytes32 profileId,bytes32 locatorCommitment)";
const MEMORY_STATE_TYPE: &str = "MemoryState(bytes32 prevStateRoot,bytes32 transitionId)";
const MEMORY_SPACE_TYPE: &str = "MemorySpace(address initialController,bytes32 salt)";
const EXPERIENCE_DELTA_TYPEHASH: &str =
    "0x4f020f86bc06d852f1fde17853b4d92a70214eeab8e09718028124af097d070d";
const MEMORY_STATE_TYPEHASH: &str =
    "0xf3148762556cbf851baf4b9a205e18ff4e6b366a58a3a1ef58e8626ba41beadb";
const MEMORY_SPACE_TYPEHASH: &str =
    "0x9ae5478f084ad3b841da58a9cb2354d153cddec59ee64d0cb741fa9d08884531";
const EIP712_DOMAIN_TYPE: &str =
    "EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)";
const EIP712_NAME: &str = "AgentMemoryState";
const EIP712_VERSION: &str = "1";

#[derive(Debug, Error)]
pub enum VerificationError {
    #[error("{0}")]
    Rejected(String),
    #[error("could not read evidence file: {0}")]
    Io(#[from] std::io::Error),
    #[error("invalid evidence JSON: {0}")]
    Json(#[from] serde_json::Error),
    #[error("invalid hex value: {0}")]
    Hex(String),
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct VerificationReport {
    pub verdict: String,
    pub schema: String,
    pub registry_identity: String,
    pub spec_snapshot: String,
    pub transition_ids: String,
    pub state_roots: String,
    pub sequence_continuity: String,
    pub predecessor_continuity: String,
    pub authority_history: String,
    pub authorization_proof: String,
    pub head_reconstruction: String,
    pub privacy_boundary: String,
    pub attack_evidence: String,
    pub transition_count: usize,
    pub rejected_mutation_count: usize,
    pub final_root: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct RecoveryVerificationReport {
    pub verdict: String,
    pub receipt_integrity: String,
    pub evidence_bundle: String,
    pub policy_id: String,
    pub source_class: String,
    pub authority_history: String,
    pub transition_authorization: String,
    pub classification: String,
    pub recommended_action: String,
    pub candidate_commitment: String,
    pub decision_id: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct PortabilityVerificationCheck {
    pub name: String,
    pub status: String,
    pub detail: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct PortabilityVerificationReport {
    pub verdict: String,
    pub observation_count: usize,
    pub transition_projection: String,
    pub state_roots: String,
    pub authority_history: String,
    pub stale_predecessor_rejection: String,
    pub domain_separation: String,
    pub checks: Vec<PortabilityVerificationCheck>,
}

#[derive(Serialize)]
struct RecoveryReceiptDigestInput<'a> {
    #[serde(rename = "schemaVersion")]
    schema_version: &'a str,
    #[serde(rename = "policyId")]
    policy_id: &'a str,
    candidate: &'a RecoveryCandidate,
    evidence: &'a RecoveryEvidence,
    decision: &'a RecoveryDecision,
    assurance: &'a RecoveryAssurance,
    limitations: &'a [String],
}

const RECOVERY_LIMITATIONS: [&str; 3] = [
    "does_not_assess_semantic_truth",
    "does_not_prove_off_chain_availability",
    "does_not_claim_runtime_enforcement",
];

fn reject(message: impl Into<String>) -> VerificationError {
    VerificationError::Rejected(message.into())
}

fn validate_source_class(source_class: &str) -> Result<(), VerificationError> {
    if matches!(
        source_class,
        SOURCE_DEMO_SPACE_V2_LOCAL
            | SOURCE_PROTOCOL_CORPUS_LOCAL
            | SOURCE_SEPOLIA_REFERENCE_OBSERVATION
            | SOURCE_LEGACY_UNDECLARED
    ) {
        Ok(())
    } else {
        Err(reject("RECOVERY_SOURCE_CLASS_UNSUPPORTED"))
    }
}

fn parse_hex(value: &str) -> Result<Vec<u8>, VerificationError> {
    let encoded = value
        .strip_prefix("0x")
        .ok_or_else(|| VerificationError::Hex(value.to_owned()))?;
    hex::decode(encoded).map_err(|_| VerificationError::Hex(value.to_owned()))
}

fn bytes32(value: &str) -> Result<[u8; 32], VerificationError> {
    let decoded = parse_hex(value)?;
    decoded
        .try_into()
        .map_err(|_| VerificationError::Hex(value.to_owned()))
}

fn address_word(value: &str) -> Result<[u8; 32], VerificationError> {
    let decoded = parse_hex(value)?;
    if decoded.len() != 20 {
        return Err(VerificationError::Hex(value.to_owned()));
    }
    let mut word = [0u8; 32];
    word[12..].copy_from_slice(&decoded);
    Ok(word)
}

fn uint_word(value: u64) -> [u8; 32] {
    let mut word = [0u8; 32];
    word[24..].copy_from_slice(&value.to_be_bytes());
    word
}

fn abi_words(words: &[[u8; 32]]) -> Vec<u8> {
    words.iter().flat_map(|word| word.iter().copied()).collect()
}

fn keccak256(value: &[u8]) -> [u8; 32] {
    let mut output = [0u8; 32];
    let mut hasher = Keccak::v256();
    hasher.update(value);
    hasher.finalize(&mut output);
    output
}

fn hex_value(value: &[u8]) -> String {
    format!("0x{}", hex::encode(value))
}

fn type_hash(value: &str) -> String {
    hex_value(&keccak256(value.as_bytes()))
}

fn space_identifier(controller: &str, salt: &str) -> Result<String, VerificationError> {
    let words = [
        bytes32(MEMORY_SPACE_TYPEHASH)?,
        address_word(controller)?,
        bytes32(salt)?,
    ];
    Ok(hex_value(&keccak256(&abi_words(&words))))
}

fn transition_identifier(transition: &TransitionRecord) -> Result<String, VerificationError> {
    let delta = &transition.delta;
    let words = [
        bytes32(EXPERIENCE_DELTA_TYPEHASH)?,
        bytes32(&delta.space_id)?,
        uint_word(delta.sequence),
        bytes32(&delta.prev_state_root)?,
        bytes32(&delta.delta_commitment)?,
        bytes32(&delta.provenance_commitment)?,
        bytes32(&delta.profile_id)?,
        bytes32(&delta.locator_commitment)?,
    ];
    Ok(hex_value(&keccak256(&abi_words(&words))))
}

fn state_root(previous_root: &str, transition_id: &str) -> Result<String, VerificationError> {
    let words = [
        bytes32(MEMORY_STATE_TYPEHASH)?,
        bytes32(previous_root)?,
        bytes32(transition_id)?,
    ];
    Ok(hex_value(&keccak256(&abi_words(&words))))
}

fn eip712_domain_separator(
    chain_id: &str,
    verifying_contract: &str,
) -> Result<String, VerificationError> {
    let chain_id = chain_id
        .parse::<u64>()
        .map_err(|_| reject("AUTHORIZATION_CHAIN_ID_INVALID"))?;
    let words = [
        bytes32(&type_hash(EIP712_DOMAIN_TYPE))?,
        keccak256(EIP712_NAME.as_bytes()),
        keccak256(EIP712_VERSION.as_bytes()),
        uint_word(chain_id),
        address_word(verifying_contract)?,
    ];
    Ok(hex_value(&keccak256(&abi_words(&words))))
}

fn eip712_signing_digest(
    struct_hash: &str,
    chain_id: &str,
    verifying_contract: &str,
) -> Result<String, VerificationError> {
    let domain = bytes32(&eip712_domain_separator(chain_id, verifying_contract)?)?;
    let structure = bytes32(struct_hash)?;
    let mut encoded = Vec::with_capacity(66);
    encoded.extend_from_slice(&[0x19, 0x01]);
    encoded.extend_from_slice(&domain);
    encoded.extend_from_slice(&structure);
    Ok(hex_value(&keccak256(&encoded)))
}

fn recovered_address(digest: &str, signature: &str) -> Result<String, VerificationError> {
    let signature = parse_hex(signature)?;
    if signature.len() != 65 {
        return Err(reject("AUTHORIZATION_SIGNATURE_INVALID"));
    }
    let recovery_byte = match signature[64] {
        27 | 28 => signature[64] - 27,
        _ => return Err(reject("AUTHORIZATION_SIGNATURE_INVALID")),
    };
    let signature = Signature::from_slice(&signature[..64])
        .map_err(|_| reject("AUTHORIZATION_SIGNATURE_INVALID"))?;
    let recovery_id = RecoveryId::try_from(recovery_byte)
        .map_err(|_| reject("AUTHORIZATION_SIGNATURE_INVALID"))?;
    let verifying_key =
        VerifyingKey::recover_from_prehash(&bytes32(digest)?, &signature, recovery_id)
            .map_err(|_| reject("AUTHORIZATION_SIGNATURE_INVALID"))?;
    let public_key = verifying_key.to_encoded_point(false);
    let digest = keccak256(&public_key.as_bytes()[1..]);
    Ok(hex_value(&digest[12..]))
}

fn verify_authorization_proofs(
    bundle: &EvidenceBundleV2,
) -> Result<&'static str, VerificationError> {
    if bundle.authorization_proofs.is_empty() {
        return Ok("NOT_INCLUDED");
    }
    if bundle.authorization_proofs.len() != bundle.transitions.len() {
        return Err(reject("AUTHORIZATION_PROOF_INCOMPLETE"));
    }

    let mut seen_sequences = Vec::with_capacity(bundle.authorization_proofs.len());
    for proof in &bundle.authorization_proofs {
        verify_authorization_proof(proof, bundle)?;
        if seen_sequences.contains(&proof.sequence) {
            return Err(reject("AUTHORIZATION_PROOF_DUPLICATE_SEQUENCE"));
        }
        seen_sequences.push(proof.sequence);
    }
    for transition in &bundle.transitions {
        if !seen_sequences.contains(&transition.delta.sequence) {
            return Err(reject("AUTHORIZATION_PROOF_SEQUENCE_MISSING"));
        }
    }
    Ok("EOA_SIGNATURES_VERIFIED")
}

fn verify_authorization_proof(
    proof: &AuthorizationProof,
    bundle: &EvidenceBundleV2,
) -> Result<(), VerificationError> {
    if proof.authorization_type != "EOA_EIP712" {
        return Err(reject("AUTHORIZATION_PROOF_TYPE_UNSUPPORTED"));
    }
    let transition = bundle
        .transitions
        .iter()
        .find(|transition| transition.delta.sequence == proof.sequence)
        .ok_or_else(|| reject("AUTHORIZATION_PROOF_SEQUENCE_UNKNOWN"))?;
    if transition.transition_id != proof.transition_id
        || proof.struct_hash != transition.transition_id
    {
        return Err(reject("AUTHORIZATION_PROOF_TRANSITION_MISMATCH"));
    }
    let config_nonce = proof
        .config_nonce
        .ok_or_else(|| reject("AUTHORIZATION_CONFIG_NONCE_MISSING"))?;
    let active_authority = bundle
        .authorization_history
        .iter()
        .filter_map(|authority| {
            authority
                .effective_from_sequence
                .filter(|effective| *effective <= proof.sequence)
                .map(|effective| (effective, authority))
        })
        .max_by_key(|(effective, _)| *effective)
        .map(|(_, authority)| authority)
        .ok_or_else(|| reject("AUTHORIZATION_ACTIVE_AUTHORITY_MISSING"))?;
    if active_authority.config_nonce != config_nonce
        || !active_authority
            .authorizer
            .eq_ignore_ascii_case(&proof.authorizer)
    {
        return Err(reject("AUTHORIZATION_ACTIVE_SIGNER_MISMATCH"));
    }
    if proof.chain_id != bundle.network.chain_id
        || !proof
            .verifying_contract
            .eq_ignore_ascii_case(&bundle.registry.address)
    {
        return Err(reject("AUTHORIZATION_PROOF_DOMAIN_CONTEXT_MISMATCH"));
    }
    let expected_domain = eip712_domain_separator(&proof.chain_id, &proof.verifying_contract)?;
    if proof.domain_separator != expected_domain {
        return Err(reject("AUTHORIZATION_DOMAIN_SEPARATOR_MISMATCH"));
    }
    let expected_digest = eip712_signing_digest(
        &proof.struct_hash,
        &proof.chain_id,
        &proof.verifying_contract,
    )?;
    if proof.signing_digest != expected_digest {
        return Err(reject("AUTHORIZATION_SIGNING_DIGEST_MISMATCH"));
    }
    let recovered = recovered_address(&expected_digest, &proof.signature)?;
    if !recovered.eq_ignore_ascii_case(&proof.authorizer) {
        return Err(reject("AUTHORIZATION_SIGNER_MISMATCH"));
    }
    Ok(())
}

fn authority_history_status(bundle: &EvidenceBundleV2) -> &'static str {
    if !bundle.authorization_proofs.is_empty()
        && bundle
            .authorization_history
            .iter()
            .all(|authority| authority.effective_from_sequence.is_some())
    {
        "TIMELINE_BOUND"
    } else {
        "STRUCTURE_ONLY"
    }
}

fn verify_transition(
    transition: &TransitionRecord,
    expected_space: Option<&str>,
    expected_profile: Option<&str>,
    expected_sequence: Option<u64>,
    expected_root: Option<&str>,
) -> Result<(), VerificationError> {
    let delta = &transition.delta;
    if let Some(space) = expected_space
        && delta.space_id != space
    {
        return Err(reject("SPACE_ID_MISMATCH"));
    }
    if let Some(profile) = expected_profile
        && delta.profile_id != profile
    {
        return Err(reject("PROFILE_ID_MISMATCH"));
    }
    if let Some(sequence) = expected_sequence
        && delta.sequence != sequence
    {
        return Err(reject("SEQUENCE_MISMATCH"));
    }
    if let Some(root) = expected_root
        && delta.prev_state_root != root
    {
        return Err(reject("BAD_PREVIOUS_STATE"));
    }

    let computed_transition = transition_identifier(transition)?;
    if computed_transition != transition.transition_id {
        return Err(reject("TRANSITION_ID_MISMATCH"));
    }
    let computed_root = state_root(&delta.prev_state_root, &computed_transition)?;
    if computed_root != transition.next_state_root {
        return Err(reject("NEXT_ROOT_MISMATCH"));
    }
    Ok(())
}

fn verify_attack_evidence(bundle: &EvidenceBundleV2) -> Result<&'static str, VerificationError> {
    let Some(attack) = bundle.attack.as_ref() else {
        return Ok("NOT PRESENT");
    };
    let has_extended_evidence = attack.stale_predecessor.is_some()
        || attack.canonical_predecessor.is_some()
        || attack.execution_source.is_some()
        || attack.transaction_broadcast.is_some()
        || attack.fixture_id.is_some();
    if !has_extended_evidence {
        return Ok("NOT REPLAYED / LEGACY RECORD");
    }

    if attack.status != "REJECTED" || attack.reason.as_deref() != Some("BAD_PREVIOUS_STATE") {
        return Err(reject("ATTACK_RESULT_MISMATCH"));
    }
    if attack.transaction_broadcast != Some(false) {
        return Err(reject("ATTACK_BROADCAST_STATUS_MISMATCH"));
    }
    if attack.execution_source.as_deref().is_none_or(str::is_empty) {
        return Err(reject("ATTACK_EXECUTION_SOURCE_MISSING"));
    }

    let stale_predecessor = attack
        .stale_predecessor
        .as_deref()
        .ok_or_else(|| reject("ATTACK_STALE_PREDECESSOR_MISSING"))?;
    let canonical_predecessor = attack
        .canonical_predecessor
        .as_deref()
        .ok_or_else(|| reject("ATTACK_CANONICAL_PREDECESSOR_MISSING"))?;
    let restored_sequence = attack
        .restored_snapshot_sequence
        .ok_or_else(|| reject("ATTACK_RESTORED_SEQUENCE_MISSING"))?;
    let attempted_sequence = attack
        .attempted_sequence
        .ok_or_else(|| reject("ATTACK_SEQUENCE_MISSING"))?;

    let _ = bytes32(stale_predecessor)?;
    if canonical_predecessor != bundle.head.state_root {
        return Err(reject("ATTACK_CANONICAL_ROOT_MISMATCH"));
    }
    if attempted_sequence
        != bundle
            .head
            .sequence
            .checked_add(1)
            .ok_or_else(|| reject("ATTACK_SEQUENCE_OVERFLOW"))?
    {
        return Err(reject("ATTACK_SEQUENCE_MISMATCH"));
    }
    if restored_sequence >= bundle.head.sequence
        || !bundle.transitions.iter().any(|transition| {
            transition.delta.sequence == restored_sequence
                && transition.next_state_root == stale_predecessor
        })
    {
        return Err(reject("ATTACK_STALE_PREDECESSOR_MISMATCH"));
    }

    Ok("PASS")
}

pub fn verify_bundle(bundle: &PublicReplayBundle) -> Result<VerificationReport, VerificationError> {
    if bundle.evidence_type != PUBLIC_REPLAY_BUNDLE {
        return Err(reject("EVIDENCE_TYPE_MISMATCH"));
    }
    if bundle.raw_payload_stored {
        return Err(reject("PRIVACY_BOUNDARY_VIOLATION"));
    }
    let _ = parse_hex(&bundle.registry_address)?;
    if parse_hex(&bundle.registry_address)?.len() != 20 {
        return Err(reject("REGISTRY_ADDRESS_INVALID"));
    }

    let conformance = &bundle.conformance;
    let expected = &conformance.expected;
    let computed_space = space_identifier(
        &conformance.inputs.initial_controller,
        &conformance.inputs.space_salt,
    )?;
    if computed_space != expected.space_id
        || conformance.independently_computed.space_id != expected.space_id
    {
        return Err(reject("SPACE_ID_REPLAY_MISMATCH"));
    }
    if type_hash(EXPERIENCE_DELTA_TYPE) != expected.experience_delta_type_hash
        || type_hash(MEMORY_STATE_TYPE) != expected.memory_state_type_hash
        || type_hash(MEMORY_SPACE_TYPE) != expected.memory_space_type_hash
        || type_hash(EXPERIENCE_DELTA_TYPE) != conformance.contract.typehashes.experience_delta
        || type_hash(MEMORY_STATE_TYPE) != conformance.contract.typehashes.memory_state
        || type_hash(MEMORY_SPACE_TYPE) != conformance.contract.typehashes.memory_space
    {
        return Err(reject("TYPEHASH_MISMATCH"));
    }
    verify_transition(
        &conformance.full_transition,
        Some(&expected.space_id),
        None,
        Some(1),
        Some(ZERO_ROOT),
    )?;
    if conformance.full_transition.transition_id != expected.transition_id
        || conformance.full_transition.next_state_root != expected.next_state_root
        || conformance.independently_computed.transition_id != expected.transition_id
        || conformance.independently_computed.next_state_root != expected.next_state_root
        || conformance.contract.transition_id != expected.transition_id
        || conformance.contract.next_state_root != expected.next_state_root
        || !conformance.all_match
    {
        return Err(reject("CONFORMANCE_ARTIFACT_MISMATCH"));
    }

    let history = &bundle.valid_history;
    if history.len() < 4 {
        return Err(reject("VALID_HISTORY_TOO_SHORT"));
    }
    let space = history[0].delta.space_id.clone();
    let profile = history[0].delta.profile_id.clone();
    let mut expected_sequence = 1u64;
    let mut previous_root = ZERO_ROOT.to_owned();
    for transition in history {
        verify_transition(
            transition,
            Some(&space),
            Some(&profile),
            Some(expected_sequence),
            Some(&previous_root),
        )?;
        previous_root = transition.next_state_root.clone();
        expected_sequence = expected_sequence
            .checked_add(1)
            .ok_or_else(|| reject("SEQUENCE_OVERFLOW"))?;
    }

    if bundle.authority_history.is_empty() {
        return Err(reject("AUTHORITY_HISTORY_EMPTY"));
    }
    let mut previous_nonce = None;
    for authority in &bundle.authority_history {
        if parse_hex(&authority.controller)?.len() != 20
            || parse_hex(&authority.authorizer)?.len() != 20
        {
            return Err(reject("AUTHORITY_ADDRESS_INVALID"));
        }
        if let Some(previous) = previous_nonce
            && authority.config_nonce < previous
        {
            return Err(reject("AUTHORITY_NONCE_REORDERED"));
        }
        previous_nonce = Some(authority.config_nonce);
    }

    let mut rejected_mutations = 0usize;
    for mutation in &bundle.mutation_matrix {
        verify_transition(&mutation.transition, None, None, None, None)?;
        match mutation.expected.as_str() {
            "REJECT" => {
                if mutation.observed.status != "REJECT" || mutation.observed.reason.is_none() {
                    return Err(reject(format!("MUTATION_NOT_REJECTED:{}", mutation.name)));
                }
                rejected_mutations += 1;
            }
            "PASS" => {
                if mutation.observed.status != "PASS" || mutation.observed.reason.is_some() {
                    return Err(reject(format!("MUTATION_PASS_MISMATCH:{}", mutation.name)));
                }
            }
            _ => {
                return Err(reject(format!(
                    "UNKNOWN_MUTATION_EXPECTATION:{}",
                    mutation.name
                )));
            }
        }
    }

    Ok(VerificationReport {
        verdict: "VERIFIED".to_owned(),
        schema: "PASS".to_owned(),
        registry_identity: "PASS".to_owned(),
        spec_snapshot: "PASS".to_owned(),
        transition_ids: "PASS".to_owned(),
        state_roots: "PASS".to_owned(),
        sequence_continuity: "PASS".to_owned(),
        predecessor_continuity: "PASS".to_owned(),
        authority_history: "STRUCTURE_ONLY".to_owned(),
        authorization_proof: "NOT_INCLUDED".to_owned(),
        head_reconstruction: "MATCH".to_owned(),
        privacy_boundary: "PASS".to_owned(),
        attack_evidence: "NOT PRESENT".to_owned(),
        transition_count: history.len(),
        rejected_mutation_count: rejected_mutations,
        final_root: previous_root,
    })
}

pub fn verify_v2_bundle(
    bundle: &EvidenceBundleV2,
) -> Result<VerificationReport, VerificationError> {
    if bundle.schema_version != EVIDENCE_V2 || bundle.evidence_type != "memorylineage_evidence_v2" {
        return Err(reject("SCHEMA_VERSION_MISMATCH"));
    }
    if bundle.spec.name != SPEC_NAME || bundle.spec.snapshot != SPEC_SNAPSHOT {
        return Err(reject("SPEC_SNAPSHOT_MISMATCH"));
    }
    validate_source_class(&bundle.source_class)?;
    if bundle.privacy.raw_memory_on_chain {
        return Err(reject("PRIVACY_BOUNDARY_VIOLATION"));
    }
    if parse_hex(&bundle.registry.address)?.len() != 20 {
        return Err(reject("REGISTRY_ADDRESS_INVALID"));
    }
    if let Some(code_hash) = &bundle.registry.code_hash {
        let _ = bytes32(code_hash)?;
    }
    if bundle.transitions.is_empty() {
        return Err(reject("EMPTY_TRANSITION_HISTORY"));
    }
    let space = &bundle.registry.space_id;
    let profile = &bundle.transitions[0].delta.profile_id;
    let mut expected_sequence = 1u64;
    let mut previous_root = ZERO_ROOT.to_owned();
    for transition in &bundle.transitions {
        verify_transition(
            transition,
            Some(space),
            Some(profile),
            Some(expected_sequence),
            Some(&previous_root),
        )?;
        previous_root = transition.next_state_root.clone();
        expected_sequence = expected_sequence
            .checked_add(1)
            .ok_or_else(|| reject("SEQUENCE_OVERFLOW"))?;
    }
    let last = bundle.transitions.last().expect("non-empty checked above");
    if bundle.head.sequence != last.delta.sequence
        || bundle.head.transition_id != last.transition_id
        || bundle.head.state_root != last.next_state_root
    {
        return Err(reject("HEAD_RECONSTRUCTION_MISMATCH"));
    }
    if bundle.authorization_history.is_empty() {
        return Err(reject("AUTHORITY_HISTORY_EMPTY"));
    }
    let mut previous_nonce = None;
    let mut previous_effective_sequence = None;
    for authority in &bundle.authorization_history {
        if parse_hex(&authority.controller)?.len() != 20
            || parse_hex(&authority.authorizer)?.len() != 20
        {
            return Err(reject("AUTHORITY_ADDRESS_INVALID"));
        }
        if let Some(previous) = previous_nonce
            && authority.config_nonce < previous
        {
            return Err(reject("AUTHORITY_NONCE_REORDERED"));
        }
        if let Some(effective) = authority.effective_from_sequence {
            if effective == 0 {
                return Err(reject("AUTHORITY_EFFECTIVE_SEQUENCE_INVALID"));
            }
            if let Some(previous) = previous_effective_sequence
                && effective <= previous
            {
                return Err(reject("AUTHORITY_EFFECTIVE_SEQUENCE_REORDERED"));
            }
            previous_effective_sequence = Some(effective);
        }
        previous_nonce = Some(authority.config_nonce);
    }
    let authorization_proof = verify_authorization_proofs(bundle)?;
    let authority_history = authority_history_status(bundle);
    for observation in &bundle.observations {
        if observation.observed_head.sequence > bundle.head.sequence {
            return Err(reject("OBSERVATION_AHEAD_OF_BUNDLE"));
        }
    }
    let attack_evidence = verify_attack_evidence(bundle)?.to_owned();

    Ok(VerificationReport {
        verdict: "VERIFIED".to_owned(),
        schema: "PASS".to_owned(),
        registry_identity: "PASS".to_owned(),
        spec_snapshot: "PASS".to_owned(),
        transition_ids: "PASS".to_owned(),
        state_roots: "PASS".to_owned(),
        sequence_continuity: "PASS".to_owned(),
        predecessor_continuity: "PASS".to_owned(),
        authority_history: authority_history.to_owned(),
        authorization_proof: authorization_proof.to_owned(),
        head_reconstruction: "MATCH".to_owned(),
        privacy_boundary: "PASS".to_owned(),
        attack_evidence,
        transition_count: bundle.transitions.len(),
        rejected_mutation_count: 0,
        final_root: previous_root,
    })
}

fn portability_projection(bundle: &EvidenceBundleV2) -> Vec<(&str, &str)> {
    bundle
        .transitions
        .iter()
        .map(|transition| {
            (
                transition.transition_id.as_str(),
                transition.next_state_root.as_str(),
            )
        })
        .collect()
}

/// Independently replay the two detailed observations inside a portability
/// report, then compare their invariant projections. This function validates
/// the report format and both evidence bundles; it does not perform a network
/// call or treat the local target-context observation as a deployment.
pub fn verify_portability_report(
    report: &PortabilityRehearsalReport,
) -> Result<PortabilityVerificationReport, VerificationError> {
    if report.schema_version != ml_spec_types::PORTABILITY_REHEARSAL_V1 {
        return Err(reject("PORTABILITY_SCHEMA_VERSION_MISMATCH"));
    }
    if report.report_type != "polkadot_hub_revm_portability_rehearsal"
        || report.status != "LOCAL_REHEARSAL_PASS"
    {
        return Err(reject("PORTABILITY_STATUS_MISMATCH"));
    }
    if report.target.chain_id != ml_spec_types::POLKADOT_HUB_TESTNET_CHAIN_ID
        || report.target.deployment != "NOT_PERFORMED"
        || report.target.public_rpc_observation != "NOT_PERFORMED"
    {
        return Err(reject("PORTABILITY_TARGET_BOUNDARY_MISMATCH"));
    }
    if report.observations.len() != 2 {
        return Err(reject("PORTABILITY_OBSERVATION_COUNT_MISMATCH"));
    }

    let ethereum = report
        .observations
        .iter()
        .find(|observation| observation.label == "Ethereum-local")
        .ok_or_else(|| reject("PORTABILITY_ETHEREUM_OBSERVATION_MISSING"))?;
    let polkadot = report
        .observations
        .iter()
        .find(|observation| observation.label == "Polkadot Hub TestNet chain context")
        .ok_or_else(|| reject("PORTABILITY_POLKADOT_OBSERVATION_MISSING"))?;

    if ethereum.evidence.network.chain_id == report.target.chain_id
        || polkadot.evidence.network.chain_id != report.target.chain_id
    {
        return Err(reject("PORTABILITY_CHAIN_CONTEXT_MISMATCH"));
    }
    if ethereum.evidence.registry != polkadot.evidence.registry {
        return Err(reject("PORTABILITY_REGISTRY_IDENTITY_MISMATCH"));
    }

    verify_v2_bundle(&ethereum.evidence)
        .map_err(|error| reject(format!("PORTABILITY_ETHEREUM_BUNDLE_INVALID:{error}")))?;
    verify_v2_bundle(&polkadot.evidence)
        .map_err(|error| reject(format!("PORTABILITY_POLKADOT_BUNDLE_INVALID:{error}")))?;

    if portability_projection(&ethereum.evidence) != portability_projection(&polkadot.evidence) {
        return Err(reject("PORTABILITY_TRANSITION_PROJECTION_MISMATCH"));
    }
    if ethereum.evidence.authorization_history != polkadot.evidence.authorization_history {
        return Err(reject("PORTABILITY_AUTHORITY_HISTORY_MISMATCH"));
    }

    let ethereum_attack = ethereum
        .evidence
        .attack
        .as_ref()
        .ok_or_else(|| reject("PORTABILITY_ETHEREUM_ATTACK_MISSING"))?;
    let polkadot_attack = polkadot
        .evidence
        .attack
        .as_ref()
        .ok_or_else(|| reject("PORTABILITY_POLKADOT_ATTACK_MISSING"))?;
    if ethereum_attack.status != "REJECTED"
        || polkadot_attack.status != "REJECTED"
        || ethereum_attack.reason.as_deref() != Some("BAD_PREVIOUS_STATE")
        || polkadot_attack.reason.as_deref() != Some("BAD_PREVIOUS_STATE")
        || ethereum_attack.reason != polkadot_attack.reason
    {
        return Err(reject("PORTABILITY_ATTACK_RESULT_MISMATCH"));
    }

    if ethereum.domain_separator == polkadot.domain_separator {
        return Err(reject("PORTABILITY_DOMAIN_SEPARATION_MISMATCH"));
    }
    for observation in [ethereum, polkadot] {
        let proof_domain = observation
            .evidence
            .authorization_proofs
            .first()
            .ok_or_else(|| reject("PORTABILITY_AUTHORIZATION_PROOF_MISSING"))?
            .domain_separator
            .as_str();
        if observation.domain_separator != proof_domain {
            return Err(reject("PORTABILITY_DOMAIN_OBSERVATION_MISMATCH"));
        }
    }

    if report.comparison.canonical_transitions != "MATCH"
        || report.comparison.state_roots != "MATCH"
        || report.comparison.authority_history != "MATCH"
        || report.comparison.stale_predecessor_rejection != "BAD_PREVIOUS_STATE"
        || report.comparison.eip712_domain != "CHAIN_BOUND_DIFFERENT"
        || report.comparison.solidity_artifact != "SAME_COMMITTED_CREATION_BYTECODE"
    {
        return Err(reject("PORTABILITY_COMPARISON_SUMMARY_MISMATCH"));
    }

    Ok(PortabilityVerificationReport {
        verdict: "VERIFIED".to_owned(),
        observation_count: report.observations.len(),
        transition_projection: "MATCH".to_owned(),
        state_roots: "MATCH".to_owned(),
        authority_history: "MATCH".to_owned(),
        stale_predecessor_rejection: "BAD_PREVIOUS_STATE".to_owned(),
        domain_separation: "CHAIN_BOUND_DIFFERENT".to_owned(),
        checks: vec![
            PortabilityVerificationCheck {
                name: "Ethereum evidence bundle".to_owned(),
                status: "PASS".to_owned(),
                detail: "independent V2 replay".to_owned(),
            },
            PortabilityVerificationCheck {
                name: "Polkadot target-context bundle".to_owned(),
                status: "PASS".to_owned(),
                detail: "independent V2 replay; local only".to_owned(),
            },
            PortabilityVerificationCheck {
                name: "Transition and root projection".to_owned(),
                status: "PASS".to_owned(),
                detail: "MATCH".to_owned(),
            },
            PortabilityVerificationCheck {
                name: "Stale predecessor result".to_owned(),
                status: "PASS".to_owned(),
                detail: "BAD_PREVIOUS_STATE".to_owned(),
            },
            PortabilityVerificationCheck {
                name: "EIP-712 domain separation".to_owned(),
                status: "PASS".to_owned(),
                detail: "CHAIN_BOUND_DIFFERENT".to_owned(),
            },
        ],
    })
}

pub fn verify_portability_json(
    json: &str,
) -> Result<PortabilityVerificationReport, VerificationError> {
    let report: PortabilityRehearsalReport = serde_json::from_str(json)?;
    verify_portability_report(&report)
}

pub fn verify_portability_file(
    path: impl AsRef<Path>,
) -> Result<PortabilityVerificationReport, VerificationError> {
    let bytes = std::fs::read(path)?;
    verify_portability_json(
        std::str::from_utf8(&bytes).map_err(|_| reject("PORTABILITY_NOT_UTF8"))?,
    )
}

pub fn evidence_bundle_hash(bundle: &EvidenceBundleV2) -> Result<String, VerificationError> {
    let encoded = serde_json::to_vec(bundle)?;
    Ok(hex_value(&keccak256(&encoded)))
}

fn recovery_decision_for_candidate(
    candidate: &RecoveryCandidate,
    bundle: &EvidenceBundleV2,
) -> Result<RecoveryDecision, VerificationError> {
    if !matches!(
        candidate.snapshot_profile.as_str(),
        SNAPSHOT_PROFILE_V1 | SNAPSHOT_PROFILE_V2
    ) {
        return Ok(RecoveryDecision {
            classification: RECOVERY_UNVERIFIED.to_owned(),
            reason_code: "SNAPSHOT_PROFILE_UNSUPPORTED".to_owned(),
            recommended_action: RECOVERY_BLOCK_UNVERIFIED.to_owned(),
        });
    }

    let _ = bytes32(&candidate.candidate_commitment)?;
    let matching_sequence = bundle
        .transitions
        .iter()
        .filter(|transition| {
            transition
                .delta
                .delta_commitment
                .eq_ignore_ascii_case(&candidate.candidate_commitment)
        })
        .map(|transition| transition.delta.sequence)
        .max();

    let Some(sequence) = matching_sequence else {
        return Ok(RecoveryDecision {
            classification: RECOVERY_UNKNOWN_OR_DIVERGED.to_owned(),
            reason_code: "CANDIDATE_NOT_IN_CANONICAL_HISTORY".to_owned(),
            recommended_action: RECOVERY_HOLD_FOR_REVIEW.to_owned(),
        });
    };

    if sequence != candidate.snapshot_sequence {
        return Ok(RecoveryDecision {
            classification: RECOVERY_UNVERIFIED.to_owned(),
            reason_code: "CANDIDATE_SEQUENCE_MISMATCH".to_owned(),
            recommended_action: RECOVERY_BLOCK_UNVERIFIED.to_owned(),
        });
    }

    if sequence == bundle.head.sequence {
        return Ok(RecoveryDecision {
            classification: RECOVERY_CURRENT_HEAD.to_owned(),
            reason_code: "CANDIDATE_MATCHES_EVIDENCE_HEAD".to_owned(),
            recommended_action: RECOVERY_RESUME_ALLOWED.to_owned(),
        });
    }

    if sequence < bundle.head.sequence {
        return Ok(RecoveryDecision {
            classification: RECOVERY_HISTORICAL_CHECKPOINT.to_owned(),
            reason_code: "CANDIDATE_IS_BEHIND_CANONICAL_HEAD".to_owned(),
            recommended_action: RECOVERY_REHEARSE_ONLY.to_owned(),
        });
    }

    Ok(RecoveryDecision {
        classification: RECOVERY_UNVERIFIED.to_owned(),
        reason_code: "CANDIDATE_AHEAD_OF_EVIDENCE_HEAD".to_owned(),
        recommended_action: RECOVERY_BLOCK_UNVERIFIED.to_owned(),
    })
}

fn recovery_assurance(
    source_class: &str,
    authority_history: &str,
    authorization_proof: &str,
) -> RecoveryAssurance {
    RecoveryAssurance {
        lineage_replay: "VERIFIED".to_owned(),
        authority_history: authority_history.to_owned(),
        transition_authorization: authorization_proof.to_owned(),
        source: source_class.to_owned(),
    }
}

fn recovery_digest(receipt: &RecoveryDecisionReceipt) -> Result<String, VerificationError> {
    let input = RecoveryReceiptDigestInput {
        schema_version: &receipt.schema_version,
        policy_id: &receipt.policy_id,
        candidate: &receipt.candidate,
        evidence: &receipt.evidence,
        decision: &receipt.decision,
        assurance: &receipt.assurance,
        limitations: &receipt.limitations,
    };
    let encoded = serde_json::to_vec(&input)?;
    Ok(hex_value(&keccak256(&encoded)))
}

pub fn build_recovery_receipt(
    bundle: &EvidenceBundleV2,
    candidate_commitment: &str,
    snapshot_sequence: u64,
    source_class: &str,
    block_context: Option<RecoveryBlockContext>,
) -> Result<RecoveryDecisionReceipt, VerificationError> {
    build_recovery_receipt_with_snapshot_profile(
        bundle,
        candidate_commitment,
        snapshot_sequence,
        source_class,
        block_context,
        SNAPSHOT_PROFILE_V1,
    )
}

pub fn build_recovery_receipt_with_snapshot_profile(
    bundle: &EvidenceBundleV2,
    candidate_commitment: &str,
    snapshot_sequence: u64,
    source_class: &str,
    block_context: Option<RecoveryBlockContext>,
    snapshot_profile: &str,
) -> Result<RecoveryDecisionReceipt, VerificationError> {
    if !matches!(snapshot_profile, SNAPSHOT_PROFILE_V1 | SNAPSHOT_PROFILE_V2) {
        return Err(reject("RECOVERY_SNAPSHOT_PROFILE_UNSUPPORTED"));
    }
    verify_v2_bundle(bundle)?;
    validate_source_class(source_class)?;
    if bundle.source_class == SOURCE_LEGACY_UNDECLARED || bundle.source_class != source_class {
        return Err(reject("RECOVERY_SOURCE_CLASS_MISMATCH"));
    }

    let verification = verify_v2_bundle(bundle)?;
    let authorization_proof = verification.authorization_proof.clone();
    let authority_history = verification.authority_history.clone();
    let candidate = RecoveryCandidate {
        snapshot_profile: snapshot_profile.to_owned(),
        candidate_commitment: candidate_commitment.to_owned(),
        snapshot_sequence,
    };
    let decision = recovery_decision_for_candidate(&candidate, bundle)?;
    let receipt = RecoveryDecisionReceipt {
        schema_version: RECOVERY_RECEIPT_V1.to_owned(),
        decision_id: String::new(),
        policy_id: RECOVERY_POLICY_STRICT_CURRENT_HEAD_V1.to_owned(),
        candidate,
        evidence: RecoveryEvidence {
            source_class: source_class.to_owned(),
            bundle_hash: evidence_bundle_hash(bundle)?,
            spec_snapshot: bundle.spec.snapshot.clone(),
            registry_address: bundle.registry.address.clone(),
            space_id: bundle.registry.space_id.clone(),
            head: bundle.head.clone(),
            block_context,
        },
        decision,
        assurance: recovery_assurance(source_class, &authority_history, &authorization_proof),
        limitations: RECOVERY_LIMITATIONS
            .iter()
            .map(|limitation| (*limitation).to_owned())
            .collect(),
    };
    let decision_id = recovery_digest(&receipt)?;
    Ok(RecoveryDecisionReceipt {
        decision_id,
        ..receipt
    })
}

pub fn verify_recovery_receipt(
    receipt: &RecoveryDecisionReceipt,
    bundle: &EvidenceBundleV2,
) -> Result<RecoveryVerificationReport, VerificationError> {
    if receipt.schema_version != RECOVERY_RECEIPT_V1 {
        return Err(reject("RECOVERY_RECEIPT_SCHEMA_MISMATCH"));
    }
    verify_v2_bundle(bundle)?;
    if receipt.policy_id != RECOVERY_POLICY_STRICT_CURRENT_HEAD_V1 {
        return Err(reject("RECOVERY_POLICY_MISMATCH"));
    }
    validate_source_class(&receipt.evidence.source_class)?;
    if receipt.evidence.source_class != bundle.source_class {
        return Err(reject("RECOVERY_SOURCE_CLASS_MISMATCH"));
    }
    if receipt.evidence.bundle_hash != evidence_bundle_hash(bundle)? {
        return Err(reject("RECOVERY_EVIDENCE_HASH_MISMATCH"));
    }
    if receipt.evidence.spec_snapshot != bundle.spec.snapshot
        || receipt.evidence.registry_address != bundle.registry.address
        || receipt.evidence.space_id != bundle.registry.space_id
        || receipt.evidence.head != bundle.head
    {
        return Err(reject("RECOVERY_EVIDENCE_CONTEXT_MISMATCH"));
    }
    if !matches!(
        receipt.candidate.snapshot_profile.as_str(),
        SNAPSHOT_PROFILE_V1 | SNAPSHOT_PROFILE_V2
    ) {
        return Err(reject("RECOVERY_SNAPSHOT_PROFILE_UNSUPPORTED"));
    }
    let expected_decision = recovery_decision_for_candidate(&receipt.candidate, bundle)?;
    if receipt.decision != expected_decision {
        return Err(reject("RECOVERY_DECISION_MISMATCH"));
    }
    let verification = verify_v2_bundle(bundle)?;
    let expected_assurance = recovery_assurance(
        &receipt.evidence.source_class,
        &verification.authority_history,
        &verification.authorization_proof,
    );
    if receipt.assurance != expected_assurance {
        return Err(reject("RECOVERY_ASSURANCE_MISMATCH"));
    }
    let expected_limitations = RECOVERY_LIMITATIONS
        .iter()
        .map(|limitation| (*limitation).to_owned())
        .collect::<Vec<_>>();
    if receipt.limitations != expected_limitations {
        return Err(reject("RECOVERY_LIMITATIONS_MISMATCH"));
    }
    if receipt.decision_id != recovery_digest(receipt)? {
        return Err(reject("RECOVERY_DECISION_ID_MISMATCH"));
    }

    Ok(RecoveryVerificationReport {
        verdict: "VERIFIED".to_owned(),
        receipt_integrity: "PASS".to_owned(),
        evidence_bundle: "PASS".to_owned(),
        policy_id: receipt.policy_id.clone(),
        source_class: receipt.evidence.source_class.clone(),
        authority_history: verification.authority_history,
        transition_authorization: verification.authorization_proof,
        classification: receipt.decision.classification.clone(),
        recommended_action: receipt.decision.recommended_action.clone(),
        candidate_commitment: receipt.candidate.candidate_commitment.clone(),
        decision_id: receipt.decision_id.clone(),
    })
}

pub fn verify_json(json: &str) -> Result<VerificationReport, VerificationError> {
    let bundle: PublicReplayBundle = serde_json::from_str(json)?;
    verify_bundle(&bundle)
}

pub fn verify_v2_json(json: &str) -> Result<VerificationReport, VerificationError> {
    let bundle: EvidenceBundleV2 = serde_json::from_str(json)?;
    verify_v2_bundle(&bundle)
}

pub fn verify_any_json(json: &str) -> Result<VerificationReport, VerificationError> {
    let value: serde_json::Value = serde_json::from_str(json)?;
    if value
        .get("schemaVersion")
        .and_then(serde_json::Value::as_str)
        == Some(EVIDENCE_V2)
    {
        verify_v2_json(json)
    } else {
        verify_json(json)
    }
}

pub fn verify_file(path: impl AsRef<Path>) -> Result<VerificationReport, VerificationError> {
    let bytes = std::fs::read(path)?;
    verify_any_json(std::str::from_utf8(&bytes).map_err(|_| reject("EVIDENCE_NOT_UTF8"))?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ml_spec_types::{
        AttackObservation, EVIDENCE_V2, EvidenceNetwork, Head, PrivacyBoundary,
        RegistryObservation, SNAPSHOT_PROFILE_V2, SOURCE_DEMO_SPACE_V2_LOCAL,
        SOURCE_PROTOCOL_CORPUS_LOCAL, SPEC_NAME, SPEC_SNAPSHOT, SpecSnapshot, VerificationMetadata,
    };

    fn current_bundle() -> PublicReplayBundle {
        let path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../evidence/local/memory_lineage_evm_evidence.json"
        );
        let bytes = std::fs::read(path).expect("current evidence exists");
        serde_json::from_slice(&bytes).expect("current evidence is typed")
    }

    fn current_v2_bundle() -> EvidenceBundleV2 {
        let bundle = current_bundle();
        let last = bundle.valid_history.last().expect("history is non-empty");
        EvidenceBundleV2 {
            schema_version: EVIDENCE_V2.to_owned(),
            evidence_type: "memorylineage_evidence_v2".to_owned(),
            source_class: SOURCE_PROTOCOL_CORPUS_LOCAL.to_owned(),
            network: EvidenceNetwork {
                name: "published-local-evidence".to_owned(),
                chain_id: bundle.chain_id,
            },
            registry: RegistryObservation {
                address: bundle.registry_address,
                code_hash: Some(bundle.registry_bytecode_keccak256),
                space_id: last.delta.space_id.clone(),
            },
            spec: SpecSnapshot {
                name: SPEC_NAME.to_owned(),
                snapshot: SPEC_SNAPSHOT.to_owned(),
                vector_hash: None,
            },
            head: Head {
                transition_id: last.transition_id.clone(),
                state_root: last.next_state_root.clone(),
                sequence: last.delta.sequence,
            },
            transitions: bundle.valid_history,
            authorization_proofs: Vec::new(),
            authorization_history: bundle.authority_history,
            observations: Vec::new(),
            attack: None,
            privacy: PrivacyBoundary {
                raw_memory_on_chain: false,
            },
            verification_metadata: VerificationMetadata {
                producer: "test".to_owned(),
                generated_at: None,
                legacy_source: None,
            },
        }
    }

    #[test]
    fn current_bundle_is_verified() {
        let report = verify_bundle(&current_bundle()).expect("published corpus should verify");
        assert_eq!(report.verdict, "VERIFIED");
        assert_eq!(report.transition_count, 4);
        assert_eq!(report.rejected_mutation_count, 20);
        assert_eq!(report.authority_history, "STRUCTURE_ONLY");
        assert_eq!(report.authorization_proof, "NOT_INCLUDED");
    }

    #[test]
    fn tampered_transition_fails_closed() {
        let mut bundle = current_bundle();
        bundle.valid_history[1].transition_id = ZERO_ROOT.to_owned();
        let error = verify_bundle(&bundle).expect_err("tampered evidence must fail");
        assert!(matches!(
            error,
            VerificationError::Rejected(message) if message == "TRANSITION_ID_MISMATCH"
        ));
    }

    #[test]
    fn v2_projection_is_verified_independently() {
        let report = verify_v2_bundle(&current_v2_bundle()).expect("v2 bundle should verify");
        assert_eq!(report.verdict, "VERIFIED");
        assert_eq!(report.transition_count, 4);
        assert_eq!(report.rejected_mutation_count, 0);
        assert_eq!(report.attack_evidence, "NOT PRESENT");
        assert_eq!(report.authority_history, "STRUCTURE_ONLY");
        assert_eq!(report.authorization_proof, "NOT_INCLUDED");
    }

    #[test]
    fn demo_space_v2_recovers_each_eoa_authorizer_from_the_published_signature() {
        let path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../evidence/local/demo_space_v2_evidence.json"
        );
        let bytes = std::fs::read(path).expect("Demo Space V2 evidence exists");
        let bundle: EvidenceBundleV2 = serde_json::from_slice(&bytes)
            .expect("Demo Space V2 evidence with EOA proofs is typed");
        let report = verify_v2_bundle(&bundle).expect("Demo Space V2 proofs should verify");
        assert_eq!(report.authority_history, "TIMELINE_BOUND");
        assert_eq!(report.authorization_proof, "EOA_SIGNATURES_VERIFIED");
        assert_eq!(bundle.authorization_proofs.len(), bundle.transitions.len());
    }

    #[test]
    fn demo_space_v2_signature_tampering_fails_closed() {
        let path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../evidence/local/demo_space_v2_evidence.json"
        );
        let bytes = std::fs::read(path).expect("Demo Space V2 evidence exists");
        let mut bundle: EvidenceBundleV2 = serde_json::from_slice(&bytes)
            .expect("Demo Space V2 evidence with EOA proofs is typed");
        bundle.authorization_proofs[0].signature = format!("0x{}", "00".repeat(65));
        let error = verify_v2_bundle(&bundle).expect_err("tampered signature must fail");
        assert!(matches!(
            error,
            VerificationError::Rejected(message) if message == "AUTHORIZATION_SIGNATURE_INVALID"
        ));
    }

    #[test]
    fn demo_space_v2_rejects_a_valid_history_signer_at_the_wrong_sequence() {
        let path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../evidence/local/demo_space_v2_evidence.json"
        );
        let bytes = std::fs::read(path).expect("Demo Space V2 evidence exists");
        let mut bundle: EvidenceBundleV2 = serde_json::from_slice(&bytes)
            .expect("Demo Space V2 evidence with EOA proofs is typed");
        bundle.authorization_proofs[0].authorizer =
            bundle.authorization_proofs[2].authorizer.clone();
        bundle.authorization_proofs[0].config_nonce = Some(1);
        let error = verify_v2_bundle(&bundle)
            .expect_err("a rotated signer must not authorize an earlier sequence");
        assert!(matches!(
            error,
            VerificationError::Rejected(message) if message == "AUTHORIZATION_ACTIVE_SIGNER_MISMATCH"
        ));
    }

    #[test]
    fn recovery_receipt_classifies_current_historical_and_unknown_candidates() {
        let bundle = current_v2_bundle();
        let current_commitment = bundle
            .transitions
            .last()
            .expect("history is non-empty")
            .delta
            .delta_commitment
            .clone();
        let current = build_recovery_receipt(
            &bundle,
            &current_commitment,
            4,
            "PROTOCOL_CORPUS_LOCAL",
            None,
        )
        .expect("current candidate should produce a receipt");
        assert_eq!(current.decision.classification, RECOVERY_CURRENT_HEAD);
        assert_eq!(current.decision.recommended_action, RECOVERY_RESUME_ALLOWED);
        assert_eq!(
            verify_recovery_receipt(&current, &bundle)
                .expect("current receipt should verify")
                .verdict,
            "VERIFIED"
        );

        let historical_commitment = bundle.transitions[0].delta.delta_commitment.clone();
        let historical = build_recovery_receipt(
            &bundle,
            &historical_commitment,
            1,
            "PROTOCOL_CORPUS_LOCAL",
            None,
        )
        .expect("historical candidate should produce a receipt");
        assert_eq!(
            historical.decision.classification,
            RECOVERY_HISTORICAL_CHECKPOINT
        );
        assert_eq!(
            historical.decision.recommended_action,
            RECOVERY_REHEARSE_ONLY
        );

        let unknown = build_recovery_receipt(
            &bundle,
            &format!("0x{}", "11".repeat(32)),
            99,
            "PROTOCOL_CORPUS_LOCAL",
            None,
        )
        .expect("unknown candidate should produce a held receipt");
        assert_eq!(
            unknown.decision.classification,
            RECOVERY_UNKNOWN_OR_DIVERGED
        );
        assert_eq!(
            unknown.decision.recommended_action,
            RECOVERY_HOLD_FOR_REVIEW
        );
    }

    #[test]
    fn recovery_receipt_supports_the_explicit_v2_snapshot_profile() {
        let bundle = current_v2_bundle();
        let candidate = bundle
            .transitions
            .last()
            .expect("history is non-empty")
            .delta
            .delta_commitment
            .clone();
        let receipt = build_recovery_receipt_with_snapshot_profile(
            &bundle,
            &candidate,
            bundle.head.sequence,
            "PROTOCOL_CORPUS_LOCAL",
            None,
            SNAPSHOT_PROFILE_V2,
        )
        .expect("the explicit V2 profile should produce a receipt");

        assert_eq!(receipt.candidate.snapshot_profile, SNAPSHOT_PROFILE_V2);
        assert_eq!(
            verify_recovery_receipt(&receipt, &bundle)
                .expect("the V2-profile receipt should verify")
                .verdict,
            "VERIFIED"
        );
    }

    #[test]
    fn recovery_receipt_rejects_tampered_decision() {
        let bundle = current_v2_bundle();
        let candidate = bundle.transitions[0].delta.delta_commitment.clone();
        let mut receipt = build_recovery_receipt(
            &bundle,
            &candidate,
            1,
            "PROTOCOL_CORPUS_LOCAL",
            Some(RecoveryBlockContext {
                tag: "finalized".to_owned(),
                number: 123,
                hash: format!("0x{}", "22".repeat(32)),
            }),
        )
        .expect("receipt should build");
        receipt.decision.recommended_action = RECOVERY_RESUME_ALLOWED.to_owned();
        let error = verify_recovery_receipt(&receipt, &bundle)
            .expect_err("a changed recovery action must fail closed");
        assert!(matches!(
            error,
            VerificationError::Rejected(message) if message == "RECOVERY_DECISION_MISMATCH"
        ));
    }

    #[test]
    fn recovery_receipt_rejects_tampered_candidate_commitment() {
        let bundle = current_v2_bundle();
        let candidate = bundle.transitions[0].delta.delta_commitment.clone();
        let mut receipt =
            build_recovery_receipt(&bundle, &candidate, 1, "PROTOCOL_CORPUS_LOCAL", None)
                .expect("receipt should build");
        receipt.candidate.candidate_commitment = format!("0x{}", "33".repeat(32));
        let error = verify_recovery_receipt(&receipt, &bundle)
            .expect_err("a changed candidate must fail closed");
        assert!(matches!(
            error,
            VerificationError::Rejected(message) if message == "RECOVERY_DECISION_MISMATCH"
        ));
    }

    #[test]
    fn recovery_receipt_binds_the_named_policy() {
        let bundle = current_v2_bundle();
        let candidate = bundle
            .transitions
            .last()
            .expect("history is non-empty")
            .delta
            .delta_commitment
            .clone();
        let mut receipt = build_recovery_receipt(
            &bundle,
            &candidate,
            bundle.head.sequence,
            SOURCE_PROTOCOL_CORPUS_LOCAL,
            None,
        )
        .expect("receipt should build");
        receipt.policy_id = "allow-any-restore".to_owned();
        let error = verify_recovery_receipt(&receipt, &bundle)
            .expect_err("a changed policy must fail closed");
        assert!(matches!(
            error,
            VerificationError::Rejected(message) if message == "RECOVERY_POLICY_MISMATCH"
        ));
    }

    #[test]
    fn recovery_receipt_cannot_relabel_a_bundle_source() {
        let bundle = current_v2_bundle();
        let candidate = bundle
            .transitions
            .last()
            .expect("history is non-empty")
            .delta
            .delta_commitment
            .clone();
        let error = build_recovery_receipt(
            &bundle,
            &candidate,
            bundle.head.sequence,
            SOURCE_DEMO_SPACE_V2_LOCAL,
            None,
        )
        .expect_err("a protocol corpus cannot be relabeled as the demo source");
        assert!(matches!(
            error,
            VerificationError::Rejected(message) if message == "RECOVERY_SOURCE_CLASS_MISMATCH"
        ));
    }

    #[test]
    fn extended_attack_evidence_is_bound_to_an_earlier_canonical_root() {
        let mut bundle = current_v2_bundle();
        let stale = bundle.transitions[0].next_state_root.clone();
        bundle.attack = Some(AttackObservation {
            name: "silent-rollback".to_owned(),
            status: "REJECTED".to_owned(),
            reason: Some("BAD_PREVIOUS_STATE".to_owned()),
            restored_snapshot_sequence: Some(1),
            attempted_sequence: Some(bundle.head.sequence + 1),
            stale_predecessor: Some(stale.clone()),
            canonical_predecessor: Some(bundle.head.state_root.clone()),
            execution_source: Some("Rust/revm test execution".to_owned()),
            transaction_broadcast: Some(false),
            fixture_id: Some("silent-rollback-v2".to_owned()),
        });

        let report = verify_v2_bundle(&bundle).expect("coherent attack evidence should verify");
        assert_eq!(report.attack_evidence, "PASS");

        let attack = bundle.attack.as_mut().expect("attack was set above");
        attack.stale_predecessor = Some(ZERO_ROOT.to_owned());
        let error = verify_v2_bundle(&bundle).expect_err("unbound stale root must fail closed");
        assert!(matches!(
            error,
            VerificationError::Rejected(message)
                if message == "ATTACK_STALE_PREDECESSOR_MISMATCH"
        ));
    }

    #[test]
    fn extended_attack_evidence_rejects_broadcast_claims() {
        let mut bundle = current_v2_bundle();
        let stale = bundle.transitions[0].next_state_root.clone();
        bundle.attack = Some(AttackObservation {
            name: "silent-rollback".to_owned(),
            status: "REJECTED".to_owned(),
            reason: Some("BAD_PREVIOUS_STATE".to_owned()),
            restored_snapshot_sequence: Some(1),
            attempted_sequence: Some(bundle.head.sequence + 1),
            stale_predecessor: Some(stale),
            canonical_predecessor: Some(bundle.head.state_root.clone()),
            execution_source: Some("Rust/revm test execution".to_owned()),
            transaction_broadcast: Some(true),
            fixture_id: Some("silent-rollback-v2".to_owned()),
        });

        let error = verify_v2_bundle(&bundle).expect_err("broadcast claim must be rejected");
        assert!(matches!(
            error,
            VerificationError::Rejected(message)
                if message == "ATTACK_BROADCAST_STATUS_MISMATCH"
        ));
    }

    #[test]
    fn auto_detected_json_verifies_v2_bundle() {
        let json = serde_json::to_string(&current_v2_bundle()).expect("v2 bundle serializes");
        let report = verify_any_json(&json).expect("auto-detected v2 bundle should verify");
        assert_eq!(report.verdict, "VERIFIED");
        assert_eq!(report.transition_count, 4);
    }

    #[test]
    fn portability_report_is_replayable_by_the_independent_verifier() {
        let path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../evidence/local/polkadot_hub_portability_rehearsal.json"
        );
        let bytes = std::fs::read(path).expect("portability report exists");
        let report = verify_portability_json(
            std::str::from_utf8(&bytes).expect("portability report is UTF-8"),
        )
        .expect("the published portability report should replay independently");

        assert_eq!(report.verdict, "VERIFIED");
        assert_eq!(report.observation_count, 2);
        assert_eq!(report.transition_projection, "MATCH");
        assert_eq!(report.stale_predecessor_rejection, "BAD_PREVIOUS_STATE");
    }

    #[test]
    fn portability_report_rejects_tampered_nested_evidence() {
        let path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../evidence/local/polkadot_hub_portability_rehearsal.json"
        );
        let bytes = std::fs::read(path).expect("portability report exists");
        let mut value: serde_json::Value =
            serde_json::from_slice(&bytes).expect("portability report is JSON");
        value["observations"][1]["evidence"]["head"]["stateRoot"] =
            serde_json::Value::String(format!("0x{}", "44".repeat(32)));

        let error = verify_portability_json(
            &serde_json::to_string(&value).expect("tampered report serializes"),
        )
        .expect_err("a changed nested head must fail closed");
        assert!(matches!(
            error,
            VerificationError::Rejected(message)
                if message == "PORTABILITY_POLKADOT_BUNDLE_INVALID:HEAD_RECONSTRUCTION_MISMATCH"
        ));
    }
}
