use ml_evidence::submission::{
    INCIDENT_SCHEMA, IncidentManifest, ROLLBACK_SCHEMA, RollbackRehearsal, SubmissionArtifact,
    SubmissionManifest, SubmissionSection, artifact_path, read_manifest, verify_artifacts,
};
use ml_memory_store::{read_snapshot, snapshot_commitment_v2};
use ml_spec_types::{
    EvidenceBundleV2, RecoveryDecisionReceipt, SOURCE_DEMO_SPACE_V2_LOCAL,
    SOURCE_PROTOCOL_CORPUS_LOCAL,
};
use ml_verifier_independent::{verify_file, verify_recovery_receipt, verify_v2_bundle};
use serde::de::DeserializeOwned;
use std::path::{Path, PathBuf};

const INCIDENT_ID: &str = "demo-space-v2-silent-rollback";
const INCIDENT_PATH: &str = "evidence/submission/demo-space-v2/manifest.json";
const ROLLBACK_PATH: &str = "evidence/submission/demo-space-v2/rollback-rehearsal.json";
const REVM_PATH: &str = "evidence/submission/demo-space-v2/revm-observation.json";
const EVIDENCE_PATH: &str = "evidence/local/demo_space_v2_evidence.json";
const FIXTURE_PATH: &str = "fixtures/silent-rollback-v2/manifest.json";
const SNAPSHOT_PATHS: [&str; 3] = [
    "fixtures/silent-rollback-v2/snapshot-1.db",
    "fixtures/silent-rollback-v2/snapshot-2.db",
    "fixtures/silent-rollback-v2/snapshot-3.db",
];

fn row(name: &str, detail: impl std::fmt::Display) -> String {
    format!("{name} FAIL: {detail}")
}

fn read_artifact<T: DeserializeOwned>(root: &Path, path: &str) -> Result<T, String> {
    let full = artifact_path(root, path).map_err(|error| error.to_string())?;
    let bytes = std::fs::read(full).map_err(|error| error.to_string())?;
    serde_json::from_slice(&bytes).map_err(|error| format!("{path}: {error}"))
}

fn required_artifact<'a>(
    manifest: &'a SubmissionManifest,
    path: &str,
    source: &str,
) -> Result<&'a SubmissionArtifact, String> {
    manifest
        .artifacts
        .iter()
        .find(|item| item.path == path && item.source_class == source)
        .ok_or_else(|| format!("missing source-labeled artifact: {path}"))
}

fn check_section(
    section: &SubmissionSection,
    source: &str,
    status: &str,
    path: Option<&str>,
) -> Result<(), String> {
    if section.source_class != source
        || section.status != status
        || section.artifact_path.as_deref() != path
    {
        Err("submission section source/status/path mismatch".to_owned())
    } else {
        Ok(())
    }
}

fn check_claim_scope(manifest: &SubmissionManifest) -> Result<(), String> {
    let expected: [(&str, &str, &[&str]); 6] = [
        (
            "silent-rollback",
            "VERIFIED_LOCAL",
            &[INCIDENT_PATH, ROLLBACK_PATH, EVIDENCE_PATH],
        ),
        (
            "recovery-decision",
            "VERIFIED_LOCAL",
            &[
                "evidence/local/demo_space_v2_recovery_receipt.json",
                "evidence/local/demo_space_v2_historical_recovery_receipt.json",
            ],
        ),
        (
            "protocol-corpus",
            "VERIFIED_LOCAL",
            &["evidence/local/memory_lineage_evm_evidence.json"],
        ),
        (
            "reference-runtime",
            "VERIFIED_LOCAL",
            &["evidence/local/reference_agent_runtime.json"],
        ),
        (
            "bounded-assurance",
            "BOUNDED_LOCAL",
            &["evidence/local/security_assurance_report.json"],
        ),
        ("external-reproduction", "NOT_YET_DEMONSTRATED", &[]),
    ];
    let command = "cargo run -q -p ml-cli -- submission verify evidence/submission/manifest.json";
    if manifest.artifacts.len() != 13
        || manifest.source_classes.len() != 3
        || manifest.claims.len() != expected.len()
        || !manifest
            .verification_commands
            .iter()
            .any(|recorded| recorded == command)
        || manifest.limitations.is_empty()
    {
        return Err("SUBMISSION_CLAIM_SCOPE_MISMATCH".to_owned());
    }
    for (id, status, paths) in expected {
        let Some(claim) = manifest.claims.iter().find(|claim| claim.id == id) else {
            return Err("SUBMISSION_CLAIM_SCOPE_MISMATCH".to_owned());
        };
        if claim.status != status
            || claim.artifact_paths.len() != paths.len()
            || !paths
                .iter()
                .all(|path| claim.artifact_paths.iter().any(|listed| listed == path))
        {
            return Err("SUBMISSION_CLAIM_SCOPE_MISMATCH".to_owned());
        }
    }
    Ok(())
}

