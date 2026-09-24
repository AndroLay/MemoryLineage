# MemoryLineage: Restore Preflight & Recovery Rehearsal

Status: product direction approved for development; runtime integration and production commitment profile are not yet demonstrated.

## Product goal

Make it possible to answer one operational question before a persistent AI agent resumes from a restored snapshot:

> **Does this snapshot match the current committed history, match a known historical checkpoint, diverge from the history, or lack enough evidence to decide?**

MemoryLineage remains an independent auditor for private AI-agent memory history. The product extension is a **Restore Preflight** followed by an optional **Recovery Rehearsal**. It does not replace the registry, detect malicious meaning in memory, or become a general agent safety platform.

The tagline remains:

> **Verify the history, not the memory.**

## The problem in concrete terms

An agent operator restores a private backup after a crash, migration, or suspected compromise. The restored database may be the current state, a legitimate older checkpoint, an altered state, or a state whose history cannot be checked. The operator's local database alone cannot establish which case applies to an independent controller or auditor.

This is a state-continuity and recovery decision. It is related to memory security, but it is not the same as semantic memory poisoning. OWASP treats memory and context poisoning as an agent security risk, and OWASP Agent Memory Guard addresses content and operation screening at the memory-store boundary. MemoryLineage addresses whether a snapshot's commitment belongs to a known, authorized, ordered history. The tools can complement each other; neither substitutes for the other's check.

Research on persistent-memory poisoning demonstrates attack feasibility in evaluated harness settings. It does not establish a universal production incident rate. MemoryLineage must describe this as a security-relevant failure mode, not invent prevalence, losses, or a claim that lineage checks remove poisoning risk.

## Target user and trust condition

The first credible user is a developer or operator of a long-lived agent whose runtime is hosted or managed by one party while another party controls policy, approves state transitions, or audits recovery. Examples include a DAO or protocol team delegating an agent runtime to an operator while a controller or auditor retains independent authority over the agent's committed state history.

The blockchain trust benefit depends on this separation:

```text
runtime operator ≠ transition authorizer/controller ≠ independent auditor
```

If one operator controls the runtime, local database, signing authority, and only verifier, an append-only local log may be sufficient. MemoryLineage should say so. A public registry is justified when multiple parties need a shared canonical ordering and the agent operator must not be the only party trusted to preserve it.

## Product definition

**Restore Preflight** evaluates a candidate snapshot against a specifically identified, independently replayed lineage and an explicitly labeled chain observation. It returns a state classification and the evidence behind that decision. It is an assessment, not a claim that a third-party agent runtime has been blocked.

**Recovery Rehearsal** lets an operator inspect or replay recovery from a known historical checkpoint in an isolated, non-production path. It makes the attempted predecessor, expected canonical head, authorization evidence, chain observation, and exact result visible before any production continuation. A rehearsal must not broadcast a transaction or perform external side effects.

**Runtime Resume Gate** is a later adapter capability. It can be claimed only after an actual agent-framework integration demonstrates that the runtime checks the assessment before loading state and fails closed when policy requires it. A web page or local adapter that the operator can bypass is not a universal prevention mechanism.

## The concept in one operating decision

The product should not make an auditor infer what a green hash means. It should make the recovery choice explicit:

```text
I am about to resume this agent from a restored snapshot.
Is it the current checkpoint, a known older checkpoint, divergent, or unverified?
What evidence produced that classification, and what is safe to do next?
```

The answer is an evidence-backed classification plus a bounded next step:

| Result | Product action | What the result does not mean |
| --- | --- | --- |
| `MATCHES DEMO EVIDENCE HEAD` | Show the current head match for the named fixture/bundle. | It is not a live registry read, a production commitment check, or permission to resume an external runtime. |
| `KNOWN HISTORICAL CHECKPOINT` | Offer isolated Recovery Rehearsal and show the later committed head. | It is not automatically malicious and is not the current canonical continuation. |
| `UNKNOWN / DIVERGED` | Preserve the normal path as unresolved and show the commitment mismatch. | It does not identify who changed the snapshot or why. |
| `UNVERIFIED` | Show the failed/missing check and require another evidence source. | It is not a rejection of the underlying memory's meaning. |

