"""Deterministic audit ledger for the 4.7 candidate-search loop.

This is a research control, not a product scorer.  It deliberately keeps
external evidence, local evidence, and conditional targets separate so a
research rating cannot silently become an actual MVP score.
"""

from __future__ import annotations

import argparse
import json
from decimal import Decimal, ROUND_HALF_UP
from pathlib import Path


THRESHOLD = Decimal("4.7")


def half_up(values: list[str]) -> str:
    average = sum((Decimal(value) for value in values), Decimal("0")) / len(values)
    return str(average.quantize(Decimal("0.1"), rounding=ROUND_HALF_UP))


def load_memory_local_gates() -> dict[str, bool]:
    """Read the latest workspace evidence instead of carrying stale gate flags."""
    root = Path(__file__).resolve().parents[1]
    try:
        local_evm = json.loads(
            (root / "evm" / "artifacts" / "local_evm_summary.json").read_text()
        )
        independent = json.loads(
            (root / "spike" / "artifacts" / "memory_lineage_evm_replay.json").read_text()
        )
    except (OSError, json.JSONDecodeError):
        return {
            "contract_or_fork": False,
            "independent_verifier": False,
            "testnet_or_live_flow": False,
            "user_validation_and_measured_impact": False,
        }

    contract_pass = (
        local_evm.get("evidenceType") == "workspace_owned_local_evm"
        and local_evm.get("conformance", {}).get("allMatch") is True
        and local_evm.get("validHistory", {}).get("finalHead", {}).get("sequence") == 4
        and local_evm.get("mutationCount") == local_evm.get("mutationRejectedCount") == 20
    )
    independent_pass = (
        independent.get("verdict") == "PASS"
        and independent.get("validTransitions") == 4
        and independent.get("mutationRejected") == 20
    )
    return {
        "contract_or_fork": contract_pass,
        "independent_verifier": independent_pass,
        "testnet_or_live_flow": False,
        "user_validation_and_measured_impact": False,
    }


def candidate(
    name: str,
    round_id: int,
    dimensions: list[str],
    evidence_class: str,
    source_urls: list[str],
    gates: dict[str, bool],
    *,
    existing_project: bool = False,
    conditional_dimensions: list[str] | None = None,
) -> dict:
    research_average = sum((Decimal(value) for value in dimensions), Decimal("0")) / 4
    actual_gate_pass = all(gates.values()) and not existing_project
    actual_score = (
        str(research_average.quantize(Decimal("0.1"), rounding=ROUND_HALF_UP))
        if actual_gate_pass
        else None
    )
    conditional_score = (
        half_up(conditional_dimensions)
        if conditional_dimensions is not None
        else None
    )
    return {
        "name": name,
        "round": round_id,
        "research_dimensions": dimensions,
        "research_score": half_up(dimensions),
        "conditional_dimensions": conditional_dimensions,
        "conditional_score": conditional_score,
        "evidence_class": evidence_class,
        "source_urls": source_urls,
        "gates": gates,
        "existing_project": existing_project,
        "actual_score": actual_score,
        "actual_4_7": actual_score is not None and Decimal(actual_score) >= THRESHOLD,
    }


