"""Generate deterministic evidence for the SlippageTruth research spike."""

from __future__ import annotations

import json
from argparse import ArgumentParser
from pathlib import Path
from typing import Any

from slippage_truth import evaluate_case, replay_evidence


CASES: list[dict[str, Any]] = [
    {
        "case_id": "adverse-output",
        "static_min_out": 900,
        "reference_out": 1000,
        "expected_cost_bps": 300,
        "max_deviation_bps": 100,
        "hard_floor": 0,
        "recipient_delta": 930,
        "route_reported_out": 930,
        "oracle_fresh": True,
        "deadline_ok": True,
    },
    {
        "case_id": "honest-output",
        "static_min_out": 900,
        "reference_out": 1000,
        "expected_cost_bps": 300,
        "max_deviation_bps": 100,
        "hard_floor": 950,
        "recipient_delta": 965,
        "route_reported_out": 965,
        "oracle_fresh": True,
        "deadline_ok": True,
    },
    {
        "case_id": "unpaid-route-report",
        "static_min_out": 900,
        "reference_out": 1000,
        "expected_cost_bps": 300,
        "max_deviation_bps": 100,
        "hard_floor": 0,
        "recipient_delta": 0,
        "route_reported_out": 1000,
        "oracle_fresh": True,
        "deadline_ok": True,
    },
    {
        "case_id": "stale-oracle",
        "static_min_out": 900,
        "reference_out": 1000,
        "expected_cost_bps": 300,
        "max_deviation_bps": 100,
        "hard_floor": 0,
        "recipient_delta": 1000,
        "route_reported_out": 1000,
        "oracle_fresh": False,
        "deadline_ok": True,
    },
    {
        "case_id": "expired-intent",
        "static_min_out": 900,
        "reference_out": 1000,
        "expected_cost_bps": 300,
        "max_deviation_bps": 100,
        "hard_floor": 0,
        "recipient_delta": 1000,
        "route_reported_out": 1000,
        "oracle_fresh": True,
        "deadline_ok": False,
    },
]


def run_spike(output_dir: Path) -> dict[str, Any]:
    output_dir.mkdir(parents=True, exist_ok=True)
    results = [evaluate_case(case) for case in CASES]
    replay_mismatches = sum(
        replay_evidence(result["evidence"]) != result["verdict"] for result in results
    )
    summary = {
        "spike": "SlippageTruth Lab",
        "throwaway": True,
        "case_count": len(results),
        "replay_mismatches": replay_mismatches,
        "verdicts": [result["verdict"] for result in results],
        "results": results,
    }
    (output_dir / "slippage_truth_summary.json").write_text(
        json.dumps(summary, indent=2, sort_keys=True) + "\n"
    )
    return summary


def main() -> int:
    parser = ArgumentParser()
    parser.add_argument("--output-dir", type=Path, default=Path("evidence/generated/research-spikes"))
    args = parser.parse_args()
    summary = run_spike(args.output_dir)
    print(json.dumps(summary, indent=2, sort_keys=True))
    return 0 if summary["replay_mismatches"] == 0 else 1


if __name__ == "__main__":
    raise SystemExit(main())
