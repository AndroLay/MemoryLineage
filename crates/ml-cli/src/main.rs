#![forbid(unsafe_code)]

use ml_agent_runtime::ReferenceAgentRuntime;
use ml_conformance::run_pinned;
use ml_ethereum::{inspect_registry, simulate_silent_rollback_with_predecessor};
use ml_evidence::{load_public_replay_bundle, public_replay_to_v2, write_v2_bundle};
use ml_memory_store::{
    create_demo_space_v2_fixture, create_silent_rollback_fixture, demo_space_v2_observations,
    inspect_demo_space_v2_fixture, inspect_silent_rollback_fixture, restore_snapshot,
    snapshot_observations,
};
use ml_recovery_gate::{ProtectedResumeError, protected_resume};
use ml_spec_types::EvidenceBundleV2;
use ml_verifier_independent::{build_recovery_receipt, verify_file, verify_recovery_receipt};
use std::path::{Path, PathBuf};

fn repository_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn default_fixture_root() -> PathBuf {
    repository_root().join("fixtures/silent-rollback")
}

fn default_demo_fixture_root() -> PathBuf {
    repository_root().join("fixtures/silent-rollback-v2")
}

fn usage() {
    eprintln!(
        "Usage:\n  \
  cargo run -p ml-cli -- fixture create [ROOT]\n  \
  cargo run -p ml-cli -- fixture inspect [ROOT]\n  \
  cargo run -p ml-cli -- fixture manifest [ROOT] [OUTPUT]\n  \
  cargo run -p ml-cli -- fixture demo-create [ROOT]\n  \
  cargo run -p ml-cli -- fixture demo-manifest [ROOT] [OUTPUT]\n  \
  cargo run -p ml-cli -- fixture restore <SOURCE> <DESTINATION>\n  \
  cargo run -p ml-cli -- verify <EVIDENCE.json>\n  \
  cargo run -p ml-cli -- conformance\n  \
  cargo run -p ml-cli -- revm silent-rollback\n  \
  cargo run -p ml-cli -- revm mutations\n  \
  cargo run -p ml-cli -- revm erc1271\n  \
  cargo run -p ml-cli -- revm authority-rotation\n  \
  cargo run -p ml-cli -- revm demo-space-v2 [FIXTURE_ROOT]\n  \
  cargo run -p ml-cli -- security bounded-audit [OUTPUT]\n  \
  cargo run -p ml-cli -- evidence export-v2 [SOURCE] [DESTINATION]\n  \
  cargo run -p ml-cli -- evidence demo-v2 [FIXTURE_ROOT] [DESTINATION]\n  \
  cargo run -p ml-cli -- recover preflight <SNAPSHOT> [EVIDENCE] [RECEIPT] [SOURCE]\n  \
  cargo run -p ml-cli -- recover verify <RECEIPT> [EVIDENCE]\n  \
  cargo run -p ml-cli -- recover enforce <SNAPSHOT> [EVIDENCE]\n  \
  cargo run -p ml-cli -- agent reference-demo [FIXTURE_ROOT] [EVIDENCE] [OUTPUT]\n  \
  cargo run -p ml-cli -- demo silent-rollback [FIXTURE_ROOT]\n  \
  cargo run -p ml-cli -- demo silent-rollback [FIXTURE_ROOT]\n  \
  cargo run -p ml-cli -- demo legacy-silent-rollback [ROOT]\n  \
  cargo run -p ml-cli -- live inspect\n  \
  cargo run -p ml-cli -- live rollback"
    );
}

