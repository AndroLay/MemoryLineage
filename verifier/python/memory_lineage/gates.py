"""Evaluate MemoryLineage evidence gates without trusting narrative claims."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from .paths import REPOSITORY_ROOT


ACCEPTANCE_FIELDS = (
    "validTransitionRecognized",
    "sequenceGapRecognized",
    "signatureBindingRecognized",
    "privacyBoundaryRecognized",
    "semanticTruthLimitationRecognized",
    "observedVerdictsMatchIndependentReplay",
)
RUN_FIELDS = {
    "developerAlias",
    "timestamp",
    "checkoutHash",
    "answers",
    "observedVerdicts",
    "replayMilliseconds",
    "mismatches",
}


def evaluate_gates(
    *,
    local: dict[str, Any],
    replay: dict[str, Any],
    deployment: dict[str, Any],
    reread: dict[str, Any],
    impact: dict[str, Any],
    machine_impact: dict[str, Any] | None = None,
) -> dict[str, Any]:
    valid_count = len(local.get("validHistory", {}).get("transitions", []))
    contract_conformance = (
        local.get("evidenceType") == "workspace_owned_local_evm"
        and local.get("conformance", {}).get("allMatch") is True
        and valid_count == 4
        and local.get("mutationCount") == 20
        and local.get("mutationRejectedCount") == 20
    )
    independent_replay = (
        replay.get("verdict") == "PASS"
        and replay.get("validTransitions") == valid_count == 4
        and replay.get("replayMismatches") == 0
        and replay.get("mutationRejected") == 20
    )
    deployment_present = (
        deployment.get("evidenceType") == "workspace_owned_public_sepolia_deployment"
        and deployment.get("chainId") == "11155111"
        and bool(deployment.get("registryAddress"))
    )
    reread_present = (
        reread.get("evidenceType")
        == "second_rpc_reread_workspace_owned_public_sepolia_deployment"
        and reread.get("verdict") == "PASS"
        and reread.get("registryAddress") == deployment.get("registryAddress")
        and bool(reread.get("checks"))
        and all(reread["checks"].values())
    )
    testnet_gate = deployment_present and reread_present
    acceptance = impact.get("acceptance", {})
    runs = impact.get("runs", [])
    aliases = [run.get("developerAlias") for run in runs if isinstance(run, dict)]
    runs_complete = (
        len(runs) >= 2
        and len(set(aliases)) == len(aliases)
        and all(RUN_FIELDS.issubset(run) for run in runs[:2] if isinstance(run, dict))
    )
    developer_gate = (
        impact.get("status") == "PASS"
        and int(impact.get("completed", 0)) >= 2
        and int(impact.get("requiredIndependentDevelopers", 0)) >= 2
        and runs_complete
        and all(acceptance.get(field) is True for field in ACCEPTANCE_FIELDS)
    )
    actual = contract_conformance and independent_replay and testnet_gate and developer_gate
    human_validation = impact.get("humanDeveloperValidationStatus") == "PASS"
    result = {
        "candidate": "MemoryLineage Auditor / ERC-8350",
        "evidenceGatesPass": actual,
        "localEvm": {
            "contractConformance": contract_conformance,
            "independentReplay": independent_replay,
            "validTransitions": valid_count,
            "mutationCases": local.get("mutationCount", 0),
            "mutationRejected": local.get("mutationRejectedCount", 0),
            "replayMismatches": replay.get("replayMismatches"),
        },
        "externalGates": {
            "workspacePublicTestnetDeployment": testnet_gate,
            "secondRpcOrIndexerReread": reread_present,
            "independentDeveloperValidation": developer_gate,
        },
        "validationEvidence": {
            "mode": impact.get("validationMode", "unspecified"),
            "humanDeveloperValidation": human_validation,
        },
        "evidencePaths": {
            "deployment": "evidence/sepolia/sepolia_deployment.json",
            "reread": "evidence/sepolia/sepolia_reread.json",
            "developerValidation": "evidence/generated/research-spikes/impact_validation.json",
        },
        "reason": (
            "All local and external gates passed."
            if actual
            else "At least one required gate is missing or not independently evidenced."
        ),
    }
    if machine_impact:
        result["machineImpact"] = {
            "validCorpus": machine_impact.get("machineMetrics", {}).get("validCorpus"),
            "validCorpusAccepted": machine_impact.get("machineMetrics", {}).get("validCorpusAccepted"),
            "falseRejectRateOnValidCorpus": machine_impact.get("machineMetrics", {}).get("falseRejectRateOnValidCorpus"),
            "mutationRejectionRate": machine_impact.get("machineMetrics", {}).get("mutationRejectionRate"),
            "gasFirstTransition": machine_impact.get("machineMetrics", {}).get("gas", {}).get("firstTransition"),
            "gasContinuationMean": machine_impact.get("machineMetrics", {}).get("gas", {}).get("continuationMean"),
            "replayBenchmarkRuns": machine_impact.get("replayBenchmark", {}).get("runs"),
            "replayBenchmarkMedianMs": machine_impact.get("replayBenchmark", {}).get("median"),
        }
    return result


def read_json(path: Path) -> dict[str, Any]:
    try:
        return json.loads(path.read_text())
    except (FileNotFoundError, json.JSONDecodeError):
        return {}


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--root", type=Path, default=REPOSITORY_ROOT)
    args = parser.parse_args()
    root = args.root.resolve()
    result = evaluate_gates(
        local=read_json(root / "evidence" / "generated" / "local_evm_summary.json"),
        replay=read_json(root / "evidence" / "generated" / "memory_lineage_evm_replay.json"),
        deployment=read_json(root / "evidence" / "sepolia" / "sepolia_deployment.json"),
        reread=read_json(root / "evidence" / "sepolia" / "sepolia_reread.json"),
        impact=read_json(root / "evidence" / "generated" / "research-spikes" / "impact_validation.json"),
        machine_impact=read_json(root / "evidence" / "generated" / "memory_lineage_impact.json"),
    )
    output = root / "evidence" / "generated" / "memory_lineage_evidence_gates.json"
    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_text(json.dumps(result, indent=2) + "\n")
    print(json.dumps(result, indent=2))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
