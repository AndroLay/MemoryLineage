use ml_memory_store::{MemorySnapshot, read_snapshot};
use ml_spec_types::{RECOVERY_RECEIPT_V3, SNAPSHOT_PROFILE_BLINDED_V1, SOURCE_DEMO_SPACE_V2_LOCAL};
use serde_json::{Value, json};
use std::fs;
use std::io::Write;
use std::process::{Command, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};

fn run_cli_stdin(arguments: &[&str], input: &Value) -> std::process::Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_ml-cli"))
        .args(arguments)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("CLI process should start");
    child
        .stdin
        .take()
        .expect("CLI stdin should be piped")
        .write_all(&serde_json::to_vec(input).expect("input serializes"))
        .expect("CLI should receive its bounded JSON input");
    child.wait_with_output().expect("CLI should complete")
}

#[test]
fn blinded_evidence_and_recovery_use_stdin_without_exporting_snapshot_or_secret() {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock is after Unix epoch")
        .as_nanos();
    let temp_root = std::env::temp_dir().join(format!(
        "memorylineage-cli-blinded-{}-{nonce}",
        std::process::id()
    ));
    fs::create_dir_all(&temp_root).expect("temporary output directory should be created");

    let fixture_root =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/silent-rollback-v2");
    let mut snapshots: [MemorySnapshot; 3] = [1, 2, 3].map(|sequence| {
        read_snapshot(fixture_root.join(format!("snapshot-{sequence}.db")))
            .expect("synthetic SQLite fixture should be readable")
    });
    let private_marker = "ML_PRIVATE_SENTINEL_DO_NOT_EXPORT_6b43d9";
    snapshots[2]
        .values
        .insert("private_probe".to_owned(), private_marker.to_owned());
    let secret = [0x5au8; 32];
    let secret_hex = ml_core::format_hex(&secret);
    let evidence_input = json!({
        "schemaVersion": "memorylineage-blinded-evidence-input-v1",
        "snapshotProfile": SNAPSHOT_PROFILE_BLINDED_V1,
        "blindingSecret": secret_hex,
        "snapshots": snapshots,
    });

    let evidence_output = run_cli_stdin(&["evidence", "demo-v2-blinded-json"], &evidence_input);
    assert!(
        evidence_output.status.success(),
        "evidence CLI failed: {}",
        String::from_utf8_lossy(&evidence_output.stderr)
    );
    let evidence: Value = serde_json::from_slice(&evidence_output.stdout)
        .expect("evidence CLI outputs a JSON bundle");
    assert_eq!(evidence["sourceClass"], SOURCE_DEMO_SPACE_V2_LOCAL);
    let evidence_path = temp_root.join("evidence.json");
    fs::write(&evidence_path, &evidence_output.stdout).expect("evidence bundle is public");
    let evidence_path = evidence_path.to_str().expect("temp path is UTF-8");

    let current_input = json!({
        "schemaVersion": "memorylineage-recovery-input-v1",
        "snapshotProfile": SNAPSHOT_PROFILE_BLINDED_V1,
        "sourceClass": SOURCE_DEMO_SPACE_V2_LOCAL,
        "blindingSecret": secret_hex,
        "snapshot": snapshots[2],
    });
    let receipt_output = run_cli_stdin(
        &["recover", "preflight-json", evidence_path],
        &current_input,
    );
    assert!(
        receipt_output.status.success(),
        "preflight CLI failed: {}",
        String::from_utf8_lossy(&receipt_output.stderr)
    );
    let receipt: Value =
        serde_json::from_slice(&receipt_output.stdout).expect("preflight emits one receipt");
    assert_eq!(receipt["schemaVersion"], RECOVERY_RECEIPT_V3);
    assert_eq!(
        receipt["candidate"]["snapshotProfile"],
        SNAPSHOT_PROFILE_BLINDED_V1
    );
    assert_eq!(receipt["decision"]["recommendedAction"], "RESUME_ALLOWED");

    let verified = run_cli_stdin(
        &["recover", "verify-json", evidence_path],
        &serde_json::from_slice(&receipt_output.stdout).expect("receipt is JSON"),
    );
    assert!(verified.status.success(), "receipt replay should pass");
    let verification: Value =
        serde_json::from_slice(&verified.stdout).expect("verification CLI emits a JSON report");
    assert_eq!(verification["verdict"], "VERIFIED");

    let wrong_secret_input = json!({
        "schemaVersion": "memorylineage-recovery-input-v1",
        "snapshotProfile": SNAPSHOT_PROFILE_BLINDED_V1,
        "sourceClass": SOURCE_DEMO_SPACE_V2_LOCAL,
        "blindingSecret": ml_core::format_hex(&[0x91; 32]),
        "snapshot": snapshots[2],
    });
    let wrong_secret = run_cli_stdin(
        &["recover", "preflight-json", evidence_path],
        &wrong_secret_input,
    );
    assert!(
        wrong_secret.status.success(),
        "a well-formed mismatch is a hold"
    );
    let wrong_receipt: Value =
        serde_json::from_slice(&wrong_secret.stdout).expect("wrong secret produces a receipt");
    assert_eq!(
        wrong_receipt["decision"]["recommendedAction"],
        "HOLD_FOR_REVIEW"
    );

    let zero_secret_input = json!({
        "schemaVersion": "memorylineage-recovery-input-v1",
        "snapshotProfile": SNAPSHOT_PROFILE_BLINDED_V1,
        "sourceClass": SOURCE_DEMO_SPACE_V2_LOCAL,
        "blindingSecret": ml_core::format_hex(&[0; 32]),
        "snapshot": snapshots[2],
    });
    let zero_secret = run_cli_stdin(
        &["recover", "preflight-json", evidence_path],
        &zero_secret_input,
    );
    assert!(!zero_secret.status.success());
    assert!(zero_secret.stdout.is_empty());

    for output in [
        &evidence_output.stdout,
        &receipt_output.stdout,
        &verified.stdout,
    ] {
        assert!(!String::from_utf8_lossy(output).contains(&secret_hex));
        assert!(!String::from_utf8_lossy(output).contains(private_marker));
    }
    assert!(!String::from_utf8_lossy(&wrong_secret.stdout).contains(private_marker));
    assert!(!String::from_utf8_lossy(&zero_secret.stderr).contains(&secret_hex));

    fs::remove_dir_all(temp_root).expect("temporary evidence should be cleaned up");
}
