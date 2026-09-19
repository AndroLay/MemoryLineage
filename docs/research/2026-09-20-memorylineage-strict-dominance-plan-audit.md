# MemoryLineage — Strict Dominance Plan Audit

Status: **baseline audit; local implementation follow-up recorded below**
Date: **20 September 2026**

## Follow-up implementation note — 20 September 2026

The P0 follow-up has now landed in the working tree. The Inspector source
labels distinguish local evidence from optional Sepolia observation, a tracked
`memorylineage-recovery-receipt-v1` is generated from the Demo Space V2
current-head snapshot, the Rust/WASM Verify surface can export/import/tamper it,
and `cargo xtask verify` checks reproducible receipt generation plus the
protected CLI decision gate. Demo Space V2 now also carries three EIP-712 EOA
authorization proofs that the independent Rust verifier recovers. The receipt
and gate are still fixture-scoped;
they do not prove production agent-loader integration or human adoption. The
remaining findings below should be read as the original audit criteria and
their current limits, not as claims that these local gates are absent.

This is a second audit of the MemoryLineage product-evolution plan. It checks
the plan against the current source, evidence, verification commands, and the
public Forkline benchmark. It does not promise a Devpost ranking. It defines the
conditions under which MemoryLineage can make a defensible claim of superiority
on the dimensions that the repository can measure.

## Verdict

The plan is directionally complete, but it was not yet complete as a release
plan. The main design is correct:

```text
MemoryLineage
  → Restore Preflight
  → Recovery Rehearsal
  → exact registry/evidence result
  → portable Recovery Decision Receipt
  → independent Rust verification
```

If all gates in this document pass, MemoryLineage can be materially stronger
than Forkline in:

- evidence depth;
- independent replay;
- explicit assurance levels;
- privacy boundary;
- authority history;
- portable recovery decisions;
- negative-path coverage;
- reproducible Rust developer workflow.

It still cannot honestly claim superiority in public hosting, video quality, or
external adoption while those activities remain out of scope. It also cannot
guarantee a judge's interpretation. “Surpass Forkline” must therefore mean a
measured dominance envelope, not a blanket competition claim.

## Verification performed in this audit

The current repository was inspected as source plus committed evidence. The
current Rust verification gate was run:

```text
cargo xtask verify
```

Result:

```text
PASS format
PASS clippy
PASS workspace tests
PASS fixture manifest
PASS Demo Space V2 evidence
PASS pinned conformance
PASS independent replay
PASS revm Silent Rollback
PASS revm mutation lane
PASS revm ERC-1271
PASS revm authority rotation
PASS WASM compile
PASS public package boundary
```

The static browser smoke command also returned success when run in an environment
with the required browser operation permission. Its first sandboxed invocation
returned `Operation not permitted`; that is an environment limitation, not
evidence of a product failure. The smoke script covers the 11 routes, local
rollback interaction, evidence tamper/restore flow, and the 390px layout. The
repository records this as local browser evidence; it is not external human
reproduction.

## Findings

### Resolved locally — Source labels needed to distinguish evidence universes

The main product flow uses published local Rust/revm evidence. The optional
Sepolia probe is a separate read-only observation. In the baseline audited
checkout, two UI defaults weakened that distinction:

- `Scenario::SilentRollback` returns `"LIVE"` from
  [`apps/inspector/src/data.rs:63-68`](../../apps/inspector/src/data.rs:63);
- the top navigation always renders `SEPOLIA / READ-ONLY` from
  [`apps/inspector/src/components.rs:36-38`](../../apps/inspector/src/components.rs:36).

The follow-up now labels the local scenario `LOCAL EVIDENCE` and the top bar
`SEPOLIA / OBSERVED`; the actual hero result remains labeled `PUBLISHED LOCAL
REVM`. The optional Sepolia probe is separately described in the lab. The
source ambiguity is therefore resolved in the current tree, while this finding
records the original failure mode.

The baseline hero result was labeled `PUBLISHED LOCAL REVM` in
[`apps/inspector/src/pages.rs:71`](../../apps/inspector/src/pages.rs:71), and the
optional Sepolia probe is separately described in the lab. A careful reader can
resolve the distinction, but a first-time judge should not have to.

**Failure scenario:** a screenshot or short walkthrough makes the reviewer
believe the primary Silent Rollback was executed against Sepolia when it was
actually a published local revm record.

**Implemented fix:** replace the scenario marker with a source capability label
such as `LOCAL EVIDENCE` or `LIVE PROBE AVAILABLE`, and make the top-bar source
context route-aware. The header should say `LOCAL REVM EVIDENCE` on Home,
Inspect, History, and the primary Lab result; it may say `SEPOLIA / READ-ONLY`
only beside the optional live observation.

