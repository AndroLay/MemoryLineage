# MemoryLineage — Peak-Potential Product and Evidence Plan

Status: **strategy and execution baseline**
Date: **20 September 2026**
Scope: **the product, evidence, and engineering work required to make MemoryLineage as strong as the available evidence allows**

This document is a decision record. It separates:

1. facts observed in the repository or public project material;
2. product decisions;
3. targets that are not achieved until their evidence exists.

It does not predict the judges' decision and it does not claim that MemoryLineage
currently beats every submission. The official hackathon gallery is not a
complete public census, and the official page does not publish criterion
weights. Any statement about the whole field must therefore be treated as a
public-index comparison, not a complete competition result.

## The product decision

MemoryLineage should become:

> **A recovery integrity gate and independent auditor for private AI-agent memory. Before an agent loads a restored snapshot, it verifies whether that snapshot is the authorized continuation of a committed history, without publishing the raw memory.**

The product question is:

> **Can this private snapshot be safely classified before it is loaded into the agent?**

The answer must be one of a small number of explicit decisions:

```text
RESUME_ALLOWED
REHEARSE_ONLY
HOLD_FOR_REVIEW
FAIL_CLOSED
```

This is a sharper product boundary than “blockchain for AI memory”, “memory
poisoning detector”, or “AI black box”. MemoryLineage verifies committed
lineage and authorization. It does not decide whether memory content is true,
safe, or semantically useful.

## What has been established

The current repository already contains substantial implementation evidence:

| Area | Evidence-backed status | Correct interpretation |
| --- | --- | --- |
| Solidity registry | Implemented and tested | Ordered transitions, predecessor continuity, authorization, rotation, and replay protection are enforced in the tested contract path. |
| Rust reference path | Implemented | Published protocol calculations and evidence behavior are reproduced in Rust. |
| Independent Rust verifier | Implemented | Evidence is replayed by a separate verification path; this is not a formal proof. |
| Rust/revm lane | Implemented | Local bytecode execution covers the published conformance, mutation, ERC-1271, and authority-rotation paths. |
| SQLite fixture | Implemented | A deterministic private-memory demonstration exists. It is a fixture, not production adoption. |
| Inspector | Implemented | The website presents inspection, history, tampering, verification, evidence, scope, and reproduction surfaces. |
| Sepolia observation | Implemented separately | Public-chain observations must remain labeled separately from the local Demo Space V2. |
| External human reproduction | Not demonstrated | Automated or internal runs are not independent developer reproduction. |
| Production agent integration | Not demonstrated | A local reference runtime is not adoption by an external agent framework. |
| Formal security audit | Not performed | Bounded assurance and adversarial testing must not be called formal verification or a third-party audit. |

There is also a concrete hardening item before claiming a fully hardened
snapshot commitment profile: the current memory-store serialization uses
unescaped delimiters for key/value data. Different maps can therefore produce
the same serialized bytes in edge cases. This must be replaced by an
unambiguous length-prefixed or ABI-compatible encoding and covered by a
regression test.

## The central distinction from the strongest public benchmark

The public Forkline material is a useful benchmark for clarity, replayability,
and a complete developer-facing story. Its public repository describes
replay-first agent tracing, deterministic offline replay, first divergence,
redaction, schema checks, and CI-oriented use.

MemoryLineage should not imitate that product. The products answer different
questions:

```text
Forkline       Where did an execution diverge, and can I replay it?
MemoryLineage  May this private snapshot be loaded as the authorized history?
```

The difference becomes meaningful only when MemoryLineage has a real pre-load
integration point and a receipt that another party can verify. Until then,
“recovery gate” is a product target, not a proven adoption claim.

The competitive goal is therefore measurable:

```text
match Forkline on problem clarity and reproduction simplicity
exceed it on public-chain authorization, private-snapshot recovery decisions,
portable evidence, independent verification, and explicit trust boundaries
```

This is a target, not a claim that the current submission has already achieved
all of it.

## Lessons from the public 3rd-Web-Hack field

The current public index is incomplete: the official gallery reports 242
participants at the time of inspection and says the gallery has not been
published by the managers. The following register is therefore the set of
publicly resolved candidates and benchmarks, not a complete list of every
submission.

