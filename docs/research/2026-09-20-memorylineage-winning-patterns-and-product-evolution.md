# MemoryLineage — Winning Patterns, Problem Maturity, and Product Evolution

Status: **decision research for the next implementation phase**
Date: **20 September 2026**

This document records a focused research pass over 3rd-Web-Hack, relevant Web3
hackathons from 2025–2026, public winner submissions, open-source repositories,
Ethereum standards, and current agent-memory security research. It turns those
findings into a bounded product decision for MemoryLineage.

It is a product and evidence decision document. It does not change the Solidity
protocol, claim that MemoryLineage has already beaten a competitor, or authorize
deployment, staging, or demo-video work.

## Decision

Keep **MemoryLineage** and keep its current protocol core.

Evolve the product surface from a general “memory lineage inspector” into:

> **MemoryLineage — Restore Preflight for private agent history.**

The existing tagline remains:

> **Verify the history, not the memory.**

The product question becomes:

> **Before a persistent agent resumes from a restored private snapshot, is that snapshot the current canonical checkpoint, a known historical checkpoint, a divergent state, or an unverified state?**

This is a sharper product than “blockchain for AI memory,” and it is more
defensible than “memory-poisoning detection.” It uses the existing commitments,
linear registry, authority model, Rust verifier, SQLite fixture, Rust/revm lane,
Inspector, and evidence bundle. It adds a decision layer around recovery without
claiming that the registry understands the semantic truth of memory.

The recommended public product name is therefore:

```text
MemoryLineage
Restore Preflight for private AI-agent history
```

The product has three visible verbs:

```text
INSPECT → REHEARSE → VERIFY
```

The implementation should eventually expose a fourth verb through a bounded
adapter:

```text
RESUME — only when the host application accepts the preflight result
```

The website itself must remain an auditor and rehearsal surface. It must not say
that it universally blocks an external agent runtime until an actual runtime
adapter demonstrates that behavior.

## What was researched

The research used four evidence groups:

1. the official 3rd-Web-Hack page and its requirements;
2. official winner announcements and public repositories from relevant 2025–2026
   Web3 hackathons;
3. current Ethereum standards and agent-memory security work;
4. the current MemoryLineage repository, evidence files, and competitor audit.

Public submission descriptions are not automatically implementation evidence. For
each external project, this document separates the official award or submission
status, repository/readme evidence, and claims that were not independently
replayed here.

## What 3rd-Web-Hack actually rewards

