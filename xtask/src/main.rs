#![forbid(unsafe_code)]

use std::fs;
use std::path::PathBuf;
use std::process::{Command, Stdio};

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
        if receipt.get("policyId").and_then(serde_json::Value::as_str)
            != Some("strict-current-head-only-v1")
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
        {
            return Err("historical recovery receipt did not record REHEARSE_ONLY".to_owned());
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
    verify_recovery_preflight()?;
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

fn release_manifest(output: Option<&str>) -> Result<(), String> {
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
        "gitRevision": revision,
        "workingTreeClean": working_tree_clean,
        "rustToolchain": "1.97.1",
        "website": "Rust/WASM Dioxus static artifact",
        "ethereum": {
            "network": "Sepolia",
            "registry": "0x36fE9FA585565615Adcfe8680a126F770931E160",
            "deployment": "existing evidence; no deployment performed by this command"
        },
        "demo": {
            "status": "LOCAL REVM EVIDENCE / NOT DEPLOYED",
            "fixture": "fixtures/silent-rollback-v2/manifest.json",
            "evidence": "evidence/local/demo_space_v2_evidence.json",
            "headSequence": 3,
            "authorityRecords": 2,
            "authorityTimeline": "TIMELINE_BOUND",
            "transitionAuthorization": "EOA_SIGNATURES_VERIFIED",
            "stalePredecessorSource": "transition 1 nextStateRoot",
            "expectedReason": "BAD_PREVIOUS_STATE",
            "transactionBroadcast": false
        },
        "legacyFixture": "fixtures/silent-rollback/manifest.json",
        "evidence": [
            "evidence/local/demo_space_v2_evidence.json",
            "evidence/local/demo_space_v2_recovery_receipt.json",
            "evidence/local/demo_space_v2_historical_recovery_receipt.json",
            "evidence/local/memory_lineage_evm_evidence.json",
            "evidence/local/rust_revm_conformance.json",
            "evidence/local/rust_revm_mutation_matrix.json",
            "evidence/local/rust_revm_erc1271.json",
            "evidence/local/rust_revm_authority_rotation.json",
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
        "package-check" => run("bash", &["scripts/check-public-package.sh"]),
        "release" => release_prep(),
        "reproduce" => reproduce(),
        "reviewer-package" => reviewer_package(std::env::args().nth(2).as_deref()),
        "smoke-web" => run("python3", &["scripts/smoke_web.py"]),
        "release-manifest" => release_manifest(std::env::args().nth(2).as_deref()),
        other => Err(format!(
            "unknown xtask command {other}; use doctor, verify, conformance, fixture, build-web, package-check, release, reproduce, reviewer-package, smoke-web, or release-manifest"
        )),
    };

    if let Err(error) = result {
        eprintln!("xtask: {error}");
        std::process::exit(1);
    }
}