### Forkline

Public material:

- [Devpost entry](https://devpost.com/software/forkline-rehearse-the-rollback)
- [public repository](https://github.com/sauravvenkat/forkline)

What to learn:

- one failure story is easier to judge than a broad platform story;
- replay, redaction, schema validation, and CLI/CI usage make technical claims
  reproducible;
- a narrow workflow can feel more complete than a larger but disconnected
  protocol.

What MemoryLineage must add:

- a first-viewport explanation that is equally immediate;
- one coherent incident from private snapshot to on-chain rejection;
- a real pre-load decision path, not only an inspector;
- evidence that can be exported and independently replayed.

The Devpost page was not fully retrievable during the current public-index
pass. Claims about its exact Web3 implementation must therefore be checked
against the repository and the live entry before being stated as facts.

### FinalityDesk

Public entry: [FinalityDesk on Devpost](https://devpost.com/software/finalitydesk).

The prior local audit found a deliberately narrow payment verifier covering
exact transfer semantics, recipient, amount, canonical block/finality checks,
precision, malformed RPC responses, and mutation cases. Its lesson is scope
discipline: a small verifier with exact failure behavior is more credible than
a large system with vague guarantees.

MemoryLineage should apply the same discipline to recovery decisions. Every
decision needs an input, a named rule, a machine-readable reason, and a
replayable artifact.

### Kinetic

Public material:

- [Devpost entry](https://devpost.com/software/kinetic-m9i5gv)
- [public repository](https://github.com/Shivanikinagi/KINETIC)
- [live application](https://kinetic-pink.vercel.app/)

Kinetic demonstrates the appeal of a visible Web3 system: registry, escrow,
testnet contracts, provider flow, and a user-facing compute marketplace. Its
public README also exposes the architecture and deployed TestNet application
IDs.

The caution is equally useful. A hash chain of reported execution steps does
not by itself prove that a GPU performed the claimed work; that is an
architectural limitation inferred from the described protocol, not a claim of
misconduct. “Fully decentralized” also needs a precise trust analysis when a
central service orchestrates jobs.

MemoryLineage should take the public on-chain demonstration and explicit
architecture from Kinetic, while keeping the scope narrow and refusing to
claim more than the evidence proves.

### ArcLight AI, DEDSEC Shadow NET, and PrithviScan

Public entries:

- [ArcLight AI](https://devpost.com/software/arclight-ai)
- [DEDSEC Shadow NET](https://devpost.com/software/dedsec-shadow-net)
- [PrithviScan](https://devpost.com/software/prithviscan)

These are useful product lessons even where public material does not show a
strong blockchain layer:

- ArcLight demonstrates visual hierarchy and a polished command-center
  presentation;
- DEDSEC demonstrates a memorable, immediately understandable interaction;
- PrithviScan demonstrates how a concrete user workflow and real input can
  make a technical system feel useful.

MemoryLineage should borrow those product qualities, not their claims or
architecture. A polished interface cannot substitute for Web3 necessity.

### Adjacent memory-provenance work

[MemLineage](https://arxiv.org/abs/2605.14421) and its
[public repository](https://github.com/amurlaniakea/memlineage) are not
3rd-Web-Hack submissions and are not a Web3 registry. They are relevant because
they show a different layer: signed memory entries, append-only Merkle-log
provenance, derivation relationships, and semantic poisoning signals.

The distinction must remain explicit:

```text
MemLineage    semantic provenance and memory-safety analysis
MemoryLineage canonical succession, authority, recovery classification,
              and portable Web3 evidence
```

MemoryLineage may interoperate with semantic safety tools later, but it should
not claim to replace them.

## Lessons from Polkadot and other Web3 hackathons

The official [Polkadot 2025 hackathon](https://polkadot.devpost.com/) required
a public GitHub repository, a clear README with setup/use/dependencies, and a
working project. Its judging categories explicitly include technological
implementation, design, potential impact, and creativity. The
[Builder Party report](https://forum.polkadot.network/t/polkadot-builder-party-hackathon-report/16254)
also shows that the event had a large field and multiple tracks; there is no
single feature pattern that guarantees a win.

The public [OpenGuild project list](https://openguild.wtf/projects) and
[past-winner list](https://build.openguild.wtf/past-hackathon-winners) show
recurring patterns in recognized projects:

1. the user can complete a meaningful flow, not merely inspect a contract;
2. the chain or ecosystem is necessary to the workflow;
3. onboarding and UI are part of the product, not an afterthought;
4. tooling, interoperability, or portability can create impact beyond one
   demo;
5. the README, code quality, and reproducibility reduce reviewer uncertainty.

These observations are patterns from public descriptions, not a causal proof
of why any particular judge selected a project.

The correct Polkadot lesson for MemoryLineage is not “add another chain”. It
is:

```text
make the user workflow complete first;
make the ecosystem role explicit;
then prove portability only if it solves a real trust or recovery problem.
```

An optional future portability experiment can target the same evidence format
and recovery invariants on a Polkadot EVM/PolkaVM environment. It must not be
added before the Ethereum vertical slice, external reproduction, and evidence
package are complete.

## The final product shape

MemoryLineage should have three connected surfaces.

### 1. Inspector

The website answers:

```text
What is the canonical head?
Who was authorized?
What changed between states?
Which attacks are rejected?
What is public and what remains private?
```

### 2. Recovery Preflight

An adapter receives a candidate private snapshot and runs the same evidence
checks before invoking the agent's loader:

```text
candidate snapshot
        ↓
read-only evidence replay
        ↓
canonical / historical / branch / divergent / invalid classification
        ↓
policy decision
        ↓
load, rehearse, hold, or fail closed
```

The first adapter should be deliberately small and real. A local SQLite
adapter is acceptable as the initial reference. A second adapter for one open
agent runtime is the adoption milestone. It must be called an adapter, not
“production support”, until an external runtime is actually tested.

### 3. Recovery Receipt and Independent Verifier

Every decision should produce a portable receipt containing, where available:

```text
candidate snapshot commitment
registry and space identity
chain ID and spec/vector pin
observed canonical head
candidate classification
decision
exact reason code
authority/configuration observation
verification timestamp
evidence hash
```

The receipt must never contain raw memory or private locator content. It must
fail verification if a security-relevant field is changed. This receipt is the
main product extension that connects the protocol to an operational recovery
workflow.

## Execution roadmap

### Phase 0 — Correctness and evidence freeze

Exit only when:

- snapshot commitment encoding is unambiguous and collision regression tests
  pass;
- `cargo test --workspace` passes without special filtering;
- `cargo xtask verify` covers the behavior it claims to cover;
- the current legacy lanes remain available as compatibility oracles;
- the exact Demo Space V2 inputs and output hashes are recorded.

Do not build new product layers on top of ambiguous serialization or a gate
that reports more checks than it executes.

### Phase 1 — One coherent vertical slice

Create one deterministic incident:

```text
SQLite snapshot 1 → snapshot 2 → snapshot 3
                         │
                   authority rotation
                         │
                   canonical head 3

restore snapshot 1 locally
attempt successor with snapshot-1 root
read-only contract simulation
BAD_PREVIOUS_STATE
```

The website, evidence bundle, Rust CLI, and README must all use this same
incident. Keep the existing broader protocol corpus separate and label it as
the conformance/mutation corpus.

If the Demo Space V2 is not deployed, say so. Do not present local evidence as
Sepolia evidence. If deployment is later authorized, record the deployment
transaction, code hash, space ID, transition events, and second observation.

### Phase 2 — Recovery semantics

Implement and test:

```text
current canonical head   → RESUME_ALLOWED
known older checkpoint   → REHEARSE_ONLY
authorized new branch    → branch policy / explicit authorization
divergent candidate      → HOLD_FOR_REVIEW
invalid evidence         → FAIL_CLOSED
```

A historical checkpoint must not automatically be called an attack. The
decision depends on policy and authority evidence. The system must distinguish
“old but known” from “unknown or conflicting”.

### Phase 3 — Real loader boundary

The reference runtime must enforce ordering:

```text
candidate received
        ↓
preflight verification
        ↓
decision
        ├─ RESUME_ALLOWED      → invoke loader
        ├─ REHEARSE_ONLY       → no canonical mutation / rehearsal only
        ├─ HOLD_FOR_REVIEW      → do not load
        └─ FAIL_CLOSED          → do not load
```

Tests must prove that the loader is not invoked for held or invalid evidence.
The test output must identify the invocation count and the exact decision.

### Phase 4 — Evidence and independent replay

Use one versioned Evidence V2 bundle across the website, CLI, documentation,
and release artifact. It should include registry identity, space, spec pin,
head, transitions, observations, authority history, attack result, privacy
boundary, and verification metadata.

The browser/WASM verifier and independent CLI must agree on:

```text
transition IDs
state roots
sequence continuity
predecessor continuity
head reconstruction
authority history where the bundle supports it
tamper failure reason
```

EOA signatures can be independently checked when the bundle contains all
required EIP-712 inputs and signature data. ERC-1271 must remain labeled as
on-chain acceptance unless historical signer-contract re-execution is actually
implemented.

### Phase 5 — Product and UX hardening

The first viewport must make the problem understandable without a briefing:

```text
An agent restored an old private snapshot.
Can it be loaded as the canonical history?

[Run Silent Rollback]
[Verify Evidence]
```

The primary reviewer journey is:

```text
Home → Inspect → History → Tampering Lab → Verify → Independent CLI
Understand → Inspect → Trace → Falsify → Verify → Reproduce
```

Use exact source labels:

```text
LIVE RPC
PUBLISHED EVIDENCE
LOCAL DEMO
```

Never show a fallback as live. Keep the privacy boundary visible and show
semantic poisoning as `OUT OF SCOPE` when the lineage is structurally valid.

### Phase 6 — Reproduction and external validation

The external reproduction packet should contain only the repository URL,
README, and exact release commit. An external developer should be able to:

```text
clone
cargo xtask verify
build the website
run Silent Rollback
export evidence
run the independent verifier
```

Record the exact commit, OS, toolchain, commands, outputs, and two comprehension
answers:

```text
What does MemoryLineage prove?
What does it explicitly not prove?
```

This validates usability and reproducibility. It does not constitute a
security audit or market adoption.

### Phase 7 — Optional ecosystem portability

Only after the Ethereum path is frozen, reproduce the evidence format and
recovery classification in another execution environment if there is a real
benefit. The target is:

```text
same evidence semantics
same decision vocabulary
same independent replay expectations
different execution anchor
```

Do not call this “multichain support” until a second chain is actually tested
and documented. Do not trade away the primary Ethereum demo for portability.

## Target matrix

These are exit targets, not current results.

| Dimension | Target | Evidence required before claiming it |
| --- | --- | --- |
| Problem clarity | A first-time reviewer can explain the stale private snapshot problem in 10–15 seconds. | Comprehension notes from external reviewers or a documented usability run. |
| Web3 necessity | A public authority and canonical history are necessary to resolve competing recovery claims. | Architecture and live/local contract flow showing the trust boundary. |
| Technical feasibility | Registry, preflight, receipt, independent verifier, and local bytecode path agree. | Passing vector, mutation, runtime-gate, and receipt tests. |
| Uniqueness | The product is specifically a pre-load recovery gate with a portable decision receipt. | Feature works end to end and prior-work comparison is documented. |
| Evidence coherence | One Demo Space V2 incident drives the site, CLI, and bundle. | Matching IDs, roots, sequence, source labels, and hashes. |
| Reproducibility | One documented command verifies the release; an external developer repeats it. | Clean-checkout report at an exact commit. |
| Privacy | Raw memory never enters the registry or portable evidence. | Schema tests, secret scan, and inspection of generated bundles. |
| Negative-path realism | Stale, skipped, parallel, wrong-authority, domain, and commitment mutations fail with exact reasons. | Published mutation corpus and UI/CLI results. |
| Recovery usefulness | A loader is actually gated by the decision, including hold/fail-closed paths. | Runtime adapter tests with loader invocation counts. |
| Design | The product is as immediately understandable as the strongest public benchmark and as inspectable as the protocol requires. | 1440px/390px screenshots, keyboard pass, and real-data flows. |
| Impact | One real external runtime or developer can use the preflight/receipt workflow. | External integration evidence; do not infer adoption from a local fixture. |
| Security honesty | Formal audit, formal verification, and production adoption remain clearly separated. | Claim matrix and security page use exact status vocabulary. |

## What success would mean

If Phases 0–6 pass, MemoryLineage can make a defensible case that it is
stronger than the known public candidates in the following areas:

- more precise recovery semantics than a generic agent dashboard;
- stronger Web3 necessity than an ordinary AI/smart-city/chat application;
- a public authorization and canonical-history anchor;
- portable evidence rather than a dashboard-only verdict;
- independent replay in browser and CLI;
- a real pre-load decision boundary rather than post-hoc explanation only;
- explicit privacy and semantic-safety limits;
- a complete reviewer journey with a narrow, reproducible command.

It can borrow the best qualities of the public field without copying their
scope:

```text
Forkline       clarity, replay, CI-oriented reproduction
FinalityDesk   narrow checks and exact failure behavior
Kinetic        visible Web3 workflow and public testnet evidence
ArcLight       information hierarchy and visual polish
DEDSEC         memorable interaction and direct story
PrithviScan    concrete user workflow and real input
Polkadot work  ecosystem relevance, onboarding, complete product flow,
               code quality, design, impact, and creativity
```

This still cannot guarantee a judge outcome. Hidden submissions, unpublished
implementation details, judging interpretation, and the absence of official
criterion weights remain outside the repository's control.

## Non-goals before the vertical slice is complete

Do not add any of the following before the recovery workflow is real and
independently reproducible:

- token, NFT, DAO, marketplace, or DeFi surface;
- generic AI chatbot or semantic memory classifier;
- claim of preventing all memory poisoning;
- wallet product or autonomous payment system;
- new chain solely to increase technology count;
- ZK system without a concrete verification requirement;
- backend database that becomes an authoritative source;
- production-adoption claim based only on the local reference runtime.

## Final release gate

The product is ready for a strong submission package when a first-time reviewer
can perform this sequence:

```text
open the website
understand the recovery problem
inspect the canonical head
trace authority history
restore the old local snapshot
run the stale-root simulation
see BAD_PREVIOUS_STATE
export the evidence
tamper one commitment
see TRANSITION_ID_MISMATCH
restore the bundle
see VERIFIED
run the independent Rust CLI
obtain the same verdict
read the security page
understand the exact non-claims
clone the exact release commit
reproduce the verification gate
```

The final submission should then state only what the evidence supports:

> **MemoryLineage verifies the integrity and authorization of a committed
> private-agent history before recovery. It does not determine whether the
> underlying memory is true or safe.**

## Source register

Primary event and project sources used for this plan:

- [3rd-Web-Hack event page](https://3rd-web-hack.devpost.com/)
- [3rd-Web-Hack public gallery](https://3rd-web-hack.devpost.com/project-gallery)
- [Forkline](https://devpost.com/software/forkline-rehearse-the-rollback) and
  [repository](https://github.com/sauravvenkat/forkline)
- [FinalityDesk](https://devpost.com/software/finalitydesk)
- [Kinetic](https://devpost.com/software/kinetic-m9i5gv) and
  [repository](https://github.com/Shivanikinagi/KINETIC)
- [ArcLight AI](https://devpost.com/software/arclight-ai)
- [DEDSEC Shadow NET](https://devpost.com/software/dedsec-shadow-net)
- [PrithviScan](https://devpost.com/software/prithviscan)
- [Polkadot 2025 hackathon](https://polkadot.devpost.com/)
- [Polkadot Builder Party report](https://forum.polkadot.network/t/polkadot-builder-party-hackathon-report/16254)
- [OpenGuild project list](https://openguild.wtf/projects)
- [OpenGuild past winners](https://build.openguild.wtf/past-hackathon-winners)
- [MemLineage paper](https://arxiv.org/abs/2605.14421) and
  [repository](https://github.com/amurlaniakea/memlineage)

Repository sources that constrain the plan:

- [product contract](../product/product-contract.md)
- [current public README](../../README.md)
- [snapshot commitment implementation](../../crates/ml-memory-store/src/lib.rs)
- [current public competitor audit](2026-09-20-3rd-web-hack-public-project-audit.md)
