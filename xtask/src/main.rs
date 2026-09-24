#![forbid(unsafe_code)]

use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

fn run(program: &str, args: &[&str]) -> Result<(), String> {
    let output = Command::new(program)
        .args(args)
        .env("CCACHE_DISABLE", "1")
        .env_remove("RUSTC_WRAPPER")
        .stdin(Stdio::inherit())
        .output()
        .map_err(|error| format!("could not start {program}: {error}"))?;
    if output.status.success() {
        Ok(())
    } else {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(format!(
            "{program} exited with {}\n{}{}",
            output.status, stdout, stderr
        ))
    }
}

fn step(label: &str, program: &str, args: &[&str]) -> Result<(), String> {
    run(program, args)?;
    println!("PASS {label}");
    Ok(())
}

fn verify_fixture_manifest() -> Result<(), String> {
    let destination = std::env::temp_dir().join(format!(
        "memorylineage-fixture-manifest-{}.json",
        std::process::id()
    ));
    let destination_text = destination
        .to_str()
        .ok_or_else(|| "temporary fixture path is not valid UTF-8".to_owned())?;
    let result = run(
        "cargo",
        &[
            "run",
            "-q",
            "-p",
            "ml-cli",
            "--",
            "fixture",
            "manifest",
            "fixtures/silent-rollback",
            destination_text,
        ],
    );
    if let Err(error) = result {
        let _ = fs::remove_file(&destination);
        return Err(error);
    }
    let expected = fs::read("fixtures/silent-rollback/manifest.json")
        .map_err(|error| format!("could not read committed fixture manifest: {error}"))?;
    let generated = fs::read(&destination)
        .map_err(|error| format!("could not read generated fixture manifest: {error}"))?;
    let _ = fs::remove_file(&destination);
    if expected != generated {
        return Err("fixture manifest is not reproducibly generated".to_owned());
    }
    println!("PASS fixture manifest");
    Ok(())
}

