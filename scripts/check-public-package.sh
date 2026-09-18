#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
MODE="${1:-working}"

required=(README.md LICENSE THIRD_PARTY_NOTICES.md Cargo.toml Cargo.lock rust-toolchain.toml .cargo package.json package-lock.json contracts contracts/artifacts verifier evidence docs scripts apps crates fixtures xtask)
for path in "${required[@]}"; do
  if [[ ! -e "$path" ]]; then
    echo "FAIL: required public path is missing: $path" >&2
    exit 1
  fi
done

for pattern in 'target/' 'node_modules/' '__pycache__/' 'evm/artifacts/' 'evidence/generated/' '.env' '*.pem' '*.key'; do
  if ! grep -Fq "$pattern" .gitignore; then
    echo "FAIL: generated or sensitive path is not ignored: $pattern" >&2
    exit 1
  fi
done

if rg -n --glob '!*.md' --glob '!*.json' --glob '!*.lock' '/home/andro/' contracts evm verifier apps crates fixtures xtask; then
  echo "FAIL: executable source contains a machine-local absolute path" >&2
  exit 1
fi

if find contracts verifier evm scripts apps crates fixtures xtask -type f \( -name '*.pem' -o -name '*.key' -o -name '.env' \) -print -quit | grep -q .; then
  echo "FAIL: credential-shaped file is inside the public source boundary" >&2
  exit 1
fi

if [[ "$MODE" == "--release" ]]; then
  if [[ -e internal ]]; then
    echo "FAIL: release package contains private internal material: internal" >&2
    exit 1
  fi
  for generated_path in evidence/generated; do
    if [[ -e "$generated_path" ]]; then
      echo "FAIL: release package contains generated path: $generated_path" >&2
      exit 1
    fi
  done
  if find . -type d \( -name target -o -name node_modules -o -name .next -o -name out \) -print -quit | grep -q .; then
    echo "FAIL: release package contains dependency or build output" >&2
    exit 1
  fi
  if find . -type d -name '__pycache__' -o -type f \( -name '*.pyc' -o -name '*.pyo' \) | grep -q .; then
    echo "FAIL: release package contains Python cache output" >&2
    exit 1
  fi
fi

echo "PASS: public package shape"
