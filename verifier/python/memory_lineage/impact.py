"""Measure reproducible MemoryLineage impact metrics without claiming user validation."""

from __future__ import annotations

import argparse
import json
import statistics
import subprocess
import sys
import time
from pathlib import Path
from typing import Any


def summarize_evidence(
    local: dict[str, Any],
    replay: dict[str, Any],
    *,
    replay_runs: int,
    replay_ms: list[float],
) -> dict[str, Any]:
    valid_transitions = local.get("validHistory", {}).get("transitions", [])
    gas = [
        int(item["totalGasSpent"])
        for item in local.get("validHistory", {}).get("gas", [])
    ]
    valid_count = len(valid_transitions)
    valid_replay_count = int(replay.get("validTransitions", 0))
    mutation_count = int(local.get("mutationCount", 0))
    mutation_rejected = int(local.get("mutationRejectedCount", 0))
    valid_corpus_accepted = (
        replay.get("verdict") == "PASS"
        and valid_replay_count == valid_count
    )
    continuation_gas = gas[1:]
    machine_metrics = {
        "validCorpus": valid_count,
        "validCorpusAccepted": valid_replay_count,
        "falseRejectRateOnValidCorpus": (
            0.0 if valid_corpus_accepted else 1.0
        ),
        "mutationCorpus": mutation_count,
        "mutationRejected": mutation_rejected,
        "mutationRejectionRate": (
            mutation_rejected / mutation_count if mutation_count else None
        ),
        "replayMismatchRate": 0.0 if valid_corpus_accepted else 1.0,
        "gas": {
            "firstTransition": gas[0] if gas else None,
            "continuationMean": (
                statistics.mean(continuation_gas) if continuation_gas else None
            ),
            "allTransitions": gas,
        },
    }
    benchmark = {
        "runs": replay_runs,
        "unit": "milliseconds",
        "samples": replay_ms,
        "minimum": min(replay_ms) if replay_ms else None,
        "median": statistics.median(replay_ms) if replay_ms else None,
        "maximum": max(replay_ms) if replay_ms else None,
    }
    return {
        "evidenceType": "workspace_local_impact_measurement",
        "machineMetrics": machine_metrics,
        "replayBenchmark": benchmark,
        "developerValidation": {
            "status": "PENDING",
            "requiredIndependentDevelopers": 2,
            "completed": 0,
            "artifact": "evidence/generated/impact_validation.json",
        },
        "limits": [
            "metrics use the local EthereumJS VM and the exported evidence bundle",
            "false reject rate is measured only on the supplied valid corpus",
            "gas is a local VM estimate, not a public-network fee quote",
            "human comprehension and independent developer validation are not automated",
        ],
    }


def run_replay_benchmark(root: Path, repeats: int) -> list[float]:
    if repeats < 1:
        raise ValueError("repeats must be positive")
    command = [sys.executable, str(root / "verifier" / "python" / "memory_lineage" / "replay.py")]
    samples: list[float] = []
    for _ in range(repeats):
        started = time.perf_counter()
        completed = subprocess.run(
            command,
            cwd=root,
            capture_output=True,
            text=True,
            check=False,
        )
        elapsed = (time.perf_counter() - started) * 1000
        if completed.returncode != 0:
            raise RuntimeError(completed.stderr.strip() or "independent replay failed")
        output = json.loads(completed.stdout)
        if output.get("verdict") != "PASS":
            raise RuntimeError("independent replay did not return PASS")
        samples.append(round(elapsed, 3))
    return samples


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--root", type=Path, default=Path(__file__).resolve().parents[1])
    parser.add_argument("--repeats", type=int, default=5)
    args = parser.parse_args()
    root = args.root.resolve()
    local = json.loads((root / "evidence" / "generated" / "local_evm_summary.json").read_text())
    replay = json.loads((root / "evidence" / "generated" / "memory_lineage_evm_replay.json").read_text())
    samples = run_replay_benchmark(root, args.repeats)
    result = summarize_evidence(
        local,
        replay,
        replay_runs=len(samples),
        replay_ms=samples,
    )
    output_path = root / "evidence" / "generated" / "memory_lineage_impact.json"
    output_path.parent.mkdir(parents=True, exist_ok=True)
    output_path.write_text(json.dumps(result, indent=2) + "\n")
    print(json.dumps(result, indent=2))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
