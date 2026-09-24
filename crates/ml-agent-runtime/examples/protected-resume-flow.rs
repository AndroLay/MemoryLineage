#![forbid(unsafe_code)]

use ml_agent_runtime::{ReferenceAgentRuntime, RuntimeResumeOutcome};
use ml_memory_store::write_snapshot;
use ml_spec_types::{EvidenceBundleV2, SOURCE_DEMO_SPACE_V2_LOCAL};
use serde::Serialize;
use std::path::{Path, PathBuf};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct CaseReport {
    case: &'static str,
    snapshot_sequence: u64,
    classification: String,
    recommended_action: String,
    loader_invoked: bool,
    raw_memory_exported: bool,
    decision_receipt_produced: bool,
    result: String,
    active_session_sequence: Option<u64>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct FlowReport {
    report_type: &'static str,
    source_class: &'static str,
    all_expected: bool,
    cases: Vec<CaseReport>,
}

struct TemporarySnapshot(PathBuf);

impl Drop for TemporarySnapshot {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}

fn checked_case(
    name: &'static str,
    snapshot: &Path,
    evidence: &EvidenceBundleV2,
    classification: &str,
    action: &str,
    result: &str,
    active: Option<u64>,
) -> Result<CaseReport, Box<dyn std::error::Error>> {
    let mut runtime = ReferenceAgentRuntime::new();
    let outcome: RuntimeResumeOutcome =
        runtime.resume(snapshot, evidence, SOURCE_DEMO_SPACE_V2_LOCAL)?;
    let loader_invoked = runtime.loader_invocations() == 1;
    if outcome.classification != classification
        || outcome.recommended_action != action
        || outcome.status != result
        || outcome.loader_invoked != loader_invoked
        || outcome.raw_memory_exported
        || runtime.active_sequence() != active
        || loader_invoked != active.is_some()
    {
        return Err(
            format!("{name}: protected loader decision differed from expected policy").into(),
        );
    }
    Ok(CaseReport {
        case: name,
        snapshot_sequence: outcome.snapshot_sequence,
        classification: outcome.classification,
        recommended_action: outcome.recommended_action,
        loader_invoked,
        raw_memory_exported: outcome.raw_memory_exported,
        decision_receipt_produced: true,
        result: outcome.status,
        active_session_sequence: runtime.active_sequence(),
    })
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1);
    let repository_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let fixture_root = args
        .next()
        .map(PathBuf::from)
        .unwrap_or_else(|| repository_root.join("fixtures/silent-rollback-v2"));
    let evidence_path = args
        .next()
        .map(PathBuf::from)
        .unwrap_or_else(|| repository_root.join("evidence/local/demo_space_v2_evidence.json"));
    if args.next().is_some() {
        return Err("usage: protected-resume-flow [FIXTURE_ROOT] [EVIDENCE_JSON]".into());
    }
    let evidence: EvidenceBundleV2 = serde_json::from_slice(&std::fs::read(evidence_path)?)?;
    let mut cases = vec![
        checked_case(
            "currentHead",
            &fixture_root.join("snapshot-3.db"),
            &evidence,
            "CURRENT_HEAD",
            "RESUME_ALLOWED",
            "RESUMED",
            Some(3),
        )?,
        {
            let mut unsigned = evidence.clone();
            unsigned.authorization_proofs.clear();
            checked_case(
                "missingAuthorizationProof",
                &fixture_root.join("snapshot-3.db"),
                &unsigned,
                "CURRENT_HEAD",
                "BLOCK_UNVERIFIED",
                "HELD",
                None,
            )?
        },
        checked_case(
            "historicalCheckpoint",
            &fixture_root.join("snapshot-1.db"),
            &evidence,
            "KNOWN_HISTORICAL_CHECKPOINT",
            "REHEARSE_ONLY",
            "HELD",
            None,
        )?,
    ];

    let temporary = TemporarySnapshot(std::env::temp_dir().join(format!(
        "memorylineage-protected-resume-{}-{}.db",
        std::process::id(),
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)?.as_nanos()
    )));
    let values = [("unexpected".to_owned(), "synthetic-local-only".to_owned())]
        .into_iter()
        .collect();
    write_snapshot(&temporary.0, 3, &values)?;
    cases.push(checked_case(
        "divergedSnapshot",
        &temporary.0,
        &evidence,
        "UNKNOWN_OR_DIVERGED",
        "HOLD_FOR_REVIEW",
        "HELD",
        None,
    )?);

    let mut invalid = evidence.clone();
    invalid.transitions[0].transition_id = format!("0x{}", "00".repeat(32));
    let mut runtime = ReferenceAgentRuntime::new();
    if runtime
        .resume(
            fixture_root.join("snapshot-3.db"),
            &invalid,
            SOURCE_DEMO_SPACE_V2_LOCAL,
        )
        .is_ok()
        || runtime.loader_invocations() != 0
        || runtime.active_sequence().is_some()
    {
        return Err("invalid evidence reached or bypassed the protected loader".into());
    }
    cases.push(CaseReport {
        case: "invalidEvidence",
        snapshot_sequence: 3,
        classification: "VERIFICATION_ERROR".to_owned(),
        recommended_action: "FAIL_CLOSED_ERROR".to_owned(),
        loader_invoked: false,
        raw_memory_exported: false,
        decision_receipt_produced: false,
        result: "FAIL_CLOSED".to_owned(),
        active_session_sequence: None,
    });
    let report = FlowReport {
        report_type: "memorylineage-protected-resume-flow-v1",
        source_class: SOURCE_DEMO_SPACE_V2_LOCAL,
        all_expected: true,
        cases,
    };
    println!("{}", serde_json::to_string_pretty(&report)?);
    Ok(())
}
