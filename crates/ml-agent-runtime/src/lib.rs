#![forbid(unsafe_code)]

use ml_recovery_gate::{ProtectedResumeError, protected_resume_with_profile};
use ml_spec_types::{
    EvidenceBundleV2, RecoveryDecisionReceipt, SNAPSHOT_PROFILE_V1, SNAPSHOT_PROFILE_V2,
    SOURCE_DEMO_SPACE_V2_LOCAL,
};
use std::path::Path;
use thiserror::Error;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AgentMemorySession {
    sequence: u64,
    values: std::collections::BTreeMap<String, String>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct RuntimeResumeOutcome {
    pub runtime: String,
    pub status: String,
    pub classification: String,
    pub recommended_action: String,
    pub snapshot_sequence: u64,
    pub candidate_commitment: String,
    pub loader_invoked: bool,
    pub raw_memory_exported: bool,
}

#[derive(Debug, Error)]
pub enum RuntimeError {
    #[error("protected resume failed: {0}")]
    Gate(#[from] ProtectedResumeError),
}

pub struct ReferenceAgentRuntime {
    active: Option<AgentMemorySession>,
    loader_invocations: u64,
}

impl ReferenceAgentRuntime {
    pub fn new() -> Self {
        Self {
            active: None,
            loader_invocations: 0,
        }
    }

    pub fn resume(
        &mut self,
        snapshot_path: impl AsRef<Path>,
        evidence: &EvidenceBundleV2,
        source_class: &str,
    ) -> Result<RuntimeResumeOutcome, RuntimeError> {
        let profile = if source_class == SOURCE_DEMO_SPACE_V2_LOCAL {
            SNAPSHOT_PROFILE_V2
        } else {
            SNAPSHOT_PROFILE_V1
        };
        self.resume_with_profile(snapshot_path, evidence, source_class, profile)
    }

    pub fn resume_with_profile(
        &mut self,
        snapshot_path: impl AsRef<Path>,
        evidence: &EvidenceBundleV2,
        source_class: &str,
        snapshot_profile: &str,
    ) -> Result<RuntimeResumeOutcome, RuntimeError> {
        self.active = None;
        let result = protected_resume_with_profile(
            snapshot_path,
            evidence,
            source_class,
            snapshot_profile,
            |snapshot| {
                Ok::<AgentMemorySession, String>(AgentMemorySession {
                    sequence: snapshot.sequence,
                    values: snapshot.values.clone(),
                })
            },
        );

        match result {
            Ok(resumed) => {
                self.loader_invocations += 1;
                let outcome = outcome_from_receipt(&resumed.receipt, "RESUMED", true);
                self.active = Some(resumed.loaded);
                Ok(outcome)
            }
            Err(ProtectedResumeError::ResumeHeld { receipt, .. }) => {
                Ok(outcome_from_receipt(&receipt, "HELD", false))
            }
            Err(error) => Err(RuntimeError::Gate(error)),
        }
    }

    pub fn active_sequence(&self) -> Option<u64> {
        self.active.as_ref().map(|session| session.sequence)
    }

    pub fn active_value(&self, key: &str) -> Option<&str> {
        self.active
            .as_ref()
            .and_then(|session| session.values.get(key).map(String::as_str))
    }

    pub fn loader_invocations(&self) -> u64 {
        self.loader_invocations
    }
}

fn outcome_from_receipt(
    receipt: &RecoveryDecisionReceipt,
    status: &str,
    loader_invoked: bool,
) -> RuntimeResumeOutcome {
    RuntimeResumeOutcome {
        runtime: "memorylineage-reference-agent-runtime-v1".to_owned(),
        status: status.to_owned(),
        classification: receipt.decision.classification.clone(),
        recommended_action: receipt.decision.recommended_action.clone(),
        snapshot_sequence: receipt.candidate.snapshot_sequence,
        candidate_commitment: receipt.candidate.candidate_commitment.clone(),
        loader_invoked,
        raw_memory_exported: false,
    }
}

impl Default for ReferenceAgentRuntime {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::ReferenceAgentRuntime;
    use ml_spec_types::SOURCE_DEMO_SPACE_V2_LOCAL;
    use std::path::PathBuf;

    const EVIDENCE: &str = include_str!("../../../evidence/local/demo_space_v2_evidence.json");

    fn evidence() -> ml_spec_types::EvidenceBundleV2 {
        serde_json::from_str(EVIDENCE).expect("Demo Space V2 evidence is valid")
    }

    fn snapshot(sequence: u64) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!(
            "../../fixtures/silent-rollback-v2/snapshot-{sequence}.db"
        ))
    }

    #[test]
    fn current_head_is_loaded_into_a_real_runtime_session() {
        let mut runtime = ReferenceAgentRuntime::new();
        let outcome = runtime
            .resume(snapshot(3), &evidence(), SOURCE_DEMO_SPACE_V2_LOCAL)
            .expect("current head should be resumable");

        assert_eq!(outcome.status, "RESUMED");
        assert_eq!(outcome.recommended_action, "RESUME_ALLOWED");
        assert!(outcome.loader_invoked);
        assert!(!outcome.raw_memory_exported);
        assert_eq!(runtime.active_sequence(), Some(3));
        assert_eq!(runtime.active_value("privacy"), Some("raw-memory-private"));
        assert_eq!(runtime.loader_invocations(), 1);
    }

    #[test]
    fn historical_snapshot_is_held_before_the_runtime_loader() {
        let mut runtime = ReferenceAgentRuntime::new();
        let outcome = runtime
            .resume(snapshot(1), &evidence(), SOURCE_DEMO_SPACE_V2_LOCAL)
            .expect("historical snapshot should produce a decision");

        assert_eq!(outcome.status, "HELD");
        assert_eq!(outcome.classification, "KNOWN_HISTORICAL_CHECKPOINT");
        assert_eq!(outcome.recommended_action, "REHEARSE_ONLY");
        assert!(!outcome.loader_invoked);
        assert_eq!(runtime.active_sequence(), None);
        assert_eq!(runtime.loader_invocations(), 0);
    }

    #[test]
    fn current_head_without_authorization_proofs_is_held_before_the_loader() {
        let mut unsigned = evidence();
        unsigned.authorization_proofs.clear();
        let mut runtime = ReferenceAgentRuntime::new();
        let outcome = runtime
            .resume(snapshot(3), &unsigned, SOURCE_DEMO_SPACE_V2_LOCAL)
            .expect("missing authorization should produce a held decision");

        assert_eq!(outcome.status, "HELD");
        assert_eq!(outcome.classification, "CURRENT_HEAD");
        assert_eq!(outcome.recommended_action, "BLOCK_UNVERIFIED");
        assert!(!outcome.loader_invoked);
        assert!(runtime.active_sequence().is_none());
        assert_eq!(runtime.loader_invocations(), 0);
    }

    #[test]
    fn divergent_snapshot_and_invalid_evidence_fail_closed() {
        let path = std::env::temp_dir().join(format!(
            "memorylineage-reference-runtime-diverged-{}.db",
            std::process::id()
        ));
        let values = [("unexpected".to_owned(), "local-only-state".to_owned())]
            .into_iter()
            .collect();
        ml_memory_store::write_snapshot(&path, 3, &values)
            .expect("divergent snapshot should be writable");

        let mut runtime = ReferenceAgentRuntime::new();
        let outcome = runtime
            .resume(&path, &evidence(), SOURCE_DEMO_SPACE_V2_LOCAL)
            .expect("divergent snapshot should yield a hold decision");
        assert_eq!(outcome.status, "HELD");
        assert_eq!(outcome.classification, "UNKNOWN_OR_DIVERGED");
        assert_eq!(outcome.recommended_action, "HOLD_FOR_REVIEW");
        assert!(!outcome.loader_invoked);
        assert_eq!(runtime.loader_invocations(), 0);

        let mut invalid = evidence();
        invalid.transitions[0].transition_id = "0x00".to_owned();
        let error = runtime.resume(&path, &invalid, SOURCE_DEMO_SPACE_V2_LOCAL);
        assert!(error.is_err(), "invalid evidence must fail closed");
        let _ = std::fs::remove_file(path);
    }
}