fn fixture_command(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    let action = args.first().map(String::as_str).unwrap_or("inspect");
    match action {
        "create" => {
            let root = args
                .get(1)
                .map(PathBuf::from)
                .unwrap_or_else(default_fixture_root);
            create_silent_rollback_fixture(&root)?;
            println!(
                "created deterministic Silent Rollback fixture at {}",
                root.display()
            );
        }
        "demo-create" => {
            let root = args
                .get(1)
                .map(PathBuf::from)
                .unwrap_or_else(default_demo_fixture_root);
            create_demo_space_v2_fixture(&root)?;
            println!(
                "created deterministic Demo Space V2 fixture at {}",
                root.display()
            );
        }
        "inspect" => {
            let root = args
                .get(1)
                .map(PathBuf::from)
                .unwrap_or_else(default_fixture_root);
            let snapshots = inspect_silent_rollback_fixture(&root)?;
            println!("MemoryLineage private fixture");
            for snapshot in snapshots {
                println!(
                    "  #{}: {} private fields",
                    snapshot.sequence,
                    snapshot.values.len()
                );
            }
        }
        "manifest" => {
            let root = args
                .get(1)
                .map(PathBuf::from)
                .unwrap_or_else(default_fixture_root);
            let output = args
                .get(2)
                .map(PathBuf::from)
                .unwrap_or_else(|| root.join("manifest.json"));
            let mut manifest: serde_json::Value =
                serde_json::from_str(&std::fs::read_to_string(root.join("manifest.json"))?)?;
            let original_snapshots = manifest
                .get("snapshots")
                .and_then(serde_json::Value::as_array)
                .cloned()
                .unwrap_or_default();
            let mut observations = serde_json::to_value(snapshot_observations(&root)?)?;
            if let Some(observations) = observations.as_array_mut() {
                for observation in observations {
                    let sequence = observation
                        .get("sequence")
                        .and_then(serde_json::Value::as_u64);
                    if let Some(original) = original_snapshots.iter().find(|snapshot| {
                        snapshot.get("sequence").and_then(serde_json::Value::as_u64) == sequence
                    }) && let Some(label) = original.get("visibleLabel")
                    {
                        observation["visibleLabel"] = label.clone();
                    }
                }
            }
            manifest["commitmentDomain"] = serde_json::json!("memorylineage/private-snapshot/v1");
            manifest["generatedBy"] = serde_json::json!("ml-cli fixture manifest");
            manifest["snapshots"] = observations;
            std::fs::write(
                &output,
                format!("{}\n", serde_json::to_string_pretty(&manifest)?),
            )?;
            println!("wrote private-snapshot commitments to {}", output.display());
        }
        "demo-manifest" => {
            let root = args
                .get(1)
                .map(PathBuf::from)
                .unwrap_or_else(default_demo_fixture_root);
            let output = args
                .get(2)
                .map(PathBuf::from)
                .unwrap_or_else(|| root.join("manifest.json"));
            let labels = [
                "Language preference recorded",
                "Evidence-backed claim policy recorded",
                "Raw-memory privacy boundary recorded",
            ];
            let mut observations = demo_space_v2_observations(&root)?;
            for (observation, label) in observations.iter_mut().zip(labels) {
                observation.visible_label = Some(label.to_owned());
            }
            let manifest = serde_json::json!({
                "fixtureId": "silent-rollback-v2",
                "synthetic": true,
                "description": "Registry-aligned synthetic SQLite values modeling private agent snapshots for the unified Demo Space V2.",
                "commitmentDomain": "memorylineage/private-snapshot/v1",
                "generatedBy": "ml-cli fixture demo-manifest",
                "snapshots": observations,
                "attack": {
                    "name": "silent-rollback",
                    "restoredSequence": 1,
                    "attemptedSequence": 4,
                    "expectedContractReason": "BAD_PREVIOUS_STATE"
                }
            });
            std::fs::write(
                &output,
                format!("{}\n", serde_json::to_string_pretty(&manifest)?),
            )?;
            println!("wrote Demo Space V2 manifest to {}", output.display());
        }
        "restore" => {
            let source = args
                .get(1)
                .ok_or("fixture restore requires a source database")?;
            let destination = args
                .get(2)
                .ok_or("fixture restore requires a destination database")?;
            restore_snapshot(source, destination)?;
            println!("restored private snapshot to {destination}");
        }
        _ => {
            usage();
            return Err(format!("unknown fixture action: {action}").into());
        }
    }
    Ok(())
}

fn verify_command(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let report = verify_file(path)?;
    println!("{}", serde_json::to_string_pretty(&report)?);
    Ok(())
}

fn conformance_command() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", serde_json::to_string_pretty(&run_pinned()?)?);
    Ok(())
}

