#![forbid(unsafe_code)]

pub mod submission;

use ml_spec_types::{
    EVIDENCE_V2, EvidenceBundleV2, EvidenceNetwork, Head, PrivacyBoundary, PublicReplayBundle,
    RegistryObservation, SOURCE_PROTOCOL_CORPUS_LOCAL, SPEC_NAME, SPEC_SNAPSHOT, SpecSnapshot,
    VerificationMetadata,
};
use std::path::Path;
use thiserror::Error;

pub const PUBLIC_REPLAY_BUNDLE: &str = "public_commitment_replay_bundle";
pub const INSPECTOR_EXPORT_V1: &str = "memorylineage-evidence-v1";
pub const INSPECTOR_EXPORT_V2: &str = "memorylineage-evidence-v2";

#[derive(Debug, Error)]
pub enum EvidenceError {
    #[error("could not read evidence file: {0}")]
    Io(#[from] std::io::Error),
    #[error("invalid evidence JSON: {0}")]
    Json(#[from] serde_json::Error),
}

pub fn load_public_replay_bundle(
    path: impl AsRef<Path>,
) -> Result<PublicReplayBundle, EvidenceError> {
    let bytes = std::fs::read(path)?;
    Ok(serde_json::from_slice(&bytes)?)
}

pub fn parse_public_replay_bundle(json: &str) -> Result<PublicReplayBundle, EvidenceError> {
    Ok(serde_json::from_str(json)?)
}

pub fn write_public_replay_bundle(
    bundle: &PublicReplayBundle,
    path: impl AsRef<Path>,
) -> Result<(), EvidenceError> {
    let json = serde_json::to_vec_pretty(bundle)?;
    std::fs::write(path, [json, b"\n".to_vec()].concat())?;
    Ok(())
}

pub fn public_replay_to_v2(bundle: &PublicReplayBundle) -> EvidenceBundleV2 {
    let last = bundle
        .valid_history
        .last()
        .expect("public replay bundles require a non-empty valid history");
    EvidenceBundleV2 {
        schema_version: EVIDENCE_V2.to_owned(),
        evidence_type: "memorylineage_evidence_v2".to_owned(),
        source_class: SOURCE_PROTOCOL_CORPUS_LOCAL.to_owned(),
        network: EvidenceNetwork {
            name: "published-local-evidence".to_owned(),
            chain_id: bundle.chain_id.clone(),
        },
        registry: RegistryObservation {
            address: bundle.registry_address.clone(),
            code_hash: Some(bundle.registry_bytecode_keccak256.clone()),
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
        transitions: bundle.valid_history.clone(),
        authorization_proofs: Vec::new(),
        authorization_history: bundle.authority_history.clone(),
        observations: Vec::new(),
        attack: None,
        privacy: PrivacyBoundary {
            raw_memory_on_chain: bundle.raw_payload_stored,
        },
        verification_metadata: VerificationMetadata {
            producer: "ml-evidence::public-replay-to-v2".to_owned(),
            generated_at: None,
            legacy_source: Some(PUBLIC_REPLAY_BUNDLE.to_owned()),
        },
    }
}

pub fn write_v2_bundle(
    bundle: &EvidenceBundleV2,
    path: impl AsRef<Path>,
) -> Result<(), EvidenceError> {
    let json = serde_json::to_vec_pretty(bundle)?;
    std::fs::write(path, [json, b"\n".to_vec()].concat())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn current_public_bundle_is_typed() {
        let bundle = load_public_replay_bundle(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../evidence/local/memory_lineage_evm_evidence.json"
        ))
        .expect("current public evidence should remain readable");
        assert_eq!(bundle.evidence_type, PUBLIC_REPLAY_BUNDLE);
        assert_eq!(bundle.valid_history.len(), 4);
        assert_eq!(bundle.mutation_matrix.len(), 21);
    }

    #[test]
    fn current_public_bundle_has_a_versioned_v2_projection() {
        let bundle = load_public_replay_bundle(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../evidence/local/memory_lineage_evm_evidence.json"
        ))
        .expect("current public evidence should remain readable");
        let v2 = public_replay_to_v2(&bundle);
        assert_eq!(v2.schema_version, "memorylineage-evidence-v2");
        assert_eq!(v2.transitions.len(), 4);
        assert_eq!(v2.head.sequence, 4);
        let serialized = serde_json::to_string(&v2).expect("v2 should serialize");
        assert!(!serialized.contains("raw memory"));
    }
}