**Acceptance test now passing locally:** a route/source audit must find no visible `LIVE` label for a
local result. A browser smoke assertion must verify that the local rollback
result contains `PUBLISHED LOCAL REVM` and does not claim `SEPOLIA / LIVE`.

### Resolved locally — Recovery Decision Receipt is now a tracked evidence artifact

The initial audit found that the evidence structures contained
`AttackObservation`, head, transitions, authority records, and verification
metadata, but no dedicated recovery-decision receipt. The current tree now has
the versioned receipt and schema described below; Restore Preflight remains a
fixture-scoped assessment and the protected CLI gate is not a production
runtime gate.

**Failure scenario:** the site can display a classification, but a developer
cannot download the exact input, evidence source, assurance level, and decision
that produced it. That leaves the product closer to a dashboard than to a
reproducible operational tool.

**Implemented fix:** add a versioned receipt structure separate from the registry
history bundle. It must include the candidate commitment, evidence bundle ID or
hash, classification, source class, exact reason, authority assurance, block
context when observed, recommended recovery mode, and limitations. It must never
include raw memory, salts, private locators, or secrets.

**Acceptance test now passing locally:** the browser exports a receipt; the independent Rust CLI
parses it; changing the candidate commitment changes the receipt verdict and
causes verification to fail; the receipt explicitly says whether the decision is
local evidence, live observation, or unverified.

### High — The three evidence universes are coherent only locally

Current Demo Space V2 is a local synthetic SQLite/revm incident with three
transitions and an authority rotation. The existing Sepolia deployment is a
separate space and observation. The four-transition protocol corpus is another
evidence set. The claim matrix correctly keeps them separate.

**Failure scenario:** a page, README, or future receipt combines the three
sources into one implied public incident. That would create the same coherence
problem the plan is trying to fix.

**Required fix:** define a named evidence-source model:

```text
DEMO_SPACE_V2_LOCAL
PROTOCOL_CORPUS_LOCAL
SEPOLIA_REFERENCE_OBSERVATION
```

Every page, result, receipt, CLI output, and downloaded bundle must carry one of
these source identities. The website may present them together as an evidence
workspace, but it must never merge their sequence counts or history roots.

**Acceptance test:** a static scan and JSON test reject an artifact that claims
Demo Space V2 is a Sepolia history. A first-time reviewer can identify which
source produced every displayed result.

### Scoped — Historical authorization proof differs by evidence source

The protocol-corpus projection remains `STRUCTURE_ONLY` and
`NOT_INCLUDED`. Demo Space V2 is now a scoped exception: every local transition
contains EIP-712 domain, digest, signature, and declared EOA authorizer data,
and the independent verifier recovers the signer before reporting
`EOA_SIGNATURES_VERIFIED`. Its proof is also bound to the active config nonce
and effective sequence window, reported as `TIMELINE_BOUND`. Contract and revm tests still do not automatically
prove historical ERC-1271 signer state.

**Failure scenario:** a future “receipt” or visual badge upgrades authority
rotation evidence into full historical authorization proof without adding the
signature, typed-data domain, event/receipt, or historical ERC-1271 state needed
for independent verification.

**Required fix:** keep assurance dimensions separate. For EOA transitions,
include enough typed-data and signature material for independent recovery (now
done for Demo Space V2). For ERC-1271, report
`ON-CHAIN ACCEPTANCE OBSERVED` unless historical signer code/state can actually
be replayed.

**Acceptance test:** tampering with a signature, domain, authorizer, or evidence
block context fails closed. A bundle without those fields remains
`NOT_INCLUDED`, never `VERIFIED`; Demo Space V2's published proof set returns
`EOA_SIGNATURES_VERIFIED` only after independent recovery.

### Resolved locally — Generic protected adapter exists; production integration remains open

The current browser compares public synthetic fixture commitments and launches a
rehearsal. The new `ml-recovery-gate` crate reads a named SQLite snapshot,
verifies the receipt, and invokes a supplied loader callback only for the
current-head decision. It still does not compute a production hiding
commitment, intercept a real agent framework loader, or prevent an external
runtime from starting.

**Failure scenario:** marketing copy changes “preflight assessment” into
“prevents rollback” or “blocks poisoned memory.”

**Implemented local fix:** keep the current website claim as
assessment/rehearsal and add the framework-neutral adapter as the authoritative
reference place that refuses a load under the receipt policy. A real framework
integration remains a separate, unclaimed follow-up.