fn security_command(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    match args.first().map(String::as_str) {
        Some("bounded-audit") => {
            let output = args.get(1).map(PathBuf::from).unwrap_or_else(|| {
                repository_root().join("evidence/local/security_assurance_report.json")
            });
            let report = ml_local_evm::run_bounded_security_assurance()?;
            std::fs::write(
                &output,
                format!("{}\n", serde_json::to_string_pretty(&report)?),
            )?;
            println!(
                "wrote bounded security assurance report to {}",
                output.display()
            );
            Ok(())
        }
        Some(other) => Err(format!("unknown security action: {other}").into()),
        None => Err("security requires bounded-audit".into()),
    }
}

fn revm_command(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    let action = args.first().map(String::as_str).unwrap_or("");
    match action {
        "silent-rollback" => {
            let observation = ml_local_evm::run_silent_rollback()?;
            println!("{}", serde_json::to_string_pretty(&observation)?);
            Ok(())
        }
        "mutations" => {
            let report = ml_local_evm::run_core_mutations()?;
            println!("{}", serde_json::to_string_pretty(&report)?);
            Ok(())
        }
        "erc1271" => {
            let observation = ml_local_evm::run_erc1271()?;
            println!("{}", serde_json::to_string_pretty(&observation)?);
            Ok(())
        }
        "authority-rotation" => {
            let observation = ml_local_evm::run_authority_rotation()?;
            println!("{}", serde_json::to_string_pretty(&observation)?);
            Ok(())
        }
        "demo-space-v2" => {
            let root = args
                .get(1)
                .map(PathBuf::from)
                .unwrap_or_else(default_demo_fixture_root);
            let snapshots = inspect_demo_space_v2_fixture(&root)?;
            let commitments = snapshots
                .iter()
                .map(ml_memory_store::snapshot_commitment)
                .collect::<Vec<_>>();
            let evidence = ml_local_evm::run_demo_space_v2(&commitments)?;
            println!("{}", serde_json::to_string_pretty(&evidence)?);
            Ok(())
        }
        _ => Err(format!("unknown revm action: {action}").into()),
    }
}

fn evidence_command(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    match args.first().map(String::as_str) {
        Some("export-v2") => {
            let source = args.get(1).map(PathBuf::from).unwrap_or_else(|| {
                repository_root().join("evidence/local/memory_lineage_evm_evidence.json")
            });
            let destination = args
                .get(2)
                .map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from("memorylineage-evidence-v2.json"));
            let bundle = load_public_replay_bundle(&source)?;
            let evidence = public_replay_to_v2(&bundle);
            write_v2_bundle(&evidence, &destination)?;
            println!("wrote V2 evidence to {}", destination.display());
            Ok(())
        }
        Some("demo-v2") => {
            let root = args
                .get(1)
                .map(PathBuf::from)
                .unwrap_or_else(default_demo_fixture_root);
            let destination = args.get(2).map(PathBuf::from).unwrap_or_else(|| {
                repository_root().join("evidence/local/demo_space_v2_evidence.json")
            });
            let snapshots = inspect_demo_space_v2_fixture(&root)?;
            let commitments = snapshots
                .iter()
                .map(ml_memory_store::snapshot_commitment)
                .collect::<Vec<_>>();
            let evidence: EvidenceBundleV2 = ml_local_evm::run_demo_space_v2(&commitments)?;
            write_v2_bundle(&evidence, &destination)?;
            println!("wrote Demo Space V2 evidence to {}", destination.display());
            Ok(())
        }
        Some(other) => Err(format!("unknown evidence action: {other}").into()),
        None => Err("evidence requires an action".into()),
    }
}

fn default_demo_evidence_path() -> PathBuf {
    repository_root().join("evidence/local/demo_space_v2_evidence.json")
}

fn load_v2_evidence(
    path: impl AsRef<Path>,
) -> Result<EvidenceBundleV2, Box<dyn std::error::Error>> {
    let bytes = std::fs::read(path)?;
    Ok(serde_json::from_slice(&bytes)?)
}

fn recovery_preflight(
    args: &[String],
) -> Result<ml_spec_types::RecoveryDecisionReceipt, Box<dyn std::error::Error>> {
    let snapshot_path = args
        .first()
        .ok_or("recover preflight requires a SQLite snapshot path")?;
    let evidence_path = args
        .get(1)
        .map(PathBuf::from)
        .unwrap_or_else(default_demo_evidence_path);
    let source_class = args
        .get(3)
        .map(String::as_str)
        .unwrap_or("DEMO_SPACE_V2_LOCAL");
    let snapshot = ml_memory_store::read_snapshot(snapshot_path)?;
    let evidence = load_v2_evidence(evidence_path)?;
    Ok(build_recovery_receipt(
        &evidence,
        &ml_memory_store::snapshot_commitment(&snapshot),
        snapshot.sequence,
        source_class,
        None,
    )?)
}