The [official 3rd-Web-Hack page](https://3rd-web-hack.devpost.com/) asks teams to
solve real-world or unsolved Blockchain/Web3 problems with a problem statement,
solution, working prototype or MVP, technology stack, GitHub setup and usage
instructions, a demo, and a presentation. Its stated criteria are Innovation,
Technical Feasibility, Uniqueness, and Design. The page does not publish weights
or a detailed scoring rubric.

That means MemoryLineage should optimize for four observable outcomes:

| Criterion | What a judge must be able to see | MemoryLineage implication |
| --- | --- | --- |
| Innovation | A non-obvious problem with a credible Web3 reason | Recovery from a private snapshot can conflict with a shared canonical history; the registry resolves continuity and authorization between independent parties. |
| Technical Feasibility | A working vertical slice with repeatable evidence | SQLite snapshot → Rust commitment/profile → Solidity registry/revm → exact rejection → portable Rust verification. |
| Uniqueness | A specific layer that is not another generic agent wallet or reputation product | Canonical recovery classification for private state history, with stale-snapshot rehearsal and evidence assurance levels. |
| Design | A first-time visitor can understand and falsify the claim | Show the recovery decision and exact error first; expose hashes, authority, and proof levels only where they help inspection. |

The correct strategy is therefore not to add the most technologies. It is to make
one Web3 failure mode as understandable and reproducible as the strongest
submissions while making the evidence boundary more explicit.

## Relevant benchmark: Polkadot Build Resilient Apps, 2025

The [Polkadot Build Resilient Apps hackathon](https://polkadot.devpost.com/)
ran from 6 October to 17 November 2025, offered $40,000 in prizes, and published
criteria around technological implementation, design, potential impact beyond
Polkadot, and creativity/uniqueness. Its [official winner update](https://polkadot.devpost.com/updates/40528-winners-announced-and-what-s-next)
named winners across Build a Blockchain, User-centric Apps, and Polkadot
Tinkerers.

### Fangorn — Polkadot Tinkerers first prize

- Devpost: [Fangorn](https://devpost.com/software/fangorn)
- GitHub: [driemworks/fangorn](https://github.com/driemworks/fangorn)

Fangorn explains a concrete gap in one sentence: NFTs may prove ownership but do
not provide access control. Its solution combines intent-bound data, threshold
encryption, a Polkadot contract, a Rust/Iroh network, a CLI/TUI, and a React
visualizer. The repository has a setup guide, architecture documentation, and a
working-surface inventory. It also says clearly that the hackathon version is a
proof of concept with local shared storage, public witnesses, and no economic
incentives yet.

**Why this is relevant to MemoryLineage:**

- a difficult primitive is explained through a human consequence;
- the repository contains several surfaces that reveal the same architecture;
- the README separates what exists from future work;
- the project creates a complete flow rather than only a contract.

**What to adopt:** a one-sentence incident, one coherent architecture, a direct
setup path, and visible limitations.  **What not to adopt:** more components just
to resemble a larger protocol. MemoryLineage already has enough protocol depth.

### Nani — User-centric Apps first prize

- Devpost: [Nani](https://devpost.com/software/nani-65mnxo)
- GitHub: [cenwadike/nani](https://github.com/cenwadike/nani)

Nani turns Polkadot event monitoring into a developer product with filtering,
multi-tenancy, notification plugins, encrypted tenant configuration, failover,
REST endpoints, and a documented quick start. Its README provides a five-minute
setup flow and a live API surface. Performance and scale numbers in the README
are project claims; they are useful as positioning evidence but should not be
treated as independently proven here.

**Why this is relevant:** the pain is immediately recognizable to a developer,
and the chain integration is embedded in a useful workflow rather than displayed
as a technical ornament.

**What to adopt:** one primary command, a visible operational result, a clean
developer path, and a clear boundary between tested behavior and roadmap claims.
For MemoryLineage, this means `cargo xtask verify`, a one-click local rehearsal,
and a portable report that another developer can inspect.

### Agora — Build a Blockchain first prize

- Devpost: [Agora](https://devpost.com/software/agora-bkfo60)
- GitHub: [suyash101101/Agora](https://github.com/suyash101101/Agora)

Agora presents a commit–reveal and XCM-based off-chain computation workflow with
workers, verification, and economic consequences. Its important lesson is the
distinction between a hash-linked report and proof that a claimed computation was
actually executed. A sequence of hashes can preserve report integrity while the
underlying report remains false.

**Why this matters for MemoryLineage:** it reinforces the project’s strongest
claim boundary. A valid transition proves an authorized, continuous commitment
under the registry rules. It does not prove that the private memory was true,
available, loaded by the agent, or semantically safe.

### PolkaShield — Web2 integration and operational completeness

- GitHub: [FredMunene/polkaShield](https://github.com/FredMunene/polkaShield)

PolkaShield connects a custom Polkadot chain to a gateway, dashboard, and Web2
editor portal. Its repository describes CI for the chain, gateway, dashboard, and
editor, and includes product requirements, architecture decisions, and a runbook.
Some features are still described as planned, so the README is not proof that
every listed capability is complete.

**Why this matters:** the strongest way to prove a blockchain primitive is often
to place it in the workflow of an existing application. MemoryLineage should
eventually offer a small, generic `preflight(snapshot)` adapter contract that a
host application can call before resuming an agent. It should not add a large
framework integration before the core decision is proven.

### OceanFin — simulate before action

- GitHub: [Tizun71/OceanFin](https://github.com/Tizun71/OceanFin)

OceanFin makes “simulate first, then execute” the center of a DeFi experience.
Its README explains a visual strategy builder, strategy library, simulation, and
one-click execution while keeping the wallet non-custodial.

**What to adopt:** MemoryLineage should make “rehearse before resume” the center
of the product. The recovery path should present a candidate snapshot, the
canonical head, the classification, and the next safe mode before an operator
loads the state into a real runtime.

### ANTS — simple human consequence

- Devpost: [ANTS — Authentic Video Chain](https://devpost.com/software/ants-authentic-video-chain)

ANTS takes a large problem—AI-generated video undermining evidence—and reduces it
to camera-only capture, a hash, on-chain metadata, and a verification action.

**What to adopt:** use an incident a judge can understand without knowing the
standard. For MemoryLineage, the incident is: “The operator restored an old
private snapshot. Is it still safe to resume as if it were current?”

## Relevant benchmark: Chainlink Convergence, 2026

The [2026 Chainlink Convergence winners page](https://chain.link/hackathon/winners)
lists more than 3,000 participants and 500+ projects. The current winner set is
especially relevant because it repeatedly turns trust into a pre-execution
decision, a receipt, or a portable validation result.

### AegisGate — binary privacy-preserving compliance

- Project: [AegisGate](https://chain.link/hack-26/projects/aegis-gate)
- GitHub: [Abbas-Dev-786/aegis-gate](https://github.com/Abbas-Dev-786/aegis-gate)

AegisGate reduces a complex KYC/AML/accreditation workflow to a small output:
whether the user satisfies a rule, without placing raw identity or financial data
on-chain. Its strength is the explicit table separating what is on-chain from
what is never published.

**MemoryLineage lesson:** every preflight result should state the input evidence,
the classification, and the data that remains private. A judge should not have
to infer the privacy boundary from architecture prose.

### SentinelCRE — a visible gate before execution

- Project: [SentinelCRE](https://chain.link/hackathon/winners/sentinel-cre)
- GitHub: [ProjectWaja/SentinelCRE](https://github.com/ProjectWaja/SentinelCRE)

SentinelCRE presents three layers of pre-execution risk assessment and records a
decision before an autonomous action. Its official project page describes the
three layers and explicitly says the participant descriptions are not an audit.

**MemoryLineage lesson:** show a decision point before a sensitive transition,
but keep the decision tied to the actual invariant. MemoryLineage should have a
resume gate for snapshot history, not a generic AI-risk score or a trading
firewall.

### CRE Risk Router — dry-run, broadcast, and receipts

- GitHub: [lancekrogers/cre-risk-router](https://github.com/lancekrogers/cre-risk-router)

CRE Risk Router separates `simulate` from `broadcast`, provides a one-command
demo, returns structured decision codes, and records immutable decision receipts.
The README also gives a clear quick start and lists the exact denial reasons.

**MemoryLineage lesson:** the website should make dry-run/rehearsal explicit and
never imply that a read-only simulation wrote to Ethereum. A Recovery Decision
Receipt should include the candidate snapshot commitment, observed head, evidence
source, classification, exact reason, and limitations.

### AgentScore — accountability with a concrete consequence

- Project: [AgentScore](https://chain.link/hack-26/projects/agentscore)
- GitHub: [agentscore-trustless/agent-score](https://github.com/agentscore-trustless/agent-score)

AgentScore frames low-quality autonomous work as an accountability problem and
uses service-level or audit concepts to make the output economically meaningful.

**MemoryLineage lesson:** “history integrity” becomes more useful when it produces
a decision that a host application can consume. The next product layer should
not be a reputation score; it should be a bounded recovery classification that a
runtime may accept or hold.

### Aegis Protocol V5 — enforcement boundary

- Project: [Aegis Protocol V5](https://chain.link/hackathon/winners/aegis-protocol-v5)
- GitHub: [vjb/aegis-v5](https://github.com/vjb/aegis-v5)

Aegis places a firewall before autonomous trading execution and distinguishes
wallet custody, session permissions, execution modules, and risk review.

**MemoryLineage lesson:** if we later claim “resume is blocked,” the code must
actually sit on the resume path. A dashboard warning is an audit result, not an
enforcement mechanism.

## Relevant benchmark: ETHGlobal 2025–2026

ETHGlobal submissions provide a useful additional signal: the strongest agent
projects describe a complete journey rather than a collection of standards.

- [Autonome](https://ethglobal.com/showcase/autonome-d8cxe) was an ETHGlobal New
  Delhi 2025 finalist and won a Polygon x402 agentic-payments prize. It combines
  identity, payments, and access in an agent-facing web workflow.
- [ACL](https://ethglobal.com/showcase/acl-6hwos) won 0G and ENS prizes by
  combining agent identity, private negotiation, work verification, independent
  evaluation, content-addressed evidence, and settlement. Its page emphasizes
  that the demos use real on-chain settlement and Merkle-rooted artifacts.
- [Contragent](https://ethglobal.com/showcase/contragent-8x0c9) connects verified
  inputs to an AI signal and an on-chain action, making the input → decision →
  action chain visible.

**MemoryLineage lesson:** the vertical slice should be:

```text
candidate private snapshot
        ↓
commitment and lineage check
        ↓
canonical / historical / diverged / unverified decision
        ↓
portable receipt
        ↓
host application chooses whether to resume
```

This is the correct analogue of input → decision → action without claiming a
causal relationship between memory and an external action that the current
protocol does not prove.

## Current standards and threat evidence

The [ERC-8350 draft](https://eips.ethereum.org/EIPS/eip-8350) validates the
technical center of MemoryLineage: private agent-memory transitions can be
represented as authorized fixed-width commitments, with sequence and prior-root
continuity enforced on a linear per-space registry. The draft explicitly says
that it does not prove the private witness is true or available, that the agent
used the memory, or that a decision was correct. MemoryLineage must attribute the
standard and pin the exact snapshot it implements; it must not claim to have
invented the primitive or to implement a final standard when the draft changes.

The [ERC-8004 Trustless Agents draft](https://eips.ethereum.org/EIPS/eip-8004)
addresses identity, reputation, and validation registries for agents. The
[ERC-8126 AI Agent Verification standard](https://eips.ethereum.org/EIPS/eip-8126)
adds multi-layer verification and risk-oriented interfaces. These make generic
“AI agent identity,” “agent reputation,” and “agent verification” crowded
positions. MemoryLineage should be complementary: it verifies the canonical
history of private state, not the agent’s identity, global reputation, or model
quality.

The [OWASP Agent Memory Guard](https://owasp.org/projects/agent-memory-guard)
describes a runtime defense that screens memory operations for poisoning. OWASP
also treats memory and context poisoning as an agent security surface. Recent
research reports persistent memory poisoning and delayed “sleeper” memory
attacks in evaluated agent systems, including
[PMPA](https://arxiv.org/abs/2609.13889) and
[Hidden in Memory](https://arxiv.org/abs/2605.15338).

These sources establish that persistent memory deserves security attention. They
do **not** prove that MemoryLineage detects semantic poisoning. A malicious
memory write that is correctly authorized and correctly appended can still have a
valid lineage. A harmless memory can still fail lineage if the operator restores
an old root or changes a signed locator. This distinction is a product strength
when stated plainly.

## The problem, made clearer and more urgent

### One-sentence problem

> **When a persistent AI agent is restored from a private backup, the operator can possess an old or altered snapshot while presenting it as the current state; a local database cannot tell an independent auditor whether that snapshot is the authorized continuation of the shared history.**

### Thirty-second explanation

A long-lived agent may keep its memory in SQLite, a vector store, or another
private database. After a crash, migration, provider change, or suspected
compromise, the operator restores a backup. That backup can be the current head,
a legitimate historical checkpoint, an altered state, or an incomplete state.

If the same operator controls the database, the signing process, and the only
audit log, it can simply report whatever history it wants. A signed log proves
that a record was signed; it does not by itself prove that the record is the
unique successor of a shared canonical head. An independent registry can enforce
the sequence and predecessor relation without publishing the memory contents.

MemoryLineage therefore gives the recovery process a narrow, falsifiable answer:

```text
CURRENT HEAD
KNOWN HISTORICAL CHECKPOINT
UNKNOWN / DIVERGED
UNVERIFIED
```

### Why the problem is urgent now

Persistent memory changes a one-session prompt-injection problem into a
cross-session state problem. OWASP’s memory work and 2026 academic studies show
that agents can carry attacker-controlled information forward and act on it in a
later session. That makes recovery, migration, and state continuity security
questions rather than merely database operations.

The honest connection is:

```text
memory security makes the state boundary important;
MemoryLineage makes the committed history boundary auditable.
```

MemoryLineage does not claim that it removes the semantic attack surface. It
adds a different control: an operator cannot silently present a stale committed
predecessor as the next canonical state under the registry rules.

### Why this is a real Web3 problem

The blockchain is justified only under a separated trust model:

```text
runtime operator ≠ controller/authorizer ≠ independent auditor
```

If one operator owns the runtime, storage, key, and audit function, a local signed
log may be enough. MemoryLineage is useful when separate parties need one shared
reference for ordering and authorization, such as provider migration, DAO or
protocol operations, an external auditor, or a controlled recovery process.

The public registry is not being used as a database for raw memory. It is being
used as a common commitment and succession anchor that the operator cannot
rewrite privately.

## Product evolution

### 1. Restore Preflight becomes the primary product

The Inspector should not ask the user to interpret a green hash. It should ask:

```text
Which snapshot are you about to resume?
What canonical history is it being compared against?
What evidence supports the result?
What is the safe next mode?
```

The current fixture-scoped implementation can already demonstrate a known
historical checkpoint and a stale-predecessor rejection. The UI must label this
as local Demo Space V2 evidence until the same flow is supported against a live
public space.

### 2. Recovery Decision Receipt

Add a portable, versioned report for each preflight decision. It should contain:

```text
candidate snapshot commitment
candidate sequence / claimed root, when available
registry address and network
space ID
observed canonical head
block number and block hash, when observed
classification
lineage checks
authority assurance level
evidence source
exact machine reason, if rejected
recommended next mode
privacy boundary
limitations
```

This is a decision receipt, not a claim that the memory content is public or
true. It should be independently verifiable by the Rust CLI and Rust/WASM
Inspector.

### 3. Explicit recovery modes

The product should expose these modes:

| Mode | Meaning | Current status |
| --- | --- | --- |
| `RESUME_CURRENT_HEAD` | Candidate matches the named current head under the required evidence policy. | Classification design exists; production runtime gate not demonstrated. |
| `REHEARSE_HISTORICAL_CHECKPOINT` | Candidate matches an earlier known checkpoint and is safe to inspect in an isolated rehearsal. | Demonstrable with the local fixture. |
| `RECONCILE_DIVERGED_STATE` | Candidate does not match the known history or conflicts with it. | Must remain a hold/review result. |
| `HOLD_UNVERIFIED` | Evidence, observation, authority proof, or schema support is insufficient. | Must fail closed for policies that require verification. |

The host application owns the policy. The Inspector reports evidence and does not
silently resume an external runtime.

### 4. A small adapter contract, later

Define a framework-neutral interface before adding a framework-specific adapter:

```text
preflight(candidate_snapshot, evidence_policy) -> RecoveryDecision
```

The adapter may refuse to load a snapshot when the policy requires a verified
head. It must record overrides and never convert a cryptographic mismatch into a
verified result.

This is the first meaningful way to move from “audit dashboard” toward “runtime
utility.” It is also the point where a prevention claim becomes valid. Until the
adapter is implemented and tested around an actual loader, use “assessment” and
“rehearsal,” not “prevents restore.”

### 5. Make proof levels visible

Do not collapse every check into one green badge. The report should show:

| Dimension | Valid status examples | Meaning |
| --- | --- | --- |
| Commitment computation | `MATCH` / `MISMATCH` | Candidate and profile produce the compared commitment. |
| Lineage replay | `PASS` / `REJECTED` | Sequence, predecessor, transition ID, and root relation are consistent. |
| Authority structure | `STRUCTURE_ONLY` | Addresses and nonce order are present; signatures are not proven. |
| Transition authorization | `VERIFIED` / `OBSERVED_ON_CHAIN` / `NOT_INCLUDED` | Depends on whether signatures, receipts, or historical signer state are actually available. |
| Registry observation | `OBSERVED` | Named RPC returned the recorded state at the named block context. |
| Finality context | `FINALIZED` / `SAFE` / `LATEST_PROVISIONAL` / `UNAVAILABLE` | Describes the observation assumption. |
| Runtime loading | `NOT_ATTESTED` | Registry evidence does not prove what the agent loaded. |
| Semantic safety | `OUT OF SCOPE` | No content or reasoning judgment was made. |

This research baseline was written before the signed Demo Space V2 extension.
The protocol-corpus projection still does not include per-transition signatures
and therefore remains `STRUCTURE_ONLY` / `NOT_INCLUDED`. The current Demo Space
V2 evidence is the scoped exception: it carries EIP-712 domain, digest, and
signature material for all three local transitions, and the independent Rust
verifier reports `EOA_SIGNATURES_VERIFIED`. This does not turn ERC-1271 into a
full historical offline proof or establish a production privacy profile.

## How MemoryLineage should exceed Forkline

Forkline is the strongest direct benchmark in the reviewed 3rd-Web-Hack set. Its
story is compact: a blockchain event can trigger an irreversible external effect,
then a reorg can make that event non-canonical. Its product, evidence, and demo
all describe the same failure.

MemoryLineage should not try to win by adding more features. It should exceed
Forkline on measurable dimensions while accepting that video and hosting are
separate work:

| Dimension | Forkline strength | MemoryLineage target | Current truth |
| --- | --- | --- | --- |
| Problem clarity | One irreversible delivery incident | One restore decision: current, historical, diverged, or unverified | The Restore Preflight wording is defined; comprehension must still be measured externally. |
| End-to-end coherence | Event → reorg → dispatch decision | Snapshot → commitment → registry history → recovery classification → receipt | Local Demo Space V2 is coherent; Sepolia remains a separate observation. |
| Failure realism | Rehearsed reorg/outbox behavior | Actual earlier snapshot root used in a stale-predecessor registry call | Proven in local Rust/revm; public Sepolia version of the unified demo is not demonstrated. |
| Evidence depth | Published source and test artifacts | Solidity, Rust core, independent Rust replay, revm, mutation corpus, SQLite fixture, browser evidence | Strong local evidence; each UI claim must continue naming its source. |
| Independent verification | Separate checks/readbacks | Portable receipt verified in browser/WASM and Rust CLI | Implemented for the signed Demo Space V2 scope; protocol-corpus and ERC-1271 boundaries remain explicit. |
| Privacy boundary | External side-effect safety story | Raw memory stays local; only fixed-size commitments and metadata are public | Protocol/evidence boundary is defined; fixture is synthetic and not a production privacy profile. |
| Authority model | Reorg/canonicality focus | Controller, authorizer, rotation, EOA/ERC-1271 lanes | Demo Space V2 proves its three EOA signatures independently; ERC-1271 remains execution-observation evidence. |
| Negative paths | Clear stale/reorg failure | Rollback, gap, parallel successor, wrong signer/domain, commitment tampering, semantic out-of-scope | Corpus and UI paths exist; maintain exact source labels. |
| Reproduction | Strong public quick path | One Cargo gate plus one local rehearsal and one verifier command | Local gate passes; external human reproduction is not yet demonstrated. |
| Product decision | Dispatch or hold | Resume current, rehearse historical, reconcile, or hold unverified | Decision model exists; runtime adapter is future work. |
| Public availability | Hosted public lab and video | Not counted in this decision because deployment/video are outside current scope | Do not claim superiority on those dimensions. |

The success condition is not “a bigger dashboard.” It is:

```text
one incident
one named evidence bundle
one exact failure
one portable receipt
one independent verifier
one honest boundary between assessment and enforcement
```

## What to adopt from the benchmark set

| Pattern | Adopt in MemoryLineage | Do not turn it into |
| --- | --- | --- |
| Forkline incident rehearsal | Restore Preflight and Recovery Rehearsal | An external delivery/outbox system |
| FinalityDesk exact mismatch checks | Named classifications, exact machine errors, source badges | A generic payment verifier |
| Fangorn modular protocol + UI + CLI | Shared evidence vocabulary across Rust core, CLI, and Inspector | A second network or encryption protocol |
| Nani developer product | Five-minute setup, visible API/CLI result, operational runbook | A hosted multi-tenant service |
| Agora proof boundary | Distinguish record integrity from truth of execution | A claim that commitments prove memory truth |
| PolkaShield integration | Small host adapter and CI/runbook | A new custom chain or Web2 gateway suite |
| OceanFin simulation | Rehearse before resume and show outcome before action | Wallet execution or DeFi strategy building |
| ANTS human story | Simple restore-and-check interaction | Absolute authenticity or erasure claims |
| AegisGate privacy table | On-chain vs local/private table | KYC or financial compliance product |
| CRE Risk Router receipts | Dry-run, structured result, portable decision receipt | Chainlink dependency without a product need |
| SentinelCRE pre-execution gate | Visible hold/allow boundary and attack scenarios | A semantic AI risk score |
| AgentScore accountability | Make a result consumable by a host policy | Generic reputation marketplace |
| ACL independent evaluator | Separate evidence producer from verifier | Full autonomous commerce platform |

## What must not be added before the core slice is stronger

Do not add these as substitutes for clarity:

- a generic AI-agent wallet firewall;
- semantic memory-poisoning detection;
- a reputation score or agent passport;
- token, NFT, DAO, marketplace, or DeFi features;
- ZK or TEE solely to appear more advanced;
- a second chain before the Ethereum vertical slice is externally reproducible;
- a large LangChain, LangGraph, MCP, or A2A integration;
- an action-causality claim that the current registry cannot prove.

The direct presence of a 2026 [agent-memory submission in the Chainlink gallery](https://chain.link/hack-26/projects/cl-26-agent-memory)
also means that “agent memory on-chain” is not enough differentiation. The
differentiator must be the recovery decision, stale-snapshot rehearsal, explicit
proof levels, and independently replayable evidence.

## Development goals

These goals are ordered by hackathon value and evidence risk.

### P0 — Make the vertical slice unambiguously real

1. **Unify the hero incident.** Keep the local Demo Space V2 fixture as one named
   incident: real synthetic SQLite snapshots, three committed transitions,
   authority rotation, and a fourth attempt using an actual earlier root.
2. **Make the recovery decision visible.** Add a Restore Preflight view model and
   Recovery Decision Receipt without claiming runtime prevention.
3. **Keep the evidence sources separate and labeled.** Demo Space V2, the four-
   transition protocol corpus, and existing Sepolia observation must not be
   merged into one misleading count.
4. **Preserve exact machine reasons.** `BAD_PREVIOUS_STATE` and
   `TRANSITION_ID_MISMATCH` remain visible alongside human explanations.
5. **Expose proof levels.** Keep `STRUCTURE_ONLY`, `NOT_INCLUDED`, `OBSERVED`,
   and `OUT OF SCOPE` visible where they apply.
6. **Keep one verification command truthful.** `cargo xtask verify` must run the
   checks described by the Reproduce page; do not document checks it skips.
7. **Add negative-path acceptance coverage.** Test current head, known historical
   checkpoint, unknown/diverged, unverified/malformed evidence, tampering, RPC
   unavailable, and semantic poisoning as `OUT OF SCOPE`.

### P1 — Increase independent verifiability

1. Extend the evidence format with per-transition event/receipt references and
   signed authorization material where the protocol can actually support it.
2. Reclassify authorization assurance only after the independent verifier can
   reproduce the applicable proof. Do not promote contract test results into
   historical evidence automatically.
3. Add a framework-neutral preflight adapter interface and one local CLI loader
   demonstration. The demonstration must clearly say which runtime it covers.
4. Record one clean-checkout reproduction from an external developer before
   calling the claim independently reproduced.
5. Make the browser and CLI receipt outputs byte-stable and cross-checkable.

### P2 — Explore only after P0/P1 are complete

1. Provider migration handoff: export a checkpoint proof from one operator and
   verify it before loading it at another provider.
2. Explicit historical rehearsal: allow a known old checkpoint in an isolated
   sandbox while refusing to call it the current head.
3. Explicit branch/merge semantics in a separate protocol extension or new
   space. The current linear registry must not silently acquire branch rules.
4. Optional ERC-8004 Validation Registry publication for a Recovery Decision
   Receipt. This is a compatibility experiment, not a new trust model.
5. Finality-aware observation and stronger light-client or multi-provider proof
   only if a concrete deployment requirement justifies the complexity.
6. A separate semantic screening adapter that can consume an OWASP-style memory
   safety verdict while preserving the distinction between semantic screening
   and lineage verification.

## Potential directions worth exploring later

### Provider migration and disaster recovery

This is the most natural expansion. A long-lived agent can move from one hosting
provider to another while an independent controller checks that the new runtime
starts from the right checkpoint. The protocol’s commitments and authority model
already fit this story. The missing work is a real adapter and richer evidence,
not a new chain.

### Recovery as a signed operational decision

The Recovery Decision Receipt could be signed by an operator, controller, or
auditor and retained alongside the local snapshot. This creates a portable audit
record of “why the agent was allowed to resume” without publishing memory.

This must remain an explicit application-layer record. A receipt does not prove
that the runtime obeyed it unless the loader enforces it.

### Separate content safety from history integrity

A future deployment could combine a content-screening layer with MemoryLineage:

```text
semantic memory policy → is this write acceptable?
MemoryLineage → is this state the authorized continuation?
```

The result should be two independent statuses, never one blended “safe” score.

### Agent identity and validation compatibility

ERC-8004 may provide an identity and validation surface for an agent, while
MemoryLineage provides state-history evidence. A future adapter could publish a
validation request or receipt reference, but it should not make ERC-8004 a core
dependency unless the integration provides a concrete user benefit.

### Stronger historical authorization evidence

The current largest technical evidence gap is not the hash chain. It is the
ability to prove the authorization of each historical transition independently
from a portable bundle, especially for ERC-1271. The next evidence revision
should distinguish:

```text
contract acceptance observed
EOA signature independently verified
ERC-1271 historical signer state re-executed
authority structure only
```

This is a more valuable investment than adding more attack names.

## Final public wording

### Preferred opening

> Persistent agents keep their memory in private storage. After a crash,
> migration, or rollback, an operator may restore an old snapshot and present it
> as current. MemoryLineage checks whether that snapshot is the authorized,
> canonical continuation of a shared history without publishing the memory itself.

### Preferred solution statement

> MemoryLineage is a Rust-based Restore Preflight and independent auditor for
> private agent-memory history. It compares a candidate snapshot commitment with
> a replay-verified registry history, rehearses stale-predecessor recovery, and
> exports a portable decision receipt that another verifier can check.

### Preferred Web3 explanation

> A local signed log may be enough when one operator is the only trust domain.
> Ethereum becomes useful when the runtime operator, transition authority, and
> auditor are different parties that need one shared ordering and authorization
> reference.

### Explicit boundary

> MemoryLineage proves continuity and configured authorization of committed state
> history. It does not prove that the private memory is true or safe, that an
> agent loaded it, or that the agent’s reasoning or later actions were correct.

## Claim matrix for the evolved concept

| Claim | Evidence needed | Current status |
| --- | --- | --- |
| A stale earlier root is rejected by the linear registry | Solidity + Rust/revm Demo Space V2 | **VERIFIED locally** |
| The stale root came from a real fixture snapshot | SQLite fixture manifest and deterministic read | **VERIFIED locally** |
| The browser can explain the recovery classification | Inspector acceptance flow | **IMPLEMENTED locally; external comprehension not yet demonstrated** |
| The browser result came from a live chain | Named RPC, pinned block identity, real `eth_call` | **Only for the separate Sepolia probe; do not merge with Demo Space V2** |
| Every historical transition’s signature is independently proven | Per-transition signatures/receipts and verifier support | **NOT INCLUDED in current V1/V2 bundle** |
| Raw memory is not required in registry calldata | Solidity interface and evidence schema | **VERIFIED within the protocol boundary** |
| Memory content is semantically safe | Semantic detector/evidence | **OUT OF SCOPE** |
| Agent actually loaded the committed memory | Runtime attestation/adapter | **NOT DEMONSTRATED** |
| Agent’s action was caused by the memory state | Action binding and causal evidence | **OUT OF SCOPE** |
| External clean-checkout reproduction | Independent developer report | **NOT YET DEMONSTRATED** |
| Formal security audit | Third-party audit report | **NOT CLAIMED** |

## Research conclusion

The strongest development path is not a new protocol and not a generic AI-agent
security product. It is a sharper operational product built on the current
protocol:

```text
private snapshot
      ↓
Restore Preflight
      ↓
canonical / historical / diverged / unverified
      ↓
Recovery Rehearsal
      ↓
exact contract result
      ↓
portable Recovery Decision Receipt
      ↓
independent Rust verification
```

This path borrows the best properties of the reviewed winners:

- Forkline’s one-incident clarity;
- FinalityDesk’s precise verifier language;
- Fangorn’s complete protocol/UI/CLI packaging;
- Nani’s developer path and operational documentation;
- Agora’s distinction between record integrity and truth;
- PolkaShield’s application integration and CI discipline;
- OceanFin’s simulate-before-action interaction;
- Chainlink winners’ pre-execution decision and receipt pattern;
- ACL’s independent evaluator and portable evidence model.

It keeps MemoryLineage’s own differentiator: a private-agent state history can
be checked as an ordered, authorized, replayable canonical succession without
placing raw memory on-chain.

That is a materially better problem statement and product than “we store AI
memory hashes on Ethereum.” It is still narrow enough to finish, and every
important claim can be made falsifiable.

## Source register

### 3rd-Web-Hack and direct competitor

- [3rd-Web-Hack official page](https://3rd-web-hack.devpost.com/)
- [Forkline: Rehearse the Rollback](https://devpost.com/software/forkline-rehearse-the-rollback)
- [MemoryLineage public competitor audit](./AUDIT_SUBMISSION_3RD_WEB_HACK_2026-09-18.md)

### Polkadot 2025

- [Build Resilient Apps with Polkadot Cloud](https://polkadot.devpost.com/)
- [Official Polkadot winners update](https://polkadot.devpost.com/updates/40528-winners-announced-and-what-s-next)
- [Polkadot project gallery](https://polkadot.devpost.com/project-gallery)
- [Fangorn Devpost](https://devpost.com/software/fangorn)
- [Fangorn GitHub](https://github.com/driemworks/fangorn)
- [Nani Devpost](https://devpost.com/software/nani-65mnxo)
- [Nani GitHub](https://github.com/cenwadike/nani)
- [Agora Devpost](https://devpost.com/software/agora-bkfo60)
- [Agora GitHub](https://github.com/suyash101101/Agora)
- [ANTS Devpost](https://devpost.com/software/ants-authentic-video-chain)
- [PolkaShield GitHub](https://github.com/FredMunene/polkaShield)
- [OceanFin GitHub](https://github.com/Tizun71/OceanFin)
- [Sovseal Devpost](https://devpost.com/software/futureproof-pes1vn)

### Chainlink Convergence 2026

- [Chainlink hackathon overview](https://chain.link/hackathon)
- [Chainlink winners](https://chain.link/hackathon/winners)
- [Chainlink project gallery](https://chain.link/hack-26)
- [AegisGate](https://chain.link/hack-26/projects/aegis-gate)
- [AegisGate GitHub](https://github.com/Abbas-Dev-786/aegis-gate)
- [SentinelCRE](https://chain.link/hackathon/winners/sentinel-cre)
- [SentinelCRE GitHub](https://github.com/ProjectWaja/SentinelCRE)
- [CRE Risk Router GitHub](https://github.com/lancekrogers/cre-risk-router)
- [AgentScore](https://chain.link/hack-26/projects/agentscore)
- [AgentScore GitHub](https://github.com/agentscore-trustless/agent-score)
- [Aegis Protocol V5](https://chain.link/hackathon/winners/aegis-protocol-v5)
- [Aegis Protocol V5 GitHub](https://github.com/vjb/aegis-v5)
- [Agent memory gallery entry](https://chain.link/hack-26/projects/cl-26-agent-memory)

### ETHGlobal 2025–2026

- [Autonome](https://ethglobal.com/showcase/autonome-d8cxe)
- [ACL](https://ethglobal.com/showcase/acl-6hwos)
- [Contragent](https://ethglobal.com/showcase/contragent-8x0c9)
- [ETHGlobal project gallery](https://ethglobal.com/showcase)

### Standards and security research

- [ERC-8350: Agent Memory State Registry](https://eips.ethereum.org/EIPS/eip-8350)
- [ERC-8004: Trustless Agents](https://eips.ethereum.org/EIPS/eip-8004)
- [ERC-8126: AI Agent Verification](https://eips.ethereum.org/EIPS/eip-8126)
- [OWASP Agent Memory Guard](https://owasp.org/projects/agent-memory-guard)
- [OWASP Agentic AI threats and mitigations](https://genai.owasp.org/resource/agentic-ai-threats-and-mitigations/)
- [PMPA: persistent memory poisoning](https://arxiv.org/abs/2609.13889)
- [Hidden in Memory: sleeper memory poisoning](https://arxiv.org/abs/2605.15338)
