# System boundaries

## Trust boundary

The operator may control the off-chain memory store. The registry semantics
allow authorized commitments to be anchored on Ethereum; the existing
deployment evidence is a separate earlier Sepolia space. The current Demo Space
V2 is a local Rust/revm execution using public synthetic SQLite input, not a new
Sepolia deployment. An independent verifier can replay its portable evidence
without receiving raw memory.

## What is verified

- transition sequence continuity;
- predecessor state-root continuity;
- configured EOA or ERC-1271 authorization;
- EIP-712 domain and field binding;
- commitment and replay consistency;
- the pinned vector and mutation corpus results.

Demo Space V2 additionally records three SQLite-derived transitions, a local
authority rotation, and a transition-4 rejection using the actual sequence-1
root while the local head is sequence 3. The separate Sepolia probe is a
read-only observation against the existing deployment; it is not evidence that
the Demo Space V2 history exists on Sepolia.

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

fixtures/silent-rollback{,-v2} ──> crates/ml-memory-store and ml-cli
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
