#!/usr/bin/env python3
"""Create deterministic private-memory SQLite snapshots for the demo."""

from __future__ import annotations

import json
import sqlite3
from pathlib import Path


ROOT = Path(__file__).resolve().parent
SCHEMA = """
CREATE TABLE memory_state (
  key TEXT PRIMARY KEY,
  value TEXT NOT NULL,
  state_sequence INTEGER NOT NULL
);
"""

STATES = {
    17: {"language": "Indonesian"},
    18: {"language": "Indonesian", "claim_policy": "evidence-backed"},
    19: {
        "language": "Indonesian",
        "claim_policy": "evidence-backed",
        "privacy": "raw-memory-private",
    },
}


def write_snapshot(sequence: int, values: dict[str, str]) -> None:
    path = ROOT / f"snapshot-{sequence}.db"
    if path.exists():
        path.unlink()
    with sqlite3.connect(path) as connection:
        connection.executescript(SCHEMA)
        connection.executemany(
            "INSERT INTO memory_state(key, value, state_sequence) VALUES (?, ?, ?)",
            [(key, value, sequence) for key, value in values.items()],
        )
        connection.execute("PRAGMA user_version = 1")


def verify_snapshot(sequence: int, expected: dict[str, str]) -> None:
    path = ROOT / f"snapshot-{sequence}.db"
    with sqlite3.connect(path) as connection:
        rows = dict(connection.execute("SELECT key, value FROM memory_state"))
        recorded = {row[0] for row in connection.execute("SELECT state_sequence FROM memory_state")}
    if rows != expected or recorded != {sequence}:
        raise RuntimeError(f"snapshot-{sequence}.db does not match its manifest")


def main() -> None:
    for sequence, values in STATES.items():
        write_snapshot(sequence, values)
        verify_snapshot(sequence, values)
    print(json.dumps({"fixture": "silent-rollback-v1", "snapshots": sorted(STATES), "verified": True}))


if __name__ == "__main__":
    main()