**Acceptance test:** an integration fixture proves that `HOLD_UNVERIFIED` and
`KNOWN_HISTORICAL_CHECKPOINT` stop a protected loader, while an explicit,
recorded override remains distinguishable from a successful verification.

### High — Global dominance over Forkline cannot be guaranteed under the current scope

Forkline has a public hosted Delivery Lab, public source/evidence, and demo media.
The current MemoryLineage scope excludes deployment and video. The local audit
identifies Forkline as the strongest overall public competitor; its published
metrics remain participant evidence and were not replayed by this workspace.

**Failure scenario:** MemoryLineage claims “better than Forkline in every aspect”
while a judge cannot open a public site or see a video.

**Required fix:** use two labels:

```text
TECHNICAL / EVIDENCE DOMINANCE TARGET
PUBLIC ACCESS / MEDIA PARITY — NOT CLAIMED IN THIS SCOPE
```

The submission can still be stronger in the first category, but it cannot claim
the second until those activities are authorized and completed.

**Acceptance test:** the final claim matrix contains no global victory claim and
lists every unbuilt public surface as `NOT YET DEMONSTRATED` or `OUT OF SCOPE`.

### Medium — Production privacy is not the same as the public fixture

The Demo Space V2 manifest is explicitly synthetic and public. Its snapshot
commitments are useful for deterministic reproduction, not proof of a production
hiding profile. A production profile still needs canonical serialization,
domain separation, high-entropy blinding or encryption, secret handling, and
equality-leakage analysis.

**Required fix:** keep fixture and production profiles named separately. Do not
use the fixture to claim absolute privacy. Add production-profile vectors only as
a separately reviewed milestone.

### Medium — Local transition detail cannot show public block/receipt context

The Transition page correctly displays `NOT AVAILABLE IN THIS BUNDLE` for block
and transaction data because local revm has no public receipt. That is truthful,
but the product plan must not imply that every forensic field is already
available in the portable local bundle.

**Required fix:** include block/transaction fields only for evidence produced by a
public chain observation. Keep local revm results tagged as execution evidence,
not public chain observation.

### Medium — Browser smoke is local acceptance, not independent validation

The smoke script is valuable and covers the release artifact, but it is run by
the project’s own environment. It should not be presented as external
reproduction or user comprehension.

**Required fix:** add two separate release rows:

```text
PROJECT-OWNED BROWSER SMOKE — VERIFIED
EXTERNAL HUMAN CLEAN-CHECKOUT — NOT YET DEMONSTRATED
```

The external test must ask the developer to explain what MemoryLineage proves
and does not prove in their own words.

### Medium — The dominance plan needs a stop rule

The current plan contains many good future directions, but adding them all before
the vertical slice would recreate the breadth risk seen in large hackathon
projects. The plan needs an explicit freeze rule.

**Required fix:** after P0 passes, freeze protocol and product scope. P1 may only
add evidence assurance or one small adapter. P2 is post-submission exploration.

## Dominance envelope

The following is the strongest honest comparison that can be made.

| Dimension | Current MemoryLineage | Required final evidence | Can exceed Forkline? |
| --- | --- | --- | --- |
| Incident clarity | Good local Silent Rollback story | First-time comprehension test passes | Yes, if wording is simpler and source labels are unambiguous |
| Public end-to-end access | Local static artifact; no hosting in scope | Public deployment and deep-link test | Not claimed in current scope |
| Protocol depth | Solidity, Rust core, revm, EOA/ERC-1271, rotation | Preserve all parity gates | Yes, on measured technical depth |
| Independent replay | Rust verifier and portable bundles | Receipt plus independent replay with tamper failure | Yes |
| Authority evidence | Contract tests; history-aware proof levels | Demo EOA signatures are independently recovered and bound to effective sequence/config-nonce windows; protocol corpus remains structural-only | Yes within the local signed-demo scope; ERC-1271 historical replay remains out of scope |
| Privacy boundary | Fixed-size commitments, raw memory excluded | Fixture/production distinction and leakage documentation | Yes in clarity; not absolute privacy |
| Negative paths | Mutation corpus and semantic out-of-scope path | Four preflight classes plus attack families | Yes |
| Product decision | Classification exists | Receipt and recovery-mode recommendation | Yes |
| Developer setup | Cargo verification gate | Clean checkout and external reproduction | Yes only after external test |
| Video/pitch | Excluded | Separate future work | No claim |
| Judge result | Unknown | No project can guarantee this | Never guaranteed |

The answer to “will it surpass Forkline if everything is applied?” is therefore:

> **It can surpass Forkline on the technical/evidence/product-decision envelope,
> but not honestly on every public-submission dimension under the current scope,
> and no plan can guarantee the judge’s ranking.**

## Revised implementation gates

### P0 — Truthfulness and one coherent vertical slice

1. Fix the `LIVE` scenario marker and static Sepolia navbar context.
2. Define `EvidenceSourceClass` and carry it through UI, JSON, CLI, and receipts.
3. Add `RecoveryDecisionReceipt` to the evidence model with schema validation.
4. Export/import the receipt and verify it independently in Rust/WASM and CLI.
5. Test all four classifications:
   - current demo head;
   - known historical checkpoint;
   - unknown/diverged commitment;
   - malformed/unsupported/unverified evidence.
6. Keep `BAD_PREVIOUS_STATE` and `TRANSITION_ID_MISMATCH` exact.
7. Add browser smoke assertions for source truth, not only route existence.
8. Keep Demo Space V2, protocol corpus, and Sepolia reference observation
   structurally separate.

### P1 — Evidence superiority

1. Add per-transition public event/receipt references where available.
2. Add EOA authorization proof only with complete typed-data/signature inputs.
3. Keep ERC-1271 at `ON-CHAIN ACCEPTANCE OBSERVED` until historical re-execution
   is truly supported.
4. Add receipt tampering tests for candidate commitment, evidence hash, source
   class, chain ID, registry address, block identity, and classification.
5. Run a clean checkout using the exact documented toolchain and record the
   result.
6. Have at least two independent developers run the flow without a briefing.

### P2 — Bounded enforcement

1. Define a framework-neutral `preflight()` adapter.
2. Integrate it with one local snapshot loader or recovery CLI.
3. Test fail-closed behavior for unverified and historical states.
4. Record explicit overrides.
5. Do not integrate multiple agent frameworks before this adapter is reliable.

### P3 — Post-submission research

Only after P0–P2:

- provider migration handoff;
- ERC-8004 validation receipt compatibility;
- content-safety adapter composition;
- explicit branch/merge extension;
- stronger finality or light-client observation;
- public deployment and media if separately authorized.

## Repository benchmark coverage

The research set is sufficient for the product decision, but it is not a claim
that every Web3 hackathon submission or every winner repository was exhaustively
replayed.

### Direct 3rd-Web-Hack benchmark

- [Forkline Devpost](https://devpost.com/software/forkline-rehearse-the-rollback)
  and its public branch were source/audit benchmarks. Its published test counts
  were not treated as our own replay results.
- [FinalityDesk](https://devpost.com/software/finalitydesk) was used as a narrow
  verifier/evidence benchmark.
- Kinetic, ArcLight AI, and DEDSEC were used for problem clarity, ambition, and
  presentation comparisons, with their public evidence limitations recorded in
  [`AUDIT_SUBMISSION_3RD_WEB_HACK_2026-09-18.md`](./AUDIT_SUBMISSION_3RD_WEB_HACK_2026-09-18.md).

### Open repositories inspected as design benchmarks

- [Fangorn](https://github.com/driemworks/fangorn)
- [Nani](https://github.com/cenwadike/nani)
- [Agora](https://github.com/suyash101101/Agora)
- [PolkaShield](https://github.com/FredMunene/polkaShield)
- [OceanFin](https://github.com/Tizun71/OceanFin)
- [SentinelCRE](https://github.com/ProjectWaja/SentinelCRE)
- [CRE Risk Router](https://github.com/lancekrogers/cre-risk-router)
- [AgentScore](https://github.com/agentscore-trustless/agent-score)
- [Aegis Protocol V5](https://github.com/vjb/aegis-v5)

Additional winner/project pages such as [AegisGate](https://chain.link/hack-26/projects/aegis-gate),
[ACL](https://ethglobal.com/showcase/acl-6hwos), and
[Autonome](https://ethglobal.com/showcase/autonome-d8cxe) were used for product
patterns and public workflow analysis. Their participant descriptions should not
be treated as independent security audits.

## Final recommendation

Do not add another idea or another chain. Start the next implementation phase
with the two truthfulness fixes, then build the Recovery Decision Receipt. This
is the smallest work that converts the current plan from a strong concept into a
measurable product advantage.

The hard gate is:

```text
If a judge cannot tell whether a result is local, live, published, observed,
or merely planned, the product is not ready to claim evidence superiority.
```

After the source labels, receipt, unified evidence identity, and independent
replay gates pass, MemoryLineage will have a defensible basis to say it exceeds
Forkline in its technical/evidence domain. Until then, use “target,” “verified
locally,” and “not yet demonstrated” exactly as the claim matrix requires.
