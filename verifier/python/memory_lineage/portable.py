"""Verify the portable evidence bundle exported by MemoryLineage Inspector."""

from __future__ import annotations

import json
import sys
from pathlib import Path
from typing import Any


SCHEMA_VERSION = "memorylineage-evidence-v1"


def verify_portable_bundle(bundle: dict[str, Any]) -> dict[str, Any]:
    if bundle.get("schemaVersion") != SCHEMA_VERSION:
        raise AssertionError("SCHEMA_VERSION_MISMATCH")
    if bundle.get("evidenceType") != "memorylineage_inspector_export":
        raise AssertionError("EVIDENCE_TYPE_MISMATCH")
    if bundle.get("privacy", {}).get("rawMemoryOnChain") is not False:
        raise AssertionError("PRIVACY_BOUNDARY_VIOLATION")

    transitions = bundle.get("transitions")
    if not isinstance(transitions, list) or not transitions:
        raise AssertionError("EMPTY_TRANSITION_HISTORY")

    public_forbidden = {"payload", "provenance", "locator", "rawMemory", "memoryText"}
    serialized = json.dumps(bundle, sort_keys=True)
    for field in public_forbidden:
        if f'"{field}"' in serialized:
            raise AssertionError(f"PRIVATE_FIELD_EXPORTED:{field}")

    expected_sequence = 1
    previous_root = "0x" + "00" * 32
    for transition in transitions:
        if transition.get("sequence") != expected_sequence:
            raise AssertionError("BAD_SEQUENCE")
        if transition.get("prevStateRoot") != previous_root:
            raise AssertionError("BAD_PREVIOUS_STATE")
        previous_root = transition.get("nextStateRoot")
        if not isinstance(previous_root, str):
            raise AssertionError("MISSING_NEXT_ROOT")
        expected_sequence += 1

    head = bundle.get("head", {})
    last = transitions[-1]
    if head.get("sequence") != last.get("sequence"):
        raise AssertionError("HEAD_SEQUENCE_MISMATCH")
    if head.get("transitionId") != last.get("transitionId"):
        raise AssertionError("HEAD_TRANSITION_MISMATCH")
    if head.get("stateRoot") != last.get("nextStateRoot"):
        raise AssertionError("HEAD_ROOT_MISMATCH")

    return {
        "verdict": "VERIFIED",
        "checks": {
            "schema": "PASS",
            "sequenceContinuity": "PASS",
            "predecessorContinuity": "PASS",
            "headMatch": "PASS",
            "privacyBoundary": "PASS",
        },
        "sequence": last["sequence"],
        "finalRoot": last["nextStateRoot"],
        "transitionCount": len(transitions),
    }


def verify_file(path: str | Path) -> dict[str, Any]:
    return verify_portable_bundle(json.loads(Path(path).read_text()))


def main(argv: list[str] | None = None) -> int:
    args = argv or sys.argv[1:]
    if len(args) != 1:
        print("usage: python -m memory_lineage.portable <evidence.json>", file=sys.stderr)
        return 2
    try:
        result = verify_file(args[0])
    except (AssertionError, json.JSONDecodeError, OSError) as error:
        print(json.dumps({"verdict": "REJECTED", "reason": str(error)}))
        return 1
    print(json.dumps(result, indent=2, sort_keys=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