fn verify_demo_space_v2() -> Result<(), String> {
    let temp_root = std::env::temp_dir().join(format!(
        "memorylineage-demo-space-v2-{}",
        std::process::id()
    ));
    let fixture_root = temp_root.join("fixture");
    let manifest_destination = temp_root.join("manifest.json");
    let evidence_destination = temp_root.join("evidence.json");
    fs::create_dir_all(&temp_root)
        .map_err(|error| format!("could not create temporary Demo Space directory: {error}"))?;
    let fixture_text = fixture_root
        .to_str()
        .ok_or_else(|| "temporary demo fixture path is not valid UTF-8".to_owned())?;
    let manifest_text = manifest_destination
        .to_str()
        .ok_or_else(|| "temporary demo manifest path is not valid UTF-8".to_owned())?;
    let evidence_text = evidence_destination
        .to_str()
        .ok_or_else(|| "temporary demo evidence path is not valid UTF-8".to_owned())?;

    let result = (|| {
        run(
            "cargo",
            &[
                "run",
                "-q",
                "-p",
                "ml-cli",
                "--",
                "fixture",
                "demo-create",
                fixture_text,
            ],
        )?;
        run(
            "cargo",
            &[
                "run",
                "-q",
                "-p",
                "ml-cli",
                "--",
                "fixture",
                "demo-manifest",
                fixture_text,
                manifest_text,
            ],
        )?;
        run(
            "cargo",
            &[
                "run",
                "-q",
                "-p",
                "ml-cli",
                "--",
                "evidence",
                "demo-v2",
                fixture_text,
                evidence_text,
            ],
        )?;
        let expected_manifest = fs::read("fixtures/silent-rollback-v2/manifest.json")
            .map_err(|error| format!("could not read Demo Space V2 manifest: {error}"))?;
        let generated_manifest = fs::read(&manifest_destination)
            .map_err(|error| format!("could not read generated Demo Space V2 manifest: {error}"))?;
        if expected_manifest != generated_manifest {
            return Err("Demo Space V2 manifest is not reproducibly generated".to_owned());
        }
        let expected_evidence = fs::read("evidence/local/demo_space_v2_evidence.json")
            .map_err(|error| format!("could not read Demo Space V2 evidence: {error}"))?;
        let generated_evidence = fs::read(&evidence_destination)
            .map_err(|error| format!("could not read generated Demo Space V2 evidence: {error}"))?;
        if expected_evidence != generated_evidence {
            return Err("Demo Space V2 evidence is not reproducibly generated".to_owned());
        }

        let manifest: serde_json::Value =
            serde_json::from_slice(&generated_manifest).map_err(|error| {
                format!("generated Demo Space V2 manifest is invalid JSON: {error}")
            })?;
        let evidence: serde_json::Value =
            serde_json::from_slice(&generated_evidence).map_err(|error| {
                format!("generated Demo Space V2 evidence is invalid JSON: {error}")
            })?;
        let snapshots = manifest
            .get("snapshots")
            .and_then(serde_json::Value::as_array)
            .ok_or_else(|| "Demo Space V2 fixture has no snapshot list".to_owned())?;
        if manifest
            .get("fixtureId")
            .and_then(serde_json::Value::as_str)
            != Some("silent-rollback-v2")
            || manifest
                .get("synthetic")
                .and_then(serde_json::Value::as_bool)
                != Some(true)
        {
            return Err("Demo Space V2 must identify its public fixture as synthetic".to_owned());
        }
        if evidence
            .get("sourceClass")
            .and_then(serde_json::Value::as_str)
            != Some("DEMO_SPACE_V2_LOCAL")
        {
            return Err(
                "Demo Space V2 evidence must carry DEMO_SPACE_V2_LOCAL source identity".to_owned(),
            );
        }
        let transitions = evidence
            .get("transitions")
            .and_then(serde_json::Value::as_array)
            .ok_or_else(|| "Demo Space V2 evidence has no transition list".to_owned())?;
        if snapshots.len() != 3 || transitions.len() != 3 {
            return Err(
                "Demo Space V2 must contain exactly three snapshots and transitions".to_owned(),
            );
        }
        for (index, (snapshot, transition)) in snapshots.iter().zip(transitions).enumerate() {
            let expected_sequence = (index + 1) as u64;
            if snapshot.get("sequence").and_then(serde_json::Value::as_u64)
                != Some(expected_sequence)
                || transition
                    .get("sequence")
                    .and_then(serde_json::Value::as_u64)
                    != Some(expected_sequence)
                || snapshot.get("snapshotCommitment") != transition.get("deltaCommitment")
            {
                return Err(format!(
                    "Demo Space V2 snapshot/transition parity failed at sequence {expected_sequence}"
                ));
            }
        }
        let authorities = evidence
            .get("authorizationHistory")
            .and_then(serde_json::Value::as_array)
            .ok_or_else(|| "Demo Space V2 evidence has no authority history".to_owned())?;
        if authorities.len() != 2
            || authorities[0]
                .get("configNonce")
                .and_then(serde_json::Value::as_u64)
                != Some(0)
            || authorities[1]
                .get("configNonce")
                .and_then(serde_json::Value::as_u64)
                != Some(1)
            || authorities[0]
                .get("effectiveFromSequence")
                .and_then(serde_json::Value::as_u64)
                != Some(1)
            || authorities[1]
                .get("effectiveFromSequence")
                .and_then(serde_json::Value::as_u64)
                != Some(3)
            || authorities[0].get("authorizer") == authorities[1].get("authorizer")
        {
            return Err("Demo Space V2 authority rotation evidence is inconsistent".to_owned());
        }
        let authorization_proofs = evidence
            .get("authorizationProofs")
            .and_then(serde_json::Value::as_array)
            .ok_or_else(|| "Demo Space V2 has no EOA authorization proofs".to_owned())?;
        if authorization_proofs.len() != transitions.len() {
            return Err("Demo Space V2 EOA authorization proof coverage is incomplete".to_owned());
        }
        for (index, (proof, transition)) in authorization_proofs.iter().zip(transitions).enumerate()
        {
            let expected_sequence = (index + 1) as u64;
            let expected_nonce = if expected_sequence < 3 { 0 } else { 1 };
            if proof
                .get("authorizationType")
                .and_then(serde_json::Value::as_str)
                != Some("EOA_EIP712")
                || proof.get("sequence").and_then(serde_json::Value::as_u64)
                    != Some(expected_sequence)
                || proof.get("configNonce").and_then(serde_json::Value::as_u64)
                    != Some(expected_nonce)
                || proof.get("transitionId") != transition.get("transitionId")
            {
                return Err(format!(
                    "Demo Space V2 authority proof is not bound to active timeline at sequence {expected_sequence}"
                ));
            }
        }
        let attack = evidence
            .get("attack")
            .ok_or_else(|| "Demo Space V2 evidence has no rollback attack".to_owned())?;
        let manifest_attack = manifest
            .get("attack")
            .ok_or_else(|| "Demo Space V2 manifest has no rollback scenario".to_owned())?;
        let transition_one_root = transitions[0]
            .get("nextStateRoot")
            .and_then(serde_json::Value::as_str);
        let canonical_root = evidence
            .pointer("/head/stateRoot")
            .and_then(serde_json::Value::as_str);
        if attack.get("status").and_then(serde_json::Value::as_str) != Some("REJECTED")
            || attack.get("reason").and_then(serde_json::Value::as_str)
                != Some("BAD_PREVIOUS_STATE")
            || attack.get("fixtureId").and_then(serde_json::Value::as_str)
                != Some("silent-rollback-v2")
            || attack
                .get("restoredSnapshotSequence")
                .and_then(serde_json::Value::as_u64)
                != manifest_attack
                    .get("restoredSequence")
                    .and_then(serde_json::Value::as_u64)
            || attack
                .get("attemptedSequence")
                .and_then(serde_json::Value::as_u64)
                != manifest_attack
                    .get("attemptedSequence")
                    .and_then(serde_json::Value::as_u64)
            || !attack
                .get("executionSource")
                .and_then(serde_json::Value::as_str)
                .is_some_and(|source| source.contains("Rust/revm"))
            || attack
                .get("stalePredecessor")
                .and_then(serde_json::Value::as_str)
                != transition_one_root
            || attack
                .get("canonicalPredecessor")
                .and_then(serde_json::Value::as_str)
                != canonical_root
            || attack
                .get("transactionBroadcast")
                .and_then(serde_json::Value::as_bool)
                != Some(false)
            || transitions
                .last()
                .and_then(|transition| transition.get("nextStateRoot"))
                != evidence.pointer("/head/stateRoot")
            || evidence
                .pointer("/privacy/rawMemoryOnChain")
                .and_then(serde_json::Value::as_bool)
                != Some(false)
        {
            return Err("Demo Space V2 attack, head, or privacy invariants failed".to_owned());
        }
        let generated_evidence_text = String::from_utf8_lossy(&generated_evidence);
        for raw_sample_value in ["Indonesian", "evidence-backed", "raw-memory-private"] {
            if generated_evidence_text.contains(raw_sample_value) {
                return Err(format!(
                    "Demo Space V2 public evidence contains a synthetic raw-memory value: {raw_sample_value}"
                ));
            }
        }

        run(
            "cargo",
            &["run", "-q", "-p", "ml-cli", "--", "verify", evidence_text],
        )?;
        Ok(())
    })();
    let _ = fs::remove_dir_all(&temp_root);
    result.map(|_| println!("PASS Demo Space V2 evidence"))
}

