"""Strict canonical projection for the pinned LangGraph checkpoint profile."""

from __future__ import annotations

import json
import math
from collections.abc import Mapping
from dataclasses import dataclass
from typing import Any

CHECKPOINT_PROFILE = "memorylineage/langgraph-checkpoint/v2"
SNAPSHOT_PROFILE = "memorylineage/private-snapshot/blinded-v1"
SOURCE_CLASS = "DEMO_SPACE_V2_LOCAL"
RECOVERY_INPUT_SCHEMA = "memorylineage-recovery-input-v1"
MAX_TREE_NODES = 100_000
MAX_TREE_DEPTH = 64
MAX_CANONICAL_BYTES = 8 * 1024 * 1024

_CHECKPOINT_REQUIRED = {
    "v",
    "id",
    "ts",
    "channel_values",
    "channel_versions",
    "versions_seen",
    "updated_channels",
}
_CHECKPOINT_OPTIONAL = {"pending_sends"}
_SUPPORTED_CHECKPOINT_VERSIONS = frozenset({1, 2, 4})
_EPHEMERAL_RUNTIME_CONFIG_KEY = "__pregel_runtime"
_TUPLE_FIELDS = ("config", "checkpoint", "metadata", "parent_config", "pending_writes")


class CanonicalizationError(ValueError):
    """The checkpoint is outside the explicitly supported profile."""


@dataclass(frozen=True)
class CanonicalSnapshot:
    sequence: int
    values: dict[str, str]


class _Budget:
    def __init__(self) -> None:
        self.nodes = 0

    def consume(self, depth: int) -> None:
        self.nodes += 1
        if depth > MAX_TREE_DEPTH:
            raise CanonicalizationError("checkpoint exceeds the supported nesting depth")
        if self.nodes > MAX_TREE_NODES:
            raise CanonicalizationError("checkpoint contains too many values")


def _plain_json(value: Any, budget: _Budget, depth: int = 0) -> Any:
    budget.consume(depth)
    if value is None or type(value) in (bool, int, str):
        return value
    if type(value) is float:
        if not math.isfinite(value):
            raise CanonicalizationError("checkpoint contains a non-finite number")
        return value
    if isinstance(value, Mapping):
        normalized: dict[str, Any] = {}
        for key, item in value.items():
            if type(key) is not str:
                raise CanonicalizationError("checkpoint object keys must be strings")
            normalized[key] = _plain_json(item, budget, depth + 1)
        return normalized
    if isinstance(value, (list, tuple)):
        return [_plain_json(item, budget, depth + 1) for item in value]
    raise CanonicalizationError("checkpoint contains a value outside the JSON profile")


def _required_attr(value: Any, name: str) -> Any:
    try:
        return getattr(value, name)
    except AttributeError as error:
        raise CanonicalizationError(f"checkpoint tuple is missing {name}") from error


def _persistent_config_projection(config: Any) -> Any:
    """Project config to the persisted saver identity, excluding run options."""

    if not isinstance(config, Mapping):
        raise CanonicalizationError("checkpoint config must be a mapping")
    configurable = config.get("configurable")
    if not isinstance(configurable, Mapping):
        raise CanonicalizationError("checkpoint config requires a configurable mapping")

    projected_configurable = dict(configurable)
    # LangGraph injects this execution object while invoking a graph. It is
    # neither persisted checkpoint state nor stable JSON configuration.
    projected_configurable.pop(_EPHEMERAL_RUNTIME_CONFIG_KEY, None)
    # Top-level callbacks, tracing metadata, tags, and recursion controls are
    # invocation options; the saver persists and returns only `configurable`.
    return {"configurable": projected_configurable}


def canonicalize_checkpoint_tuple(checkpoint_tuple: Any) -> CanonicalSnapshot:
    """Bind config, checkpoint, metadata, parent and pending writes together.

    LangGraph checkpoint values can contain arbitrary Python classes. This
    profile accepts JSON primitives and containers only, and rejects unknown
    checkpoint fields or serializer versions so a new runtime format cannot be
    silently omitted from the commitment.
    """

    budget = _Budget()
    tuple_data = {field: _required_attr(checkpoint_tuple, field) for field in _TUPLE_FIELDS}
    checkpoint = tuple_data["checkpoint"]
    if not isinstance(checkpoint, Mapping):
        raise CanonicalizationError("checkpoint must be a mapping")

    checkpoint_keys = set(checkpoint)
    if not _CHECKPOINT_REQUIRED <= checkpoint_keys:
        missing = sorted(_CHECKPOINT_REQUIRED - checkpoint_keys)
        raise CanonicalizationError(f"checkpoint is missing required fields: {', '.join(missing)}")
    unknown = checkpoint_keys - _CHECKPOINT_REQUIRED - _CHECKPOINT_OPTIONAL
    if unknown:
        raise CanonicalizationError("checkpoint contains unreviewed fields")

    version = checkpoint["v"]
    if type(version) is not int or version not in _SUPPORTED_CHECKPOINT_VERSIONS:
        raise CanonicalizationError("unsupported LangGraph checkpoint format version")
    channel_values = checkpoint["channel_values"]
    if not isinstance(channel_values, Mapping):
        raise CanonicalizationError("checkpoint channel_values must be a mapping")
    sequence = channel_values.get("memorylineage_sequence")
    if type(sequence) is not int or not 0 < sequence < 2**64:
        raise CanonicalizationError("checkpoint requires a positive integer memorylineage_sequence")

    # LangGraph treats absent pending_sends as an empty list in checkpoint
    # copying. Canonicalize that documented default explicitly.
    normalized_checkpoint = dict(checkpoint)
    normalized_checkpoint.setdefault("pending_sends", [])
    normalized_tuple = {
        "config": _persistent_config_projection(tuple_data["config"]),
        "checkpoint": normalized_checkpoint,
        "metadata": tuple_data["metadata"],
        "parent_config": tuple_data["parent_config"],
        "pending_writes": tuple_data["pending_writes"],
    }
    normalized = _plain_json(
        {"profile": CHECKPOINT_PROFILE, "tuple": normalized_tuple}, budget
    )
    try:
        encoded = json.dumps(
            normalized,
            ensure_ascii=False,
            allow_nan=False,
            sort_keys=True,
            separators=(",", ":"),
        )
        encoded_bytes = encoded.encode("utf-8")
    except (TypeError, ValueError, UnicodeError) as error:
        raise CanonicalizationError("checkpoint cannot be encoded by the canonical profile") from error
    if len(encoded_bytes) > MAX_CANONICAL_BYTES:
        raise CanonicalizationError("checkpoint exceeds the supported canonical size")
    return CanonicalSnapshot(sequence=sequence, values={"langgraph_checkpoint_tuple_v2": encoded})


def make_recovery_input(checkpoint_tuple: Any, blinding_secret: bytes) -> dict[str, Any]:
    """Return the private stdin payload expected by `ml-cli recover preflight-json`."""

    if not isinstance(blinding_secret, bytes) or len(blinding_secret) != 32:
        raise CanonicalizationError("blinding secret must contain exactly 32 bytes")
    if not any(blinding_secret):
        raise CanonicalizationError("blinding secret must not be all zeroes")
    snapshot = canonicalize_checkpoint_tuple(checkpoint_tuple)
    return {
        "schemaVersion": RECOVERY_INPUT_SCHEMA,
        "snapshotProfile": SNAPSHOT_PROFILE,
        "sourceClass": SOURCE_CLASS,
        "blindingSecret": "0x" + blinding_secret.hex(),
        "snapshot": {"sequence": snapshot.sequence, "values": snapshot.values},
    }
