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
    let manifest = serde_json::json!({
        "manifestType": "memorylineage-release-preparation",
        "repository": "AndroLay/MemoryLineage",
        "gitRevision": revision,
        "rustToolchain": "1.97.1",
        "website": "Rust/WASM Dioxus static artifact",
        "ethereum": {
            "network": "Sepolia",
            "registry": "0x36fE9FA585565615Adcfe8680a126F770931E160",
            "deployment": "existing evidence; no deployment performed by this command"
        },
        "fixture": "fixtures/silent-rollback/manifest.json",
        "evidence": [
            "evidence/local/rust_revm_conformance.json",
            "evidence/local/rust_revm_mutation_matrix.json",
            "evidence/sepolia/sepolia_reread.json",
            "evidence/sepolia/silent_rollback_fixture_eth_call.json"
        ],
        "excludedByScope": ["demo video", "public deployment", "staging"]
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
        "smoke-web" => run("python3", &["scripts/smoke_web.py"]),
        "release-manifest" => release_manifest(std::env::args().nth(2).as_deref()),
        other => Err(format!(
            "unknown xtask command {other}; use verify, conformance, fixture, build-web, package-check, release, smoke-web, or release-manifest"
        )),
    };

    if let Err(error) = result {
        eprintln!("xtask: {error}");
        std::process::exit(1);
    }
}