def build_ledger() -> dict:
    common_unproven = {
        "contract_or_fork": False,
        "independent_verifier": False,
        "testnet_or_live_flow": False,
        "user_validation_and_measured_impact": False,
    }
    memory_local = load_memory_local_gates()
    rounds = [
        {
            "round": 1,
            "query_scope": "Existing shortlist and local spikes",
            "candidates": [
                candidate(
                    "MemoryLineage Auditor / ERC-8350",
                    1,
                    ["4.6", "4.0", "3.9", "4.5"],
                    "local_evm_conformance_plus_external",
                    [
                        "https://ethereum-magicians.org/t/erc-8350-agent-memory-state-registry/29098",
                        "https://github.com/AwareLiquid/ERC-8350",
                        "https://github.com/ethereum/ERCs/pull/1910",
                    ],
                    memory_local,
                    conditional_dimensions=["4.8", "4.6", "4.7", "4.8"],
                ),
                candidate(
                    "PlanSeal Conformance Lab / ERC-8410",
                    1,
                    ["4.3", "3.9", "3.4", "3.6"],
                    "external_source_backed",
                    ["https://github.com/ethereum/ERCs/pull/1992"],
                    common_unproven,
                ),
                candidate(
                    "ManifestTruth / ERC-8313",
                    1,
                    ["4.2", "3.9", "3.4", "3.7"],
                    "external_source_backed",
                    ["https://github.com/ethereum/ERCs/pull/1836"],
                    common_unproven,
                ),
                candidate(
                    "Warden Compromise Lab",
                    1,
                    ["4.0", "4.0", "3.4", "3.8"],
                    "external_reference_backed",
                    [
                        "https://github.com/ethereum/ERCs/pull/1687",
                        "https://github.com/AuHau/erc-warden",
                    ],
                    common_unproven,
                ),
            ],
        },
        {
            "round": 2,
            "query_scope": "ETHGlobal projects with live demos, source, or tests",
            "candidates": [
                candidate(
                    "FlexGov governance robustness report",
                    2,
                    ["4.2", "4.7", "2.8", "4.2"],
                    "existing_project_benchmark",
                    [
                        "https://ethglobal.com/showcase/flexgov-ooe2m",
                        "https://github.com/abrown1564/flexgov",
                    ],
                    common_unproven,
                    existing_project=True,
                ),
                candidate(
                    "KSwap-VM formal semantics and proofs",
                    2,
                    ["4.4", "4.7", "2.5", "4.0"],
                    "existing_project_benchmark",
                    [
                        "https://ethglobal.com/showcase/kswap-vm-aix5n",
                        "https://github.com/vovunku/swap-vm-verified",
                    ],
                    common_unproven,
                    existing_project=True,
                ),
                candidate(
                    "Doca inventory-budget controller",
                    2,
                    ["4.3", "4.4", "2.4", "4.2"],
                    "existing_project_benchmark",
                    [
                        "https://ethglobal.com/showcase/doca-finance-rjm24",
                        "https://github.com/ottodevs/doca",
                    ],
                    common_unproven,
                    existing_project=True,
                ),
                candidate(
                    "Assay challengeable agent reputation rail",
                    2,
                    ["4.4", "4.0", "2.8", "4.1"],
                    "existing_project_benchmark",
                    [
                        "https://ethglobal.com/showcase/assay-26egq",
                        "https://github.com/fiorelorenzo/assay",
                    ],
                    common_unproven,
                    existing_project=True,
                ),
                candidate(
                    "Commitment Issues human-gated signing",
                    2,
                    ["4.5", "3.9", "2.7", "4.2"],
                    "existing_project_benchmark",
                    ["https://ethglobal.com/showcase/commitment-issues-y1t2h"],
                    common_unproven,
                    existing_project=True,
                ),
            ],
        },
        {
            "round": 3,
            "query_scope": "New agent-memory rights and action-proof standards",
            "candidates": [
                candidate(
                    "MemoryRights / ERC-8264 plus Portable Capsule",
                    3,
                    ["4.6", "4.2", "3.5", "4.2"],
                    "external_reference_implementation",
                    [
                        "https://ethereum-magicians.org/t/erc-8264-ai-agent-memory-access-rights/28584",
                        "https://ethereum-magicians.org/t/erc-8269-body-lease-and-credential-broker/28597",
                        "https://github.com/clavote-boop/rmem-gateway",
                    ],
                    common_unproven,
                    existing_project=True,
                ),
                candidate(
                    "ERC-8263 inference-proof explorer",
                    3,
                    ["4.2", "4.2", "2.6", "4.0"],
                    "external_reference_implementation",
                    [
                        "https://ethereum-magicians.org/t/erc-8263-onchain-proof-layer-for-ai-agents/28577",
                        "https://truthanchor.biz/",
                    ],
                    common_unproven,
                    existing_project=True,
                ),
                candidate(
                    "ERC-8273 action-attestation gate",
                    3,
                    ["4.4", "3.6", "3.6", "4.0"],
                    "external_draft_design",
                    ["https://ethereum-magicians.org/t/erc-8273-attestation-gated-agentic-actions/28617"],
                    common_unproven,
                ),
                candidate(
                    "ERC-8257 tool registry conformance auditor",
                    3,
                    ["4.0", "4.1", "3.0", "3.8"],
                    "external_reference_implementation",
                    [
                        "https://ethereum-magicians.org/t/erc-8257-agent-tool-registry/28457",
                        "https://github.com/ProjectOpenSea/tool-registry",
                    ],
                    common_unproven,
                    existing_project=True,
                ),
            ],
        },
    ]
    all_candidates = [item for group in rounds for item in group["candidates"]]
    actual_hits = [item for item in all_candidates if item["actual_4_7"]]
    conditional_hits = [
        item
        for item in all_candidates
        if item["conditional_score"] is not None
        and Decimal(item["conditional_score"]) >= THRESHOLD
    ]
    return {
        "threshold": str(THRESHOLD),
        "round_count": len(rounds),
        "search_scope_is_exhaustive": False,
        "rounds": rounds,
        "actual_4_7_hits": actual_hits,
        "conditional_4_7_hits": conditional_hits,
        "decision": (
            "ACTUAL_4_7_FOUND"
            if actual_hits
            else "NO_ACTUAL_4_7;_CONDITIONAL_PATH_ONLY"
        ),
    }


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--output-dir", type=Path, required=True)
    args = parser.parse_args()
    ledger = build_ledger()
    args.output_dir.mkdir(parents=True, exist_ok=True)
    output = args.output_dir / "candidate_audit_loop.json"
    output.write_text(json.dumps(ledger, indent=2, sort_keys=True) + "\n")
    print(json.dumps(ledger, indent=2, sort_keys=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