The current UI implements only the first, fixture-scoped version of this decision. It selects one of the public synthetic SQLite checkpoints, compares its existing commitment with the replay-verified local Demo Space V2 bundle, and can tamper a browser copy. It does not recompute a production private-memory commitment, read the local SQLite database from the browser, use the optional Sepolia probe to classify the local fixture, or block agent startup. The separate read-only probe observes the previously deployed registry and is labeled independently; it does not turn the fixture into a production or public Demo Space V2 history. Those limits must be visible near the result, not hidden in a separate technical document.

## Why this is a real Web3 problem, and when it is not

The real problem is disagreement across trust domains, not the mere fact that an AI agent has memory. A local database, signed file, or append-only log may be the correct answer when one operator owns the runtime, storage, signing key, and audit function. A shared registry becomes useful when an independent controller or auditor needs to establish which ordered state transitions the operator was authorized to present. The registry supplies a common ordering/authorization reference; it does not make the operator honest, keep off-chain data available, or prove what the model loaded.

The relevant risk evidence is deliberately narrow. OWASP identifies memory/context poisoning as an agent attack surface, and the Agent Memory Guard project describes screening memory-store operations. A September 2026 preprint reports persistent-memory poisoning in evaluated agent-harness settings. These support the relevance of treating persistent state as security-sensitive; they do not establish market demand for MemoryLineage, production prevalence, or that lineage commitments detect malicious semantics. MemoryLineage and content-screening tools answer different questions and can be composed.

The product's decision rule is therefore:

```text
Use MemoryLineage when independent parties need a shared, replayable reference
for committed state succession. Do not add a chain if a single trusted operator
and a local signed log already satisfy the threat model.
```

## Primary user flow

```text
Select restored snapshot
        ↓
Recompute its commitment using a named profile
        ↓
Replay the supplied canonical history independently
        ↓
Observe the registry at a pinned block context
        ↓
Check authority evidence and its assurance level
        ↓
Classify the snapshot
        ↓
Resume current / recover to head / rehearse only / hold for review
```

The browser may read a public RPC and bundled or imported evidence. The portable report records its evidence source and limitations. It never treats the dashboard's own label as proof.

## Required snapshot classifications

| Classification | Meaning | Safe product guidance |
| --- | --- | --- |
| `CANONICAL_HEAD` | The candidate snapshot commitment matches the head of the identified, replay-verified history, and the required observation policy is satisfied. This is still an observation under stated RPC and finality assumptions. | Eligible for the configured resume policy. Do not imply semantic safety. |
| `KNOWN_HISTORICAL_CHECKPOINT` | The candidate matches an earlier checkpoint in the verified history, but not its head. | Rehearsal or explicit recovery only. Do not silently label it current or append it as the next state in the same linear space. |
| `UNKNOWN_OR_DIVERGED` | The candidate commitment does not match any checkpoint in the verified history, or its claimed sequence/root conflicts with the replay. | Hold the normal resume path and investigate, re-import, or migrate under a separately authorized process. |
| `UNVERIFIED` | The bundle is malformed/unsupported, replay fails, authority evidence is insufficient for the selected policy, the observation is unavailable, or the chain/finality context cannot be established. | Fail closed for any policy that requires a verified head; preserve a clearly labeled manual recovery path outside MemoryLineage's guarantee. |

These classifications describe evidence state. They do not prove the contents are true, safe, available, or faithfully loaded by a runtime.

### Historical checkpoint semantics

