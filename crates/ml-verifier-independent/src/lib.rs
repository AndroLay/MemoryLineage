#![forbid(unsafe_code)]

use ml_spec_types::{
    EVIDENCE_V2, EvidenceBundleV2, PublicReplayBundle, SPEC_NAME, SPEC_SNAPSHOT, TransitionRecord,
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
    pub head_reconstruction: String,
    pub privacy_boundary: String,
    pub attack_evidence: String,
    pub transition_count: usize,
    pub rejected_mutation_count: usize,
    pub final_root: String,
}

fn reject(message: impl Into<String>) -> VerificationError {
    VerificationError::Rejected(message.into())
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
        authority_history: "PASS".to_owned(),
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
        previous_nonce = Some(authority.config_nonce);
    }
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
        authority_history: "PASS".to_owned(),
        head_reconstruction: "MATCH".to_owned(),
        privacy_boundary: "PASS".to_owned(),
        attack_evidence,
        transition_count: bundle.transitions.len(),
        rejected_mutation_count: 0,
        final_root: previous_root,
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
        RegistryObservation, SPEC_NAME, SPEC_SNAPSHOT, SpecSnapshot, VerificationMetadata,
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
}
