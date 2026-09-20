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

## The important flows

### 1. System architecture

```text
Private SQLite snapshots
        ↓
Rust memory store + canonical commitments
        ↓
Solidity MemoryLineage Registry
        ├─ Rust/revm local execution
        └─ Ethereum Sepolia read-only observation
        ↓
Rust/WASM Inspector + portable evidence
        ↓
Independent Rust verifier and CLI
```

### 2. Reviewer journey

```text
Home → Inspect → History → Tampering Lab → Verify → Independent CLI
Understand → Canonical → Trace → Attack → Verify → Reproduce
```

The Inspector has 11 routes. Four are primary tools: `/inspect`, `/history`,
`/lab`, and `/verify`. The other routes cover the home explanation,
transition detail, public evidence, architecture, security scope,
reproduction, and submission provenance.

### 3. The Silent Rollback

```text
Snapshot 1 → Snapshot 2 → Snapshot 3
     │                              ↓
     └─ restore locally       canonical committed head
                    ↓
       attempt transition 4 with Snapshot 1 root
                    ↓
          REJECTED / BAD_PREVIOUS_STATE
```

The local Demo Space V2 contains synthetic SQLite snapshots 1, 2, and 3. The
restored snapshot remains possible locally, but its actual stale root cannot be
accepted as the successor of the later canonical head.

### 4. Evidence verification

```text
Export evidence → Import bundle → Verify in Rust/WASM
                                      ↓
                 change one commitment → TRANSITION_ID_MISMATCH
                                      ↓
                 restore original    → VERIFIED
                                      ↓
                         replay with independent Rust CLI
```

### 5. Recovery decision

```text
Candidate snapshot → replay named evidence history → decision
       ├─ current head       → RESUME_ALLOWED
       ├─ known historical   → REHEARSE_ONLY
       ├─ diverged           → HOLD_FOR_REVIEW
       └─ invalid evidence   → FAIL_CLOSED
```

### 6. Privacy boundary

```text
Raw memory, documents, prompts, private locators
                         │ stays off-chain
                         ↓
Fixed-size commitments + transition metadata
                         ↓
Registry, public observations, and portable evidence
```

Raw memory remains outside the chain and portable evidence. Demo Space V2 is
synthetic test data, not user data. A separate Sepolia deployment and readback
observation is included in the repository; the local Demo Space V2 history is
not claimed as deployed to Sepolia.

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