fn recover_command(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    let action = args.first().map(String::as_str).unwrap_or("");
    match action {
        "preflight" => {
            let receipt = recovery_preflight(&args[1..])?;
            if let Some(destination) = args.get(3) {
                std::fs::write(
                    destination,
                    format!("{}\n", serde_json::to_string_pretty(&receipt)?),
                )?;
                eprintln!("wrote recovery receipt to {destination}");
            }
            println!("{}", serde_json::to_string_pretty(&receipt)?);
            Ok(())
        }
        "verify" => {
            let receipt_path = args
                .get(1)
                .ok_or("recover verify requires a receipt JSON path")?;
            let evidence_path = args
                .get(2)
                .map(PathBuf::from)
                .unwrap_or_else(default_demo_evidence_path);
            let receipt: ml_spec_types::RecoveryDecisionReceipt =
                serde_json::from_slice(&std::fs::read(receipt_path)?)?;
            let evidence = load_v2_evidence(evidence_path)?;
            let report = verify_recovery_receipt(&receipt, &evidence)?;
            println!("{}", serde_json::to_string_pretty(&report)?);
            Ok(())
        }
        "enforce" => {
            let snapshot_path = args
                .get(1)
                .ok_or("recover enforce requires a SQLite snapshot path")?;
            let evidence_path = args
                .get(2)
                .map(PathBuf::from)
                .unwrap_or_else(default_demo_evidence_path);
            let evidence = load_v2_evidence(evidence_path)?;
            let source_class = args
                .get(4)
                .map(String::as_str)
                .unwrap_or("DEMO_SPACE_V2_LOCAL");
            match protected_resume(snapshot_path, &evidence, source_class, |snapshot| {
                Ok::<String, String>(format!(
                    "snapshot_sequence={},private_keys={}",
                    snapshot.sequence,
                    snapshot.values.len()
                ))
            }) {
                Ok(resumed) => {
                    println!("{}", serde_json::to_string_pretty(&resumed.receipt)?);
                    println!("receipt verification: VERIFIED / PASS");
                    println!("protected loader: LOADED / {}", resumed.loaded);
                    println!("protected resume permitted for the named evidence profile");
                    Ok(())
                }
                Err(ProtectedResumeError::ResumeHeld {
                    classification,
                    action,
                    receipt,
                }) => {
                    println!("{}", serde_json::to_string_pretty(receipt.as_ref())?);
                    println!("receipt verification: VERIFIED / PASS");
                    Err(format!("protected resume held: {classification} / {action}").into())
                }
                Err(error) => Err(error.into()),
            }
        }
        _ => Err("recover requires preflight, verify, or enforce".into()),
    }
}