A historical checkpoint can be a legitimate recovery artifact. It is not automatically an attack. The current linear ERC-8350-style registry has no general branch/merge semantics: a transition from an old predecessor cannot silently replace the current head. A historical state may be replayed in a sandbox, or its missing canonical deltas may be reapplied if those private deltas are available and policy permits. If the exact head cannot be reconstructed, the product reports `UNVERIFIED` or `UNKNOWN_OR_DIVERGED`; it must not make up a successful recovery.

## What is verified, and at which layer

Avoid a single green badge that collapses distinct assurances. The report should expose each dimension separately:

| Evidence dimension | Example status | What it establishes |
| --- | --- | --- |
| Commitment computation | `MATCH` / `MISMATCH` | The supplied snapshot and local commitment profile produce the compared commitment. |
| Lineage replay | `PASS` / `REJECTED` | Sequence, predecessor, transition identifier, and derived state root are internally consistent for the supplied history. |
| Authority record structure | `STRUCTURE_ONLY` | Addresses and nonce ordering are structurally plausible. This does not verify signatures or prove who authorized a historical transition. |
| Authority timeline binding | `TIMELINE_BOUND` | Signed Demo Space V2 proofs are matched to explicit effective sequence/config-nonce windows inside the published bundle; this is not a public-chain consensus proof. |
| Transition authorization | `VERIFIED`, `OBSERVED_ON_CHAIN`, or `NOT_INCLUDED` | Use `VERIFIED` only when the evidence contains enough signed data to check the applicable EOA signature or a separately supported contract-account proof. |
| Registry observation | `OBSERVED` with provider, chain, block number and hash | A named RPC endpoint returned the recorded data. An RPC response is not a consensus proof. |
| Finality context | `FINALIZED`, `SAFE`, `LATEST_PROVISIONAL`, or `UNAVAILABLE` | Describes the selected Ethereum block tag and its assumptions. `latest` may be reorged. |
| Runtime loading | `NOT_ATTESTED` unless a runtime adapter provides evidence | A registry does not prove which bytes or memory the agent actually loaded. |
| Semantic safety/truth | `OUT_OF_SCOPE` | Lineage checks do not evaluate natural-language meaning or model behavior. |

The protocol-corpus V2 projection does not carry per-transition signatures or
event/receipt references sufficient for independent historical authorization
verification. Therefore that projection reports authority history
`STRUCTURE_ONLY` and authorization proof `NOT_INCLUDED`. Demo Space V2 is the
explicit signed fixture: it carries EIP-712 EOA proof material for every local
transition, and the independent verifier reports `EOA_SIGNATURES_VERIFIED` /
`TIMELINE_BOUND` after matching each proof to the active authority window.
ERC-1271 remains execution evidence and must not be silently promoted into
historical offline proof.

For ERC-1271, `ON-CHAIN ACCEPTANCE OBSERVED` is distinct from replaying the signer contract against historical state offline. The latter requires the relevant signer code and historical state, or an equivalent verifiable proof; a JSON assertion alone is insufficient.

## Commitment and privacy requirements

The public demo fixture is synthetic and source-visible. Its existing unsalted `snapshot_commitment` is explicitly a fixture commitment and must not be presented as a production privacy profile.

A production adapter needs a versioned canonicalization profile and a high-entropy secret blinding value (or a separately reviewed hiding commitment). It must use unambiguous length-prefixed or otherwise canonical encoding, domain separation, and space/profile context. The blinding value and raw memory remain local and must never enter the evidence bundle, URL, logs, analytics, chain calldata, or screenshots. The client can locally recompute the commitment before resume; an independent verifier that does not receive the memory or blinding value can still verify the public lineage, but cannot claim it independently reconstructed the private snapshot.

The report should distinguish:

```text
PUBLIC / ON CHAIN
  sequence, previous root, transition ID, state root, fixed-size commitments,
  authorization metadata and the applicable registry observation

PRIVATE / LOCAL
  raw memory, private documents, locator contents, prompt/context, secrets,
  and any blinding value needed to recompute the private snapshot commitment
```

