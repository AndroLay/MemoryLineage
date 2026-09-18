"""Independent replay verifier for the exported MemoryLineage EVM evidence.

This module intentionally does not import the Python feasibility probe or the
JavaScript publisher. It consumes only the JSON bundle and recomputes the
public commitment state machine with its own Keccak and ABI-word routines.
It cannot establish that private memory contents are semantically true.
"""

from __future__ import annotations

import json
import re
from pathlib import Path
from typing import Any


MASK_64 = (1 << 64) - 1
ROUND_CONSTANTS = [
    0x0000000000000001,
    0x0000000000008082,
    0x800000000000808A,
    0x8000000080008000,
    0x000000000000808B,
    0x0000000080000001,
    0x8000000080008081,
    0x8000000000008009,
    0x000000000000008A,
    0x0000000000000088,
    0x0000000080008009,
    0x000000008000000A,
    0x000000008000808B,
    0x800000000000008B,
    0x8000000000008089,
    0x8000000000008003,
    0x8000000000008002,
    0x8000000000000080,
    0x000000000000800A,
    0x800000008000000A,
    0x8000000080008081,
    0x8000000000008080,
    0x0000000080000001,
    0x8000000080008008,
]
ROTATION_OFFSETS = [
    [0, 36, 3, 41, 18],
    [1, 44, 10, 45, 2],
    [62, 6, 43, 15, 61],
    [28, 55, 25, 21, 56],
    [27, 20, 39, 8, 14],
]

ZERO_ROOT = "0x" + "00" * 32
EXPERIENCE_DELTA_TYPEHASH = bytes.fromhex(
    "4f020f86bc06d852f1fde17853b4d92a70214eeab8e09718028124af097d070d"
)
MEMORY_STATE_TYPEHASH = bytes.fromhex(
    "f3148762556cbf851baf4b9a205e18ff4e6b366a58a3a1ef58e8626ba41beadb"
)
MEMORY_SPACE_TYPEHASH = bytes.fromhex(
    "9ae5478f084ad3b841da58a9cb2354d153cddec59ee64d0cb741fa9d08884531"
)
AUTH_MUTATIONS = {
    "wrong_eoa_signer",
    "missing_signature",
    "truncated_signature",
    "signature_byte_tamper",
    "signature_invalid_v",
    "wrong_chain_domain",
    "wrong_contract_domain",
    "deltaCommitment_signature_binding",
    "provenanceCommitment_signature_binding",
    "locatorCommitment_signature_binding",
}
FORBIDDEN_PRIVATE_KEYS = {"payload", "provenance", "locator", "deltaSalt", "provenanceSalt", "locatorSalt"}
HEX32 = re.compile(r"^0x[0-9a-fA-F]{64}$")


def _rotl(value: int, offset: int) -> int:
    if offset == 0:
        return value & MASK_64
    return ((value << offset) | (value >> (64 - offset))) & MASK_64


def _keccak_round(state: list[int]) -> None:
    for constant in ROUND_CONSTANTS:
        parity = [
            state[x] ^ state[x + 5] ^ state[x + 10] ^ state[x + 15] ^ state[x + 20]
            for x in range(5)
        ]
        correction = [parity[(x - 1) % 5] ^ _rotl(parity[(x + 1) % 5], 1) for x in range(5)]
        for x in range(5):
            for y in range(5):
                state[x + 5 * y] ^= correction[x]

        permuted = [0] * 25
        for x in range(5):
            for y in range(5):
                destination = y + 5 * ((2 * x + 3 * y) % 5)
                permuted[destination] = _rotl(state[x + 5 * y], ROTATION_OFFSETS[x][y])
        for x in range(5):
            for y in range(5):
                state[x + 5 * y] = permuted[x + 5 * y] ^ (
                    (~permuted[(x + 1) % 5 + 5 * y]) & permuted[(x + 2) % 5 + 5 * y]
                )
        state[0] ^= constant


