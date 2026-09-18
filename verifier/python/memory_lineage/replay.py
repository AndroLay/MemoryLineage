"""Run the independent verifier over the exported local EVM evidence."""

from __future__ import annotations

import argparse
import json
from pathlib import Path

try:
    from .independent import verify_file
except ImportError:
    from memory_lineage.independent import verify_file


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--input",
        type=Path,
        default=Path(__file__).resolve().parents[3] / "evidence" / "local" / "memory_lineage_evm_evidence.json",
    )
    parser.add_argument(
        "--output",
        type=Path,
        default=Path(__file__).resolve().parents[3] / "evidence" / "generated" / "memory_lineage_evm_replay.json",
    )
    args = parser.parse_args()
    result = verify_file(args.input)
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(result, indent=2, sort_keys=True) + "\n")
    print(json.dumps(result, indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