fn check_runtime_case(
    report: &serde_json::Value,
    case_name: &str,
    expected_status: &str,
    expected_classification: Option<&str>,
    expected_action: Option<&str>,
    expected_loader_invoked: bool,
) -> Result<(), String> {
    let case = report
        .pointer(&format!("/cases/{case_name}"))
        .ok_or_else(|| format!("reference runtime outcomes omit {case_name}"))?;
    if case["status"] != expected_status
        || case
            .get("loader_invoked")
            .or_else(|| case.get("loaderInvoked"))
            .and_then(serde_json::Value::as_bool)
            != Some(expected_loader_invoked)
        || expected_classification.is_some_and(|value| case["classification"] != value)
        || expected_action.is_some_and(|value| case["recommended_action"] != value)
    {
        return Err(format!(
            "reference runtime outcomes mismatch for {case_name}"
        ));
    }
    Ok(())
}

fn check_runtime_outcomes(report: &serde_json::Value) -> Result<(), String> {
    if report["reportType"] != "memorylineage-reference-agent-runtime-v1"
        || report["runtime"] != "ReferenceAgentRuntime"
        || report["evidenceSource"] != SOURCE_DEMO_SPACE_V2_LOCAL
        || report["allExpected"] != true
        || report["rawMemoryExported"] != false
    {
        return Err("reference runtime outcomes header mismatch".to_owned());
    }

    check_runtime_case(
        report,
        "currentHead",
        "RESUMED",
        Some("CURRENT_HEAD"),
        Some("RESUME_ALLOWED"),
        true,
    )?;
    check_runtime_case(
        report,
        "missingAuthorizationProof",
        "HELD",
        Some("CURRENT_HEAD"),
        Some("BLOCK_UNVERIFIED"),
        false,
    )?;
    check_runtime_case(
        report,
        "historicalCheckpoint",
        "HELD",
        Some("KNOWN_HISTORICAL_CHECKPOINT"),
        Some("REHEARSE_ONLY"),
        false,
    )?;
    check_runtime_case(
        report,
        "divergedSnapshot",
        "HELD",
        Some("UNKNOWN_OR_DIVERGED"),
        Some("HOLD_FOR_REVIEW"),
        false,
    )?;
    check_runtime_case(report, "invalidEvidence", "FAIL_CLOSED", None, None, false)
}

