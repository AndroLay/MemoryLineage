#!/usr/bin/env python3
"""Build the local Demo Space V2 submission envelope from checked-in evidence."""

from __future__ import annotations

import hashlib
import json
import subprocess
import tempfile
from pathlib import Path


ROOT = Path(__file__).resolve().parent.parent
OUT = ROOT / "evidence/submission"
INCIDENT = OUT / "demo-space-v2"


def cli(*args: str) -> str:
    return subprocess.check_output(
        ["cargo", "run", "-q", "-p", "ml-cli", "--", *args],
        cwd=ROOT,
        text=True,
    )


def write_json(path: Path, value: object) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(value, indent=2, ensure_ascii=False) + "\n")


def artifact(path: str, source: str, generated_by: str, status: str = "VERIFIED_LOCAL") -> dict:
    return {
        "path": path,
        "sha256": hashlib.sha256((ROOT / path).read_bytes()).hexdigest(),
        "sourceClass": source,
        "generatedBy": generated_by,
        "status": status,
    }


def main() -> None:
    with tempfile.TemporaryDirectory(prefix="memorylineage-submission-") as temp:
        temporary_manifest = Path(temp) / "fixture-manifest.json"
        cli("fixture", "demo-manifest", "fixtures/silent-rollback-v2", str(temporary_manifest))
        fixture_path = ROOT / "fixtures/silent-rollback-v2/manifest.json"
        if temporary_manifest.read_bytes() != fixture_path.read_bytes():
            raise SystemExit("Demo fixture manifest differs from the Rust-generated version")

    fixture = json.loads(fixture_path.read_text())
    evidence = json.loads((ROOT / "evidence/local/demo_space_v2_evidence.json").read_text())
    revm = json.loads(cli("revm", "demo-space-v2", "fixtures/silent-rollback-v2"))
    if revm != evidence:
        raise SystemExit("Demo evidence differs from current Rust/revm execution")
    write_json(INCIDENT / "revm-observation.json", revm)

    source = "DEMO_SPACE_V2_LOCAL"
    incident_id = "demo-space-v2-silent-rollback"
    snapshot_path = "fixtures/silent-rollback-v2/snapshot-1.db"
    snapshot_commitment = fixture["snapshots"][0]["snapshotCommitment"]
    stale_root = evidence["transitions"][0]["nextStateRoot"]
    head = evidence["head"]["stateRoot"]
    attack = evidence["attack"]
    if (
        attack["stalePredecessor"] != stale_root
        or attack["canonicalPredecessor"] != head
        or snapshot_commitment != evidence["transitions"][0]["deltaCommitment"]
        or snapshot_commitment == stale_root
    ):
        raise SystemExit("Demo snapshot commitment/root relationship differs")

    rollback_path = "evidence/submission/demo-space-v2/rollback-rehearsal.json"
    write_json(
        ROOT / rollback_path,
        {
            "schemaVersion": "memorylineage-rollback-rehearsal-v1",
            "incidentId": incident_id,
            "sourceClass": source,
            "snapshotCommitment": snapshot_commitment,
            "stalePredecessor": stale_root,
            "canonicalHead": head,
            "attemptedSequence": attack["attemptedSequence"],
            "revertReason": attack["reason"],
            "transactionBroadcast": attack["transactionBroadcast"],
            "executionSource": attack["executionSource"],
        },
    )
    incident_paths = [
        "fixtures/silent-rollback-v2/manifest.json",
        *[f"fixtures/silent-rollback-v2/snapshot-{index}.db" for index in (1, 2, 3)],
        "evidence/local/demo_space_v2_evidence.json",
        "evidence/submission/demo-space-v2/revm-observation.json",
        rollback_path,
        "evidence/local/demo_space_v2_recovery_receipt.json",
        "evidence/local/demo_space_v2_historical_recovery_receipt.json",
    ]
    incident_path = "evidence/submission/demo-space-v2/manifest.json"
    write_json(
        ROOT / incident_path,
        {
            "schemaVersion": "memorylineage-submission-incident-v1",
            "incidentId": incident_id,
            "sourceClass": source,
            "fixtureId": fixture["fixtureId"],
            "canonical": {
                "headSequence": evidence["head"]["sequence"],
                "headStateRoot": head,
                "evidence": "evidence/local/demo_space_v2_evidence.json",
            },
            "restoredSnapshot": {
                "sequence": fixture["snapshots"][0]["sequence"],
                "snapshot": snapshot_path,
                "snapshotCommitment": snapshot_commitment,
                "staleRoot": stale_root,
            },
            "attempt": {
                "sequence": attack["attemptedSequence"],
                "revertReason": attack["reason"],
                "transactionBroadcast": attack["transactionBroadcast"],
                "execution": "Rust/revm local execution",
            },
            "artifacts": incident_paths,
        },
    )

    metadata = {
        incident_path: (source, "scripts/build_submission_bundle.py"),
        rollback_path: (source, "scripts/build_submission_bundle.py"),
        "evidence/submission/demo-space-v2/revm-observation.json": (source, "ml-cli revm demo-space-v2"),
        "fixtures/silent-rollback-v2/manifest.json": (source, "ml-cli fixture demo-manifest"),
        **{f"fixtures/silent-rollback-v2/snapshot-{index}.db": (source, "ml-cli fixture demo-create") for index in (1, 2, 3)},
        "evidence/local/demo_space_v2_evidence.json": (source, "ml-cli evidence demo-v2"),
        "evidence/local/demo_space_v2_recovery_receipt.json": (source, "ml-cli recover preflight"),
        "evidence/local/demo_space_v2_historical_recovery_receipt.json": (source, "ml-cli recover preflight"),
        "evidence/local/memory_lineage_evm_evidence.json": ("PROTOCOL_CORPUS_LOCAL", "published local Rust/revm protocol corpus"),
        "evidence/local/reference_agent_runtime.json": (source, "ml-cli agent reference-demo"),
        "evidence/local/security_assurance_report.json": ("PROTOCOL_CORPUS_LOCAL", "ml-cli security bounded-audit"),
    }
    artifacts = [artifact(path, *meta) for path, meta in metadata.items()]
    write_json(
        OUT / "manifest.json",
        {
            "schemaVersion": "memorylineage-submission-v1",
            "submissionCommit": None,
            "incidentId": incident_id,
            "sourceClasses": [source, "PROTOCOL_CORPUS_LOCAL", "EXTERNAL_REPRODUCTION"],
            "artifacts": artifacts,
            "claims": [
                {"id": "silent-rollback", "status": "VERIFIED_LOCAL", "artifactPaths": [incident_path, rollback_path, "evidence/local/demo_space_v2_evidence.json"]},
                {"id": "recovery-decision", "status": "VERIFIED_LOCAL", "artifactPaths": ["evidence/local/demo_space_v2_recovery_receipt.json", "evidence/local/demo_space_v2_historical_recovery_receipt.json"]},
                {"id": "protocol-corpus", "status": "VERIFIED_LOCAL", "artifactPaths": ["evidence/local/memory_lineage_evm_evidence.json"]},
                {"id": "reference-runtime", "status": "VERIFIED_LOCAL", "artifactPaths": ["evidence/local/reference_agent_runtime.json"]},
                {"id": "bounded-assurance", "status": "BOUNDED_LOCAL", "artifactPaths": ["evidence/local/security_assurance_report.json"]},
                {"id": "external-reproduction", "status": "NOT_YET_DEMONSTRATED", "artifactPaths": []},
            ],
            "verificationCommands": [
                "cargo run -q -p ml-cli -- submission verify evidence/submission/manifest.json",
                "cargo xtask verify",
                "cargo xtask smoke-web",
            ],
            "limitations": [
                "Local Rust/revm execution is not a public network transaction.",
                "The V2 source class is a declared scope; public RPC provenance is separate.",
                "External human reproduction has not been demonstrated.",
                "The bounded security report is not a formal audit.",
                "The reference runtime is local and framework neutral.",
            ],
            "demoIncident": {"sourceClass": source, "status": "VERIFIED_LOCAL", "artifactPath": incident_path},
            "protocolCorpus": {"sourceClass": "PROTOCOL_CORPUS_LOCAL", "status": "VERIFIED_LOCAL", "artifactPath": "evidence/local/memory_lineage_evm_evidence.json"},
            "runtimeIntegration": {"sourceClass": source, "status": "VERIFIED_LOCAL", "artifactPath": "evidence/local/reference_agent_runtime.json"},
            "boundedAssurance": {"sourceClass": "PROTOCOL_CORPUS_LOCAL", "status": "BOUNDED_LOCAL", "artifactPath": "evidence/local/security_assurance_report.json"},
            "externalReproduction": {"sourceClass": "EXTERNAL_REPRODUCTION", "status": "NOT_YET_DEMONSTRATED", "artifactPath": None},
        },
    )
    print("wrote deterministic local submission bundle")


if __name__ == "__main__":
    main()