fn agent_command(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    match args.first().map(String::as_str) {
        Some("reference-demo") => {
            let fixture_root = args
                .get(1)
                .map(PathBuf::from)
                .unwrap_or_else(default_demo_fixture_root);
            let evidence_path = args
                .get(2)
                .map(PathBuf::from)
                .unwrap_or_else(default_demo_evidence_path);
            let output_path = args.get(3).map(PathBuf::from);
            let evidence = load_v2_evidence(&evidence_path)?;
            let mut runtime = ReferenceAgentRuntime::new();
            let current = runtime.resume(
                fixture_root.join("snapshot-3.db"),
                &evidence,
                ml_spec_types::SOURCE_DEMO_SPACE_V2_LOCAL,
            )?;
            let historical = runtime.resume(
                fixture_root.join("snapshot-1.db"),
                &evidence,
                ml_spec_types::SOURCE_DEMO_SPACE_V2_LOCAL,
            )?;

            let diverged_path = std::env::temp_dir().join(format!(
                "memorylineage-reference-agent-diverged-{}.db",
                std::process::id()
            ));
            let diverged_values = [("unexpected".to_owned(), "local-only-state".to_owned())]
                .into_iter()
                .collect();
            ml_memory_store::write_snapshot(&diverged_path, 3, &diverged_values)?;
            let diverged = runtime.resume(
                &diverged_path,
                &evidence,
                ml_spec_types::SOURCE_DEMO_SPACE_V2_LOCAL,
            )?;
            let _ = std::fs::remove_file(&diverged_path);

            let mut invalid_evidence = evidence.clone();
            invalid_evidence.transitions[0].transition_id = "0x00".to_owned();
            let invalid = runtime.resume(
                fixture_root.join("snapshot-3.db"),
                &invalid_evidence,
                ml_spec_types::SOURCE_DEMO_SPACE_V2_LOCAL,
            );

            let report = serde_json::json!({
                "reportType": "memorylineage-reference-agent-runtime-v1",
                "runtime": "ReferenceAgentRuntime",
                "evidenceSource": ml_spec_types::SOURCE_DEMO_SPACE_V2_LOCAL,
                "rawMemoryExported": false,
                "cases": {
                    "currentHead": current,
                    "historicalCheckpoint": historical,
                    "divergedSnapshot": diverged,
                    "invalidEvidence": {
                        "status": if invalid.is_err() { "FAIL_CLOSED" } else { "UNEXPECTED_SUCCESS" },
                        "loaderInvoked": false,
                    },
                },
                "expected": {
                    "currentHead": "RESUME_ALLOWED / loader invoked",
                    "historicalCheckpoint": "REHEARSE_ONLY / loader not invoked",
                    "divergedSnapshot": "HOLD_FOR_REVIEW / loader not invoked",
                    "invalidEvidence": "FAIL_CLOSED",
                },
                "allExpected": current.status == "RESUMED"
                    && current.loader_invoked
                    && historical.status == "HELD"
                    && !historical.loader_invoked
                    && historical.recommended_action == "REHEARSE_ONLY"
                    && diverged.status == "HELD"
                    && !diverged.loader_invoked
                    && diverged.recommended_action == "HOLD_FOR_REVIEW"
                    && invalid.is_err(),
            });
            if !report
                .get("allExpected")
                .and_then(serde_json::Value::as_bool)
                .unwrap_or(false)
            {
                return Err(
                    "reference agent runtime validation did not meet expected outcomes".into(),
                );
            }
            if let Some(path) = output_path {
                std::fs::write(
                    &path,
                    format!("{}\n", serde_json::to_string_pretty(&report)?),
                )?;
                eprintln!("wrote reference runtime report to {}", path.display());
            }
            println!("{}", serde_json::to_string_pretty(&report)?);
            Ok(())
        }
        _ => Err("agent requires reference-demo".into()),
    }
}

fn legacy_silent_rollback_demo(root: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let snapshots = inspect_silent_rollback_fixture(root)?;
    let canonical = snapshots.last().ok_or("fixture has no snapshots")?;
    let restored = snapshots.first().ok_or("fixture has no snapshots")?;
    println!("MemoryLineage — The Silent Rollback");
    println!("canonical private history: #17 → #18 → #19");
    println!("canonical local head: #{}", canonical.sequence);
    println!("operator restores private snapshot: #{}", restored.sequence);
    println!(
        "attempted continuation: #{} → #20 with a stale predecessor",
        restored.sequence
    );
    println!("expected registry result: REJECTED");
    println!("exact Solidity reason: BAD_PREVIOUS_STATE");
    println!(
        "explanation: the restored local snapshot cannot replace the previously committed canonical history"
    );
    println!("live RPC simulation: run `cargo run -p ml-cli -- live rollback`");
    Ok(())
}