fn check_incident(
    manifest: &SubmissionManifest,
    incident: &IncidentManifest,
    rollback: &RollbackRehearsal,
    fixture: &serde_json::Value,
    bundle: &EvidenceBundleV2,
    commitments: &[String; 3],
) -> Result<(), String> {
    if manifest.incident_id != INCIDENT_ID
        || incident.schema_version != INCIDENT_SCHEMA
        || rollback.schema_version != ROLLBACK_SCHEMA
        || incident.incident_id != INCIDENT_ID
        || rollback.incident_id != INCIDENT_ID
        || incident.source_class != SOURCE_DEMO_SPACE_V2_LOCAL
        || rollback.source_class != SOURCE_DEMO_SPACE_V2_LOCAL
        || incident.fixture_id != "silent-rollback-v2"
        || bundle.source_class != SOURCE_DEMO_SPACE_V2_LOCAL
        || bundle.head.sequence != 3
        || bundle.transitions.len() != 3
        || incident.canonical.head_sequence != 3
        || incident.canonical.head_state_root != bundle.head.state_root
        || incident.canonical.evidence != EVIDENCE_PATH
        || incident.restored_snapshot.sequence != 1
        || incident.restored_snapshot.snapshot != SNAPSHOT_PATHS[0]
        || incident.restored_snapshot.snapshot_commitment != commitments[0]
        || incident.restored_snapshot.stale_root != bundle.transitions[0].next_state_root
        || incident.attempt.sequence != 4
        || incident.attempt.revert_reason != "BAD_PREVIOUS_STATE"
        || incident.attempt.transaction_broadcast
        || incident.attempt.execution != "Rust/revm local execution"
        || rollback.snapshot_commitment != commitments[0]
        || rollback.stale_predecessor != bundle.transitions[0].next_state_root
        || rollback.canonical_head != bundle.head.state_root
        || rollback.attempted_sequence != 4
        || rollback.revert_reason != "BAD_PREVIOUS_STATE"
        || rollback.transaction_broadcast
        || rollback.execution_source != "Rust/revm against published Solidity bytecode"
        || commitments[0] == rollback.stale_predecessor
    {
        return Err("DEMO_INCIDENT_REFERENCE_MISMATCH".to_owned());
    }
    let snapshots = fixture["snapshots"]
        .as_array()
        .ok_or("fixture snapshots missing")?;
    if fixture["fixtureId"] != "silent-rollback-v2"
        || fixture["attack"]["attemptedSequence"] != 4
        || fixture["attack"]["expectedContractReason"] != "BAD_PREVIOUS_STATE"
        || snapshots.len() != 3
    {
        return Err("DEMO_INCIDENT_REFERENCE_MISMATCH".to_owned());
    }
    for (index, commitment) in commitments.iter().enumerate() {
        if snapshots[index]["sequence"] != index as u64 + 1
            || snapshots[index]["snapshotCommitment"] != *commitment
            || bundle.transitions[index].delta.delta_commitment != *commitment
            || !incident
                .artifacts
                .contains(&SNAPSHOT_PATHS[index].to_owned())
        {
            return Err("DEMO_INCIDENT_REFERENCE_MISMATCH".to_owned());
        }
    }
    for path in [
        FIXTURE_PATH,
        EVIDENCE_PATH,
        REVM_PATH,
        ROLLBACK_PATH,
        "evidence/local/demo_space_v2_recovery_receipt.json",
    ] {
        if !incident.artifacts.contains(&path.to_owned())
            || !manifest
                .artifacts
                .iter()
                .any(|artifact| artifact.path == path)
        {
            return Err("DEMO_INCIDENT_REFERENCE_MISMATCH".to_owned());
        }
    }
    let attack = bundle
        .attack
        .as_ref()
        .ok_or("DEMO_INCIDENT_REFERENCE_MISMATCH")?;
    if attack.reason.as_deref() != Some("BAD_PREVIOUS_STATE")
        || attack.stale_predecessor.as_deref() != Some(rollback.stale_predecessor.as_str())
        || attack.canonical_predecessor.as_deref() != Some(rollback.canonical_head.as_str())
        || attack.restored_snapshot_sequence != Some(1)
        || attack.attempted_sequence != Some(4)
        || attack.transaction_broadcast != Some(false)
        || attack.execution_source.as_deref() != Some(rollback.execution_source.as_str())
    {
        return Err("DEMO_INCIDENT_REFERENCE_MISMATCH".to_owned());
    }
    Ok(())
}