def independent_keccak(data: bytes) -> bytes:
    rate = 136
    padded = bytearray(data)
    padded.append(0x01)
    while len(padded) % rate != rate - 1:
        padded.append(0)
    padded.append(0x80)
    state = [0] * 25
    for offset in range(0, len(padded), rate):
        block = padded[offset : offset + rate]
        for index in range(rate // 8):
            state[index] ^= int.from_bytes(block[index * 8 : index * 8 + 8], "little")
        _keccak_round(state)
    return b"".join(word.to_bytes(8, "little") for word in state)[:32]


def hex_bytes(value: str) -> bytes:
    if not isinstance(value, str) or not value.startswith("0x"):
        raise ValueError(f"not a hex value: {value!r}")
    return bytes.fromhex(value[2:])


def bytes32(value: str) -> bytes:
    if not HEX32.match(value):
        raise ValueError(f"not bytes32: {value!r}")
    return hex_bytes(value)


def word(value: int) -> bytes:
    if not isinstance(value, int) or value < 0:
        raise ValueError(f"invalid uint: {value!r}")
    return value.to_bytes(32, "big")


def address_word(value: str) -> bytes:
    raw = hex_bytes(value)
    if len(raw) != 20:
        raise ValueError("address must contain 20 bytes")
    return raw.rjust(32, b"\0")


def abi_words(parts: list[bytes]) -> bytes:
    if any(len(part) != 32 for part in parts):
        raise ValueError("ABI word has the wrong width")
    return b"".join(parts)


def hex_value(value: bytes) -> str:
    return "0x" + value.hex()


def space_identifier(controller: str, salt: str) -> str:
    return hex_value(independent_keccak(abi_words([MEMORY_SPACE_TYPEHASH, address_word(controller), bytes32(salt)])))


def transition_identifier(transition: dict[str, Any]) -> str:
    return hex_value(
        independent_keccak(
            abi_words(
                [
                    EXPERIENCE_DELTA_TYPEHASH,
                    bytes32(transition["spaceId"]),
                    word(int(transition["sequence"])),
                    bytes32(transition["prevStateRoot"]),
                    bytes32(transition["deltaCommitment"]),
                    bytes32(transition["provenanceCommitment"]),
                    bytes32(transition["profileId"]),
                    bytes32(transition["locatorCommitment"]),
                ]
            )
        )
    )


def state_root(previous_root: str, transition_id: str) -> str:
    return hex_value(independent_keccak(abi_words([MEMORY_STATE_TYPEHASH, bytes32(previous_root), bytes32(transition_id)])))


def public_transition_valid(transition: dict[str, Any], *, expected_space: str | None = None, expected_profile: str | None = None, expected_sequence: int | None = None, expected_root: str | None = None) -> None:
    required = {
        "spaceId",
        "sequence",
        "prevStateRoot",
        "deltaCommitment",
        "provenanceCommitment",
        "profileId",
        "locatorCommitment",
        "transitionId",
        "nextStateRoot",
    }
    if set(transition) != required:
        raise AssertionError(f"public transition field set changed: {set(transition) ^ required}")
    if expected_space is not None and transition["spaceId"] != expected_space:
        raise AssertionError("space id changed")
    if expected_profile is not None and transition["profileId"] != expected_profile:
        raise AssertionError("profile id changed")
    sequence = int(transition["sequence"])
    if expected_sequence is not None and sequence != expected_sequence:
        raise AssertionError("sequence mismatch")
    if expected_root is not None and transition["prevStateRoot"] != expected_root:
        raise AssertionError("predecessor mismatch")
    for key in required - {"sequence"}:
        bytes32(transition[key])
    computed_id = transition_identifier(transition)
    if computed_id != transition["transitionId"]:
        raise AssertionError("transition id mismatch")
    computed_root = state_root(transition["prevStateRoot"], computed_id)
    if computed_root != transition["nextStateRoot"]:
        raise AssertionError("next root mismatch")


def structural_reason(transition: dict[str, Any], *, space: str, profile: str, sequence: int, root: str) -> str | None:
    if transition["spaceId"] != space:
        return "UNKNOWN_SPACE"
    if transition["deltaCommitment"] == ZERO_ROOT:
        return "ZERO_DELTA_COMMITMENT"
    if transition["profileId"] == ZERO_ROOT:
        return "ZERO_PROFILE_ID"
    if int(transition["sequence"]) != sequence + 1:
        return "BAD_SEQUENCE"
    if transition["prevStateRoot"] != root:
        return "BAD_PREVIOUS_STATE"
    if transition["profileId"] != profile:
        return "PROFILE_MISMATCH"
    if transition_identifier(transition) != transition["transitionId"]:
        return "TRANSITION_ID_MISMATCH"
    if state_root(transition["prevStateRoot"], transition["transitionId"]) != transition["nextStateRoot"]:
        return "NEXT_ROOT_MISMATCH"
    return None


def assert_no_private_payload(value: Any) -> None:
    if isinstance(value, dict):
        forbidden = FORBIDDEN_PRIVATE_KEYS.intersection(value)
        if forbidden:
            raise AssertionError(f"private payload leaked into evidence: {sorted(forbidden)}")
        for nested in value.values():
            assert_no_private_payload(nested)
    elif isinstance(value, list):
        for nested in value:
            assert_no_private_payload(nested)


def verify_bundle(bundle: dict[str, Any]) -> dict[str, Any]:
    assert_no_private_payload(bundle)
    conformance = bundle["conformance"]
    expected = conformance["expected"]
    inputs = conformance["inputs"]
    full = conformance["fullTransition"]
    if space_identifier(inputs["initialController"], inputs["spaceSalt"]) != expected["spaceId"]:
        raise AssertionError("published space id replay mismatch")
    if conformance["independentlyComputed"]["spaceId"] != expected["spaceId"]:
        raise AssertionError("publisher space id differs from expected")
    if transition_identifier(full) != expected["transitionId"]:
        raise AssertionError("published transition id replay mismatch")
    if state_root(full["prevStateRoot"], full["transitionId"]) != expected["nextStateRoot"]:
        raise AssertionError("published state root replay mismatch")
    if conformance["contract"]["transitionId"] != expected["transitionId"] or conformance["contract"]["nextStateRoot"] != expected["nextStateRoot"]:
        raise AssertionError("contract output differs from published vector")
    if not conformance["allMatch"]:
        raise AssertionError("conformance artifact is not marked complete")

    history = bundle["validHistory"]
    if len(history) < 4:
        raise AssertionError("valid history is too short")
    space = history[0]["spaceId"]
    profile = history[0]["profileId"]
    sequence = 0
    root = ZERO_ROOT
    for transition in history:
        public_transition_valid(transition, expected_space=space, expected_profile=profile, expected_sequence=sequence + 1, expected_root=root)
        sequence = int(transition["sequence"])
        root = transition["nextStateRoot"]
    if bundle.get("finalHead") is not None:
        if int(bundle["finalHead"]["sequence"]) != sequence or bundle["finalHead"]["root"] != root:
            raise AssertionError("final head mismatch")

    rejected = 0
    mutation_results = []
    mutation_sequence = 1
    mutation_root = history[0]["nextStateRoot"]
    for entry in bundle["mutationMatrix"]:
        name = entry["name"]
        transition = entry["transition"]
        observed = entry["observed"]
        if name == "branch_setup":
            if structural_reason(transition, space=space, profile=profile, sequence=mutation_sequence, root=mutation_root) is not None or observed["status"] != "PASS":
                raise AssertionError("branch setup did not establish a valid competing head")
            mutation_sequence = int(transition["sequence"])
            mutation_root = transition["nextStateRoot"]
            mutation_results.append({"name": name, "independent": "PASS"})
            continue

        reason = structural_reason(transition, space=space, profile=profile, sequence=mutation_sequence, root=mutation_root)
        if observed["status"] != "REJECT":
            raise AssertionError(f"mutation passed unexpectedly: {name}")
        rejected += 1
        if reason is None:
            if name not in AUTH_MUTATIONS:
                raise AssertionError(f"unclassified structurally valid mutation: {name}")
            independent = "AUTH_REQUIRED"
        else:
            independent = reason
            if reason not in observed["reason"]:
                raise AssertionError(f"independent reason differs for {name}: {reason} vs {observed['reason']}")
        mutation_results.append({"name": name, "independent": independent, "observed": observed["status"]})

    if rejected != 20:
        raise AssertionError(f"expected 20 rejected mutations, got {rejected}")
    return {
        "verdict": "PASS",
        "validTransitions": len(history),
        "finalSequence": sequence,
        "finalRoot": root,
        "replayMismatches": 0,
        "mutationRejected": rejected,
        "mutationResults": mutation_results,
    }


def verify_file(path: str | Path) -> dict[str, Any]:
    bundle = json.loads(Path(path).read_text())
    return verify_bundle(bundle)


if __name__ == "__main__":
    evidence_path = Path(__file__).resolve().parents[3] / "evidence" / "local" / "memory_lineage_evm_evidence.json"
    result = verify_file(evidence_path)
    print(json.dumps(result, indent=2, sort_keys=True))