fn verify_polkadot_hub_portability() -> Result<(), String> {
    let destination = std::env::temp_dir().join(format!(
        "memorylineage-polkadot-portability-{}.json",
        std::process::id()
    ));
    let destination_text = destination
        .to_str()
        .ok_or_else(|| "temporary portability path is not valid UTF-8".to_owned())?;
    let result = run(
        "cargo",
        &[
            "run",
            "-q",
            "-p",
            "ml-cli",
            "--",
            "portability",
            "polkadot-hub-rehearsal",
            destination_text,
        ],
    );
    if let Err(error) = result {
        let _ = fs::remove_file(&destination);
        return Err(error);
    }
    let expected = fs::read("evidence/local/polkadot_hub_portability_rehearsal.json")
        .map_err(|error| format!("could not read committed portability report: {error}"))?;
    let generated = fs::read(&destination)
        .map_err(|error| format!("could not read generated portability report: {error}"))?;
    if expected != generated {
        let _ = fs::remove_file(&destination);
        return Err("Polkadot Hub portability rehearsal is not reproducibly generated".to_owned());
    }
    if let Err(error) = run(
        "cargo",
        &[
            "run",
            "-q",
            "-p",
            "ml-cli",
            "--",
            "portability",
            "verify",
            destination_text,
        ],
    ) {
        let _ = fs::remove_file(&destination);
        return Err(error);
    }
    let report: serde_json::Value = match serde_json::from_slice(&generated) {
        Ok(report) => report,
        Err(error) => {
            let _ = fs::remove_file(&destination);
            return Err(format!("portability report is invalid JSON: {error}"));
        }
    };
    if report.get("status").and_then(serde_json::Value::as_str) != Some("LOCAL_REHEARSAL_PASS")
        || report
            .pointer("/target/deployment")
            .and_then(serde_json::Value::as_str)
            != Some("NOT_PERFORMED")
        || report
            .pointer("/target/publicRpcObservation")
            .and_then(serde_json::Value::as_str)
            != Some("NOT_PERFORMED")
        || report
            .pointer("/comparison/stalePredecessorRejection")
            .and_then(serde_json::Value::as_str)
            != Some("BAD_PREVIOUS_STATE")
    {
        let _ = fs::remove_file(&destination);
        return Err("Polkadot Hub portability report has an invalid status boundary".to_owned());
    }
    let schema: serde_json::Value =
        match fs::read("evidence/schemas/portability-rehearsal-v1.schema.json")
            .map_err(|error| format!("could not read portability schema: {error}"))
            .and_then(|bytes| {
                serde_json::from_slice(&bytes)
                    .map_err(|error| format!("portability schema is invalid JSON: {error}"))
            }) {
            Ok(schema) => schema,
            Err(error) => {
                let _ = fs::remove_file(&destination);
                return Err(error);
            }
        };
    if schema
        .pointer("/properties/schemaVersion/const")
        .and_then(serde_json::Value::as_str)
        != Some("memorylineage-portability-v1")
    {
        let _ = fs::remove_file(&destination);
        return Err("portability schema is not pinned to v1".to_owned());
    }
    let _ = fs::remove_file(&destination);
    println!("PASS Polkadot Hub portability rehearsal and independent replay");
    Ok(())
}

