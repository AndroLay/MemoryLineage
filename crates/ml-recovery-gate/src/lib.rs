#![forbid(unsafe_code)]

use ml_memory_store::{MemorySnapshot, MemoryStoreError, read_snapshot, snapshot_commitment};
use ml_spec_types::{
    EvidenceBundleV2, RECOVERY_BLOCK_UNVERIFIED, RECOVERY_HOLD_FOR_REVIEW, RECOVERY_REHEARSE_ONLY,
    RECOVERY_RESUME_ALLOWED, RecoveryDecisionReceipt,
};
use ml_verifier_independent::{VerificationError, build_recovery_receipt, verify_recovery_receipt};
use std::path::Path;
use thiserror::Error;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProtectedResume<T> {
    pub snapshot_sequence: u64,
    pub receipt: RecoveryDecisionReceipt,
    pub loaded: T,
}

#[derive(Debug, Error)]
pub enum ProtectedResumeError {
    #[error("could not read recovery snapshot: {0}")]
    Snapshot(#[from] MemoryStoreError),
    #[error("recovery receipt could not be built or verified: {0}")]
    Verification(#[from] VerificationError),
    #[error("protected resume held: {classification} / {action}")]
    ResumeHeld {
        classification: String,
        action: String,
        receipt: Box<RecoveryDecisionReceipt>,
    },
    #[error("protected loader failed: {0}")]
    Loader(String),
}

/// Read a private snapshot, derive its commitment, and produce a decision
/// receipt without invoking a loader. This is the policy boundary used by the
/// protected adapter and by tooling that only wants a preflight result.
pub fn preflight_snapshot(
    snapshot_path: impl AsRef<Path>,
    evidence: &EvidenceBundleV2,
    source_class: &str,
) -> Result<(MemorySnapshot, RecoveryDecisionReceipt), ProtectedResumeError> {
    let snapshot = read_snapshot(snapshot_path)?;
    let receipt = build_recovery_receipt(
        evidence,
        &snapshot_commitment(&snapshot),
        snapshot.sequence,
        source_class,
        None,
    )?;
    verify_recovery_receipt(&receipt, evidence)?;
    Ok((snapshot, receipt))
}

/// Resume through a policy gate. The loader callback is called only after the
/// receipt has been independently verified and its action is exactly
/// `RESUME_ALLOWED`. Historical, divergent, and unverified snapshots return a
/// portable receipt in `ResumeHeld` and never reach the callback.
pub fn protected_resume<T, F>(
    snapshot_path: impl AsRef<Path>,
    evidence: &EvidenceBundleV2,
    source_class: &str,
    loader: F,
) -> Result<ProtectedResume<T>, ProtectedResumeError>
where
    F: FnOnce(&MemorySnapshot) -> Result<T, String>,
{
    let (snapshot, receipt) = preflight_snapshot(snapshot_path, evidence, source_class)?;
    if receipt.decision.recommended_action != RECOVERY_RESUME_ALLOWED {
        return Err(ProtectedResumeError::ResumeHeld {
            classification: receipt.decision.classification.clone(),
            action: receipt.decision.recommended_action.clone(),
            receipt: Box::new(receipt),
        });
    }

    let loaded = loader(&snapshot).map_err(ProtectedResumeError::Loader)?;
    Ok(ProtectedResume {
        snapshot_sequence: snapshot.sequence,
        receipt,
        loaded,
    })
}

pub fn is_protected_resume_hold(receipt: &RecoveryDecisionReceipt) -> bool {
    matches!(
        receipt.decision.recommended_action.as_str(),
        RECOVERY_REHEARSE_ONLY | RECOVERY_HOLD_FOR_REVIEW | RECOVERY_BLOCK_UNVERIFIED
    )
}

#[cfg(test)]
mod tests {
    use super::{ProtectedResumeError, is_protected_resume_hold, protected_resume};
    use ml_memory_store::write_snapshot;
    use ml_spec_types::{EvidenceBundleV2, SOURCE_DEMO_SPACE_V2_LOCAL};
    use std::collections::BTreeMap;
    use std::path::PathBuf;
    use std::sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    };

    const EVIDENCE: &str = include_str!("../../../evidence/local/demo_space_v2_evidence.json");

    fn evidence() -> EvidenceBundleV2 {
        serde_json::from_str(EVIDENCE).expect("fixture evidence is valid V2")
    }

    fn snapshot(sequence: u64) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!(
            "../../fixtures/silent-rollback-v2/snapshot-{sequence}.db"
        ))
    }

    #[test]
    fn current_head_is_the_only_fixture_that_reaches_loader() {
        let called = Arc::new(AtomicBool::new(false));
        let marker = called.clone();
        let result = protected_resume(
            snapshot(3),
            &evidence(),
            SOURCE_DEMO_SPACE_V2_LOCAL,
            move |snapshot| {
                marker.store(true, Ordering::SeqCst);
                Ok(snapshot.sequence)
            },
        )
        .expect("current head should pass the reference gate");

        assert_eq!(result.snapshot_sequence, 3);
        assert_eq!(result.loaded, 3);
        assert!(called.load(Ordering::SeqCst));
        assert_eq!(result.receipt.decision.recommended_action, "RESUME_ALLOWED");
    }

    #[test]
    fn historical_checkpoint_is_held_before_loader() {
        let called = Arc::new(AtomicBool::new(false));
        let marker = called.clone();
        let result = protected_resume(
            snapshot(1),
            &evidence(),
            SOURCE_DEMO_SPACE_V2_LOCAL,
            move |_| {
                marker.store(true, Ordering::SeqCst);
                Ok(())
            },
        );

        match result {
            Err(ProtectedResumeError::ResumeHeld { receipt, .. }) => {
                assert_eq!(
                    receipt.decision.classification,
                    "KNOWN_HISTORICAL_CHECKPOINT"
                );
                assert_eq!(receipt.decision.recommended_action, "REHEARSE_ONLY");
                assert!(is_protected_resume_hold(&receipt));
            }
            other => panic!("expected a protected hold, got {other:?}"),
        }
        assert!(!called.load(Ordering::SeqCst));
    }

    #[test]
    fn diverged_snapshot_is_held_and_invalid_evidence_fails_closed() {
        let path = std::env::temp_dir().join(format!(
            "memorylineage-recovery-gate-diverged-{}.db",
            std::process::id()
        ));
        let mut values = BTreeMap::new();
        values.insert("unexpected".to_owned(), "local-only-state".to_owned());
        write_snapshot(&path, 3, &values).expect("temporary diverged snapshot should be writable");

        let result = protected_resume(
            &path,
            &evidence(),
            SOURCE_DEMO_SPACE_V2_LOCAL,
            |_| -> Result<(), String> { panic!("diverged snapshot must not reach the loader") },
        );
        match result {
            Err(ProtectedResumeError::ResumeHeld { receipt, .. }) => {
                assert_eq!(receipt.decision.classification, "UNKNOWN_OR_DIVERGED");
                assert_eq!(receipt.decision.recommended_action, "HOLD_FOR_REVIEW");
            }
            other => panic!("expected diverged snapshot hold, got {other:?}"),
        }
        let _ = std::fs::remove_file(path);

        let mut invalid = evidence();
        invalid.transitions[0].transition_id = "0x00".to_owned();
        let result = protected_resume(
            snapshot(3),
            &invalid,
            SOURCE_DEMO_SPACE_V2_LOCAL,
            |_| -> Result<(), String> { panic!("invalid evidence must not reach the loader") },
        );
        assert!(matches!(result, Err(ProtectedResumeError::Verification(_))));
    }
}
