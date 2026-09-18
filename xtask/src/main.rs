#![forbid(unsafe_code)]

use std::process::{Command, Stdio};

fn run(program: &str, args: &[&str]) -> Result<(), String> {
    let status = Command::new(program)
        .args(args)
        .env("CCACHE_DISABLE", "1")
        .env_remove("RUSTC_WRAPPER")
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .map_err(|error| format!("could not start {program}: {error}"))?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("{program} exited with {status}"))
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

fn main() {
    let command = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "verify".to_owned());
    let result = match command.as_str() {
        "verify" => run("cargo", &["fmt", "--all", "--", "--check"])
            .and_then(|_| {
                run(
                    "cargo",
                    &[
                        "clippy",
                        "--workspace",
                        "--all-targets",
                        "--",
                        "-D",
                        "warnings",
                    ],
                )
            })
            .and_then(|_| run("cargo", &["test", "--workspace"]))
            .and_then(|_| run("cargo", &["run", "-q", "-p", "ml-cli", "--", "conformance"]))
            .and_then(|_| {
                run(
                    "cargo",
                    &[
                        "check",
                        "-p",
                        "memorylineage-inspector",
                        "--target",
                        "wasm32-unknown-unknown",
                    ],
                )
            }),
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
        "release" => run("bash", &["scripts/check-public-package.sh", "--release"]),
        other => Err(format!(
            "unknown xtask command {other}; use verify, conformance, fixture, build-web, package-check, or release"
        )),
    };

    if let Err(error) = result {
        eprintln!("xtask: {error}");
        std::process::exit(1);
    }
}