fn verify_recovery_preflight() -> Result<(), String> {
    let temp_root = std::env::temp_dir().join(format!(
        "memorylineage-recovery-receipt-{}",
        std::process::id()
    ));
    fs::create_dir_all(&temp_root)
        .map_err(|error| format!("could not create recovery receipt directory: {error}"))?;
    let current_receipt = temp_root.join("current.json");
    let historical_receipt = temp_root.join("historical.json");
    let current_receipt_text = current_receipt
        .to_str()
        .ok_or_else(|| "temporary current receipt path is not valid UTF-8".to_owned())?;
    let historical_receipt_text = historical_receipt
        .to_str()
        .ok_or_else(|| "temporary historical receipt path is not valid UTF-8".to_owned())?;
    let result = (|| {
        run(
            "cargo",
            &[
                "run",
                "-q",
                "-p",
                "ml-cli",
                "--",
                "recover",
                "preflight",
                "fixtures/silent-rollback-v2/snapshot-3.db",
                "evidence/local/demo_space_v2_evidence.json",
                current_receipt_text,
                "DEMO_SPACE_V2_LOCAL",
            ],
        )?;
        let generated = fs::read(&current_receipt)
            .map_err(|error| format!("could not read generated recovery receipt: {error}"))?;
        let published = fs::read("evidence/local/demo_space_v2_recovery_receipt.json")
            .map_err(|error| format!("could not read published recovery receipt: {error}"))?;
        if generated != published {
            return Err("published recovery receipt is not reproducibly generated".to_owned());
        }
        run(
            "cargo",
            &[
                "run",
                "-q",
                "-p",
                "ml-cli",
                "--",
                "recover",
                "preflight",
                "fixtures/silent-rollback-v2/snapshot-1.db",
                "evidence/local/demo_space_v2_evidence.json",
                historical_receipt_text,
                "DEMO_SPACE_V2_LOCAL",
            ],
        )?;
        let historical_generated = fs::read(&historical_receipt)
            .map_err(|error| format!("could not read generated historical receipt: {error}"))?;
        let historical_published =
            fs::read("evidence/local/demo_space_v2_historical_recovery_receipt.json")
                .map_err(|error| format!("could not read published historical receipt: {error}"))?;
        if historical_generated != historical_published {
            return Err(
                "published historical recovery receipt is not reproducibly generated".to_owned(),
            );
        }
        let receipt_text = String::from_utf8_lossy(&published);
        let receipt: serde_json::Value = serde_json::from_slice(&published)
            .map_err(|error| format!("published recovery receipt is invalid JSON: {error}"))?;
        if receipt
            .get("schemaVersion")
            .and_then(serde_json::Value::as_str)
            != Some("memorylineage-recovery-receipt-v2")
            || receipt.get("policyId").and_then(serde_json::Value::as_str)
                != Some("strict-authorized-current-head-v2")
            || receipt
                .pointer("/candidate/snapshotProfile")
                .and_then(serde_json::Value::as_str)
                != Some("memorylineage/private-snapshot/v2")
            || receipt
                .pointer("/evidence/sourceClass")
                .and_then(serde_json::Value::as_str)
                != Some("DEMO_SPACE_V2_LOCAL")
            || receipt
                .pointer("/assurance/authorityHistory")
                .and_then(serde_json::Value::as_str)
                != Some("TIMELINE_BOUND")
            || receipt
                .pointer("/assurance/transitionAuthorization")
                .and_then(serde_json::Value::as_str)
                != Some("EOA_SIGNATURES_VERIFIED")
        {
            return Err("recovery receipt policy or source identity is not pinned".to_owned());
        }
        for raw_sample_value in ["Indonesian", "evidence-backed", "raw-memory-private"] {
            if receipt_text.contains(raw_sample_value) {
                return Err(format!(
                    "published recovery receipt contains a synthetic raw-memory value: {raw_sample_value}"
                ));
            }
        }
        run(
            "cargo",
            &[
                "run",
                "-q",
                "-p",
                "ml-cli",
                "--",
                "recover",
                "verify",
                current_receipt_text,
                "evidence/local/demo_space_v2_evidence.json",
            ],
        )?;
        let historical_receipt_value: serde_json::Value =
            serde_json::from_slice(&historical_published)
                .map_err(|error| format!("historical recovery receipt is invalid JSON: {error}"))?;
        if historical_receipt_value
            .pointer("/decision/classification")
            .and_then(serde_json::Value::as_str)
            != Some("KNOWN_HISTORICAL_CHECKPOINT")
            || historical_receipt_value
                .pointer("/decision/recommendedAction")
                .and_then(serde_json::Value::as_str)
                != Some("REHEARSE_ONLY")
            || historical_receipt_value
                .get("schemaVersion")
                .and_then(serde_json::Value::as_str)
                != Some("memorylineage-recovery-receipt-v2")
            || historical_receipt_value
                .get("policyId")
                .and_then(serde_json::Value::as_str)
                != Some("strict-authorized-current-head-v2")
        {
            return Err(
                "historical recovery receipt did not record the V2 policy and REHEARSE_ONLY"
                    .to_owned(),
            );
        }
        run(
            "cargo",
            &[
                "run",
                "-q",
                "-p",
                "ml-cli",
                "--",
                "recover",
                "verify",
                "evidence/local/demo_space_v2_historical_recovery_receipt.json",
                "evidence/local/demo_space_v2_evidence.json",
            ],
        )?;
        run(
            "cargo",
            &[
                "run",
                "-q",
                "-p",
                "ml-cli",
                "--",
                "recover",
                "verify",
                "evidence/local/demo_space_v2_recovery_receipt.json",
                "evidence/local/demo_space_v2_evidence.json",
            ],
        )?;
        let current = Command::new("cargo")
            .args([
                "run",
                "-q",
                "-p",
                "ml-cli",
                "--",
                "recover",
                "enforce",
                "fixtures/silent-rollback-v2/snapshot-3.db",
                "evidence/local/demo_space_v2_evidence.json",
            ])
            .env("CCACHE_DISABLE", "1")
            .env_remove("RUSTC_WRAPPER")
            .output()
            .map_err(|error| format!("could not start current protected resume check: {error}"))?;
        if !current.status.success()
            || !String::from_utf8_lossy(&current.stdout).contains("protected loader: LOADED")
        {
            return Err(format!(
                "current snapshot did not reach the protected loader:\n{}{}",
                String::from_utf8_lossy(&current.stdout),
                String::from_utf8_lossy(&current.stderr)
            ));
        }

        let historical = Command::new("cargo")
            .args([
                "run",
                "-q",
                "-p",
                "ml-cli",
                "--",
                "recover",
                "enforce",
                "fixtures/silent-rollback-v2/snapshot-1.db",
                "evidence/local/demo_space_v2_evidence.json",
            ])
            .env("CCACHE_DISABLE", "1")
            .env_remove("RUSTC_WRAPPER")
            .output()
            .map_err(|error| format!("could not start recovery hold check: {error}"))?;
        if historical.status.success() {
            return Err("historical snapshot unexpectedly passed protected resume".to_owned());
        }
        if String::from_utf8_lossy(&historical.stdout).contains("protected loader: LOADED") {
            return Err("historical snapshot reached the protected loader".to_owned());
        }
        Ok(())
    })();
    let _ = fs::remove_dir_all(&temp_root);
    result.map(|_| println!("PASS recovery preflight and protected resume gate"))
}

