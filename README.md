# MemoryLineage

> **Verify the history, not the memory.**

MemoryLineage is an independent auditor for private AI-agent memory history.
It verifies whether a committed memory state is the continuous, authorized
successor of the previously committed state without publishing raw memory
on-chain.

Release baseline: **`v1.0.0`**

## The problem

An agent can restore an old private database after a crash, migration, or
incident. The local database alone cannot tell an independent reviewer whether
that snapshot is the current checkpoint, a known historical checkpoint, a
divergent state, or an unverifiable state.

MemoryLineage records only fixed-size commitments and transition metadata in an
Ethereum registry. The memory, documents, prompts, and private locator contents
remain off-chain.

## The core flow

```text
Private SQLite memory
        ↓
Rust commitments
        ↓
Solidity registry on Ethereum
        ↓
Portable evidence
        ↓
Independent Rust verification
```

### The Silent Rollback

The local Demo Space V2 contains three synthetic snapshots:

```text
Snapshot 1 → Snapshot 2 → Snapshot 3  ← canonical committed head
```

The operator restores Snapshot 1 and attempts to continue the history. The
Rust/revm execution uses Snapshot 1's actual stale state root against the
canonical head at Snapshot 3. The registry rejects the attempt with the exact
Solidity reason:

```text
REJECTED
BAD_PREVIOUS_STATE
```

The local restore is still possible. MemoryLineage proves that the restored
snapshot cannot silently become the next canonical committed state under the
registry rules.

## Try the product

The Rust/WASM Dioxus Inspector is the primary product:

```text
Home → Inspect → History → Tampering Lab → Verify
```

The website lets a reviewer:

1. inspect the canonical head and authority history;
2. run the Silent Rollback rehearsal;
3. export or import portable evidence;
4. change one commitment and receive `TRANSITION_ID_MISMATCH`;
5. restore the original bundle and receive `VERIFIED`;
6. reproduce the result with the independent Rust CLI.

The Inspector has 11 routes. Four are primary tools: `/inspect`,
`/history`, `/lab`, and `/verify`. The supporting routes are `/`,
`/history/:sequence`, `/evidence`, `/architecture`, `/security`,
`/reproduce`, and `/prior-work`.

## What it verifies

- sequence and predecessor continuity;
- configured EOA authorization and authority rotation;
- commitment and deterministic state-root integrity;
- replayable evidence and canonical committed head;
- the published Solidity behavior through Rust/revm and read-only observations.

## What it does not verify

MemoryLineage does not determine:

- whether private memory is semantically true or safe;
- whether an AI's reasoning or inference is correct;
- whether an agent's behavior is harmless;
- whether off-chain memory remains available;
- whether a memory state caused a later action;
- whether every form of memory poisoning is prevented.

The bounded security report is not a formal proof or a third-party security
audit. ERC-1271 results distinguish on-chain acceptance from full historical
offline signer-contract reexecution.

## Architecture

```text
SQLite snapshots
      ↓
Rust memory store and canonicalization
      ↓
Solidity MemoryLineage Registry
      ↓
Ethereum Sepolia / read-only RPC
      ↓
Rust/WASM Inspector
      ↓
Evidence bundle → independent Rust verifier and CLI
```

The local Demo Space V2 is synthetic and is not user data. A separate Sepolia
deployment and readback observation is included in the repository; the local
Demo Space V2 history is not claimed as deployed to Sepolia.

## Technology

| Layer | Technology |
| --- | --- |
| Protocol | Solidity `0.8.36`, EIP-712, EOA/ERC-1271 authorization |
| Application | Rust `1.97.1` |
| Website | Dioxus `0.8.0-alpha.1` Web/WASM |
| Ethereum access | Alloy and read-only JSON-RPC |
| Local execution | `revm` |
| Fixture | Deterministic SQLite |
| Independent verifier | Rust `ml-verifier-independent` |
| Compatibility lanes | JavaScript/EthereumJS and Python |

The pinned ERC-8350 draft is treated as a compatibility target. It is not
described as a final standard.

## Quick start

Requirements: Rust `1.97.1`, the `wasm32-unknown-unknown` target, the pinned
Dioxus CLI, Node.js/npm, and Python 3.

