# MemoryLineage

> **Verify the history, not the memory.**

## The problem

Persistent AI-agent memory can live in private storage controlled by its
operator. After a crash or restore, the operator may load a snapshot older than
the state already recorded by an independent controller or auditor. The key
recovery question is not whether that old snapshot is malicious; it is whether
it is the current checkpoint, a known earlier checkpoint, divergent, or
unverifiable—and what evidence supports that decision.

MemoryLineage makes that decision inspectable. In the local Demo Space V2,
three synthetic SQLite snapshots produce three committed transitions. Restoring
snapshot 1 does not move the recorded head at state 3. A transition 4 attempt
using state 1's actual root is executed against the published Solidity bytecode
in Rust/revm and rejected with `BAD_PREVIOUS_STATE`.

The local restore itself remains possible. The current Restore Preflight is a
fixture-scoped assessment against replayed local evidence; it does not gate an
external agent runtime, inspect memory meaning, or read the live chain. The
optional Sepolia probe is a separate read-only observation. The public synthetic
fixture contains sample values, while portable evidence contains commitments
rather than those values.

## What MemoryLineage does

MemoryLineage is an independent auditor for private AI-agent memory history. It
checks whether committed states form a continuous history and makes the
evidence for restore decisions reviewable without requiring raw memory to be
published on-chain. Demo Space V2 carries EIP-712 EOA signatures for each of
its three transitions and the independent verifier recovers those signers.
The separate protocol-corpus projection remains `STRUCTURE_ONLY` with
transition authorization proof `NOT_INCLUDED`; ERC-1271 remains an execution
observation rather than a historical offline proof.

~~~text
Private memory → Commitment → Canonical registry → Portable evidence → Independent replay
~~~

The product's central question is:

> **Before an agent resumes, does this snapshot match the recorded head, a
> known earlier checkpoint, a divergent state, or insufficient evidence?**

MemoryLineage is not a semantic memory-safety detector, an AI reasoning
evaluator, a causal action proof, or a general agent-wallet guard. It verifies
the history that was committed, not whether the underlying memory is true.

