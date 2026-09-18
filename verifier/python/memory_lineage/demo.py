"""Run the throwaway MemoryLineage Auditor spike and write JSON evidence."""

from __future__ import annotations

import argparse
import copy
import json
from pathlib import Path

from .model import (
    PUBLISHED_VECTOR,
    ZERO_ROOT,
    evidence_for,
    make_transition,
    space_id,
    verify_published_vector,
)


AUTHOR = "0x3333333333333333333333333333333333333333"
OTHER = "0x4444444444444444444444444444444444444444"
SPACE = space_id("0x2222222222222222222222222222222222222222", PUBLISHED_VECTOR["space_salt"])
PROFILE = PUBLISHED_VECTOR["profile_id"]
SALT = "0x" + "aa" * 32
PROVENANCE_SALT = "0x" + "cc" * 32
LOCATOR_SALT = "0x" + "bb" * 32


def valid_history() -> list:
    history = []
    root = ZERO_ROOT
    for sequence in range(1, 5):
        transition = make_transition(
            space=SPACE,
            sequence=sequence,
            previous_root=root,
            authorizer=AUTHOR,
            payload=f'{{"op":"upsert","resourceId":"memory-{sequence}"}}',
            provenance=f'{{"input":"0x{sequence:02x}"}}',
            locator=f"ipfs://memory-{sequence}",
            profile_id=PROFILE,
            delta_salt=SALT,
            provenance_salt=PROVENANCE_SALT,
            locator_salt=LOCATOR_SALT,
        )
        history.append(transition)
        root = transition.next_state_root
    return history


def mutated_cases() -> dict[str, list]:
    base = valid_history()

    rollback = copy.deepcopy(base)
    rollback[2] = make_transition(
        space=SPACE,
        sequence=3,
        previous_root=base[0].next_state_root,
        authorizer=AUTHOR,
        payload=base[2].payload,
        provenance=base[2].provenance,
        locator=base[2].locator,
        profile_id=PROFILE,
        delta_salt=SALT,
        provenance_salt=PROVENANCE_SALT,
        locator_salt=LOCATOR_SALT,
    )

    gap = copy.deepcopy(base)
    gap[2] = make_transition(
        space=SPACE,
        sequence=4,
        previous_root=base[1].next_state_root,
        authorizer=AUTHOR,
        payload=base[2].payload,
        provenance=base[2].provenance,
        locator=base[2].locator,
        profile_id=PROFILE,
        delta_salt=SALT,
        provenance_salt=PROVENANCE_SALT,
        locator_salt=LOCATOR_SALT,
    )

    parallel = copy.deepcopy(base)
    parallel.insert(2, base[1])

    unauthorized = copy.deepcopy(base)
    unauthorized[1] = make_transition(
        space=SPACE,
        sequence=2,
        previous_root=base[0].next_state_root,
        authorizer=OTHER,
        payload=base[1].payload,
        provenance=base[1].provenance,
        locator=base[1].locator,
        profile_id=PROFILE,
        delta_salt=SALT,
        provenance_salt=PROVENANCE_SALT,
        locator_salt=LOCATOR_SALT,
    )

    locator_substitution = copy.deepcopy(base)
    locator_substitution[1] = copy.deepcopy(base[1])
    locator_substitution[1].locator = "ipfs://substituted-locator"

    payload_tamper = copy.deepcopy(base)
    payload_tamper[1] = copy.deepcopy(base[1])
    payload_tamper[1].payload = '{"op":"upsert","resourceId":"poisoned"}'

    return {
        "valid": base,
        "rollback": rollback,
        "sequence-gap": gap,
        "parallel-history": parallel,
        "unauthorized-authorizer": unauthorized,
        "locator-substitution": locator_substitution,
        "payload-tamper": payload_tamper,
    }


def run_spike(output_dir: Path) -> dict:
    output_dir.mkdir(parents=True, exist_ok=True)
    vector_checks = verify_published_vector()
    cases = [
        evidence_for(case_id, history, AUTHOR)
        for case_id, history in mutated_cases().items()
    ]
    replay_mismatches = sum(
        item["verdict"] != item["replay_verdict"] for item in cases
    )
    summary = {
        "candidate": "MemoryLineage Auditor / ERC-8350",
        "scope": "local commitment and linear-history model; no live chain or signature verification",
        "published_vector_checks": vector_checks,
        "case_count": len(cases),
        "cases": [
            {
                "case_id": item["case_id"],
                "verdict": item["verdict"],
                "replay_verdict": item["replay_verdict"],
                "failure_codes": item["failure_codes"],
            }
            for item in cases
        ],
        "mutation_non_pass_count": sum(item["verdict"] != "PASS" for item in cases[1:]),
        "replay_mismatches": replay_mismatches,
        "gate_pass": all(vector_checks.values()) and replay_mismatches == 0,
    }
    (output_dir / "memory_lineage_summary.json").write_text(
        json.dumps(summary, indent=2, sort_keys=True) + "\n"
    )
    (output_dir / "memory_lineage_evidence.json").write_text(
        json.dumps(cases, indent=2, sort_keys=True) + "\n"
    )
    return summary


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--output-dir", type=Path, required=True)
    args = parser.parse_args()
    summary = run_spike(args.output_dir)
    print(json.dumps(summary, indent=2, sort_keys=True))
    return 0 if summary["gate_pass"] else 1


if __name__ == "__main__":
    raise SystemExit(main())
