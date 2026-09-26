import asyncio
import json
import os
import tempfile
import unittest
from pathlib import Path
from types import SimpleNamespace

from memorylineage_langgraph.checkpointer import (
    MemoryLineageCheckpointer,
    RecoveryGateError,
    RecoveryHeld,
    generate_blinded_demo_evidence,
)


def sample_tuple(sequence):
    checkpoint_id = f"synthetic-{sequence}"
    return SimpleNamespace(
        config={"configurable": {"thread_id": "demo-thread", "checkpoint_id": checkpoint_id}},
        checkpoint={
            "v": 2,
            "id": checkpoint_id,
            "ts": f"2026-09-27T00:00:0{sequence}Z",
            "channel_values": {
                "memorylineage_sequence": sequence,
                "notes": [f"synthetic step {sequence}"],
            },
            "channel_versions": {"memorylineage_sequence": str(sequence)},
            "versions_seen": {"worker": {"memorylineage_sequence": str(sequence - 1)}},
            "updated_channels": ["notes"],
            "pending_sends": [],
        },
        metadata={"source": "loop", "step": sequence, "parents": {}},
        parent_config=None if sequence == 1 else {"configurable": {"checkpoint_id": f"synthetic-{sequence - 1}"}},
        pending_writes=[],
    )


class FakeSaver:
    def __init__(self, current):
        self.current = current

    def get_tuple(self, config):
        if config and config.get("which") == "old":
            return sample_tuple(1)
        return self.current

    def list(self, config, *, filter=None, before=None, limit=None):
        yield self.current
        yield sample_tuple(1)

    async def aget_tuple(self, config):
        return self.get_tuple(config)

    async def alist(self, config, *, filter=None, before=None, limit=None):
        yield self.current
        yield sample_tuple(1)


@unittest.skipUnless(os.environ.get("ML_CLI_PATH"), "requires the built Rust ml-cli")
class CheckpointerGateTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory(prefix="memorylineage-langgraph-")
        self.addCleanup(self.temp.cleanup)
        self.cli_path = Path(os.environ["ML_CLI_PATH"])
        self.secret = bytes(range(1, 33))
        self.evidence_path = Path(self.temp.name) / "evidence.json"
        evidence = generate_blinded_demo_evidence(
            [sample_tuple(1), sample_tuple(2), sample_tuple(3)], self.secret, self.cli_path
        )
        self.evidence_path.write_text(json.dumps(evidence), encoding="utf-8")

    def make_gate(self, saver, *, secret=None, evidence_path=None, receipts=None):
        actual_secret = self.secret if secret is None else secret
        return MemoryLineageCheckpointer(
            saver,
            cli_path=self.cli_path,
            evidence_path=evidence_path or self.evidence_path,
            secret_provider=lambda: actual_secret,
            on_receipt=receipts.append if receipts is not None else None,
        )

    def test_current_head_is_returned_only_after_a_verified_receipt(self):
        receipts = []
        current = sample_tuple(3)
        gate = self.make_gate(FakeSaver(current), receipts=receipts)

        result = gate.get_tuple({"configurable": {"thread_id": "demo-thread"}})

        self.assertIs(result, current)
        self.assertEqual(len(receipts), 1)
        self.assertEqual(receipts[0]["schemaVersion"], "memorylineage-recovery-receipt-v3")
        self.assertEqual(receipts[0]["decision"]["recommendedAction"], "RESUME_ALLOWED")

    def test_historical_checkpoint_is_held_for_sync_async_and_list_paths(self):
        gate = self.make_gate(FakeSaver(sample_tuple(3)))

        with self.assertRaises(RecoveryHeld) as sync_hold:
            gate.get_tuple({"which": "old"})
        self.assertEqual(
            sync_hold.exception.receipt["decision"]["recommendedAction"], "REHEARSE_ONLY"
        )

        async def check_async_paths():
            with self.assertRaises(RecoveryHeld):
                await gate.aget_tuple({"which": "old"})
            with self.assertRaises(RecoveryHeld):
                async for _ in gate.alist({"configurable": {"thread_id": "demo-thread"}}):
                    pass

        asyncio.run(asyncio.wait_for(check_async_paths(), timeout=5))

        with self.assertRaises(RecoveryHeld):
            list(gate.list({"configurable": {"thread_id": "demo-thread"}}))

    def test_wrong_secret_or_unavailable_evidence_fails_closed(self):
        wrong_secret = bytes([0x91]) * 32
        wrong_gate = self.make_gate(FakeSaver(sample_tuple(3)), secret=wrong_secret)
        with self.assertRaises(RecoveryHeld) as held:
            wrong_gate.get_tuple({})
        self.assertEqual(
            held.exception.receipt["decision"]["recommendedAction"], "HOLD_FOR_REVIEW"
        )

        missing_evidence = Path(self.temp.name) / "missing.json"
        missing_gate = self.make_gate(
            FakeSaver(sample_tuple(3)), evidence_path=missing_evidence
        )
        with self.assertRaises(RecoveryGateError):
            missing_gate.get_tuple({})

    def test_async_calls_require_async_saver_methods(self):
        class SyncOnlySaver:
            def get_tuple(self, config):
                return sample_tuple(3)

        gate = self.make_gate(SyncOnlySaver())

        async def check_missing_async_methods():
            with self.assertRaisesRegex(RecoveryGateError, "no async get_tuple"):
                await gate.aget_tuple({})
            with self.assertRaisesRegex(RecoveryGateError, "no async list"):
                async for _ in gate.alist({}):
                    pass

        asyncio.run(asyncio.wait_for(check_missing_async_methods(), timeout=1))


if __name__ == "__main__":
    unittest.main()