[![Rust](https://img.shields.io/badge/Rust-1.97.1-orange?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Dioxus](https://img.shields.io/badge/Dioxus-0.8.0--alpha.1-1769f5)](https://dioxuslabs.com/)
[![Solidity](https://img.shields.io/badge/Solidity-0.8.36-363636?logo=solidity&logoColor=white)](https://soliditylang.org/)
[![Ethereum](https://img.shields.io/badge/Ethereum-Sepolia-627eea?logo=ethereum&logoColor=white)](https://ethereum.org/en/developers/docs/networks/#sepolia)
[![License](https://img.shields.io/badge/License-MIT-2f855a)](./LICENSE)

## At a glance

| Area | Decision |
| --- | --- |
| Product | Evidence workspace for auditing private AI-agent memory lineage |
| User decision | Restore Preflight: classify a candidate against a named, replayed evidence history |
| Hero scenario | The Silent Rollback: a restored snapshot's actual earlier root is rejected as the next predecessor |
| Browser | Rust/WASM Dioxus Inspector with Inspect, History, Tampering Lab, and Verify surfaces |
| Trust anchor | Solidity registry on Ethereum Sepolia with EIP-712 and ERC-1271 authorization paths |
| Private domain | SQLite snapshots, raw memory, documents, prompts, and locator contents remain off-chain |
| Evidence | Unified local Demo Space V2 (three SQLite-derived transitions, one authority rotation, a stale-root rejection, and a replayable Recovery Decision Receipt), a separate four-transition protocol corpus, 20 Rust/revm mutation cases, and existing Sepolia deployment/readback |
| Independent path | Separate Rust verifier; historical Python and JavaScript lanes remain compatibility oracles |
| Repository license | MIT; see the single root [LICENSE](LICENSE) file |

## Current status

The repository contains the working Rust-first application and verification
stack:

- a Solidity registry that enforces sequence, predecessor, authorization,
  state-root, and authorization-rotation rules;
- a deterministic SQLite Demo Space V2 with synthetic snapshots 1, 2, and 3
  that model private agent memory; the public sample database is not user data,
  and portable evidence contains commitments only;
  the historical 17/18/19 fixture is preserved separately;
- Rust reference, evidence, Ethereum RPC, revm, CLI, and independent-verifier
  crates;
- a Dioxus Web Inspector with 11 routes, evidence-backed data, clearly labeled
  local-demo and separate Sepolia observations, browser evidence export/import,
  Rust/WASM-side tamper verification, fixture-scoped Restore Preflight, and a
  Recovery Decision Receipt workbench;
- a Rust `ml-recovery-gate` adapter and CLI recovery gate that verify a receipt
  before invoking a protected loader for a current head, while holding
  historical and unverified candidates for rehearsal/review;
- published local reports for conformance, mutation rejection, ERC-1271
  acceptance/rejection, and authority rotation;
- an Ethereum Sepolia deployment at
  [`0x36fE9FA585565615Adcfe8680a126F770931E160`](https://sepolia.etherscan.io/address/0x36fE9FA585565615Adcfe8680a126F770931E160),
  with a recorded readback observation.

The current read-only Sepolia rollback observation is published at
[`evidence/sepolia/silent_rollback_fixture_eth_call.json`](evidence/sepolia/silent_rollback_fixture_eth_call.json).
It belongs to the previously deployed Sepolia space and returns
`BAD_PREVIOUS_STATE` without broadcasting a transaction. It is separate from
the local Demo Space V2 incident; the repository does not claim that this demo
history has been deployed to Sepolia.

The static Dioxus release artifact and browser interaction smoke path are
verified. The pinned Dioxus development emitter still has a known WASM
exceptions-proposal limitation, so `dx serve --web` is documented separately
from the verified static release path.

The following remain open submission work and are not claimed as complete:

- independent clean-checkout reproduction by external developers;
- demo video and pitch-deck artifacts;
- a final submission tag;
- deployed hosting for the static website.

This repository intentionally does not perform public deployment, staging, or
video recording as part of its local release-preparation gate.

## Core experience

The judge-facing journey is deliberately short:

~~~mermaid
flowchart LR
    A[Open Inspector] --> B[Inspect canonical head]
    B --> C[Trace History]
    C --> D[Run Silent Rollback]
    D --> E{Registry result}
    E -->|BAD_PREVIOUS_STATE| F[Understand stale predecessor]
    F --> G[Export evidence]
    G --> H[Tamper one commitment]
    H --> I[TRANSITION_ID_MISMATCH]
    I --> J[Restore bundle]
    J --> K[VERIFIED]
~~~

The website has 11 focused surfaces. The primary navigation stays small;
supporting pages are reached through contextual links.

| Route | Question it answers |
| --- | --- |
| `/` | What is MemoryLineage? |
| `/inspect` | What is canonical right now? |
| `/history` | How did the canonical history get here? |
| `/history/:sequence` | What exactly happened in this transition? |
| `/lab` | Can I break the committed history? |
| `/verify` | Can I verify this without trusting the website? |
| `/evidence` | Where is the proof behind the visible claims? |
| `/architecture` | How does the system work? |
| `/security` | What does the system actually guarantee? |
| `/reproduce` | Can another developer reproduce these claims? |
| `/prior-work` | What existed before the hackathon, and what was built here? |

### The Silent Rollback

The unified local Demo Space V2 contains three synthetic SQLite snapshots that
model private agent state. Their sample values are included in the public test
fixture, but not in portable evidence or on-chain data:

~~~text
State 1 → State 2 → State 3  ← local canonical committed head
~~~

The operator restores snapshot 1 locally. The Rust/revm harness has executed
the three SQLite-derived commitments against the published Solidity bytecode,
including one authority rotation. It then attempts transition 4 using the
actual state root produced by transition 1 while the local registry head is
transition 3. The result is:

~~~text
REJECTED
BAD_PREVIOUS_STATE
~~~

The primary browser action replays and verifies the local Demo Space V2 bundle.
The separate Sepolia probe is optional and read-only; an unavailable or
unexpected RPC result does not replace or change the local result. The Rust CLI
gate regenerates the synthetic SQLite fixture and executes the Solidity
bytecode in `revm`. No transaction is broadcast in either flow.

The result means that a stale restored snapshot cannot silently become the next
canonical committed state under the registry rules. It does not mean that all
forms of rollback, malicious behavior, or semantic memory poisoning are
prevented.

The Inspect Preflight classifies the selected public synthetic checkpoint as a
match to the Demo Space V2 evidence head, a known historical checkpoint, or an
unknown/diverged candidate. The result is derived from the fixture manifest and
independent local evidence replay. It is not a production snapshot commitment,
a live Sepolia head assessment, or proof that an external agent runtime obeyed
the decision. The companion Recovery Decision Receipt and Rust CLI gate make the
decision portable and hold a known older checkpoint as `REHEARSE_ONLY`. A known
older checkpoint is a valid rehearsal input, not automatically an attack.

## Architecture

MemoryLineage keeps the private memory domain separate from the public
verification domain:

~~~mermaid
flowchart TB
    subgraph Private[Private memory domain]
        M[SQLite snapshots] --> S[Rust memory store]
        S --> C[Rust canonicalization and commitments]
    end

    C --> R[Solidity MemoryLineage Registry]
    R --> E[Ethereum Sepolia]

    E --> Q[Read-only RPC]
    Q --> I[Rust/WASM Dioxus Inspector]
    R --> B[Portable evidence bundle]
    B --> V[Independent Rust verifier]
    B --> L[Rust CLI]
    B --> G[Protected resume adapter]

    J[Legacy JavaScript/EthereumJS and Python lanes] -. compatibility oracles .-> R
~~~

### Trust boundaries

The public chain stores fixed-size commitments and transition metadata. It does
not need to store:

- raw memory text;
- private documents or prompts;
- private locator contents;
- signing keys or seed phrases;
- the agent's complete off-chain context.

The registry enforces the public invariants. The Rust reference path drives
the product, while the independent Rust verifier reimplements the replay path
without depending on `ml-core`. Agreement across the published corpus is
evidence of reproducibility; it is not a formal-verification claim.

### Why blockchain is used

The party operating the agent should not also be the only party trusted to
preserve its audit history. An append-only registry gives an external reviewer
a public predecessor and authorization boundary while the private memory
remains outside the chain.

## Tech stack

| Layer | Technology | Responsibility |
| --- | --- | --- |
| Protocol | Solidity `0.8.36` | Registry, EIP-712 authorization, EOA/ERC-1271 checks, ordered transitions, and authority rotation |
| Primary application | Rust `1.97.1` | Canonicalization, evidence, RPC, local EVM, memory fixture, CLI, verifier, and application data layer |
| Website | Dioxus `0.8.0-alpha.1` Web/WASM | Browser Inspector, route surfaces, evidence import/export, read-only RPC, and browser verification |
| Ethereum access | Alloy primitives and read-only JSON-RPC | Sepolia head, registry, event, code-hash, and `eth_call` observations |
| Local execution | `revm` | Rust execution slice for registry, Silent Rollback, mutation, ERC-1271, and authority-rotation cases |
| Demo fixture | Synthetic SQLite | Deterministic snapshot/restore rehearsal; no real user or agent memory |
| Independent verification | Rust `ml-verifier-independent` | Separate evidence replay and fail-closed tamper checks |
| Compatibility lanes | JavaScript/EthereumJS and Python | Preserved historical execution and cross-language replay oracles |
| Submission evidence | JSON bundles and reports | Conformance, local execution, Sepolia observation, mutation, and provenance records |

The pinned ERC-8350 draft snapshot is treated as a compatibility target. A
moving draft must not be described as final standard compliance.

## Repository map

| Path | Responsibility |
| --- | --- |
| `contracts/solidity/` | Solidity registry and mock ERC-1271 authorizer |
| `contracts/vectors/` | Pinned conformance vectors |
| `contracts/mutations/` | Named adversarial mutation corpus |
| `crates/ml-spec-types/` | Passive protocol and evidence structures |
| `crates/ml-core/` | Rust reference canonicalization and pinned semantics |
| `crates/ml-evidence/` | Versioned evidence projection and serialization |
| `crates/ml-ethereum/` | Alloy ABI/data layer and host-side Sepolia reads |
| `crates/ml-local-evm/` | Rust/revm execution slice for the curated Solidity artifact |
| `crates/ml-memory-store/` | Deterministic SQLite tooling for synthetic private-memory-shaped fixtures |
| `crates/ml-recovery-gate/` | Generic protected-resume adapter that holds non-current decisions before loader invocation |
| `crates/ml-verifier-independent/` | Independent Rust evidence replay |
| `crates/ml-cli/` | Developer and auditor command surface |
| `apps/inspector/` | Primary Dioxus Web/WASM product surface |
| `evm/` | Preserved JavaScript/EthereumJS execution and Sepolia compatibility lane |
| `verifier/python/` | Historical independent replay oracle |
| `fixtures/` | Public synthetic snapshot inputs for the hero scenario; no real private data |
| `evidence/` | Curated local, Sepolia, schema, and Rust/revm outputs |
| `docs/` | Product, architecture, testing, research, and submission documentation |
| `research/` | Exploratory work outside the product path |
| `scripts/` | Boundary and package checks |
| `xtask/` | Cargo-centered verification and web-build commands |
| `internal/` | Ignored owner-supplied design references; never part of a release package |

## Contracts and safety rules

These rules are product invariants:

- A transition must extend the registry's current sequence and predecessor
  state root.
- Transition commitments bind the signed fields, including locator and
  provenance commitments.
- Authorization changes advance `configNonce`; old authorization state is not
  silently reused after rotation.
- `BAD_PREVIOUS_STATE` remains the exact contract reason for a stale
  predecessor. UI copy may explain it as a predecessor mismatch.
- Raw memory is not required to be published on-chain.
- The browser and CLI fail closed for malformed, unsupported, or tampered
  evidence bundles.
- Demo Space V2 carries the typed-data domain, digest, and EOA signature for
  each transition, binds each proof to its active `configNonce` and effective
  sequence, and the independent Rust verifier recovers the declared authorizer
  before reporting `EOA_SIGNATURES_VERIFIED` / `TIMELINE_BOUND`.
- ERC-1271 on-chain acceptance is distinct from full historical offline
  signer-contract reexecution unless the latter is explicitly implemented.
- The registry enforces configured authorization when a transition is
  submitted. The Demo Space V2 verifier reports `EOA_SIGNATURES_VERIFIED` for
  its published EOA proof set; the separate protocol-corpus projection stays
  `STRUCTURE_ONLY` / `NOT_INCLUDED`, and ERC-1271 remains
  `ON-CHAIN ACCEPTANCE OBSERVED` without historical signer-contract replay.
- A valid lineage does not
  prove semantic truth, semantic memory safety, AI reasoning correctness,
  inference correctness, agent behavioral safety, off-chain availability, or
  a causal link from memory to an action.

## Public repository boundary

The repository contains reproducible source, synthetic fixtures, public chain
observations, tests, scripts, and documentation. The following remain outside
the release package:

- `internal/` and its private design references;
- Cargo `target/`, Next `.next/`, `node_modules/`, and caches;
- generated evidence under `evidence/generated/`;
- credentials, signing material, environment files, and private keys.

The repository checks these boundaries through `.gitignore` and the package
script. A source checkout intentionally contains owner-only `internal/` and
local dependencies, so run the release check against a release-shaped copy
where those paths have been excluded.

## Quick start

### Requirements

- Rust `1.97.1`; the version is pinned in `rust-toolchain.toml`.
- The `wasm32-unknown-unknown` Rust target for the website check.
- The pinned Dioxus CLI (`dx`) for the static web release build.
- Node.js/npm and Python 3 for the preserved JavaScript, Next.js, fixture, and
  historical Python lanes.

### Rust verification and evidence

From a checkout with access to the private repository:

~~~bash
git clone https://github.com/AndroLay/MemoryLineage.git
cd MemoryLineage

cargo xtask verify
cargo run -q -p ml-cli -- fixture inspect
cargo run -q -p ml-cli -- conformance
cargo run -q -p ml-cli -- verify evidence/local/memory_lineage_evm_evidence.json
~~~

The Rust gate runs formatting, Clippy, workspace tests, deterministic fixture
manifest regeneration, pinned conformance, independent replay, the Rust/revm
Silent Rollback/mutation/ERC-1271/authority lanes, the Dioxus WASM compile, and
the public package boundary check.

### Build the website

~~~bash
cargo xtask build-web
~~~

The static artifact is written to:

~~~text
target/dx/memorylineage-inspector/release/web/public/
~~~

`cargo xtask build-web` uses `dx` from `PATH`. Set `DX_BIN=/path/to/dx` when
the pinned CLI is installed elsewhere. A static host must route unknown paths
to `index.html` so the deep links remain available.

The pinned Dioxus development emitter currently has a known WASM
exceptions-proposal limitation. The static release build is the verified
browser path for the current toolchain.

For the complete local release-preparation gate, including the static build,
Chromium route/interaction smoke, and tracked-file package boundary check:

~~~bash
cargo xtask release
cargo xtask release-manifest /tmp/memorylineage-release-manifest.json
~~~

This prepares artifacts without deploying hosting, staging, or recording the
demo video.

### CLI execution lanes

~~~bash
cargo run -q -p ml-cli -- revm silent-rollback
cargo run -q -p ml-cli -- revm mutations
cargo run -q -p ml-cli -- revm erc1271
cargo run -q -p ml-cli -- revm authority-rotation
cargo run -q -p ml-cli -- live inspect
cargo run -q -p ml-cli -- live rollback
cargo run -q -p ml-cli -- evidence export-v2 \
  evidence/local/memory_lineage_evm_evidence.json \
  /tmp/memorylineage-evidence-v2.json
cargo run -q -p ml-cli -- verify /tmp/memorylineage-evidence-v2.json
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
~~~

The live commands are read-only observations or `eth_call` simulations. They
do not require a wallet or private key.

### Compatibility lane

The legacy JavaScript/Python lane remains available while the Rust application
is the primary product path:

~~~bash
npm ci
npm run verify
~~~

The historical Next.js Inspector can be opened with:

~~~bash
npm run dev
~~~

It is preserved as a compatibility and UX oracle, not as the final website
architecture.

## Verification

Use the smallest relevant check during development, then run the complete
gates for a release candidate:

| Evidence | Proves | Does not prove |
| --- | --- | --- |
| `cargo xtask verify` | Rust formatting, Clippy, workspace tests, both fixture manifests, reproducible Demo Space V2 evidence, recovery receipt and protected-resume invariants, stale-root/authority/privacy invariants, conformance, independent replay, revm execution lanes, WASM compilation, and package boundaries | Deployed hosting, human understanding, or semantic truth of private memory |
| `cargo xtask build-web` | The pinned Dioxus application produces a static release artifact | A deployment platform serves every deep link correctly |
| `cargo xtask smoke-web` | Chromium checks the static SPA fallback, 11 routes, Silent Rollback, evidence tamper/restore, and Recovery Decision Receipt tamper/restore | Production hosting, staging, or assistive-technology acceptance |
| Rust/revm reports | The curated Solidity artifact agrees with the Rust lane for the published Silent Rollback, mutation, ERC-1271, and authority-rotation cases | Formal verification or all possible EVM/runtime behavior |
| `npm run verify` | Preserved EVM, Python, fixture, Next.js typecheck/build, boundary, and package checks | Rust website visual quality or external user validation |
| Sepolia deployment/readback | Recorded code, receipts, head, and second-endpoint observations match the published bundle | Light-client, consensus, or multi-provider consensus proof |
| Browser smoke | The static release route and core Verify/Lab interactions render and expose exact results | Accessibility review on every device or production hosting |
| External human reproduction | Whether another developer can follow the public setup independently | A security audit or a guarantee of future adoption |

The current published Rust/revm report records:

~~~text
Pinned vector          MATCH
Published evidence    VERIFIED
Rust reference        MATCH
Independent verifier  MATCH
Silent Rollback       REJECTED / BAD_PREVIOUS_STATE
Core mutations        20 / 20 REJECTED
ERC-1271              ACCEPTED THEN REJECTED
Authority rotation    NONCE_1 / OLD_REJECTED / NEW_ACCEPTED
~~~

These results are bounded to the published corpus. They are not a score, a
formal proof, or a general security guarantee.

The repository uses evidence language deliberately:

- **implemented** means the code path exists;
- **verified** means the named command or environment was actually run;
- **observed** means a chain or provider observation was recorded;
- **rejected** means the tested input was refused with the stated result;
- **out of scope** means the product intentionally does not evaluate that
  property;
- **not yet demonstrated** means an external gate remains open.

## Documentation

- [Product contract](docs/product/product-contract.md)
- [Inspector demo runbook](docs/product/inspector-demo.md)
- [Repository structure](docs/architecture/repository-structure.md)
- [System boundaries](docs/architecture/system-boundaries.md)
- [Rust parity ledger](docs/migration/rust-parity.md)
- [Testing and evidence](docs/testing.md)
- [Submission claim matrix](docs/submission/claim-matrix.md)
- [Prior research](docs/research/README.md)
- [Third-party notices](THIRD_PARTY_NOTICES.md)

The private visual contract lives under `internal/design/` during product
development and is intentionally not part of the public release package.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md). Changes should preserve:

- the Solidity protocol semantics and exact revert reasons;
- pinned vector and evidence compatibility;
- the separation between raw private memory and public commitments;
- independent replay without sharing the reference algorithm;
- truthful local-evidence and live-observation source labels;
- the release boundary that excludes private references and generated output.

When documentation conflicts with a reproducible source, test, or evidence
artifact, the reproducible behavior wins. Claims that have not been
demonstrated should be labeled `NOT YET DEMONSTRATED` rather than promoted by
wording.

## License

MemoryLineage source is released under the [MIT License](LICENSE).

Third-party notices are recorded in
[THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md). They do not replace or add a
second root project license.