Commitments can leak equality or metadata if profiles are poorly designed. “Raw memory is not required on-chain” is accurate; “absolute privacy” is not.

## Ethereum observation and reorganization handling

Every live read must name the RPC source and a single pinned block context. Prefer resolving `finalized`, then `safe` if policy permits and the endpoint supports it; record the returned block number and hash. Read the registry head and run the read-only stale-predecessor `eth_call` at that same block number (or a block-hash selector when supported). Confirm the observed block identity before showing a result. If the required tag is unavailable or the block identity changes, return `UNVERIFIED`/`LIVE RPC UNAVAILABLE`, not a green fallback.

`latest` is a provisional head and may be reorged. Multiple RPC observations that match may be labeled `OBSERVATIONS MATCH`; they do not constitute consensus proof. A finalized/safe observation still depends on the execution/consensus clients and endpoint implementation.

## Recovery policy model

MemoryLineage reports evidence. A host application owns policy. Suggested default policy for a future runtime adapter:

```text
CANONICAL_HEAD               → continue only if required authority/finality checks pass
KNOWN_HISTORICAL_CHECKPOINT  → do not resume production; allow isolated rehearsal
UNKNOWN_OR_DIVERGED          → stop normal resume and request reconciliation
UNVERIFIED                   → fail closed for protected workflows
```

The adapter must expose an explicit override decision and audit it. It must never convert a historical checkpoint into a canonical successor by changing the expected root locally. No policy override can turn a failed cryptographic check into `VERIFIED`.

## Threat model

The product aims to detect or make auditable:

- restoring a previously committed snapshot and presenting it as the current head;
- skipping sequence values or presenting a stale predecessor;
- proposing a conflicting successor under a linear registry;
- changing a signed commitment field, including locator commitment;
- replaying a transition under the wrong signer, chain, or verifying contract domain;
- presenting incomplete, malformed, unsupported, or conflicting evidence as verified;
- confusing a provisional RPC observation with a finalized state.

Assumptions and limits:

- the enforcing contract code and address are identified correctly and are not upgradeable in a way that can replace the relevant rules;
- chain observations come from named RPC endpoints and are not a light-client proof unless a separate verifier actually supplies that proof;
- the configured controller/authorizer keys and ERC-1271 policies are governed correctly;
- commitments are computed under a pinned, canonical profile;
- the runtime adapter actually gates loading if prevention is claimed;
- private memory availability, meaning, integrity before commitment, model reasoning, and downstream actions are outside the registry's guarantee.

## How the product is distinct

| System or pattern | Its core question | MemoryLineage relationship |
| --- | --- | --- |
| Forkline | Is this blockchain event still canonical immediately before an irreversible off-chain delivery? | Strong adjacent pattern for reorg-aware preflight, one coherent failure demo, and honest limits. MemoryLineage asks whether an agent snapshot is the authorized continuation of committed memory history before recovery/resume. It must not copy Forkline's outbox product. |
| FinalityDesk | Did the expected payment event, recipient, amount, token, and finality actually match? | Inspiration for exact semantic checks, narrow results, and clear mismatch reasons. |
| OWASP Agent Memory Guard | Is a memory operation/content unsafe under screening and policy? | Complementary content/runtime defense. MemoryLineage does not scan semantics. |
| LangGraph checkpoint/time-travel | Can a developer inspect a checkpoint and replay from it to explore an alternate path? | Legitimate development capability. MemoryLineage should preserve that workflow as explicit rehearsal while preventing an older checkpoint from masquerading as the shared canonical head. |
| Local signed/append-only logs | Can one operator retain a tamper-evident record? | Sufficient when that operator is the only trust domain. A public registry adds value only when independent parties need a shared order/authority reference. |
| Kinetic and proof-of-compute projects | Was a claimed computation actually executed? | Reminder that a hash-linked report is not proof of execution. MemoryLineage proves commitment lineage, not memory semantics or runtime loading. |