fn silent_rollback_demo(root: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let snapshots = inspect_demo_space_v2_fixture(root)?;
    let commitments = snapshots
        .iter()
        .map(ml_memory_store::snapshot_commitment)
        .collect::<Vec<_>>();
    let evidence = ml_local_evm::run_demo_space_v2(&commitments)?;
    let attack = evidence
        .attack
        .as_ref()
        .ok_or("Demo Space V2 is missing attack evidence")?;

    println!("MemoryLineage — The Silent Rollback / Demo Space V2");
    println!("synthetic SQLite snapshots: #1 → #2 → #3");
    println!("canonical committed head: #{}", evidence.head.sequence);
    println!(
        "authority records: {}",
        evidence.authorization_history.len()
    );
    println!(
        "restored private snapshot: #{}",
        attack.restored_snapshot_sequence.unwrap_or(0)
    );
    println!(
        "attempted transition: #{}",
        attack.attempted_sequence.unwrap_or(0)
    );
    println!(
        "stale predecessor from transition 1: {}",
        attack
            .stale_predecessor
            .as_deref()
            .unwrap_or("NOT AVAILABLE")
    );
    println!(
        "result: {} / {}",
        attack.status,
        attack.reason.as_deref().unwrap_or("NO REASON")
    );
    println!(
        "execution: {}; transaction broadcast: {}",
        attack.execution_source.as_deref().unwrap_or("NOT RECORDED"),
        attack.transaction_broadcast.unwrap_or(true)
    );
    println!("evidence: evidence/local/demo_space_v2_evidence.json");
    println!(
        "verify with: cargo run -q -p ml-cli -- verify evidence/local/demo_space_v2_evidence.json"
    );
    Ok(())
}

fn live_command(action: &str) -> Result<(), Box<dyn std::error::Error>> {
    let rpc = std::env::var("MEMORYLINEAGE_SEPOLIA_RPC")
        .unwrap_or_else(|_| "https://ethereum-sepolia-rpc.publicnode.com".to_owned());
    let registry = std::env::var("MEMORYLINEAGE_REGISTRY")
        .unwrap_or_else(|_| "0x36fE9FA585565615Adcfe8680a126F770931E160".to_owned());
    let space = std::env::var("MEMORYLINEAGE_SPACE").unwrap_or_else(|_| {
        "0x910968e7e2ae2899858c72b71683d55d3b1b11a69aae9f38448ee4cbb896580c".to_owned()
    });
    let runtime = tokio::runtime::Runtime::new()?;
    match action {
        "inspect" => {
            let inspection = runtime.block_on(inspect_registry(&rpc, &registry, &space))?;
            println!("live Sepolia inspection: {inspection:#?}");
        }
        "rollback" => {
            let manifest = std::fs::read_to_string(
                repository_root().join("fixtures/silent-rollback/manifest.json"),
            )?;
            let manifest: serde_json::Value = serde_json::from_str(&manifest)?;
            let stale_predecessor = manifest
                .get("snapshots")
                .and_then(serde_json::Value::as_array)
                .and_then(|snapshots| snapshots.first())
                .and_then(|snapshot| snapshot.get("snapshotCommitment"))
                .and_then(serde_json::Value::as_str)
                .ok_or("fixture manifest has no snapshotCommitment")?;
            let simulation = runtime.block_on(simulate_silent_rollback_with_predecessor(
                &rpc,
                &registry,
                &space,
                stale_predecessor,
            ))?;
            println!("live Silent Rollback simulation: {simulation:#?}");
        }
        _ => return Err(format!("unknown live action: {action}").into()),
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1);
    match args.next().as_deref() {
        Some("fixture") => fixture_command(&args.collect::<Vec<_>>()),
        Some("verify") => {
            let path = args.next().ok_or("verify requires an evidence JSON path")?;
            verify_command(&path)
        }
        Some("conformance") => conformance_command(),
        Some("security") => security_command(&args.collect::<Vec<_>>()),
        Some("revm") => revm_command(&args.collect::<Vec<_>>()),
        Some("evidence") => evidence_command(&args.collect::<Vec<_>>()),
        Some("recover") => recover_command(&args.collect::<Vec<_>>()),
        Some("agent") => agent_command(&args.collect::<Vec<_>>()),
        Some("demo") => match args.next().as_deref() {
            Some("silent-rollback") => {
                let root = args
                    .next()
                    .map(PathBuf::from)
                    .unwrap_or_else(default_demo_fixture_root);
                silent_rollback_demo(&root)
            }
            Some("legacy-silent-rollback") => {
                let root = args
                    .next()
                    .map(PathBuf::from)
                    .unwrap_or_else(default_fixture_root);
                legacy_silent_rollback_demo(&root)
            }
            Some(other) => Err(format!("unknown demo: {other}").into()),
            None => Err("demo requires a demo name".into()),
        },
        Some("live") => {
            let action = args.next().ok_or("live requires inspect or rollback")?;
            live_command(&action)
        }
        _ => {
            usage();
            Err("missing or unknown command".into())
        }
    }
}