fn verify_package(root: &Path, manifest_path: &Path) -> Result<(), String> {
    let manifest = read_manifest(manifest_path).map_err(|error| row("Incident identity", error))?;
    if manifest.incident_id != INCIDENT_ID {
        return Err(row("Incident identity", "wrong incident ID"));
    }
    if manifest.submission_commit.is_some() {
        return Err(row(
            "Incident identity",
            "SUBMISSION_COMMIT_CANNOT_SELF_ATTEST; record the submitted SHA in release metadata",
        ));
    }
    if ![
        SOURCE_DEMO_SPACE_V2_LOCAL,
        SOURCE_PROTOCOL_CORPUS_LOCAL,
        "EXTERNAL_REPRODUCTION",
    ]
    .iter()
    .all(|source| {
        manifest
            .source_classes
            .iter()
            .any(|listed| listed == source)
    }) {
        return Err(row("Source classes", "missing required source class"));
    }
    println!("Envelope schema         PASS");
    verify_artifacts(&manifest, root).map_err(|error| row("Artifact hashes", error))?;
    println!("Artifact hashes         PASS");

    for path in [
        INCIDENT_PATH,
        ROLLBACK_PATH,
        REVM_PATH,
        EVIDENCE_PATH,
        FIXTURE_PATH,
        SNAPSHOT_PATHS[0],
        SNAPSHOT_PATHS[1],
        SNAPSHOT_PATHS[2],
        "evidence/local/demo_space_v2_recovery_receipt.json",
        "evidence/local/demo_space_v2_historical_recovery_receipt.json",
        "evidence/local/reference_agent_runtime.json",
    ] {
        required_artifact(&manifest, path, SOURCE_DEMO_SPACE_V2_LOCAL)
            .map_err(|error| row("Source classes", error))?;
    }
    required_artifact(
        &manifest,
        "evidence/local/memory_lineage_evm_evidence.json",
        SOURCE_PROTOCOL_CORPUS_LOCAL,
    )
    .map_err(|error| row("Source classes", error))?;
    required_artifact(
        &manifest,
        "evidence/local/security_assurance_report.json",
        SOURCE_PROTOCOL_CORPUS_LOCAL,
    )
    .map_err(|error| row("Source classes", error))?;
    check_section(
        &manifest.demo_incident,
        SOURCE_DEMO_SPACE_V2_LOCAL,
        "VERIFIED_LOCAL",
        Some(INCIDENT_PATH),
    )
    .map_err(|error| row("Source classes", error))?;
    check_section(
        &manifest.protocol_corpus,
        SOURCE_PROTOCOL_CORPUS_LOCAL,
        "VERIFIED_LOCAL",
        Some("evidence/local/memory_lineage_evm_evidence.json"),
    )
    .map_err(|error| row("Source classes", error))?;
    check_section(
        &manifest.runtime_integration,
        SOURCE_DEMO_SPACE_V2_LOCAL,
        "VERIFIED_LOCAL",
        Some("evidence/local/reference_agent_runtime.json"),
    )
    .map_err(|error| row("Source classes", error))?;
    check_section(
        &manifest.bounded_assurance,
        SOURCE_PROTOCOL_CORPUS_LOCAL,
        "BOUNDED_LOCAL",
        Some("evidence/local/security_assurance_report.json"),
    )
    .map_err(|error| row("Source classes", error))?;
    check_section(
        &manifest.external_reproduction,
        "EXTERNAL_REPRODUCTION",
        "NOT_YET_DEMONSTRATED",
        None,
    )
    .map_err(|error| row("Source classes", error))?;
    println!("Source classes          PASS");
    check_claim_scope(&manifest).map_err(|error| row("Claim scope", error))?;
    println!("Claim scope             PASS");

    let incident: IncidentManifest =
        read_artifact(root, INCIDENT_PATH).map_err(|error| row("Incident identity", error))?;
    for path in &incident.artifacts {
        required_artifact(&manifest, path, SOURCE_DEMO_SPACE_V2_LOCAL)
            .map_err(|error| row("Incident identity", error))?;
    }
    let rollback: RollbackRehearsal =
        read_artifact(root, ROLLBACK_PATH).map_err(|error| row("Incident identity", error))?;
    let fixture: serde_json::Value =
        read_artifact(root, FIXTURE_PATH).map_err(|error| row("Incident identity", error))?;
    let bundle: EvidenceBundleV2 =
        read_artifact(root, EVIDENCE_PATH).map_err(|error| row("Evidence replay", error))?;
    let revm: EvidenceBundleV2 =
        read_artifact(root, REVM_PATH).map_err(|error| row("Evidence replay", error))?;
    let commitments: [String; 3] = SNAPSHOT_PATHS
        .map(|path| -> Result<String, String> {
            let path = artifact_path(root, path).map_err(|error| error.to_string())?;
            let snapshot = read_snapshot(path).map_err(|error| error.to_string())?;
            Ok(snapshot_commitment_v2(&snapshot))
        })
        .into_iter()
        .collect::<Result<Vec<_>, _>>()?
        .try_into()
        .map_err(|_| "missing snapshots")?;
    check_incident(
        &manifest,
        &incident,
        &rollback,
        &fixture,
        &bundle,
        &commitments,
    )
    .map_err(|error| row("Incident identity", error))?;
    println!("Incident identity       PASS");
    let generated = ml_local_evm::run_demo_space_v2(&commitments)
        .map_err(|error| row("Evidence replay", error))?;
    verify_v2_bundle(&bundle).map_err(|error| row("Evidence replay", error))?;
    if bundle != revm || bundle != generated {
        return Err(row(
            "Evidence replay",
            "published V2 bundle differs from local Rust/revm execution",
        ));
    }
    let corpus = artifact_path(root, "evidence/local/memory_lineage_evm_evidence.json")
        .map_err(|error| row("Evidence replay", error))?;
    verify_file(corpus).map_err(|error| row("Evidence replay", error))?;
    println!("Evidence replay         PASS");

    for path in [
        "evidence/local/demo_space_v2_recovery_receipt.json",
        "evidence/local/demo_space_v2_historical_recovery_receipt.json",
    ] {
        let receipt: RecoveryDecisionReceipt =
            read_artifact(root, path).map_err(|error| row("Recovery receipts", error))?;
        verify_recovery_receipt(&receipt, &bundle)
            .map_err(|error| row("Recovery receipts", error))?;
    }
    println!("Recovery receipts       PASS");

    let runtime: serde_json::Value =
        read_artifact(root, "evidence/local/reference_agent_runtime.json")
            .map_err(|error| row("Privacy boundary", error))?;
    let assurance: serde_json::Value =
        read_artifact(root, "evidence/local/security_assurance_report.json")
            .map_err(|error| row("Privacy boundary", error))?;
    check_runtime_outcomes(&runtime).map_err(|error| row("Runtime behavior", error))?;
    if bundle.privacy.raw_memory_on_chain
        || assurance["raw_memory_exported"] != false
        || assurance["formal_status"] != "NOT_FORMALLY_VERIFIED"
    {
        return Err(row(
            "Privacy boundary",
            "local runtime or assurance scope mismatch",
        ));
    }
    println!("Privacy boundary        PASS");
    println!("VERDICT                 VERIFIED_LOCAL_PACKAGE");
    Ok(())
}