## Competitive plan: exceed Forkline on evidence, not feature count

Forkline is the strongest product-presentation benchmark in the reviewed set: a reorg can invalidate a blockchain event after it has triggered an irreversible off-chain delivery, so the system rehearses the failure and checks canonicality before dispatch. Its public story connects one Web3 failure, a runnable implementation, and an observable result. MemoryLineage should learn from that cohesion without copying its outbox/side-effect product. The two projects protect different boundaries:

```text
Forkline: chain event → reorg risk → external delivery
MemoryLineage: private snapshot restore → stale lineage risk → agent-state recovery decision
```

The benchmark is a set of observable release qualities, not a promised competition result. “Beat Forkline” means exceeding it on the dimensions MemoryLineage can control and measure. It does not mean claiming a higher jury score or winning rank. A static Inspector was recorded at the public URL; this workflow did not deploy the candidate and the current Pages content was not independently verified. External adoption remains unproven.

| Dimension | Forkline lesson / observed bar | MemoryLineage's current evidence | Required superiority gate |
| --- | --- | --- | --- |
| Problem clarity | One short incident explains why a blockchain reorg can make an already-triggered external side effect unsafe. | Silent Rollback now has one operator action: restore an old private snapshot, then attempt to extend the later recorded head. | In an unbriefed walkthrough, a developer can state the restore mismatch and outcome after seeing the Inspect/Lab flow, without first learning ERC-8350 terminology. Record wording; do not substitute an internal score. |
| End-to-end coherence | The same reorg/dispatch incident is carried through the product story and reproduction path. | The local Demo Space V2 bundle connects three synthetic SQLite-derived commitments, three Solidity transitions, a rotation, and transition 4 using the actual earlier root. Sepolia remains separate. | Home, Inspect, Lab, Verify, README, and CLI identify the same local incident and same evidence file. Keep Sepolia labeled as a separate observation. |
| Web3 necessity | Canonicality is relevant because the source event is on a reorgable blockchain and the external action cannot be reverted. | A linear shared registry is relevant only when operator and authority/auditor are separate; it establishes ordered commitments and configured transition rules. | State that trust-role condition in product copy. Explain when a local signed log is enough. No generic “blockchain makes it secure” language. |
| Human-readable result | A rehearsal shows the failure before a real side effect is emitted. | The Lab replays a published Solidity/revm attack record and shows exact `BAD_PREVIOUS_STATE`; the optional Sepolia probe is a distinct read-only call. | Always show source class (`LOCAL EVIDENCE`, `LIVE RPC`, or unavailable), exact machine reason, and why it occurred. Never present a bundle replay as a newly executed browser EVM call. |
| Verification depth | Runnable source and evidence make the behavior reviewable. | Pinned vectors, independent Rust replay, Solidity/revm lanes, SQLite fixture, and a separate mutation corpus exist. | Every UI claim points to a named artifact or command. Keep separate the 3-transition demo, 4-transition protocol corpus, and Sepolia space. |
| Authorization assurance | Be explicit about which operation is actually checked. | Protocol-corpus history remains structural; Demo Space V2 includes EIP-712 EOA proof material and signer recovery. | Show `EOA_SIGNATURES_VERIFIED` only for the signed Demo Space V2 bundle. Keep protocol corpus `STRUCTURE_ONLY` / `NOT_INCLUDED`, and keep ERC-1271 at execution-observation level. |
| Negative paths | Demonstrate the error boundary instead of only a happy-path dashboard. | Restore assessment covers evidence-head match, known historical checkpoint, unknown/diverged, and unverified; attack corpus covers named mutations. | Smoke-test all four preflight classes, bad evidence, unknown scenario, RPC unavailable, unexpected result, and exact tamper failure. Semantic poisoning remains `OUT OF SCOPE`, not “rejected.” |
| Reproducibility | A clean setup and direct test path reduce reviewer effort. | The repository has a Cargo verification gate, static WASM build, and Chromium smoke command. | Keep one documented primary gate; run it from a clean worktree and preserve the output. Do not claim an external reproduction until an external person runs it. |
| Visual and interaction design | A visible rehearsal makes an abstract chain condition legible. | The Dioxus Inspector has 11 evidence-workspace routes and a Restore Preflight selector. | Preserve target design hierarchy, keyboard operation, status text/icons, 1440px/390px layout, and browser smoke. Avoid adding extra dashboard metrics or generic card grids. |
| Public availability | A live public surface lowers the friction to try the competitor. | The last recorded static site is at [memorylineage.pages.dev](https://memorylineage.pages.dev); this workflow did not update it, and current content was not rechecked. There is no separate staging environment, and Demo Space V2 remains local. | Identify the last recorded hosted version accurately; do not imply a staging environment, a Sepolia Demo Space V2 deployment, or production agent integration. |
| External validation | Public artifacts are not a substitute for an independent person following the setup. | External human clean-checkout reproduction is not yet demonstrated. | Keep it `NOT YET DEMONSTRATED` until recorded. Ask the tester to describe what is and is not proven; this tests comprehension, not security certification. |
| Submission media | Video improves judge access to an incident. | An eight-slide PDF and 45-second local demo are reproducible from source commit `675c707`; neither was uploaded to Devpost. | Do not claim superiority on video/pitch media without judge evidence. Keep the local artifact and hosted site status separate. |

No one should convert this table into fabricated numeric “dominance scores.” It is a release checklist. The final public comparison should cite public source/repository evidence and keep the known bounds of each project; it cannot guarantee the judges' interpretation.

## Lessons from the reviewed projects

| Reviewed project or pattern | Useful lesson to adopt | Boundary to preserve |
| --- | --- | --- |
| [Forkline](https://devpost.com/software/forkline-rehearse-the-rollback) | Lead with the incident, give the user a rehearsal, connect source/tests/evidence, and state limitations. | Do not copy its event outbox, delivery queue, or irreversible side-effect claim; MemoryLineage has no causal action binding. |
| [FinalityDesk](https://devpost.com/software/finalitydesk) | Make the verifier answer one exact question and enumerate mismatch conditions precisely. | Do not imply an RPC observation is consensus proof; do not broaden into general payment verification. |
| Kinetic | Ambition and a clear market story can attract attention. | Hash-linked status reports do not prove real compute execution; MemoryLineage must not inflate a commitment into truth or execution proof. |
| ArcLight AI | Strong visual hierarchy and a polished first view improve comprehension. | Visual completeness cannot replace Web3 evidence; all displayed counts and states must come from real artifacts. |
| DEDSEC Shadow NET | A concrete, memorable action is easier to explain than abstract infrastructure. | “Deleted from memory” is not a secure-erasure guarantee; MemoryLineage makes no such claim. |
| Rehearsal / checkpoint time-travel patterns | Inspect, replay, and test from a prior checkpoint is a legitimate developer workflow. | Historical branches belong in isolated rehearsal unless an explicit branch/merge protocol exists; the current registry is linear. |
| OWASP Agent Memory Guard | Memory-operation/content screening is a complementary layer for unsafe writes. | It does not establish canonical cross-operator ordering; MemoryLineage does not inspect semantic safety. |
| ERC-8350 draft/reference work | Published semantics and vectors enable a concrete independent conformance target. | The standard predates this hackathon and remains a draft snapshot; attribute it and pin the version used. |

These are design inputs from the reviewed source/repository set, not claims that MemoryLineage has reproduced every competitor feature. The most valuable combination is: Forkline's incident clarity, FinalityDesk's exact checks, checkpoint tools' honest rehearsal model, and the existing independent replay evidence—under a single strict security boundary.

Forkline is the benchmark, not a feature checklist. MemoryLineage's route to a stronger submission is a single coherent snapshot-restore story plus more explicit evidence assurance, a portable independent report, and clear privacy/security boundaries. It should aim to exceed Forkline on measurable, controllable dimensions: scenario clarity, evidence-to-claim traceability, negative-path coverage, independent replay, and reproducibility. Hosting the static website does not prove adoption or guarantee a judge's ranking; the Demo Space V2 history remains local rather than deployed to Sepolia.

## Current repository boundary

Current behavior is a deterministic synthetic SQLite fixture with three snapshots, a local Rust/revm execution of the Solidity registry, and a separate existing Sepolia deployment/read observation. The local Demo Space V2 history is not deployed to Sepolia. The fixture's restore and `BAD_PREVIOUS_STATE` outcome are reproducible locally; they do not demonstrate an adapter intercepting a real agent runtime.

The original first implementation slice is now complete. Its delivered
behavior is recorded here for historical context:

1. make the independent verifier's authorization status truthful (`STRUCTURE_ONLY` / `NOT_INCLUDED` for evidence that lacks signatures, and `EOA_SIGNATURES_VERIFIED` for the signed Demo Space V2 bundle);
2. add a deterministic restore assessment against the replay-verified Demo Space V2 history, with explicit demo-evidence wording and no runtime-gate claim;
3. make the optional browser Sepolia observation resolve and pin one safe/finalized block context for its head read and stale-predecessor `eth_call`, and expose the observed block identity;
4. preserve the Solidity contract, current evidence files, conformance vectors, and exact `BAD_PREVIOUS_STATE` behavior;
5. bind a Recovery Decision Receipt to the evidence source class, policy identifier, bundle hash, and verified authorization assurance.

The public static site is published separately. This work does not add an agent framework integration or staging environment, deploy a new space/contract, record a demo video, or claim production readiness.

## Roadmap and measurable goals

### Goal A — Honest evidence semantics (current)

- The verifier differentiates transition/lineage replay from cryptographic authorization proof.
- Existing V1/V2 bundles remain readable; unsupported or malformed input fails closed.
- The report never says authorization `PASS` when it only checks address shape and nonce order; the signed Demo Space V2 report recovers each declared EOA signer independently.

### Goal B — Restore Preflight prototype (current)

- The Inspect surface derives the selected fixture snapshot's class from actual SQLite-derived commitments and the independently replayed V2 bundle.
- Tests cover current evidence head, known historical checkpoint, unknown/diverged commitment, and invalid evidence.
- UI calls this a synthetic fixture/evidence assessment and states that no external runtime is gated.

### Goal C — Coherent chain observation (current)

- Head read and rollback `eth_call` use one resolved block context.
- The output includes provider label, block tag/finality class, number, and hash, or explicitly reports unavailable/unverified.
- No transaction is broadcast; no claim of consensus proof is made.

### Goal D — Production adapter and privacy profile (future; requires a separate reviewed implementation)

- A real agent restore hook computes a versioned hiding commitment and evaluates policy before loading state.
- Integration tests prove normal resume, historical checkpoint rehearsal-only behavior, unknown/diverged denial, and RPC/evidence failure behavior.
- The commitment profile is reviewed for canonicalization ambiguity, low-entropy disclosure, nonce handling, domain separation, and secret leakage.

### Goal E — Submission strength against Forkline (evaluation target, not outcome promise)

- A first-time user can explain the concrete restore failure after seeing one screen and one interaction.
- Every positive and negative status links to evidence and names its source.
- A clean checkout can run the documented Rust verification gate and independently replay the public bundle.
- Competitor comparisons remain factual and do not claim the Devpost result or rubric score in advance.
- The product never claims superiority on unbuilt/publicly unavailable surfaces such as hosting, external reproduction, or demo media.

### Goal F — Restore decision clarity (product-quality target)

- The Inspector explains the four candidate classes in plain language and names the evidence source next to each result.
- Selecting the known older checkpoint routes to isolated Recovery Rehearsal; it is not labeled malicious or current.
- Browser tampering changes only a local copy and shows the exact changed value and resulting classification.
- A first-time walkthrough distinguishes “local Demo Space V2 evidence” from the separate Sepolia read within the same session.

### Goal G — Evidence assurance parity (release gate for scoped authorization claims)

- Protocol-corpus V1/V2 projections continue to state `STRUCTURE_ONLY` and `NOT_INCLUDED` for historical authorization proof.
- Demo Space V2 reports transition-level EOA proof only because it includes the exact typed-data domain, digest, signature, and expected signer and the independent verifier recovers that signer.
- ERC-1271 remains `ON-CHAIN ACCEPTANCE OBSERVED` unless the verifier can reproduce the signer contract at the relevant historical state or consume an adequate proof.
- No increase in confidence language is allowed until an adversarial fixture demonstrates tampering/replay rejection for the newly added proof fields.

### Goal H — Privacy-profile readiness (production gate, not current demo scope)

- Production snapshot commitment semantics must be versioned, canonical, domain-separated, and hiding against likely low-entropy memory values.
- Raw memory, locator content, salt/blinding material, and secrets must not enter portable evidence, telemetry, URL, RPC calldata, or public chain data.
- A threat review and test vectors cover encoding ambiguity, equality leakage, profile migration, and secret loss before any production privacy claim.

## Acceptance criteria for the current development goal

- `cargo xtask verify` passes after changes.
- A regression test proves authority history is no longer presented as cryptographically verified when the bundle lacks signature/event evidence.
- Restore assessment tests pass for all four evidence classes.
- The Inspector exposes the selected fixture assessment without confusing it with live Sepolia or runtime enforcement.
- Browser RPC code pins the related calls to a resolved block number and fails visibly if block identity cannot be held.
- Existing V1/V2 verifier inputs, Rust/revm lanes, evidence hashes, contract semantics, and exact revert reasons remain unchanged.
- Documentation uses the same claim boundaries as the UI and verifier.
- The concept, Forkline comparison, and reviewed-project lessons are recorded as explicit release goals rather than an unsupported victory claim.
- Browser JSON-RPC accepts the live stale-predecessor result only when the exact Solidity `Error(string)` revert payload decodes to `BAD_PREVIOUS_STATE`; free-form provider text alone is not treated as that contract reason.

## Research references

- [OWASP Agentic Applications: memory/context poisoning (ASI06)](https://genai.owasp.org/2026/05/13/memory-is-a-feature-it-is-also-an-attack-surface/)
- [OWASP Agent Memory Guard](https://owasp.org/projects/agent-memory-guard)
- [PMPA persistent-memory poisoning preprint](https://arxiv.org/abs/2609.13889) — evaluated harness results; not a production-prevalence estimate.
- [LangGraph time travel and checkpoint replay](https://docs.langchain.com/oss/python/langgraph/use-time-travel)
- [Ethereum Execution APIs: block tags and `eth_getBlockByNumber`](https://ethereum.github.io/execution-apis/api/methods/eth_getBlockByNumber/)
- [ERC-8350 discussion](https://ethereum-magicians.org/t/erc-8350-agent-memory-state-registry/29098) — prior standard work; not a MemoryLineage invention.
- [Forkline Devpost](https://devpost.com/software/forkline-rehearse-the-rollback) and the repository branch linked from the prior competitor audit.
- Local source review: `docs/research/AUDIT_SUBMISSION_3RD_WEB_HACK_2026-09-18.md`, `crates/ml-memory-store/src/lib.rs`, `crates/ml-verifier-independent/src/lib.rs`, `crates/ml-spec-types/src/lib.rs`, and `apps/inspector/src/browser.rs`.
