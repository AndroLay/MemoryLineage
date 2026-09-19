# Repository structure

MemoryLineage uses a small evidence-first monorepo. The structure follows the
useful repository discipline of the RevenueCat reference project while
remaining proportional to a protocol, verifier, and Rust/WASM Inspector
submission product.

## Source boundaries

```text
contracts/       protocol inputs and shared vectors
evm/             preserved JavaScript/EthereumJS compatibility lane
crates/          primary Rust protocol, evidence, RPC, EVM, recovery, CLI, and verifier lanes
verifier/python/ historical independent replay oracle
evidence/        reproducible outputs and public chain observations
apps/inspector/  Dioxus Web inspect, tampering, and portable-verification surface
fixtures/        public synthetic SQLite inputs for deterministic demo replay
docs/            product and engineering explanation
research/        exploratory work outside the product path
scripts/         repository-wide checks
```

The JavaScript and Python lanes intentionally implement the relevant hashing
and replay rules separately. Shared JSON vectors provide comparable inputs;
runtime code is not shared between them.

## Repository rules

- Product code may consume `contracts/vectors/` and approved evidence schemas.
- Product code must not import `docs/`, `research/`, or generated evidence.
- Generated outputs belong under `evidence/generated/` and are reproducible.
- Public Sepolia observations belong under `evidence/sepolia/`.
- `apps/inspector/` reads curated evidence and may issue a separate read-only
  browser RPC observation; it does not import the local EVM harness into browser
  code.
- `fixtures/silent-rollback-v2/` contains synthetic SQLite snapshots and a
  manifest for the unified local Demo Space V2. The sample values are public and
  contain no real user or agent memory; portable evidence and chain data include
  commitments only.
- `fixtures/silent-rollback/` preserves the earlier 17/18/19 fixture as a
  historical compatibility input.

## Why this is smaller than the reference repository

This project has no mobile client, database worker, billing system, or
deployment platform. The Inspector is a static Rust/WASM website; the legacy
Next.js surface and EthereumJS/Python lanes remain compatibility oracles.