pub fn command(args: &[String], root: &Path) -> Result<(), Box<dyn std::error::Error>> {
    if args.first().map(String::as_str) != Some("verify") || args.len() > 2 {
        return Err("submission requires verify [MANIFEST.json]".into());
    }
    let path = args
        .get(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| root.join("evidence/submission/manifest.json"));
    verify_package(root, &path).map_err(Into::into)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ml_evidence::submission::artifact_sha256;

    fn copy_current_submission() -> (PathBuf, PathBuf) {
        let source_root = super::super::repository_root();
        let source_manifest_path = source_root.join("evidence/submission/manifest.json");
        let manifest = read_manifest(&source_manifest_path).expect("published manifest");
        let temporary_root = std::env::temp_dir().join(format!(
            "memorylineage-submission-package-test-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("clock should be after epoch")
                .as_nanos()
        ));

        for artifact in &manifest.artifacts {
            let source = source_root.join(&artifact.path);
            let destination = temporary_root.join(&artifact.path);
            std::fs::create_dir_all(destination.parent().expect("artifact has a parent"))
                .expect("create artifact directory");
            std::fs::copy(source, destination).expect("copy submission artifact");
        }

        let manifest_path = temporary_root.join("evidence/submission/manifest.json");
        std::fs::create_dir_all(manifest_path.parent().expect("manifest has a parent"))
            .expect("create manifest directory");
        std::fs::copy(source_manifest_path, &manifest_path).expect("copy submission manifest");
        (temporary_root, manifest_path)
    }

    #[test]
    fn changing_the_stale_root_breaks_incident_identity_even_with_a_consistent_hash() {
        let root = super::super::repository_root();
        let manifest = read_manifest(root.join("evidence/submission/manifest.json"))
            .expect("published manifest");
        let incident: IncidentManifest = read_artifact(&root, INCIDENT_PATH).expect("incident");
        let mut rollback: RollbackRehearsal =
            read_artifact(&root, ROLLBACK_PATH).expect("rollback");
        let fixture: serde_json::Value = read_artifact(&root, FIXTURE_PATH).expect("fixture");
        let bundle: EvidenceBundleV2 = read_artifact(&root, EVIDENCE_PATH).expect("evidence");
        let commitments = SNAPSHOT_PATHS
            .map(|path| snapshot_commitment_v2(&read_snapshot(root.join(path)).expect("snapshot")));
        rollback.stale_predecessor = commitments[0].clone();
        assert_eq!(
            check_incident(
                &manifest,
                &incident,
                &rollback,
                &fixture,
                &bundle,
                &commitments
            ),
            Err("DEMO_INCIDENT_REFERENCE_MISMATCH".to_owned())
        );
    }

    #[test]
    fn external_reproduction_cannot_be_marked_verified_by_manifest_edit() {
        let root = super::super::repository_root();
        let mut manifest = read_manifest(root.join("evidence/submission/manifest.json"))
            .expect("published manifest");
        manifest
            .claims
            .iter_mut()
            .find(|claim| claim.id == "external-reproduction")
            .expect("external claim exists")
            .status = "VERIFIED_LOCAL".to_owned();
        assert_eq!(
            check_claim_scope(&manifest),
            Err("SUBMISSION_CLAIM_SCOPE_MISMATCH".to_owned())
        );
    }

    #[test]
    fn submission_verifier_rejects_a_runtime_that_loads_without_authorization_proof() {
        let (root, manifest_path) = copy_current_submission();
        let runtime_path = root.join("evidence/local/reference_agent_runtime.json");
        let mut runtime: serde_json::Value =
            serde_json::from_slice(&std::fs::read(&runtime_path).expect("runtime report"))
                .expect("runtime JSON");
        runtime["cases"]["missingAuthorizationProof"]["loader_invoked"] = serde_json::json!(true);
        std::fs::write(
            &runtime_path,
            format!(
                "{}\n",
                serde_json::to_string_pretty(&runtime).expect("runtime JSON")
            ),
        )
        .expect("write altered runtime report");

        let mut manifest = read_manifest(&manifest_path).expect("copied manifest");
        let runtime_artifact = manifest
            .artifacts
            .iter_mut()
            .find(|artifact| artifact.path == "evidence/local/reference_agent_runtime.json")
            .expect("runtime artifact is listed");
        runtime_artifact.sha256 =
            artifact_sha256(&std::fs::read(&runtime_path).expect("altered runtime report"));
        std::fs::write(
            &manifest_path,
            format!(
                "{}\n",
                serde_json::to_string_pretty(&manifest).expect("manifest JSON")
            ),
        )
        .expect("update runtime artifact hash");

        let result = verify_package(&root, &manifest_path);
        assert!(
            matches!(&result, Err(message) if message.contains("reference runtime outcomes")),
            "modified runtime case must fail package verification, got {result:?}"
        );
        std::fs::remove_dir_all(root).expect("temporary package cleanup");
    }
}
