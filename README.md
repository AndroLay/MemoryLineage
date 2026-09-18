# MemoryLineage

**Verify the history, not the memory.**

MemoryLineage is an independent auditor for private AI-agent memory lineage.
It checks whether committed memory transitions are ordered, continuous, and
authorized without putting the raw memory on-chain.

## The problem

An agent may keep months of private state off-chain. If an operator restores an
old snapshot, skips a transition, or presents a conflicting successor, an
external reviewer needs to determine whether the current committed state is a
valid continuation of the agreed history.

MemoryLineage records commitments to that history and provides an independent
replay path. The protocol verifies sequence, predecessor, authorization, and
commitment integrity. It does not decide whether memory content is truthful,
safe, or semantically correct.

## Current evidence

- Solidity registry with EOA and ERC-1271 authorization paths.
- Local Ethereum execution tests: 8 cases.
- A separate Python MemoryLineage replay lane, with archived ResolverCompat and
  SlippageTruth tests kept outside the product claim.
- Adversarial mutation corpus covering sequence, predecessor, authorization,
  commitment, replay, and domain failures.
- Sepolia deployment and second-endpoint readback evidence.
- A deterministic SQLite snapshot fixture and local EthereumJS Silent Rollback
  simulation that reject a stale predecessor with `BAD_PREVIOUS_STATE`.
- Rust reference and independent Rust verifier lanes that agree with the
  published conformance vector and evidence bundle.
- A Rust/revm execution slice that deploys the curated Solidity artifact,
  commits three valid transitions, compares derived roots, and rejects a stale
  predecessor with the exact `BAD_PREVIOUS_STATE` reason.
- The same Rust/revm lane now rejects all 20 named contract mutations with the
  exact expected Solidity reasons; the legacy EthereumJS lane remains the
  broader oracle for the complete corpus. It also exercises a deployed mock
  ERC-1271 authorizer, accepting a valid owner signature and failing closed
  after acceptance is disabled.
- The Rust/revm lane also exercises controller/authorizer rotation: `configNonce`
  advances, the old authorizer is rejected, and the new authorizer commits the
  next transition.
- Machine-readable Rust/revm reports are published under
  `evidence/local/rust_revm_*.json`.
- A Dioxus Web Inspector path compiled for `wasm32-unknown-unknown`; the
  existing Next Inspector remains in the repository as the migration and UX
  compatibility oracle until browser parity is independently checked.

The implementation targets a pinned ERC-8350 draft snapshot. A moving draft
must not be described as final standard compliance.

## Run locally

The Rust verification gate is the primary migration command:

```bash
cargo xtask verify
```

The Rust CLI can inspect the deterministic fixture, replay the evidence, and
read the deployed Sepolia registry:

```bash
cargo run -q -p ml-cli -- fixture inspect
cargo run -q -p ml-cli -- verify evidence/local/memory_lineage_evm_evidence.json
cargo run -q -p ml-cli -- conformance
cargo run -q -p ml-cli -- revm silent-rollback
cargo run -q -p ml-cli -- revm mutations
cargo run -q -p ml-cli -- revm erc1271
cargo run -q -p ml-cli -- revm authority-rotation
cargo run -q -p ml-cli -- evidence export-v2 evidence/local/memory_lineage_evm_evidence.json /tmp/memorylineage-evidence-v2.json
cargo run -q -p ml-cli -- live inspect
cargo run -q -p ml-cli -- live rollback
```

The browser target can be typechecked with the Rust gate and built as a static
Dioxus release artifact with:

```bash
cargo xtask build-web
```

`cargo xtask build-web` uses `dx` from `PATH`. Set `DX_BIN=/path/to/dx` when
the pinned Dioxus CLI is installed outside `PATH`.

With the pinned Dioxus CLI installed, run the website from `apps/inspector/`:

```bash
dx serve --web
```

The legacy Next.js implementation remains available while the Rust/WASM
Inspector reaches functional and visual parity:

```bash
npm ci
npm run verify
```

Open the legacy Inspector during development with:

```bash
npm run dev
```

Then use `Run Silent Rollback` to rehearse the restored snapshot, `History` to
inspect the public commitments, and `Verify` to export or import a portable
evidence bundle. The Rust/WASM verifier and the independent Rust CLI apply the
same sequence, predecessor, head, and privacy-boundary checks.

```bash
cargo run -q -p ml-cli -- verify memorylineage-evidence.json
```

The Python verifier remains in the repository as a migration oracle and
historical cross-language implementation.
The CLI verifier auto-detects the historical V1 replay shape and the portable
V2 export produced by the Inspector.

The individual commands are useful when diagnosing a failure:

```bash
npm run test:evm
npm run test:python
npm run test:research
npm run check:boundaries
npm run check:package
npm run check:release
```

## Repository map

| Path | Responsibility |
| --- | --- |
| `contracts/` | Solidity sources, conformance vectors, and mutation corpus |
| `evm/` | Local EVM harness, tests, deployment, and Sepolia readback |
| `crates/ml-core/` | Rust canonicalization and pinned protocol semantics |
| `crates/ml-evidence/` | Versioned evidence projection and serialization |
| `crates/ml-ethereum/` | Alloy ABI/data layer plus host-side Sepolia RPC |
| `crates/ml-local-evm/` | Rust/revm execution lane for curated Solidity bytecode |
| `crates/ml-verifier-independent/` | Independent Rust evidence replay |
| `crates/ml-memory-store/` | Deterministic SQLite private-memory fixture |
| `crates/ml-cli/` | Secondary Rust developer/auditor surface |
| `verifier/python/` | Historical independent replay oracle |
| `evidence/` | Curated and generated verification outputs |
| `apps/inspector/` | Dioxus Web product surface; Next.js is retained as migration oracle |
| `fixtures/silent-rollback/` | Deterministic private SQLite snapshots used by the hero demo |
| `docs/` | Product, architecture, testing, research, and submission docs |
| `research/spikes/` | Exploratory work that is not part of the product path |
| `scripts/` | Repository-wide verification and packaging gates |
| `.github/` | Continuous verification workflow |

The product surface is under `apps/inspector/`. The final website target is
Dioxus Web/WASM; use `dx serve --web` after installing the pinned CLI. The
Next.js surface can still be started with:

```bash
npm run dev
```

## Evidence boundary

The chain stores fixed-size commitments and state transitions. Raw memory,
full documents, and private locators remain off-chain. A valid lineage proves
continuity and configured authorization; it does not prove semantic truth,
memory safety, AI reasoning correctness, or agent safety.

See [`docs/product/product-contract.md`](docs/product/product-contract.md),
[`docs/architecture/repository-structure.md`](docs/architecture/repository-structure.md),
and [`docs/testing.md`](docs/testing.md) for the detailed boundaries and
reproduction instructions.
