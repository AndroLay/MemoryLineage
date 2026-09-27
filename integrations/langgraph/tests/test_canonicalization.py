import json
import math
import unittest
from types import SimpleNamespace

from memorylineage_langgraph.canonicalization import (
    CanonicalizationError,
    canonicalize_checkpoint_tuple,
    make_recovery_input,
)


def sample_tuple():
    return SimpleNamespace(
        config={"configurable": {"thread_id": "synthetic-thread", "checkpoint_id": "cp-3"}},
        checkpoint={
            "v": 2,
            "id": "cp-3",
            "ts": "2026-09-27T00:00:00Z",
            "channel_values": {"memorylineage_sequence": 3, "notes": ["sample"]},
            "channel_versions": {"memorylineage_sequence": "3"},
            "versions_seen": {"resume": {"memorylineage_sequence": "2"}},
            "updated_channels": ["notes"],
            "pending_sends": [],
        },
        metadata={"source": "loop", "step": 3, "parents": {}},
        parent_config=None,
        pending_writes=[],
    )


class CanonicalCheckpointTests(unittest.TestCase):
    def test_canonical_form_binds_the_full_checkpoint_tuple(self):
        snapshot = canonicalize_checkpoint_tuple(sample_tuple())

        self.assertEqual(snapshot.sequence, 3)
        encoded = snapshot.values["langgraph_checkpoint_tuple_v2"]
        decoded = json.loads(encoded)
        self.assertEqual(decoded["profile"], "memorylineage/langgraph-checkpoint/v2")
        self.assertEqual(
            set(decoded["tuple"]),
            {"config", "checkpoint", "metadata", "parent_config", "pending_writes"},
        )
        self.assertEqual(decoded["tuple"]["checkpoint"]["channel_values"]["notes"], ["sample"])
        self.assertEqual(decoded["tuple"]["pending_writes"], [])
        self.assertEqual(
            encoded,
            json.dumps(decoded, ensure_ascii=False, allow_nan=False, sort_keys=True, separators=(",", ":")),
        )

    def test_accepts_pinned_langgraph_checkpoint_format_v4(self):
        checkpoint_tuple = sample_tuple()
        checkpoint_tuple.checkpoint = {**checkpoint_tuple.checkpoint, "v": 4}

        snapshot = canonicalize_checkpoint_tuple(checkpoint_tuple)

        decoded = json.loads(snapshot.values["langgraph_checkpoint_tuple_v2"])
        self.assertEqual(snapshot.sequence, 3)
        self.assertEqual(decoded["tuple"]["checkpoint"]["v"], 4)

    def test_excludes_ephemeral_runtime_from_persistent_config_projection(self):
        checkpoint_tuple = sample_tuple()
        checkpoint_tuple.config = {
            **checkpoint_tuple.config,
            "callbacks": object(),
            "metadata": object(),
            "recursion_limit": 25,
            "tags": ["synthetic-trace"],
            "configurable": {
                **checkpoint_tuple.config["configurable"],
                "__pregel_runtime": object(),
            },
        }

        snapshot = canonicalize_checkpoint_tuple(checkpoint_tuple)

        decoded = json.loads(snapshot.values["langgraph_checkpoint_tuple_v2"])
        self.assertEqual(
            decoded["tuple"]["config"],
            {"configurable": {"thread_id": "synthetic-thread", "checkpoint_id": "cp-3"}},
        )

    def test_recovery_input_keeps_secret_out_of_snapshot_values(self):
        secret = bytes(range(1, 33))

        payload = make_recovery_input(sample_tuple(), secret)

        self.assertEqual(payload["blindingSecret"], "0x" + secret.hex())
        self.assertEqual(payload["snapshot"]["sequence"], 3)
        self.assertNotIn(payload["blindingSecret"], json.dumps(payload["snapshot"]))

    def test_rejects_unsupported_checkpoint_version_and_extra_fields(self):
        unreviewed_version = sample_tuple()
        unreviewed_version.checkpoint = {**unreviewed_version.checkpoint, "v": 3}
        with self.assertRaises(CanonicalizationError):
            canonicalize_checkpoint_tuple(unreviewed_version)

        unsupported = sample_tuple()
        unsupported.checkpoint = {**unsupported.checkpoint, "v": 99}
        with self.assertRaises(CanonicalizationError):
            canonicalize_checkpoint_tuple(unsupported)

        extra = sample_tuple()
        extra.checkpoint = {**extra.checkpoint, "future_restore_field": "unreviewed"}
        with self.assertRaises(CanonicalizationError):
            canonicalize_checkpoint_tuple(extra)

    def test_rejects_missing_sequence_non_json_values_and_non_finite_numbers(self):
        missing_sequence = sample_tuple()
        missing_sequence.checkpoint = {
            **missing_sequence.checkpoint,
            "channel_values": {"notes": ["sample"]},
        }
        with self.assertRaises(CanonicalizationError):
            canonicalize_checkpoint_tuple(missing_sequence)

        custom_value = sample_tuple()
        custom_value.checkpoint = {
            **custom_value.checkpoint,
            "channel_values": {"memorylineage_sequence": 3, "opaque": object()},
        }
        with self.assertRaises(CanonicalizationError):
            canonicalize_checkpoint_tuple(custom_value)

        non_finite = sample_tuple()
        non_finite.metadata = {"source": "loop", "metric": math.inf}
        with self.assertRaises(CanonicalizationError):
            canonicalize_checkpoint_tuple(non_finite)

    def test_rejects_bool_as_sequence_and_oversized_checkpoint_tree(self):
        boolean_sequence = sample_tuple()
        boolean_sequence.checkpoint = {
            **boolean_sequence.checkpoint,
            "channel_values": {"memorylineage_sequence": True},
        }
        with self.assertRaises(CanonicalizationError):
            canonicalize_checkpoint_tuple(boolean_sequence)

        deeply_nested = sample_tuple()
        nested = []
        cursor = nested
        for _ in range(70):
            child = []
            cursor.append(child)
            cursor = child
        deeply_nested.checkpoint = {
            **deeply_nested.checkpoint,
            "channel_values": {"memorylineage_sequence": 3, "nested": nested},
        }
        with self.assertRaises(CanonicalizationError):
            canonicalize_checkpoint_tuple(deeply_nested)


if __name__ == "__main__":
    unittest.main()
