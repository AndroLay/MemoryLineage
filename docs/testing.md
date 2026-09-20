# Testing and evidence

## Local gates

Rust-first gate:

```bash
cargo xtask verify
```

Reviewer-oriented automated path:

```bash
cargo xtask reproduce
```

This adds the local toolchain check, static website build, Chromium smoke, and
release boundary to the Rust verification path. Its success is automated local
evidence; it does not count as an external human reproduction. The external
reviewer runbook and report template live in `docs/reproduction/`.

After the release commit is clean, the transportable reviewer archive is
checked with:

```bash
cargo xtask reviewer-package /tmp/memorylineage-reviewer-package.tar.gz
```

To exercise the archive itself without relying on `.git` metadata:

```bash
cargo xtask reviewer-reproduce
```

This command checks the archive contents; it does not change GitHub visibility
or create a hosted website.

The gate includes Rust formatting, Clippy, workspace tests, reproducible legacy
and Demo Space V2 fixture manifests, byte-for-byte Demo Space V2 evidence
regeneration from freshly created SQLite files, snapshot/transition parity,
stale-root attack checks, Recovery Decision Receipt generation/replay, the
generic protected-resume adapter, independent evidence replay, the Rust/revm
Silent Rollback/mutation/ERC-1271/authority lanes, the WASM compile, and the
public package boundary check.

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
11 routes, checks the 390px viewport for page-level overflow on every route,
then exercises the local Silent Rollback evidence replay, evidence tampering,
evidence restore, and Recovery Decision Receipt verification, tampering, and
restore. The rollback smoke waits for the result panel's terminal state and
checks that its detail contains the exact machine reason, so the pre-run
expected-reason badge cannot count as a result. It does not deploy or contact a
staging environment. `cargo xtask release` runs the same smoke after building
the release artifact and then checks the release package boundary.

The pinned development server can be checked separately with `dx serve
--web`, but it is not part of the release gate. In the current pinned
toolchain the server starts and the browser emitter then reports
`Failed to parse import section`; this remains a documented Dioxus/toolchain
limitation rather than a reason to weaken the verified static path.

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

The unified Demo Space V2 command is:

```bash
cargo run -q -p ml-cli -- demo silent-rollback
cargo run -q -p ml-cli -- verify evidence/local/demo_space_v2_evidence.json
```

It reads three actual SQLite snapshots, derives their commitments, executes
three transitions against the published Solidity bytecode in `revm`, rotates
the authorizer before transition 3, then submits transition 4 using the
state-root from transition 1. The expected exact revert is
`BAD_PREVIOUS_STATE`; no network write or transaction broadcast occurs. The
independent verifier checks that the attack record's stale root is bound to the
restored sequence, its canonical root matches the reconstructed head, and the
reported attempt is the next sequence. Raw snapshot values do not enter the
evidence JSON.

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
contains commitments and observations only except for the Demo Space V2 EOA
proof set: that bundle carries the typed-data domain, digest, and signature for
each of its three transitions, and the independent report labels them
`EOA_SIGNATURES_VERIFIED` and authority timeline `TIMELINE_BOUND`. The historical V1/public protocol-corpus
projection still reports authority rows `STRUCTURE_ONLY` and transition
authorization proof `NOT_INCLUDED`; lineage replay does not invent signatures
that are absent from those bundles.

## Restore Preflight and Recovery Rehearsal

The Inspect preflight uses the public synthetic SQLite fixture manifest and the
locally bundled Demo Space V2 evidence. It first replays the evidence, then
classifies a selected fixture snapshot commitment as a match to the evidence
head, a known earlier checkpoint, unknown/diverged, or unverified. Native tests
cover these classes and a malformed/tampered history. The interactive browser
tamper action changes only a candidate string in memory; it does not edit the
SQLite fixture, evidence file, chain, or an agent runtime.

The Recovery Decision Receipt makes that classification portable:

```bash
cargo run -q -p ml-cli -- recover preflight \
  fixtures/silent-rollback-v2/snapshot-1.db \
  evidence/local/demo_space_v2_evidence.json \
  /tmp/memorylineage-recovery-receipt.json \
  DEMO_SPACE_V2_LOCAL
cargo run -q -p ml-cli -- recover verify \
  /tmp/memorylineage-recovery-receipt.json \
  evidence/local/demo_space_v2_evidence.json
cargo run -q -p ml-cli -- recover enforce \
  fixtures/silent-rollback-v2/snapshot-3.db \
  evidence/local/demo_space_v2_evidence.json
```

The current head is permitted by the reference gate. The historical snapshot
is deliberately held as `REHEARSE_ONLY`; it must not silently enter a protected
resume path. The tracked current-head receipt is
`evidence/local/demo_space_v2_recovery_receipt.json`, and its shape is described
by `evidence/schemas/recovery-receipt-v1.schema.json`. This gate proves a
deterministic reference decision over the supplied fixture. The receipt carries
the `strict-current-head-only-v1` policy and the loader smoke actually reports
the number of private keys loaded only after the gate permits it; it is not
evidence that a production agent runtime has already integrated the adapter.

The UI wording is intentionally `MATCHES DEMO EVIDENCE HEAD`, not
`CANONICAL_HEAD`: the current browser selection is not a live observation of
the registry and does not gate an external agent. A known historical checkpoint
is available for isolated rehearsal and is not automatically treated as
malicious.

## Public chain evidence

Sepolia deployment and readback artifacts are observations of the named
deployment at the time they were generated. A second RPC readback is an
independent endpoint observation; it is not a light-client or consensus proof.
The current read-only fixture rehearsal is recorded in
`evidence/sepolia/silent_rollback_fixture_eth_call.json`; it uses the existing
deployment and broadcasts no transaction. This is a separate Sepolia
observation, not the local Demo Space V2 history. The browser probe resolves a
`finalized` block, falling back to `safe` only when the endpoint cannot provide
the former; it reads the registry and simulates the stale predecessor at that
same numbered block and checks the block hash again. It reports the exact
`BAD_PREVIOUS_STATE` result only when the RPC error carries a decodable
Solidity `Error(string)` payload with that reason. A single endpoint response
is still an observation, not consensus proof. New public deployment is outside
this work's scope.

## Reproducibility rules

- Use the exact versions in `package-lock.json`.
- Never put real private memory in public fixtures. Synthetic sample values may
  be included for deterministic reproduction; the evidence bundle, calldata,
  and contract events must exclude those values.
- Regenerate generated evidence with the documented command.
- Pin the ERC-8350 draft/vector snapshot used by a claim.
- Report failures rather than replacing evidence with a score.
- Keep the exact `BAD_PREVIOUS_STATE` contract reason; UI prose may explain it
  as a predecessor mismatch.
- Keep `evidence/generated/`, `.next/`, and every `node_modules/` directory out
  of a release archive; `npm run check:release` checks this after verification
  output has been moved out of the package.