fn verify_reference_agent_runtime() -> Result<(), String> {
    let destination = std::env::temp_dir().join(format!(
        "memorylineage-reference-agent-runtime-{}.json",
        std::process::id()
    ));
    let destination_text = destination
        .to_str()
        .ok_or_else(|| "temporary runtime report path is not valid UTF-8".to_owned())?;
    let result = run(
        "cargo",
        &[
            "run",
            "-q",
            "-p",
            "ml-cli",
            "--",
            "agent",
            "reference-demo",
            "fixtures/silent-rollback-v2",
            "evidence/local/demo_space_v2_evidence.json",
            destination_text,
        ],
    );
    if let Err(error) = result {
        let _ = fs::remove_file(&destination);
        return Err(error);
    }
    let expected = fs::read("evidence/local/reference_agent_runtime.json")
        .map_err(|error| format!("could not read reference runtime evidence: {error}"))?;
    let generated = fs::read(&destination)
        .map_err(|error| format!("could not read generated reference runtime evidence: {error}"))?;
    let _ = fs::remove_file(&destination);
    if expected != generated {
        return Err("reference agent runtime evidence is not reproducibly generated".to_owned());
    }
    let report: serde_json::Value = serde_json::from_slice(&generated)
        .map_err(|error| format!("reference runtime evidence is invalid JSON: {error}"))?;
    if report
        .get("allExpected")
        .and_then(serde_json::Value::as_bool)
        != Some(true)
        || report
            .get("rawMemoryExported")
            .and_then(serde_json::Value::as_bool)
            != Some(false)
        || report
            .get("evidenceSource")
            .and_then(serde_json::Value::as_str)
            != Some("DEMO_SPACE_V2_LOCAL")
        || report
            .pointer("/cases/currentHead/loader_invoked")
            .and_then(serde_json::Value::as_bool)
            != Some(true)
        || report
            .pointer("/cases/historicalCheckpoint/loader_invoked")
            .and_then(serde_json::Value::as_bool)
            != Some(false)
        || report
            .pointer("/cases/divergedSnapshot/loader_invoked")
            .and_then(serde_json::Value::as_bool)
            != Some(false)
        || report
            .pointer("/cases/invalidEvidence/status")
            .and_then(serde_json::Value::as_str)
            != Some("FAIL_CLOSED")
    {
        return Err("reference agent runtime safety outcomes are incomplete".to_owned());
    }
    println!("PASS reference agent runtime");
    Ok(())
}

fn verify_security_assurance() -> Result<(), String> {
    let destination = std::env::temp_dir().join(format!(
        "memorylineage-security-assurance-{}.json",
        std::process::id()
    ));
    let destination_text = destination
        .to_str()
        .ok_or_else(|| "temporary security report path is not valid UTF-8".to_owned())?;
    let result = run(
        "cargo",
        &[
            "run",
            "-q",
            "-p",
            "ml-cli",
            "--",
            "security",
            "bounded-audit",
            destination_text,
        ],
    );
    if let Err(error) = result {
        let _ = fs::remove_file(&destination);
        return Err(error);
    }
    let expected = fs::read("evidence/local/security_assurance_report.json")
        .map_err(|error| format!("could not read bounded security assurance evidence: {error}"))?;
    let generated = fs::read(&destination)
        .map_err(|error| format!("could not read generated bounded security report: {error}"))?;
    let _ = fs::remove_file(&destination);
    if expected != generated {
        return Err("bounded security assurance evidence is not reproducibly generated".to_owned());
    }
    let report: serde_json::Value = serde_json::from_slice(&generated)
        .map_err(|error| format!("bounded security report is invalid JSON: {error}"))?;
    if report.get("status").and_then(serde_json::Value::as_str) != Some("BOUNDED_ASSURANCE_PASS")
        || report
            .get("formal_status")
            .and_then(serde_json::Value::as_str)
            != Some("NOT_FORMALLY_VERIFIED")
        || report
            .get("raw_memory_exported")
            .and_then(serde_json::Value::as_bool)
            != Some(false)
        || report
            .get("valid_transitions")
            .and_then(serde_json::Value::as_u64)
            != Some(5)
        || report
            .pointer("/stale_predecessor_cases")
            .and_then(serde_json::Value::as_array)
            .is_none_or(|cases| cases.len() != 6)
        || report
            .pointer("/sequence_gap_cases")
            .and_then(serde_json::Value::as_array)
            .is_none_or(|cases| cases.len() != 2)
        || report
            .pointer("/mutation_matrix/all_expected")
            .and_then(serde_json::Value::as_bool)
            != Some(true)
    {
        return Err("bounded security assurance invariants are incomplete".to_owned());
    }
    println!("PASS bounded security assurance");
    Ok(())
}

fn doctor() -> Result<(), String> {
    fn command_available(program: &str, args: &[&str]) -> bool {
        Command::new(program)
            .args(args)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .map(|status| status.success())
            .unwrap_or(false)
    }

    let mut ready = true;
    for (label, program, args, required) in [
        ("Rust compiler", "rustc", ["--version"].as_slice(), true),
        ("Cargo", "cargo", ["--version"].as_slice(), true),
        ("Dioxus CLI", "dx", ["--version"].as_slice(), true),
        ("Solidity compiler", "solc", ["--version"].as_slice(), false),
    ] {
        let available = command_available(program, args);
        println!(
            "{}  {}",
            label,
            if available {
                "PASS"
            } else if required {
                "NOT FOUND"
            } else {
                "OPTIONAL / NOT FOUND"
            }
        );
        if required && !available {
            ready = false;
        }
    }
    let target = command_available("rustup", &["target", "list", "--installed"])
        && String::from_utf8_lossy(
            &Command::new("rustup")
                .args(["target", "list", "--installed"])
                .output()
                .map_err(|error| format!("could not inspect Rust targets: {error}"))?
                .stdout,
        )
        .lines()
        .any(|line| line == "wasm32-unknown-unknown");
    println!(
        "wasm32 target  {}",
        if target { "PASS" } else { "NOT FOUND" }
    );
    if !target {
        ready = false;
    }
    for path in [
        "contracts/artifacts/memory_lineage_registry_creation.hex",
        "evidence/local/demo_space_v2_evidence.json",
        "fixtures/silent-rollback-v2/manifest.json",
    ] {
        let exists = PathBuf::from(path).is_file();
        println!("{path}  {}", if exists { "PASS" } else { "MISSING" });
        if !exists {
            ready = false;
        }
    }
    println!("Ready to verify  {}", if ready { "YES" } else { "NO" });
    if ready {
        Ok(())
    } else {
        Err("doctor found a missing required local capability".to_owned())
    }
}

