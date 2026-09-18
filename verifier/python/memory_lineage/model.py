"""Throwaway ERC-8350 memory-lineage feasibility probe.

The probe models the part that makes the proposal interesting for a hackathon
demo: a private memory witness is represented by commitments, while a public
linear state machine rejects rollback, gaps, branches, unauthorized writers,
and substituted locators.  It is deliberately not a Solidity implementation
and it does not verify ECDSA/ERC-1271 signatures or a live chain.
"""

from __future__ import annotations

import hashlib
import json
from dataclasses import dataclass
from typing import Any, Iterable


_MASK_64 = (1 << 64) - 1
_ROUND_CONSTANTS = [
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
_ROTATION_OFFSETS = [
    [0, 36, 3, 41, 18],
    [1, 44, 10, 45, 2],
    [62, 6, 43, 15, 61],
    [28, 55, 25, 21, 56],
    [27, 20, 39, 8, 14],
]

ZERO_ROOT = "0x" + "00" * 32
EXPERIENCE_DELTA_TYPE_STRING = (
    "ExperienceDelta(bytes32 spaceId,uint64 sequence,bytes32 prevStateRoot,"
    "bytes32 deltaCommitment,bytes32 provenanceCommitment,bytes32 profileId,"
    "bytes32 locatorCommitment)"
)
MEMORY_STATE_TYPE_STRING = "MemoryState(bytes32 prevStateRoot,bytes32 transitionId)"
MEMORY_SPACE_TYPE_STRING = "MemorySpace(address initialController,bytes32 salt)"
EXPERIENCE_DELTA_TYPEHASH = bytes.fromhex(
    "4f020f86bc06d852f1fde17853b4d92a70214eeab8e09718028124af097d070d"
)
MEMORY_STATE_TYPEHASH = bytes.fromhex(
    "f3148762556cbf851baf4b9a205e18ff4e6b366a58a3a1ef58e8626ba41beadb"
)
MEMORY_SPACE_TYPEHASH = bytes.fromhex(
    "9ae5478f084ad3b841da58a9cb2354d153cddec59ee64d0cb741fa9d08884531"
)


def _rotate_left(value: int, offset: int) -> int:
    if offset == 0:
        return value & _MASK_64
    return ((value << offset) | (value >> (64 - offset))) & _MASK_64


def _keccak_f(state: list[int]) -> None:
    for round_constant in _ROUND_CONSTANTS:
        column_parity = [
            state[x]
            ^ state[x + 5]
            ^ state[x + 10]
            ^ state[x + 15]
            ^ state[x + 20]
            for x in range(5)
        ]
        correction = [
            column_parity[(x - 1) % 5]
            ^ _rotate_left(column_parity[(x + 1) % 5], 1)
            for x in range(5)
        ]
        for x in range(5):
            for y in range(5):
                state[x + 5 * y] ^= correction[x]

        permuted = [0] * 25
        for x in range(5):
            for y in range(5):
                destination = y + 5 * ((2 * x + 3 * y) % 5)
                permuted[destination] = _rotate_left(
                    state[x + 5 * y], _ROTATION_OFFSETS[x][y]
                )
        for x in range(5):
            for y in range(5):
                state[x + 5 * y] = permuted[x + 5 * y] ^ (
                    (~permuted[(x + 1) % 5 + 5 * y])
                    & permuted[(x + 2) % 5 + 5 * y]
                )
        state[0] ^= round_constant


def keccak256(data: bytes) -> bytes:
    """Small dependency-free Keccak-256 implementation for the probe."""

    rate = 136
    padded = bytearray(data)
    padded.append(0x01)  # Keccak domain separation, not SHA3's 0x06.
    while len(padded) % rate != rate - 1:
        padded.append(0)
    padded.append(0x80)

    state = [0] * 25
    for offset in range(0, len(padded), rate):
        block = padded[offset : offset + rate]
        for word_index in range(rate // 8):
            state[word_index] ^= int.from_bytes(
                block[word_index * 8 : word_index * 8 + 8], "little"
            )
        _keccak_f(state)

    return b"".join(word.to_bytes(8, "little") for word in state)[:32]


def _hex(value: bytes) -> str:
    return "0x" + value.hex()


def _bytes32(value: str | bytes) -> bytes:
    if isinstance(value, bytes):
        if len(value) != 32:
            raise ValueError("bytes32 value must contain 32 bytes")
        return value
    if not isinstance(value, str) or not value.startswith("0x"):
        raise ValueError("bytes32 value must be a 0x-prefixed string")
    decoded = bytes.fromhex(value[2:])
    if len(decoded) != 32:
        raise ValueError("bytes32 value must contain 32 bytes")
    return decoded


def _uint_word(value: int) -> bytes:
    if not isinstance(value, int) or isinstance(value, bool) or value < 0:
        raise ValueError("uint value must be a non-negative integer")
    return value.to_bytes(32, "big")


def _address_word(address: str) -> bytes:
    if not isinstance(address, str) or not address.startswith("0x"):
        raise ValueError("address must be a 0x-prefixed string")
    raw = bytes.fromhex(address[2:])
    if len(raw) != 20:
        raise ValueError("address must contain 20 bytes")
    return raw.rjust(32, b"\0")


def _canonical_bytes(value: Any) -> bytes:
    return json.dumps(value, sort_keys=True, separators=(",", ":")).encode("utf-8")


def commitment(value: str, salt: str = "") -> str:
    return _hex(keccak256((salt + value).encode("utf-8")))


def space_id(initial_controller: str, salt: str) -> str:
    return _hex(
        keccak256(
            MEMORY_SPACE_TYPEHASH
            + _address_word(initial_controller)
            + _bytes32(salt)
        )
    )


def transition_id(delta: dict[str, Any]) -> str:
    encoded = (
        EXPERIENCE_DELTA_TYPEHASH
        + _bytes32(delta["space_id"])
        + _uint_word(delta["sequence"])
        + _bytes32(delta["prev_state_root"])
        + _bytes32(delta["delta_commitment"])
        + _bytes32(delta["provenance_commitment"])
        + _bytes32(delta["profile_id"])
        + _bytes32(delta["locator_commitment"])
    )
    return _hex(keccak256(encoded))


def next_state_root(prev_state_root: str, transition: str) -> str:
    return _hex(
        keccak256(
            MEMORY_STATE_TYPEHASH
            + _bytes32(prev_state_root)
            + _bytes32(transition)
        )
    )


@dataclass
class Transition:
    delta: dict[str, Any]
    transition_id: str
    next_state_root: str
    authorizer: str
    payload: str
    provenance: str
    locator: str
    delta_salt: str
    provenance_salt: str
    locator_salt: str

    def as_dict(self) -> dict[str, Any]:
        return {
            "delta": self.delta,
            "transition_id": self.transition_id,
            "next_state_root": self.next_state_root,
            "authorizer": self.authorizer,
            "payload": self.payload,
            "provenance": self.provenance,
            "locator": self.locator,
            "delta_salt": self.delta_salt,
            "provenance_salt": self.provenance_salt,
            "locator_salt": self.locator_salt,
        }


def make_transition(
    *,
    space: str,
    sequence: int,
    previous_root: str,
    authorizer: str,
    payload: str,
    provenance: str,
    locator: str,
    profile_id: str,
    delta_salt: str,
    provenance_salt: str,
    locator_salt: str,
) -> Transition:
    delta = {
        "space_id": space,
        "sequence": sequence,
        "prev_state_root": previous_root,
        "delta_commitment": commitment(payload, delta_salt),
        "provenance_commitment": commitment(provenance, provenance_salt),
        "profile_id": profile_id,
        "locator_commitment": commitment(locator, locator_salt),
    }
    identifier = transition_id(delta)
    return Transition(
        delta=delta,
        transition_id=identifier,
        next_state_root=next_state_root(previous_root, identifier),
        authorizer=authorizer,
        payload=payload,
        provenance=provenance,
        locator=locator,
        delta_salt=delta_salt,
        provenance_salt=provenance_salt,
        locator_salt=locator_salt,
    )


def audit_history(
    transitions: Iterable[Transition], *, expected_authorizer: str
) -> dict[str, Any]:
    current_root = ZERO_ROOT
    expected_sequence = 1
    failures: list[str] = []
    seen_sequences: set[int] = set()
    seen_transition_ids: set[str] = set()
    accepted = 0

    for transition in transitions:
        delta = transition.delta
        sequence = delta["sequence"]
        if sequence in seen_sequences:
            failures.append("PARALLEL_HISTORY")
        seen_sequences.add(sequence)

        if transition.authorizer != expected_authorizer:
            failures.append("UNAUTHORIZED_AUTHORIZER")
        if sequence != expected_sequence:
            failures.append("SEQUENCE_GAP_OR_REORDER")
        if delta["prev_state_root"] != current_root:
            failures.append("PREDECESSOR_MISMATCH")

        if delta["delta_commitment"] != commitment(
            transition.payload, transition.delta_salt
        ):
            failures.append("DELTA_COMMITMENT_MISMATCH")
        if delta["provenance_commitment"] != commitment(
            transition.provenance, transition.provenance_salt
        ):
            failures.append("PROVENANCE_COMMITMENT_MISMATCH")
        if delta["locator_commitment"] != commitment(
            transition.locator, transition.locator_salt
        ):
            failures.append("LOCATOR_COMMITMENT_MISMATCH")

        recomputed_id = transition_id(delta)
        if recomputed_id != transition.transition_id:
            failures.append("TRANSITION_ID_MISMATCH")
        if transition.transition_id in seen_transition_ids:
            failures.append("DUPLICATE_TRANSITION")
        seen_transition_ids.add(transition.transition_id)

        recomputed_root = next_state_root(delta["prev_state_root"], recomputed_id)
        if recomputed_root != transition.next_state_root:
            failures.append("NEXT_ROOT_MISMATCH")

        if not failures:
            accepted += 1
            current_root = recomputed_root
            expected_sequence += 1
        else:
            # Continue scanning to surface every independent mutation, while
            # retaining the last accepted root as the canonical state.
            current_root = recomputed_root
            expected_sequence = max(expected_sequence, sequence + 1)

    unique_failures = sorted(set(failures))
    return {
        "accepted_transitions": accepted,
        "final_root": current_root,
        "failure_codes": unique_failures,
        "verdict": "PASS" if not unique_failures else "REJECT",
    }


def replay_evidence(evidence: dict[str, Any]) -> dict[str, Any]:
    """Recompute the public decision from serialized transitions only."""

    supplied_digest = evidence.get("history_sha256")
    serialized = evidence.get("transitions")
    if not isinstance(supplied_digest, str) or not isinstance(serialized, list):
        raise ValueError("evidence must contain history_sha256 and transitions")
    unsigned = {key: value for key, value in evidence.items() if key != "history_sha256"}
    if hashlib.sha256(_canonical_bytes(unsigned)).hexdigest() != supplied_digest:
        raise ValueError("history evidence hash mismatch")

    transitions = [Transition(**item) for item in serialized]
    result = audit_history(
        transitions, expected_authorizer=evidence["expected_authorizer"]
    )
    if result["verdict"] != evidence["verdict"]:
        raise ValueError("replayed verdict differs")
    if result["failure_codes"] != evidence["failure_codes"]:
        raise ValueError("replayed failure codes differ")
    return result


PUBLISHED_VECTOR = {
    "initial_controller": "0x2222222222222222222222222222222222222222",
    "space_salt": "0x" + "dd" * 32,
    "profile_id": "0x2f53a8c7cdfafe7559a285db700638113866e8605b81c9454f0936d67ddbd759",
    "delta_commitment": "0x2cf1dc108cc32cfe1617db756a7c793f383b67165ea1bb542f456dc6100f967d",
    "provenance_commitment": "0x4791ee5d37e8fc775d00f82c3d451e6d313aa5fe16dd94de5efdbbb39dc0f05c",
    "locator_commitment": "0xc5ff571b8429eecd969f718fc45e45017c0d3a217dff401ad1cc752686bac812",
    "transition_id": "0xdd00dd6eb3aec704b5455502647a0caacf23be6c724eda4a60d9645291e7f4e5",
    "next_state_root": "0x9684a8d3571c5cd9c1e3abb1b0c0797b9fef6965e9002aeefba91e8cb1163754",
}


def verify_published_vector() -> dict[str, bool]:
    space = space_id(PUBLISHED_VECTOR["initial_controller"], PUBLISHED_VECTOR["space_salt"])
    delta = {
        "space_id": space,
        "sequence": 1,
        "prev_state_root": ZERO_ROOT,
        "delta_commitment": PUBLISHED_VECTOR["delta_commitment"],
        "provenance_commitment": PUBLISHED_VECTOR["provenance_commitment"],
        "profile_id": PUBLISHED_VECTOR["profile_id"],
        "locator_commitment": PUBLISHED_VECTOR["locator_commitment"],
    }
    computed_id = transition_id(delta)
    computed_root = next_state_root(ZERO_ROOT, computed_id)
    return {
        "space_id": space == "0xef5037465ae0323637cb58434eb554ae4a1fe1131bcc10888900f2d4cbe349d8",
        "experience_delta_typehash": keccak256(
            EXPERIENCE_DELTA_TYPE_STRING.encode("ascii")
        )
        == EXPERIENCE_DELTA_TYPEHASH,
        "memory_state_typehash": keccak256(MEMORY_STATE_TYPE_STRING.encode("ascii"))
        == MEMORY_STATE_TYPEHASH,
        "memory_space_typehash": keccak256(MEMORY_SPACE_TYPE_STRING.encode("ascii"))
        == MEMORY_SPACE_TYPEHASH,
        "transition_id": computed_id == PUBLISHED_VECTOR["transition_id"],
        "next_state_root": computed_root == PUBLISHED_VECTOR["next_state_root"],
    }


def evidence_for(
    case_id: str, transitions: list[Transition], expected_authorizer: str
) -> dict[str, Any]:
    result = audit_history(transitions, expected_authorizer=expected_authorizer)
    evidence = {
        "case_id": case_id,
        "expected_authorizer": expected_authorizer,
        "transitions": [transition.as_dict() for transition in transitions],
        "verdict": result["verdict"],
        "failure_codes": result["failure_codes"],
    }
    evidence["history_sha256"] = hashlib.sha256(
        _canonical_bytes(evidence)
    ).hexdigest()
    replay = replay_evidence(evidence)
    return {
        "case_id": case_id,
        "verdict": result["verdict"],
        "failure_codes": result["failure_codes"],
        "replay_verdict": replay["verdict"],
        "evidence": evidence,
    }
