# Testing and evidence

## Local gates

Rust-first gate:

```bash
cargo xtask verify
```

The gate includes Rust formatting, Clippy, workspace tests, pinned conformance,
independent evidence replay, the Rust/revm Silent Rollback/mutation/ERC-1271/
authority lanes, the WASM compile, and the public package boundary check.

The static browser acceptance gate is separate and reproducible after the
release artifact exists:

```bash
cargo xtask build-web
cargo xtask smoke-web
# or, for the complete local release-preparation sequence:
cargo xtask release
```

`smoke-web` uses only Python's standard library and a locally installed
Chromium. It serves the static output with an `index.html` fallback, checks all
11 routes, checks the 390px viewport for page-level overflow, then exercises
Silent Rollback, evidence tampering, and evidence restore. It does not deploy
or contact a staging environment. `cargo xtask release` runs the same smoke
after building the release artifact and then checks the release package
boundary.

The release website artifact is built separately because it invokes the pinned
Dioxus CLI:

```bash
cargo xtask build-web
```

Set `DX_BIN` if `dx` is not on `PATH`. The task disables Dioxus debug symbols
for this release build because the pinned `wasm-opt` binary aborts when it
receives the generated DWARF section. This keeps the static artifact
reproducible without changing application or protocol behavior.

It runs formatting, clippy, Rust tests, the WASM compile, and the independent
Rust verifier path. The legacy gate remains required during migration:

```bash
npm run test:evm
npm run test:python
npm run check:boundaries
npm run check:package
npm run verify
```

The current baseline includes the EVM registry suite, a separate Python
MemoryLineage verifier suite with 12 tests, and 12 archived research-spike
tests. The EVM mutation suite exercises 20 named adversarial cases. These
counts describe the supplied test corpus; they are not a general false-positive or
false-negative rate.

The product gate also regenerates the Silent Rollback SQLite snapshots,
typechecks `apps/inspector/`, and builds its production route. The local API
simulation is independently reproducible with:

```bash
node evm/scripts/simulate_silent_rollback.mjs
```

It must return `REJECTED` with `BAD_PREVIOUS_STATE` after three canonical
transitions.

## Independent replay

The Rust lane now owns the public independent verifier product path. It consumes
the typed evidence bundle and reimplements the commitment/state replay without
depending on `ml-core`. The Python lane remains a separate replay oracle and
the legacy JavaScript/EVM lane remains the contract execution oracle. Matching
results are evidence of reproducibility across implementations.

```bash
cargo run -q -p ml-cli -- verify evidence/local/memory_lineage_evm_evidence.json
cargo run -q -p ml-cli -- conformance
cargo run -q -p ml-cli -- revm silent-rollback
cargo run -q -p ml-cli -- revm mutations
cargo run -q -p ml-cli -- revm erc1271
cargo run -q -p ml-cli -- revm authority-rotation
cargo run -q -p ml-cli -- evidence export-v2 evidence/local/memory_lineage_evm_evidence.json /tmp/memorylineage-evidence-v2.json
cargo run -q -p ml-cli -- verify /tmp/memorylineage-evidence-v2.json
cargo run -q -p ml-cli -- fixture manifest
python3 verifier/verify.py evidence/local/memory_lineage_evm_evidence.json
```

The conformance command reports `MATCH` for the pinned vector, Rust reference,
and independent Rust verifier, plus `REJECTED/BAD_PREVIOUS_STATE` and `20/20
REJECTED` core mutations and the ERC-1271 accept/reject check from the Rust/revm registry execution slice. It
reports `VERIFIED` for the published evidence bundle after replaying it with
the independent verifier. These results are bounded to the published corpus;
they are not a formal-verification claim. The revm slice is not yet the
complete replacement for the legacy JavaScript mutation and ERC-1271 harness:
its 20-case lane covers ordering, predecessor, zero-value, unknown-space, EOA
signature/domain binding, replay, and branch checks. The ERC-1271 lane deploys
the published mock authorizer, accepts a valid owner signature, then observes
`INVALID_AUTHORIZATION` after the authorizer is disabled.
The authority-rotation lane checks `configNonce == 1`, rejects a transition
signed by the old authorizer, and accepts the same next transition under the
rotated authorizer.

The same deterministic outputs are kept as public local evidence:

- `evidence/local/rust_revm_conformance.json`
- `evidence/local/rust_revm_mutation_matrix.json`
- `evidence/local/rust_revm_erc1271.json`
- `evidence/local/rust_revm_authority_rotation.json`

They contain no raw memory or private keys.

The current browser bundle is projected to `memorylineage-evidence-v2`. It
contains commitments and observations only; the Dioxus Verify view accepts V2
and the historical V1 public replay shape.

## Public chain evidence

Sepolia deployment and readback artifacts are observations of the named
deployment at the time they were generated. A second RPC readback is an
independent endpoint observation; it is not a light-client or consensus proof.
The current read-only fixture rehearsal is recorded in
`evidence/sepolia/silent_rollback_fixture_eth_call.json`; it uses the existing
deployment, a commitment from fixture snapshot 17, and broadcasts no
transaction.

## Reproducibility rules

- Use the exact versions in `package-lock.json`.
- Keep raw memory out of fixtures intended for public submission.
- Regenerate generated evidence with the documented command.
- Pin the ERC-8350 draft/vector snapshot used by a claim.
- Report failures rather than replacing evidence with a score.
- Keep the exact `BAD_PREVIOUS_STATE` contract reason; UI prose may explain it
  as a predecessor mismatch.
- Keep `evidence/generated/`, `.next/`, and every `node_modules/` directory out
  of a release archive; `npm run check:release` checks this after verification
  output has been moved out of the package.