fn build_web() -> Result<(), String> {
    let dx = std::env::var("DX_BIN").unwrap_or_else(|_| "dx".to_owned());
    run(
        &dx,
        &[
            "build",
            "--package",
            "memorylineage-inspector",
            "--web",
            "--release",
            "--debug-symbols=false",
        ],
    )
}

fn serve_web() -> Result<(), String> {
    let dx = std::env::var("DX_BIN").unwrap_or_else(|_| "dx".to_owned());
    let status = Command::new(&dx)
        .args([
            "serve",
            "--package",
            "memorylineage-inspector",
            "--web",
            "--hot-patch",
            "false",
            "--open",
            "false",
        ])
        .env("CCACHE_DISABLE", "1")
        .env_remove("RUSTC_WRAPPER")
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .map_err(|error| format!("could not start {dx}: {error}"))?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("{dx} serve exited with {status}"))
    }
}

fn dev_server_app_ready(port: u16) -> bool {
    let Ok(mut stream) = TcpStream::connect(("127.0.0.1", port)) else {
        return false;
    };
    let _ = stream.set_read_timeout(Some(Duration::from_secs(2)));
    if stream
        .write_all(b"GET / HTTP/1.1\r\nHost: 127.0.0.1\r\nConnection: close\r\n\r\n")
        .is_err()
    {
        return false;
    }
    let mut response = String::new();
    if stream.read_to_string(&mut response).is_err() {
        return false;
    }
    response.contains("Verify the history")
}

fn smoke_dev_web() -> Result<(), String> {
    let dx = std::env::var("DX_BIN").unwrap_or_else(|_| "dx".to_owned());
    let ready_timeout_seconds = std::env::var("MEMORYLINEAGE_DEV_READY_TIMEOUT_SECS")
        .ok()
        .and_then(|value| value.parse::<u64>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(180);
    let port = TcpListener::bind(("127.0.0.1", 0))
        .map_err(|error| format!("could not reserve a local dev-server port: {error}"))?
        .local_addr()
        .map_err(|error| format!("could not read the local dev-server port: {error}"))?
        .port();
    let port_text = port.to_string();
    let server_args = vec![
        "serve".to_owned(),
        "--package".to_owned(),
        "memorylineage-inspector".to_owned(),
        "--web".to_owned(),
        "--addr".to_owned(),
        "127.0.0.1".to_owned(),
        "--port".to_owned(),
        port_text.clone(),
        "--open".to_owned(),
        "false".to_owned(),
        "--hot-patch".to_owned(),
        "false".to_owned(),
        "--hot-reload".to_owned(),
        "false".to_owned(),
        "--watch".to_owned(),
        "false".to_owned(),
    ];
    let mut server = Command::new(&dx)
        .args(&server_args)
        .env("CCACHE_DISABLE", "1")
        .env_remove("RUSTC_WRAPPER")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|error| format!("could not start {dx}: {error}"))?;

    let result = (|| {
        let deadline = Instant::now() + Duration::from_secs(ready_timeout_seconds);
        loop {
            if dev_server_app_ready(port) {
                break;
            }
            if let Some(status) = server
                .try_wait()
                .map_err(|error| format!("could not inspect {dx} serve: {error}"))?
            {
                return Err(format!("{dx} serve exited before becoming ready: {status}"));
            }
            if Instant::now() >= deadline {
                return Err(format!(
                    "{dx} serve did not become ready within {ready_timeout_seconds} seconds"
                ));
            }
            thread::sleep(Duration::from_millis(250));
        }

        let base_url = format!("http://127.0.0.1:{port}");
        let smoke = Command::new("python3")
            .args(["scripts/smoke_web.py", "--base-url", &base_url])
            .env("CCACHE_DISABLE", "1")
            .env_remove("RUSTC_WRAPPER")
            .output()
            .map_err(|error| format!("could not start dev-server browser smoke: {error}"))?;
        if smoke.status.success() {
            println!("PASS development browser smoke");
            Ok(())
        } else {
            Err(format!(
                "development browser smoke failed with {}\n{}{}",
                smoke.status,
                String::from_utf8_lossy(&smoke.stdout),
                String::from_utf8_lossy(&smoke.stderr)
            ))
        }
    })();

    let _ = server.kill();
    let _ = server.wait();
    result
}

fn verify() -> Result<(), String> {
    step("format", "cargo", &["fmt", "--all", "--", "--check"])?;
    step(
        "clippy",
        "cargo",
        &[
            "clippy",
            "--workspace",
            "--all-targets",
            "--all-features",
            "--",
            "-D",
            "warnings",
        ],
    )?;
    step(
        "workspace tests",
        "cargo",
        &["test", "--workspace", "--quiet"],
    )?;
    verify_fixture_manifest()?;
    verify_demo_space_v2()?;
    verify_polkadot_hub_portability()?;
    verify_recovery_preflight()?;
    verify_reference_agent_runtime()?;
    step(
        "protected resume executable example",
        "cargo",
        &[
            "run",
            "-q",
            "-p",
            "ml-agent-runtime",
            "--example",
            "protected-resume-flow",
        ],
    )?;
    verify_security_assurance()?;
    step(
        "submission bundle",
        "cargo",
        &[
            "run",
            "-q",
            "-p",
            "ml-cli",
            "--",
            "submission",
            "verify",
            "evidence/submission/manifest.json",
        ],
    )?;
    step(
        "pinned conformance",
        "cargo",
        &["run", "-q", "-p", "ml-cli", "--", "conformance"],
    )?;
    step(
        "independent replay",
        "cargo",
        &[
            "run",
            "-q",
            "-p",
            "ml-cli",
            "--",
            "verify",
            "evidence/local/memory_lineage_evm_evidence.json",
        ],
    )?;
    step(
        "revm Silent Rollback",
        "cargo",
        &["run", "-q", "-p", "ml-cli", "--", "revm", "silent-rollback"],
    )?;
    step(
        "revm mutation lane",
        "cargo",
        &["run", "-q", "-p", "ml-cli", "--", "revm", "mutations"],
    )?;
    step(
        "revm ERC-1271",
        "cargo",
        &["run", "-q", "-p", "ml-cli", "--", "revm", "erc1271"],
    )?;
    step(
        "revm authority rotation",
        "cargo",
        &[
            "run",
            "-q",
            "-p",
            "ml-cli",
            "--",
            "revm",
            "authority-rotation",
        ],
    )?;
    step(
        "WASM compile",
        "cargo",
        &[
            "check",
            "-p",
            "memorylineage-inspector",
            "--target",
            "wasm32-unknown-unknown",
        ],
    )?;
    step(
        "public package boundary",
        "bash",
        &["scripts/check-public-package.sh"],
    )?;
    Ok(())
}

