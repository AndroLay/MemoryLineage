#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

echo "[1/9] EVM tests"
npm run test:evm >/dev/null

echo "[2/9] Local EVM audit"
node evm/scripts/audit_local.mjs >/dev/null

echo "[3/9] Independent replay"
PYTHONPATH="$ROOT/verifier/python" python3 -m memory_lineage.replay >/dev/null

echo "[4/9] Python tests"
npm run test:python >/dev/null

echo "[5/9] Silent rollback fixture"
npm run fixtures:silent-rollback >/dev/null

echo "[6/9] Inspector typecheck"
npm run test:inspector >/dev/null

echo "[7/9] Inspector production build"
npm run build:inspector >/dev/null

echo "[8/9] Boundary check"
npm run check:boundaries >/dev/null

echo "[9/9] Public package check"
npm run check:package >/dev/null

echo "PASS: local verification gate"
