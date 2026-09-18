#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

if rg -n --glob '!*.md' --glob '!*.json' --glob '!*.sol' \
  "(^|[[:space:]'\"])(spike|research|docs)([./'\"]|$)|/home/andro/|evm/(contracts|test_vectors|artifacts)" \
  contracts evm verifier apps; then
  echo "FAIL: executable source crosses a repository boundary or contains a local path" >&2
  exit 1
fi

echo "PASS: architecture boundaries"
