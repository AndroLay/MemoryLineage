"""Run the throwaway ResolverCompat gate and write replayable evidence."""

from __future__ import annotations

import copy
import json
from argparse import ArgumentParser
from pathlib import Path
from typing import Any

from resolver_compat import MockResolver, evaluate_resolver, verify_evidence


POLICY = {
    "solver_address": "0xsolver",
    "allowed_assumptions": ["trusted-settler"],
    "allowed_targets": ["0xsettler"],
}


def valid_fixture() -> dict[str, Any]:
    return {
        "fixture_id": "valid-order",
        "payload": {"order_id": "order-001", "nonce": 7},
        "resolved_order": {
            "steps": [
                {
                    "target": "0xsettler",
                    "selector": "0x12345678",
                    "arguments": [],
                    "attributes": [{"type": "RevertPolicy", "policy": "abort"}],
                }
            ],
            "variables": [
                {"role": "PaymentRecipient", "value": "0xsolver"},
                {"role": "PaymentChain", "value": 1},
            ],
            "payments": [
                {
                    "token": "0xtoken",
                    "sender": "0xuser",
                    "amount": 100,
                    "recipient_var_idx": 0,
                    "chain": 1,
                    "on_step_idx": 0,
                }
            ],
            "assumptions": [{"name": "trusted-settler", "data": "0xsettler"}],
        },
    }


def mutation_fixtures() -> list[tuple[str, dict[str, Any]]]:
    mutations: list[tuple[str, dict[str, Any]]] = []

    def add(name: str, edit) -> None:
        fixture = valid_fixture()
        edit(fixture)
        fixture["fixture_id"] = f"mutation-{name}"
        mutations.append((name, fixture))

    add("target", lambda f: f["resolved_order"]["steps"][0].update(target="0xattacker"))
    add("selector", lambda f: f["resolved_order"]["steps"][0].update(selector="0x1234"))
    add(
        "recipient-value",
        lambda f: f["resolved_order"]["variables"][0].update(value="0xattacker"),
    )
    add("payment-chain", lambda f: f["resolved_order"]["payments"][0].update(chain=2))
    add(
        "recipient-index",
        lambda f: f["resolved_order"]["payments"][0].update(recipient_var_idx=99),
    )
    add("step-index", lambda f: f["resolved_order"]["payments"][0].update(on_step_idx=99))
    add("revert-policy", lambda f: f["resolved_order"]["steps"][0].update(attributes=[]))
    add(
        "variable-index",
        lambda f: f["resolved_order"]["steps"][0].update(arguments=[{"var_idx": 99}]),
    )
    add(
        "unknown-assumption",
        lambda f: f["resolved_order"]["assumptions"].__setitem__(0, {"name": "unknown-condition"}),
    )
    add(
        "recipient-role",
        lambda f: f["resolved_order"]["payments"][0].update(recipient_var_idx=1),
    )
    add("chain-type", lambda f: f["resolved_order"]["payments"][0].update(chain="0x1"))
    add("payment-structure", lambda f: f["resolved_order"]["payments"].__setitem__(0, None))
    return mutations


def _write_evidence(directory: Path, name: str, evidence: dict[str, Any]) -> str:
    path = directory / f"{name}.evidence.json"
    path.write_text(json.dumps(evidence, indent=2, sort_keys=True) + "\n")
    return str(path)


def run_spike(output_dir: Path) -> dict[str, Any]:
    """Run base fixtures and mutation corpus, returning the gate summary."""

    output_dir.mkdir(parents=True, exist_ok=True)
    fixture_dir = output_dir / "fixtures"
    mutation_dir = output_dir / "mutations"
    fixture_dir.mkdir(exist_ok=True)
    mutation_dir.mkdir(exist_ok=True)

    base_cases = [
        ("valid", valid_fixture(), "PASS"),
        (
            "payment-mismatch",
            {
                **valid_fixture(),
                "fixture_id": "payment-mismatch",
                "resolved_order": {
                    **valid_fixture()["resolved_order"],
                    "variables": [{"role": "PaymentRecipient", "value": "0xattacker"}, {"role": "PaymentChain", "value": 1}],
                },
            },
            "REJECT",
        ),
        (
            "unknown-assumption",
            {
                **valid_fixture(),
                "fixture_id": "unknown-assumption",
                "resolved_order": {
                    **valid_fixture()["resolved_order"],
                    "assumptions": [{"name": "unknown-condition", "data": "0x"}],
                },
            },
            "UNVERIFIED",
        ),
    ]

    fixture_results = []
    replay_mismatches = 0
    for name, fixture, expected in base_cases:
        resolver = MockResolver(fixture)
        result = evaluate_resolver(resolver, POLICY)
        replay = verify_evidence(result["evidence"])
        replay_mismatches += replay != result["verdict"]
        fixture_results.append(
            {
                "name": name,
                "expected": expected,
                "verdict": result["verdict"],
                "replay_verdict": replay,
                "failure_codes": result["failure_codes"],
                "evidence_file": _write_evidence(fixture_dir, name, result["evidence"]),
            }
        )

    mutation_results = []
    for name, fixture in mutation_fixtures():
        result = evaluate_resolver(MockResolver(copy.deepcopy(fixture)), POLICY)
        replay = verify_evidence(result["evidence"])
        replay_mismatches += replay != result["verdict"]
        mutation_results.append(
            {
                "name": name,
                "verdict": result["verdict"],
                "replay_verdict": replay,
                "failure_codes": result["failure_codes"],
                "evidence_file": _write_evidence(mutation_dir, name, result["evidence"]),
            }
        )

    mutation_non_pass_count = sum(item["verdict"] != "PASS" for item in mutation_results)
    gate_pass = (
        fixture_results[0]["verdict"] == "PASS"
        and fixture_results[1]["verdict"] == "REJECT"
        and fixture_results[2]["verdict"] == "UNVERIFIED"
        and mutation_non_pass_count == len(mutation_results)
        and replay_mismatches == 0
    )
    summary = {
        "spike": "ResolverCompat / FillerSafety Lab",
        "throwaway": True,
        "gate_pass": gate_pass,
        "fixture_count": len(fixture_results),
        "mutation_count": len(mutation_results),
        "mutation_non_pass_count": mutation_non_pass_count,
        "replay_mismatches": replay_mismatches,
        "fixture_results": fixture_results,
        "mutation_results": mutation_results,
    }
    (output_dir / "summary.json").write_text(json.dumps(summary, indent=2, sort_keys=True) + "\n")
    return summary


def main() -> int:
    parser = ArgumentParser()
    parser.add_argument("--output-dir", type=Path, default=Path("evidence/generated/research-spikes"))
    args = parser.parse_args()
    summary = run_spike(args.output_dir)
    print(json.dumps(summary, indent=2, sort_keys=True))
    return 0 if summary["gate_pass"] else 1


if __name__ == "__main__":
    raise SystemExit(main())
