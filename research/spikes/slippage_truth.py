"""Small deterministic model for the SlippageTruth research spike.

This is a policy model, not a router or an MEV simulator.  It exists to test
whether the proposed product can produce a falsifiable distinction between a
stale static minimum and a floor derived from a fresh reference at execution.
"""

from __future__ import annotations

import hashlib
import json
from typing import Any


_BPS_DENOMINATOR = 10_000
_REQUIRED_FIELDS = {
    "case_id",
    "static_min_out",
    "reference_out",
    "expected_cost_bps",
    "max_deviation_bps",
    "hard_floor",
    "recipient_delta",
    "route_reported_out",
    "oracle_fresh",
    "deadline_ok",
}


def _canonical_bytes(value: dict[str, Any]) -> bytes:
    return json.dumps(value, sort_keys=True, separators=(",", ":")).encode("utf-8")


def _evidence_hash(evidence: dict[str, Any]) -> str:
    return hashlib.sha256(_canonical_bytes(evidence)).hexdigest()


def _compute_floor(reference_out: int, expected_cost_bps: int, max_deviation_bps: int, hard_floor: int) -> int:
    after_known_cost = reference_out * (_BPS_DENOMINATOR - expected_cost_bps) // _BPS_DENOMINATOR
    after_adverse_deviation = (
        after_known_cost
        * (_BPS_DENOMINATOR - max_deviation_bps)
        // _BPS_DENOMINATOR
    )
    return max(after_adverse_deviation, hard_floor)


def evaluate_case(case: dict[str, Any]) -> dict[str, Any]:
    missing = _REQUIRED_FIELDS - case.keys()
    if missing:
        raise ValueError(f"missing fields: {sorted(missing)}")

    for field in (
        "static_min_out",
        "reference_out",
        "expected_cost_bps",
        "max_deviation_bps",
        "hard_floor",
        "recipient_delta",
        "route_reported_out",
    ):
        if not isinstance(case[field], int) or isinstance(case[field], bool):
            raise ValueError(f"{field} must be an integer")
        if case[field] < 0:
            raise ValueError(f"{field} must be non-negative")

    if not isinstance(case["oracle_fresh"], bool) or not isinstance(case["deadline_ok"], bool):
        raise ValueError("oracle_fresh and deadline_ok must be boolean")

    failure_codes: list[str] = []
    if case["expected_cost_bps"] > _BPS_DENOMINATOR:
        failure_codes.append("INVALID_EXPECTED_COST")
    if case["max_deviation_bps"] > _BPS_DENOMINATOR:
        failure_codes.append("INVALID_MAX_DEVIATION")
    if not case["deadline_ok"]:
        failure_codes.append("DEADLINE_EXPIRED")
    if not case["oracle_fresh"]:
        failure_codes.append("STALE_REFERENCE")

    live_floor = _compute_floor(
        case["reference_out"],
        min(case["expected_cost_bps"], _BPS_DENOMINATOR),
        min(case["max_deviation_bps"], _BPS_DENOMINATOR),
        case["hard_floor"],
    )
    static_accepted = case["recipient_delta"] >= case["static_min_out"]
    if case["recipient_delta"] < live_floor:
        failure_codes.append("RECIPIENT_OUTPUT_BELOW_LIVE_FLOOR")

    verdict = "PASS" if not failure_codes else "REJECT"
    evidence = {
        "case_id": case["case_id"],
        "static_min_out": case["static_min_out"],
        "reference_out": case["reference_out"],
        "expected_cost_bps": case["expected_cost_bps"],
        "max_deviation_bps": case["max_deviation_bps"],
        "hard_floor": case["hard_floor"],
        "recipient_delta": case["recipient_delta"],
        "route_reported_out": case["route_reported_out"],
        "oracle_fresh": case["oracle_fresh"],
        "deadline_ok": case["deadline_ok"],
        "live_floor": live_floor,
        "failure_codes": failure_codes,
        "verdict": verdict,
    }
    evidence["evidence_sha256"] = _evidence_hash(evidence)

    return {
        "case_id": case["case_id"],
        "static_accepted": static_accepted,
        "recipient_delta": case["recipient_delta"],
        "route_reported_out": case["route_reported_out"],
        "live_floor": live_floor,
        "failure_codes": failure_codes,
        "verdict": verdict,
        "evidence": evidence,
    }


def replay_evidence(evidence: dict[str, Any]) -> str:
    supplied_hash = evidence.get("evidence_sha256")
    if not isinstance(supplied_hash, str):
        raise ValueError("missing evidence hash")

    unsigned = {key: value for key, value in evidence.items() if key != "evidence_sha256"}
    if _evidence_hash(unsigned) != supplied_hash:
        raise ValueError("evidence hash mismatch")

    replayed = evaluate_case(unsigned)
    for field in ("live_floor", "failure_codes", "verdict"):
        if replayed["evidence"][field] != evidence[field]:
            raise ValueError(f"replayed {field} differs")
    return replayed["verdict"]