fn release_prep() -> Result<(), String> {
    verify()?;
    build_web()?;
    println!("PASS static release build");
    step("static browser smoke", "python3", &["scripts/smoke_web.py"])?;
    step(
        "release package boundary",
        "bash",
        &["scripts/check-public-package.sh", "--release"],
    )?;
    Ok(())
}

fn reproduce() -> Result<(), String> {
    doctor()?;
    release_prep()?;
    println!("PASS automated clean-checkout reproduction path");
    println!("NOTE external human reproduction remains NOT YET DEMONSTRATED");
    Ok(())
}

fn reviewer_package(output: Option<&str>) -> Result<(), String> {
    let status = Command::new("git")
        .args(["status", "--porcelain"])
        .output()
        .map_err(|error| format!("could not inspect worktree status: {error}"))?;
    if !status.status.success() {
        return Err("git status --porcelain failed".to_owned());
    }
    if !status.stdout.is_empty() {
        return Err(
            "reviewer package requires a clean worktree; commit changes before packaging"
                .to_owned(),
        );
    }

    let destination = output
        .map(PathBuf::from)
        .unwrap_or_else(|| std::env::temp_dir().join("memorylineage-reviewer-package.tar.gz"));
    if let Some(parent) = destination.parent() {
        fs::create_dir_all(parent)
            .map_err(|error| format!("could not create package directory: {error}"))?;
    }
    let destination_text = destination
        .to_str()
        .ok_or_else(|| "reviewer package path is not valid UTF-8".to_owned())?;
    run(
        "git",
        &[
            "archive",
            "--format=tar.gz",
            "--output",
            destination_text,
            "HEAD",
        ],
    )?;

    let listing = Command::new("tar")
        .args(["-tzf", destination_text])
        .output()
        .map_err(|error| format!("could not inspect reviewer package: {error}"))?;
    if !listing.status.success() {
        return Err("tar could not read the generated reviewer package".to_owned());
    }
    let listing_text = String::from_utf8_lossy(&listing.stdout);
    let forbidden = listing_text.lines().find(|entry| {
        entry.contains("/internal/")
            || entry.starts_with("internal/")
            || entry.contains("/target/")
            || entry.starts_with("target/")
            || entry.contains("/node_modules/")
            || entry.starts_with("node_modules/")
            || entry.contains("/.next/")
            || entry.starts_with(".next/")
            || entry.contains("/evidence/generated/")
            || entry.starts_with("evidence/generated/")
            || entry.ends_with("/__pycache__/")
            || entry.contains("/__pycache__/")
            || entry.ends_with(".pyc")
    });
    if let Some(entry) = forbidden {
        return Err(format!("reviewer package contains forbidden path: {entry}"));
    }
    println!("PASS reviewer package boundary");
    println!("wrote reviewer package to {}", destination.display());
    Ok(())
}

fn reviewer_reproduce() -> Result<(), String> {
    // Keep the clean checkout on the repository's disk-backed filesystem. The
    // default system temp directory may be a small tmpfs, while a release
    // build can legitimately need more space than that. The temporary parent
    // remains outside the repository so the initial clean-worktree check is
    // unaffected.
    let workspace = std::env::current_dir()
        .map_err(|error| format!("could not determine workspace directory: {error}"))?;
    let parent = workspace
        .parent()
        .ok_or_else(|| "workspace has no parent directory".to_owned())?;
    let temp_root = parent.join(format!(
        ".memorylineage-reviewer-reproduce-{}",
        std::process::id()
    ));
    let archive = temp_root.join("reviewer-package.tar.gz");
    let checkout = temp_root.join("checkout");
    fs::create_dir_all(&temp_root)
        .map_err(|error| format!("could not create reviewer reproduction directory: {error}"))?;
    let archive_text = archive
        .to_str()
        .ok_or_else(|| "reviewer archive path is not valid UTF-8".to_owned())?;
    let checkout_text = checkout
        .to_str()
        .ok_or_else(|| "reviewer checkout path is not valid UTF-8".to_owned())?;

    let result = (|| {
        reviewer_package(Some(archive_text))?;
        fs::create_dir_all(&checkout)
            .map_err(|error| format!("could not create reviewer checkout: {error}"))?;
        run("tar", &["-xzf", archive_text, "-C", checkout_text])?;

        let output = Command::new("cargo")
            .args(["xtask", "reproduce"])
            .env("CCACHE_DISABLE", "1")
            .env_remove("RUSTC_WRAPPER")
            .current_dir(&checkout)
            .output()
            .map_err(|error| {
                format!("could not run reproduction from reviewer archive: {error}")
            })?;
        if !output.status.success() {
            return Err(format!(
                "reviewer archive reproduction failed with {}\n{}{}",
                output.status,
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            ));
        }
        println!("PASS clean reviewer archive reproduction");
        println!("archive checkout: {}", checkout.display());
        Ok(())
    })();
    let _ = fs::remove_dir_all(&temp_root);
    result
}

