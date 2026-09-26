"""Exercise the guard through LangGraph and its SQLite checkpoint saver."""

from __future__ import annotations

import json
import os
import asyncio
import sqlite3
import tempfile
import unittest
from pathlib import Path
from typing import TypedDict

os.environ.setdefault("LANGGRAPH_STRICT_MSGPACK", "true")

try:
    from langgraph.checkpoint.base import BaseCheckpointSaver
    from langgraph.checkpoint.sqlite import SqliteSaver
    from langgraph.checkpoint.sqlite.aio import AsyncSqliteSaver
    from langgraph.graph import END, START, StateGraph

    LANGGRAPH_AVAILABLE = True
except ModuleNotFoundError:
    LANGGRAPH_AVAILABLE = False

from memorylineage_langgraph.checkpointer import (
    MemoryLineageCheckpointer,
    RecoveryHeld,
    generate_blinded_demo_evidence,
)


class AgentState(TypedDict, total=False):
    memorylineage_sequence: int
    notes: list[str]
    request: str


@unittest.skipUnless(
    LANGGRAPH_AVAILABLE and os.environ.get("ML_CLI_PATH"),
    "requires pinned LangGraph packages and the built Rust ml-cli",
)
class LangGraphResumeBoundaryTests(unittest.TestCase):
    def test_sqlite_checkpoint_is_gated_before_graph_receives_it(self):
        with tempfile.TemporaryDirectory(prefix="memorylineage-real-langgraph-") as temp:
            root = Path(temp)
            connection = sqlite3.connect(root / "agent-checkpoints.sqlite", check_same_thread=False)
            saver = SqliteSaver(connection)
            saver.setup()
            guarded_node_runs: list[int] = []
            guard_active = [False]

            def agent_step(state: AgentState) -> AgentState:
                sequence = state.get("memorylineage_sequence", 0) + 1
                if guard_active[0]:
                    guarded_node_runs.append(sequence)
                return {
                    "memorylineage_sequence": sequence,
                    "notes": [*state.get("notes", []), state.get("request", "")],
                }

            builder = StateGraph(AgentState)
            builder.add_node("agent_step", agent_step)
            builder.add_edge(START, "agent_step")
            builder.add_edge("agent_step", END)
            thread_config = {"configurable": {"thread_id": "synthetic-recovery-thread"}}
            source_graph = builder.compile(checkpointer=saver)
            for sequence in range(1, 4):
                source_graph.invoke({"request": f"synthetic turn {sequence}"}, thread_config)

            tuples_by_sequence = {}
            for checkpoint_tuple in saver.list(thread_config):
                checkpoint_sequence = checkpoint_tuple.checkpoint["channel_values"].get(
                    "memorylineage_sequence"
                )
                if checkpoint_sequence in (1, 2, 3):
                    tuples_by_sequence.setdefault(checkpoint_sequence, checkpoint_tuple)
            self.assertEqual(sorted(tuples_by_sequence), [1, 2, 3])

            # Reopen the persisted checkpoint database before guarded resume.
            connection.close()
            connection = sqlite3.connect(
                root / "agent-checkpoints.sqlite", check_same_thread=False
            )
            self.addCleanup(connection.close)
            saver = SqliteSaver(connection)
            saver.setup()

            secret = bytes(range(1, 33))
            evidence = generate_blinded_demo_evidence(
                [tuples_by_sequence[1], tuples_by_sequence[2], tuples_by_sequence[3]],
                secret,
                os.environ["ML_CLI_PATH"],
            )
            evidence_path = root / "public-evidence.json"
            evidence_path.write_text(json.dumps(evidence), encoding="utf-8")
            receipts: list[dict] = []
            guarded_saver = MemoryLineageCheckpointer(
                saver,
                cli_path=os.environ["ML_CLI_PATH"],
                evidence_path=evidence_path,
                secret_provider=lambda: secret,
                on_receipt=receipts.append,
            )
            self.assertIsInstance(guarded_saver, BaseCheckpointSaver)
            guarded_graph = builder.compile(checkpointer=guarded_saver)
            guard_active[0] = True

            current = guarded_graph.invoke({"request": "synthetic current resume"}, thread_config)
            self.assertEqual(current["memorylineage_sequence"], 4)
            self.assertEqual(guarded_node_runs, [4])
            self.assertEqual(receipts[-1]["decision"]["recommendedAction"], "RESUME_ALLOWED")

            old_tuple = tuples_by_sequence[1]
            with self.assertRaises(RecoveryHeld) as held:
                guarded_graph.invoke(
                    {"request": "synthetic stale resume"}, old_tuple.config
                )
            self.assertEqual(
                held.exception.receipt["decision"]["recommendedAction"], "REHEARSE_ONLY"
            )
            self.assertEqual(guarded_node_runs, [4])

    def test_async_sqlite_checkpoint_is_gated_before_graph_receives_it(self):
        async def exercise(root: Path) -> None:
            database_path = root / "agent-checkpoints-async.sqlite"
            guarded_node_runs: list[int] = []
            guard_active = [False]

            def agent_step(state: AgentState) -> AgentState:
                sequence = state.get("memorylineage_sequence", 0) + 1
                if guard_active[0]:
                    guarded_node_runs.append(sequence)
                return {
                    "memorylineage_sequence": sequence,
                    "notes": [*state.get("notes", []), state.get("request", "")],
                }

            builder = StateGraph(AgentState)
            builder.add_node("agent_step", agent_step)
            builder.add_edge(START, "agent_step")
            builder.add_edge("agent_step", END)
            thread_config = {"configurable": {"thread_id": "synthetic-async-thread"}}
            async with AsyncSqliteSaver.from_conn_string(str(database_path)) as writer_saver:
                await writer_saver.setup()
                source_graph = builder.compile(checkpointer=writer_saver)
                for sequence in range(1, 4):
                    await source_graph.ainvoke(
                        {"request": f"synthetic async turn {sequence}"}, thread_config
                    )

                tuples_by_sequence = {}
                async for checkpoint_tuple in writer_saver.alist(thread_config):
                    checkpoint_sequence = checkpoint_tuple.checkpoint["channel_values"].get(
                        "memorylineage_sequence"
                    )
                    if checkpoint_sequence in (1, 2, 3):
                        tuples_by_sequence.setdefault(checkpoint_sequence, checkpoint_tuple)
                self.assertEqual(sorted(tuples_by_sequence), [1, 2, 3])

            # Closing the first async saver and opening a new one exercises
            # resume from persisted state instead of the writer connection.
            secret = bytes(range(33, 65))
            evidence = generate_blinded_demo_evidence(
                [tuples_by_sequence[1], tuples_by_sequence[2], tuples_by_sequence[3]],
                secret,
                os.environ["ML_CLI_PATH"],
            )
            evidence_path = root / "public-async-evidence.json"
            evidence_path.write_text(json.dumps(evidence), encoding="utf-8")
            async with AsyncSqliteSaver.from_conn_string(str(database_path)) as saver:
                await saver.setup()
                receipts: list[dict] = []
                guarded_saver = MemoryLineageCheckpointer(
                    saver,
                    cli_path=os.environ["ML_CLI_PATH"],
                    evidence_path=evidence_path,
                    secret_provider=lambda: secret,
                    on_receipt=receipts.append,
                )
                self.assertIsInstance(guarded_saver, BaseCheckpointSaver)
                guarded_graph = builder.compile(checkpointer=guarded_saver)
                guard_active[0] = True

                current = await guarded_graph.ainvoke(
                    {"request": "synthetic async current resume"}, thread_config
                )
                self.assertEqual(current["memorylineage_sequence"], 4)
                self.assertEqual(guarded_node_runs, [4])
                self.assertEqual(receipts[-1]["decision"]["recommendedAction"], "RESUME_ALLOWED")

                with self.assertRaises(RecoveryHeld) as held:
                    await guarded_graph.ainvoke(
                        {"request": "synthetic async stale resume"},
                        tuples_by_sequence[1].config,
                    )
                self.assertEqual(
                    held.exception.receipt["decision"]["recommendedAction"], "REHEARSE_ONLY"
                )
                self.assertEqual(guarded_node_runs, [4])

        with tempfile.TemporaryDirectory(prefix="memorylineage-async-langgraph-") as temp:
            asyncio.run(exercise(Path(temp)))


if __name__ == "__main__":
    unittest.main()
