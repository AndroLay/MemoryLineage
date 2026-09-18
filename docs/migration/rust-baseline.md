# Rust migration baseline

Baseline captured on 18 September 2026 before the Rust-first migration.

## Working oracle

The existing Solidity registry, JavaScript/EthereumJS harness, Python replay
verifier, Sepolia evidence, SQLite snapshots, and Next.js Inspector remain the
compatibility oracle. Their semantics are not being rewritten in this phase.

The last verified legacy gate completed all nine repository steps: EVM tests,
local audit, independent replay, Python tests, fixture regeneration, Inspector
typecheck, Inspector production build, architecture boundary check, and public
package check.

## Environment

```text
rustc 1.97.1
cargo 1.97.1
node v26.8.2
Python 3.14.7
Dioxus CLI: not installed at baseline; source and WASM checks are now available
solc executable: not installed; legacy lane uses the pinned npm solc package
```

At the time this baseline was captured, the extracted workspace had no usable
Git history. The source, curated evidence, and `package-lock.json` were the
preserved rebuild inputs. The current repository now has a normal Git history;
this paragraph remains a historical note about the migration starting point.

## Migration rule

Rust output must match the published vector and the legacy implementation before
any legacy lane is removed. A mismatch blocks the next migration phase.

## Progress evidence

The first Rust migration slice now passes:

- `ml-core` golden vector checks for space ID, transition ID, state root, and
  EIP-712 digest construction;
- Rust SQLite read/restore checks against the existing #17/#18/#19 fixture;
- independent Rust verification of the current public evidence bundle;
- native and `wasm32-unknown-unknown` compile checks for the Dioxus Inspector;
- Alloy host checks plus live Sepolia inspection and real `eth_call` rollback
  rejection with `BAD_PREVIOUS_STATE`.

The release artifact now passes a Chromium/CDP smoke test for initial render,
Silent Rollback, Verify, and evidence tamper interaction. `dx serve --web`
dev-server smoke remains NOT YET DEMONSTRATED because the pinned Dioxus dev
emitter rejects the generated WASM exceptions proposal. The Next.js Inspector
remains the migration oracle for broader responsive parity. A first Rust/revm
registry execution slice now deploys the curated Solidity artifact and rejects
the stale predecessor; EthereumJS remains the broader execution oracle.
