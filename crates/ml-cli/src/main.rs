#![forbid(unsafe_code)]

use ml_conformance::run_pinned;
use ml_ethereum::{inspect_registry, simulate_silent_rollback};
use ml_evidence::{load_public_replay_bundle, public_replay_to_v2, write_v2_bundle};
use ml_memory_store::{
    create_silent_rollback_fixture, inspect_silent_rollback_fixture, restore_snapshot,
};
use ml_verifier_independent::verify_file;
use std::path::{Path, PathBuf};

fn repository_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn default_fixture_root() -> PathBuf {
    repository_root().join("fixtures/silent-rollback")
}

fn usage() {
    eprintln!(
        "Usage:\n  \
  cargo run -p ml-cli -- fixture create [ROOT]\n  \
  cargo run -p ml-cli -- fixture inspect [ROOT]\n  \
  cargo run -p ml-cli -- fixture restore <SOURCE> <DESTINATION>\n  \
  cargo run -p ml-cli -- verify <EVIDENCE.json>\n  \
  cargo run -p ml-cli -- conformance\n  \
  cargo run -p ml-cli -- revm silent-rollback\n  \
  cargo run -p ml-cli -- revm mutations\n  \
  cargo run -p ml-cli -- revm erc1271\n  \
  cargo run -p ml-cli -- revm authority-rotation\n  \
  cargo run -p ml-cli -- evidence export-v2 [SOURCE] [DESTINATION]\n  \
  cargo run -p ml-cli -- demo silent-rollback [ROOT]\n  \
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

fn revm_command(action: &str) -> Result<(), Box<dyn std::error::Error>> {
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
        Some(other) => Err(format!("unknown evidence action: {other}").into()),
        None => Err("evidence requires an action".into()),
    }
}

fn silent_rollback_demo(root: &Path) -> Result<(), Box<dyn std::error::Error>> {
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
            let simulation = runtime.block_on(simulate_silent_rollback(&rpc, &registry, &space))?;
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
        Some("revm") => {
            let action = args
                .next()
                .ok_or("revm requires silent-rollback or mutations")?;
            revm_command(&action)
        }
        Some("evidence") => evidence_command(&args.collect::<Vec<_>>()),
        Some("demo") => match args.next().as_deref() {
            Some("silent-rollback") => {
                let root = args
                    .next()
                    .map(PathBuf::from)
                    .unwrap_or_else(default_fixture_root);
                silent_rollback_demo(&root)
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
