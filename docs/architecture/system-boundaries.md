# System boundaries

## Trust boundary

The operator may control the off-chain memory store. MemoryLineage therefore
publishes commitments and authorized transitions to a public registry. An
independent verifier can replay the evidence without receiving the raw memory.

## What is verified

- transition sequence continuity;
- predecessor state-root continuity;
- configured EOA or ERC-1271 authorization;
- EIP-712 domain and field binding;
- commitment and replay consistency;
- the pinned vector and mutation corpus results.

## What is outside the boundary

- semantic truth of memory content;
- whether a memory write is safe or malicious;
- correctness of an AI model or its reasoning;
- availability of private off-chain content;
- proof that an arbitrary external action was caused by a particular memory
  state;
- compatibility with a future, unpinned ERC-8350 draft.

## Dependency directions

```text
contracts/vectors ───────┬──> evm
                         ├──> verifier/python
                         └──> crates/ml-core and ml-conformance
contracts/solidity ─────────> evm compiler/harness and crates/ml-local-evm

fixtures/silent-rollback ──> crates/ml-memory-store and ml-cli
evidence/local,sepolia ────> crates/ml-evidence, ml-cli, and apps/inspector
crates/ml-spec-types ──────> passive formats only
crates/ml-core ────────────> Rust reference algorithms
crates/ml-verifier-independent ──> separate replay algorithms
apps/inspector ────────────> Dioxus/WASM presentation and read-only RPC

evidence/public ─────────────> verifier and documentation tools
research/ and docs/ ──────────> no executable product code
```

The separate Python replay implementation is an intentional historical
independence boundary. The Rust independent verifier follows the same rule: it
may consume passive evidence structures, but it must not import `ml-core`'s
hash/state-transition implementation. Neither verifier may execute the EVM
harness as its verification mechanism.
