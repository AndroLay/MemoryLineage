#!/usr/bin/env python3
"""Portable MemoryLineage evidence verifier entrypoint."""

from __future__ import annotations

import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "verifier" / "python"))

from memory_lineage.portable import main  # noqa: E402


raise SystemExit(main())
