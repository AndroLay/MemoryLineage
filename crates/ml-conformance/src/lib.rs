#![forbid(unsafe_code)]

use ml_core::{ZERO_ROOT, next_state_root, space_id, transition_and_root};
use ml_spec_types::ExperienceDelta;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConformanceError {
    #[error("invalid pinned vector: {0}")]
    Vector(#[from] serde_json::Error),
    #[error("reference implementation mismatch: {0}")]
    Reference(String),
    #[error("independent evidence mismatch: {0}")]
    Independent(String),
    #[error("revm execution mismatch: {0}")]
    Revm(String),
}

#[derive(Clone, Debug, Deserialize)]
struct Vector {
    initial_controller: String,
    space_salt: String,
    profile_id: String,
    delta_commitment: String,
    provenance_commitment: String,
    locator_commitment: String,
    expected_space_id: String,
    expected_transition_id: String,
    expected_next_state_root: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ConformanceReport {
    pub pinned_vector: String,
    pub published_evidence: String,
    pub rust_reference: String,
    pub independent_verifier: String,
    pub revm_silent_rollback: String,
    pub revm_core_mutations: String,
    pub revm_erc1271: String,
    pub revm_authority_rotation: String,
    pub transition_id: String,
    pub next_state_root: String,
}

fn pinned_vector() -> Result<Vector, ConformanceError> {
    Ok(serde_json::from_str(include_str!(
        "../../../contracts/vectors/erc8350_conformance.json"
    ))?)
}

pub fn run_pinned() -> Result<ConformanceReport, ConformanceError> {
    let vector = pinned_vector()?;
    let derived_space = space_id(&vector.initial_controller, &vector.space_salt)
        .map_err(|error| ConformanceError::Reference(error.to_string()))?;
    if derived_space != vector.expected_space_id {
        return Err(ConformanceError::Reference("SPACE_ID_MISMATCH".to_owned()));
    }

    let delta = ExperienceDelta {
        space_id: vector.expected_space_id.clone(),
        sequence: 1,
        prev_state_root: ZERO_ROOT.to_owned(),
        delta_commitment: vector.delta_commitment,
        provenance_commitment: vector.provenance_commitment,
        profile_id: vector.profile_id,
        locator_commitment: vector.locator_commitment,
    };
    let (transition, root) = transition_and_root(&delta)
        .map_err(|error| ConformanceError::Reference(error.to_string()))?;
    if transition != vector.expected_transition_id || root != vector.expected_next_state_root {
        return Err(ConformanceError::Reference(
            "TRANSITION_OR_ROOT_MISMATCH".to_owned(),
        ));
    }
    if next_state_root(&delta.prev_state_root, &transition)
        .map_err(|error| ConformanceError::Reference(error.to_string()))?
        != root
    {
        return Err(ConformanceError::Reference(
            "ROOT_REPLAY_MISMATCH".to_owned(),
        ));
    }

    let evidence_path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../evidence/local/memory_lineage_evm_evidence.json"
    );
    let evidence: ml_spec_types::PublicReplayBundle =
        serde_json::from_slice(&std::fs::read(evidence_path).map_err(|error| {
            ConformanceError::Independent(format!("evidence read failed: {error}"))
        })?)
        .map_err(|error| ConformanceError::Independent(error.to_string()))?;
    ml_verifier_independent::verify_bundle(&evidence)
        .map_err(|error| ConformanceError::Independent(error.to_string()))?;
    let revm = ml_local_evm::run_silent_rollback()
        .map_err(|error| ConformanceError::Revm(error.to_string()))?;
    if revm.status != "REJECTED" || revm.reason.as_deref() != Some("BAD_PREVIOUS_STATE") {
        return Err(ConformanceError::Revm(format!(
            "expected REJECTED/BAD_PREVIOUS_STATE, observed {}/{}",
            revm.status,
            revm.reason.as_deref().unwrap_or("NONE")
        )));
    }
    let mutations = ml_local_evm::run_core_mutations()
        .map_err(|error| ConformanceError::Revm(error.to_string()))?;
    if !mutations.all_expected {
        return Err(ConformanceError::Revm(format!(
            "core mutation matrix mismatch: {}/{} rejected with expected reasons",
            mutations.rejected, mutations.total
        )));
    }
    let erc1271 =
        ml_local_evm::run_erc1271().map_err(|error| ConformanceError::Revm(error.to_string()))?;
    if !erc1271.accepted || erc1271.rejected_reason != "INVALID_AUTHORIZATION" {
        return Err(ConformanceError::Revm(format!(
            "ERC-1271 mismatch: accepted={}, rejected_reason={}",
            erc1271.accepted, erc1271.rejected_reason
        )));
    }
    let authority = ml_local_evm::run_authority_rotation()
        .map_err(|error| ConformanceError::Revm(error.to_string()))?;
    if authority.config_nonce != 1
        || authority.old_authorizer_rejected != "INVALID_AUTHORIZATION"
        || !authority.new_authorizer_accepted
    {
        return Err(ConformanceError::Revm(format!(
            "authority rotation mismatch: nonce={}, old={}, new={}",
            authority.config_nonce,
            authority.old_authorizer_rejected,
            authority.new_authorizer_accepted
        )));
    }

    Ok(ConformanceReport {
        pinned_vector: "MATCH".to_owned(),
        published_evidence: "VERIFIED".to_owned(),
        rust_reference: "MATCH".to_owned(),
        independent_verifier: "MATCH".to_owned(),
        revm_silent_rollback: "REJECTED/BAD_PREVIOUS_STATE".to_owned(),
        revm_core_mutations: format!("{}/{} REJECTED", mutations.rejected, mutations.total),
        revm_erc1271: "ACCEPTED_THEN_REJECTED/INVALID_AUTHORIZATION".to_owned(),
        revm_authority_rotation: "NONCE_1/OLD_REJECTED/NEW_ACCEPTED".to_owned(),
        transition_id: transition,
        next_state_root: root,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pinned_report_is_all_match() {
        let report = run_pinned().expect("published vector and evidence must agree");
        assert_eq!(report.pinned_vector, "MATCH");
        assert_eq!(report.published_evidence, "VERIFIED");
        assert_eq!(report.rust_reference, "MATCH");
        assert_eq!(report.independent_verifier, "MATCH");
        assert_eq!(report.revm_core_mutations, "20/20 REJECTED");
        assert_eq!(
            report.revm_erc1271,
            "ACCEPTED_THEN_REJECTED/INVALID_AUTHORIZATION"
        );
        assert_eq!(
            report.revm_authority_rotation,
            "NONCE_1/OLD_REJECTED/NEW_ACCEPTED"
        );
    }
}