Run the complete local verification gate:

```bash
cargo xtask verify
```

Build and test the static website:

```bash
cargo xtask release
```

For local development, use the repository wrapper:

```bash
cargo xtask serve-web
cargo xtask smoke-dev-web
```

The pinned Dioxus alpha's default hot-patch path cannot parse the WASM module
emitted by Rust `1.97.1`. The wrapper disables that path. Direct invocation
must use `dx serve --web --hot-patch false`.

Run the preserved compatibility lane:

```bash
npm run verify
npm audit --omit=dev --audit-level=high
```

Useful auditor commands:

```bash
cargo run -q -p ml-cli -- fixture inspect
cargo run -q -p ml-cli -- conformance
cargo run -q -p ml-cli -- revm silent-rollback
cargo run -q -p ml-cli -- revm mutations
cargo run -q -p ml-cli -- verify evidence/local/demo_space_v2_evidence.json
cargo run -q -p ml-cli -- agent reference-demo
cargo run -q -p ml-cli -- security bounded-audit
```

For a transportable clean archive:

```bash
cargo xtask reviewer-package /tmp/memorylineage-reviewer-package.tar.gz
cargo xtask reviewer-reproduce
```

The reviewer archive reproduction is automated evidence. It is not a claim of
independent human reproduction or external adoption.

## Verification status

The release gate currently covers:

- Rust formatting, Clippy, workspace tests, and WASM compilation;
- pinned vectors and independent evidence replay;
- SQLite fixture regeneration and Demo Space V2 invariants;
- Rust/revm Silent Rollback, mutation, ERC-1271, and authority lanes;
- reference agent-runtime recovery gating and bounded assurance;
- 11-route browser smoke, accessibility, keyboard, responsive, tamper, and
  restore flows;
- public package boundaries and legacy EVM/Python compatibility checks.

The dependency scans report no known vulnerability, unsoundness, or yanked
package. The RustSec scan reports two unmaintained transitive crates,
`derivative` and `paste`; they are maintenance warnings, not known
exploits.

Still intentionally outside this repository release:

- formal third-party security audit;
- external human clean-checkout reproduction and production adoption;
- public hosting, deployment, and staging;
- demo video recording.

## Repository map

| Path | Purpose |
| --- | --- |
| `apps/inspector/` | Primary Rust/WASM website |
| `contracts/solidity/` | Registry and ERC-1271 test contract |
| `crates/ml-core/` | Rust reference semantics |
| `crates/ml-evidence/` | Evidence schemas and projections |
| `crates/ml-memory-store/` | SQLite snapshot tooling |
| `crates/ml-local-evm/` | Rust/revm execution lane |
| `crates/ml-verifier-independent/` | Separate Rust replay verifier |
| `crates/ml-cli/` | Auditor and developer commands |
| `fixtures/` | Synthetic public snapshots |
| `evidence/` | Local, Sepolia, conformance, and mutation reports |
| `docs/` | Product, architecture, testing, security, and submission docs |
| `internal/` | Private visual references; excluded from releases |

## Further documentation

- [Product contract](docs/product/product-contract.md)
- [Inspector demo runbook](docs/product/inspector-demo.md)
- [Repository structure](docs/architecture/repository-structure.md)
- [System boundaries](docs/architecture/system-boundaries.md)
- [Rust parity ledger](docs/migration/rust-parity.md)
- [Testing and evidence](docs/testing.md)
- [Security assurance](docs/security/security-assurance.md)
- [Dependency audit](docs/security/dependency-audit.md)
- [Submission claim matrix](docs/submission/claim-matrix.md)
- [Prior research](docs/research/README.md)
- [Third-party notices](THIRD_PARTY_NOTICES.md)

## Contributing

Changes must preserve Solidity semantics and exact revert reasons, pinned
vectors, evidence compatibility, the separation between raw memory and public
commitments, and truthful source labels. When reproducible source, tests, or
evidence conflict with documentation, the reproducible behavior wins.

See [CONTRIBUTING.md](CONTRIBUTING.md) for the repository workflow.

## License

MemoryLineage is released under the [MIT License](LICENSE). Third-party
dependency notices are recorded in [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md).