fn release_manifest(output: Option<&str>) -> Result<(), String> {
    run(
        "cargo",
        &[
            "run",
            "-q",
            "-p",
            "ml-cli",
            "--",
            "submission",
            "verify",
            "evidence/submission/manifest.json",
        ],
    )?;
    let destination = output
        .map(PathBuf::from)
        .unwrap_or_else(|| std::env::temp_dir().join("memorylineage-release-manifest.json"));
    let revision = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .map_err(|error| format!("could not read git revision: {error}"))?;
    if !revision.status.success() {
        return Err("git rev-parse HEAD failed".to_owned());
    }
    let revision = String::from_utf8_lossy(&revision.stdout).trim().to_owned();
    let status = Command::new("git")
        .args(["status", "--porcelain"])
        .output()
        .map_err(|error| format!("could not read git worktree status: {error}"))?;
    if !status.status.success() {
        return Err("git status --porcelain failed".to_owned());
    }
    let working_tree_clean = status.stdout.is_empty();
    let manifest = serde_json::json!({
        "manifestType": "memorylineage-release-preparation",
        "repository": "AndroLay/MemoryLineage",
        "gitRevision": revision.clone(),
        "workingTreeClean": working_tree_clean,
        "submissionRevision": if working_tree_clean { Some(revision) } else { None },
        "rustToolchain": "1.97.1",
        "website": "Rust/WASM Dioxus static artifact",
        "ethereum": {
            "network": "Sepolia",
            "registry": "0x36fE9FA585565615Adcfe8680a126F770931E160",
            "deployment": "existing evidence; no deployment performed by this command"
        },
        "demo": {
            "sourceClass": "DEMO_SPACE_V2_LOCAL",
            "status": "LOCAL REVM EVIDENCE / NOT DEPLOYED",
            "fixture": "fixtures/silent-rollback-v2/manifest.json",
            "evidence": "evidence/local/demo_space_v2_evidence.json",
            "headSequence": 3,
            "authorityRecords": 2,
            "authorityTimeline": "TIMELINE_BOUND",
            "transitionAuthorization": "EOA_SIGNATURES_VERIFIED",
            "stalePredecessorSource": "transition 1 nextStateRoot",
            "expectedReason": "BAD_PREVIOUS_STATE",
            "transactionBroadcast": false,
            "referenceRuntime": "evidence/local/reference_agent_runtime.json"
        },
        "protocolCorpus": {
            "sourceClass": "PROTOCOL_CORPUS_LOCAL",
            "evidence": "evidence/local/memory_lineage_evm_evidence.json",
            "securityAssurance": "evidence/local/security_assurance_report.json",
            "status": "BOUNDED_LOCAL"
        },
        "submissionEnvelope": {
            "path": "evidence/submission/manifest.json",
            "incidentId": "demo-space-v2-silent-rollback",
            "status": "VERIFIED_LOCAL_PACKAGE"
        },
        "legacyFixture": "fixtures/silent-rollback/manifest.json",
        "evidence": [
            "evidence/submission/manifest.json",
            "evidence/local/demo_space_v2_evidence.json",
            "evidence/local/demo_space_v2_recovery_receipt.json",
            "evidence/local/demo_space_v2_historical_recovery_receipt.json",
            "evidence/local/memory_lineage_evm_evidence.json",
            "evidence/local/rust_revm_conformance.json",
            "evidence/local/rust_revm_mutation_matrix.json",
            "evidence/local/rust_revm_erc1271.json",
            "evidence/local/rust_revm_authority_rotation.json",
            "evidence/local/reference_agent_runtime.json",
            "evidence/local/security_assurance_report.json",
            "evidence/local/polkadot_hub_portability_rehearsal.json",
            "evidence/sepolia/sepolia_reread.json",
            "evidence/sepolia/silent_rollback_fixture_eth_call.json"
        ],
        "excludedByScope": ["demo video", "new public deployment", "staging"]
    });
    std::fs::write(
        &destination,
        format!("{}\n", serde_json::to_string_pretty(&manifest).unwrap()),
    )
    .map_err(|error| format!("could not write {}: {error}", destination.display()))?;
    println!(
        "wrote release preparation manifest to {}",
        destination.display()
    );
    Ok(())
}

fn main() {
    let command = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "verify".to_owned());
    let result = match command.as_str() {
        "verify" => verify(),
        "doctor" => doctor(),
        "conformance" => run(
            "cargo",
            &["test", "-p", "ml-core", "-p", "ml-verifier-independent"],
        ),
        "fixture" => run(
            "cargo",
            &["run", "-q", "-p", "ml-cli", "--", "fixture", "inspect"],
        ),
        "build-web" => build_web(),
        "serve-web" => serve_web(),
        "package-check" => run("bash", &["scripts/check-public-package.sh"]),
        "release" => release_prep(),
        "reproduce" => reproduce(),
        "reviewer-package" => reviewer_package(std::env::args().nth(2).as_deref()),
        "reviewer-reproduce" => reviewer_reproduce(),
        "smoke-web" => run("python3", &["scripts/smoke_web.py"]),
        "smoke-dev-web" => smoke_dev_web(),
        "release-manifest" => release_manifest(std::env::args().nth(2).as_deref()),
        other => Err(format!(
            "unknown xtask command {other}; use doctor, verify, conformance, fixture, build-web, serve-web, package-check, release, reproduce, reviewer-package, reviewer-reproduce, smoke-web, smoke-dev-web, or release-manifest"
        )),
    };

    if let Err(error) = result {
        eprintln!("xtask: {error}");
        std::process::exit(1);
    }
}
