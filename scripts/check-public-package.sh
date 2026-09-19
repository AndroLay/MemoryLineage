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
  if command -v git >/dev/null 2>&1 && git rev-parse --is-inside-work-tree >/dev/null 2>&1; then
    tracked_forbidden="$(git ls-files | rg '(^|/)(internal|target|node_modules|\.next|out|evidence/generated)(/|$)|(__pycache__|\.pyc|\.pyo)$' || true)"
  else
    # A reviewer archive created by `git archive` intentionally has no `.git`
    # directory. Inspect the extracted filesystem in that case; every file in
    # the archive is already a release candidate, so a tracked-file query is
    # neither available nor necessary.
    # The source archive was checked before the reproduction build. During the
    # gate, target/.next and similar directories are generated intentionally;
    # exclude them from this post-build filesystem scan while still rejecting
    # private references and generated evidence that could have been archived.
    tracked_forbidden="$(find . \
      \( -type d \( -name target -o -name node_modules -o -name .next -o -name out -o -name __pycache__ \) -prune \) -o \
      -type f -print | sed 's#^\./##' | rg '(^|/)(internal|evidence/generated)(/|$)|(__pycache__|\.pyc|\.pyo)$' || true)"
  fi
  if [[ -n "$tracked_forbidden" ]]; then
    echo "FAIL: release would contain ignored/private/generated tracked paths:" >&2
    echo "$tracked_forbidden" >&2
    exit 1
  fi

  package_root="${RELEASE_PACKAGE_ROOT:-}"
  if [[ -n "$package_root" ]]; then
    if [[ ! -d "$package_root" ]]; then
      echo "FAIL: RELEASE_PACKAGE_ROOT does not exist: $package_root" >&2
      exit 1
    fi
    if find "$package_root" -type d \( -name internal -o -name target -o -name node_modules -o -name .next -o -name out -o -name __pycache__ \) -print -quit | grep -q .; then
      echo "FAIL: extracted release package contains private or generated output" >&2
      exit 1
    fi
    if find "$package_root" -type f \( -name '*.pyc' -o -name '*.pyo' \) -print -quit | grep -q .; then
      echo "FAIL: extracted release package contains Python cache output" >&2
      exit 1
    fi
  fi
fi

echo "PASS: public package shape"
