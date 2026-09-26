# MemoryLineage — Deep Uniqueness, Impact & Adoption Audit

**Date:** 20 September 2026
**Status:** audit baseline with local implementation follow-up
**Scope:** uniqueness, Web3 necessity, product differentiation, impact evidence,
and adoption evidence. Deployment and demo video remain outside the current
implementation scope.

## Implementation update — 20 September 2026

The P0 receipt slice described by this audit is now implemented locally. The
repository contains the versioned recovery receipt, JSON schema, deterministic
decision ID, Rust independent replay, browser export/import/tamper workbench,
three independently recoverable Demo Space V2 EOA signatures, and a CLI
protected-resume reference gate. The status table below is retained as
the audit baseline where it describes what was missing at audit time; current
local evidence now moves the receipt decision from design to L1/L2 proof. It
also includes a generic `ml-recovery-gate` loader boundary whose tests prove
that a historical/diverged snapshot never invokes its callback. It does not
move external developer reproduction, integration with a real agent framework,
or adoption into a completed state.

## Executive decision

MemoryLineage masih dapat menjadi submission yang berbeda dan kuat, tetapi
keunikan itu tidak berada pada klaim umum berikut:

- `AI agent memory`;
- hash atau Merkle commitment;
- checkpoint on-chain;
- snapshot/restore;
- lineage atau provenance;
- independent verifier;
- portable receipt;
- kata `verifiable memory`.

Ruang tersebut sudah memiliki standar, paper, hackathon project, dan open-source
implementation yang berdekatan. Karena itu `MemoryLineage` tidak boleh mengklaim
sebagai yang pertama atau sebagai satu-satunya sistem memory lineage.

Wedge yang masih defensible adalah kombinasi trust topology dan keputusan
operasional yang lebih sempit:

> **Before a long-lived agent resumes from a private snapshot, MemoryLineage
> determines whether that snapshot is the current publicly authorized head, a
> known historical checkpoint, divergent, or unverified, then exports the exact
> evidence-backed recovery decision for independent verification.**

Dalam bahasa produk:

> **Restore Preflight for agents whose runtime operator must not be the only
> party trusted to preserve recovery history.**

Itu hanya menjadi pembeda nyata jika tiga hal benar-benar dibuat:

1. preflight membaca commitment dari snapshot yang benar-benar dipilih, bukan
   hanya memilih fixture berdasarkan nomor;
2. hasilnya menghasilkan `Recovery Decision Receipt` yang dapat diverifikasi dan
   dipakai oleh loader sebagai policy input;
3. ada satu adapter resume yang benar-benar menahan snapshot lama atau divergen
   sebelum state tersebut dipakai dalam protected recovery path.

Impact saat ini belum terbukti di luar technical evidence milik repository.
Adoption belum boleh diklaim sama sekali. Target realistis untuk submission
adalah menaikkan bukti dari internal replay ke external clean-checkout,
comprehension test, dan satu integrasi loader yang dapat dijalankan. Itu
menunjukkan dampak teknis dan kegunaan awal; itu belum sama dengan adopsi pasar.

## Evidence baseline yang diaudit

Audit ini menggunakan source, evidence, dan hasil gate di checkout saat ini.
`cargo xtask verify` terakhir lulus untuk format, Clippy, workspace tests,
fixture/evidence invariants, pinned conformance, independent replay,
Rust/revm Silent Rollback, mutation lane, ERC-1271, authority rotation, WASM,
dan package boundary. Hasil tersebut membuktikan bahwa fondasi protocol dan
replay bekerja pada corpus yang dipublikasikan.

Baseline tersebut belum membuktikan:

| Area | Status aktual | Konsekuensi audit |
| --- | --- | --- |
| Demo Space V2 | Synthetic SQLite + local Rust/revm | Reproducible, tetapi bukan production agent integration |
| Sepolia | Separate read-only deployment/observation | Tidak boleh digabungkan dengan local Demo Space V2 menjadi satu incident |
| Portable authorization | Demo Space V2 `EOA_SIGNATURES_VERIFIED`; protocol corpus `STRUCTURE_ONLY` / `NOT_INCLUDED` | Keep the signed local fixture separate from the unsigned protocol projection; ERC-1271 remains execution-only |
| Restore Preflight | Fixture-scoped classifier | Belum menjadi runtime gate |
| Recovery Decision Receipt | Implemented as a tracked local artifact | Receipt, schema, Rust/WASM replay, CLI replay, and tamper tests pass locally; no production runtime claim |
| External developers | Belum ada human clean-checkout report | Impact/adoption evidence masih kosong |
| Public accessibility | Deployment berada di luar scope sekarang | Tidak boleh mengklaim parity public-access dengan project yang sudah hosted |

File sumber yang menjadi dasar status ini:

- [product contract](../product/product-contract.md)
- [Restore Preflight plan](../product/restore-preflight-and-recovery-rehearsal.md)
- [claim matrix](../submission/claim-matrix.md)
- [strict dominance audit](./2026-09-20-memorylineage-strict-dominance-plan-audit.md)
- [current impact validation protocol](./IMPACT_VALIDATION.md)

## 1. Audit klaim uniqueness

### Klaim yang tidak lagi aman

| Klaim | Putusan | Alasan |
| --- | --- | --- |
| “Blockchain untuk AI memory” | Tidak cukup unik | ERC-8350 dan beberapa project publik sudah menempatkan commitment/checkpoint memory pada chain |
| “Verifiable AI-agent memory” | Tidak cukup unik | AgentMem, ZMem, Portable Agent Memory, dan project lain memakai positioning tersebut |
| “Memory lineage” | Tidak cukup unik sebagai istilah | MemLineage paper dan beberapa implementation menggunakan lineage/provenance untuk memory |
| “Detects memory poisoning” | Tidak benar untuk protocol saat ini | Protocol tidak menilai semantik raw memory; semantic poisoning yang diotorisasi tetap dapat lolos |
| “Prevents rollback” | Terlalu luas | Registry menolak stale predecessor saat continuation diajukan; local restore tetap dapat dilakukan |
| “Safe restore” | Belum terbukti | Belum ada protected loader yang membaca decision dan menahan resume |
| “First” atau “only” | Dilarang | Audit publik tidak cukup untuk membuktikan prior art universal |

### Klaim yang masih defensible

| Klaim | Syarat minimum |
| --- | --- |
| “Independent auditor for private agent history” | Tetap menyebut exact invariant yang direplay dan source evidence-nya |
| “Canonical recovery preflight under an independently controlled registry” | Harus ada tiga role yang jelas dan candidate snapshot yang benar-benar dihitung |
| “Known historical checkpoint vs current head classification” | Receipt dan test harus menunjukkan current, historical, diverged, dan unverified |
| “Recovery decision can be independently replayed” | Receipt schema, deterministic decision hash, Rust CLI, dan tamper tests harus ada |
| “Stale predecessor cannot become the next canonical transition under tested registry rules” | Tetap gunakan exact `BAD_PREVIOUS_STATE`; jangan memperluas menjadi prevention everywhere |
| “Public registry prevents the runtime operator from being the sole source of canonical ordering” | Hanya berlaku jika operator, authorizer/controller, dan auditor memang dipisahkan |

## 2. Prior-art pressure dan competitor matrix

Matrix berikut sengaja membandingkan capability yang terlihat dari source publik.
`—` berarti tidak terlihat pada material yang diaudit; itu bukan bukti bahwa
capability tersebut mustahil ada. Klaim repository lain adalah klaim project
tersebut, bukan hasil rerun independen dari workspace ini.

| Project / work | Private state / restore | Public chain anchor | Authority / policy decision | Portable receipt / offline proof | Actual runtime or user integration | What it means for MemoryLineage |
| --- | ---: | ---: | ---: | ---: | ---: | --- |
| ERC-8350 | Agent memory state model | Yes, as registry semantics | Controller/authorizer model | Spec/vector oriented | — | Underlying standard, not an innovation claim of this project |
| Agent Memory Registry | Memory files checkpointed | Yes; Monad checkpoint history | Owner/authorship proof | Merkle inclusion proof | Scheduled publisher and verifier UI | Defeats any “first on-chain memory checkpoint” claim; does not present the same recovery preflight workflow |
| ZMem | SQLite snapshots, restore, handoff | Local-first; no same public registry trust topology shown | Quarantine, review, authority, revoke | Merkle receipts, action receipts, verify | CLI, MCP, local console, agent setup | Strong adjacent product. MemoryLineage must differentiate on independent public canonical history and recovery decision, not on “memory governance” generally |
| AgentMem | Persistent work/action record | Hub is a shared service, not the same EVM registry model | Actor attribution and audit | Ed25519 attestations and offline verification | CLI, hub, CI-style reporting | Shows that a usable CLI + shared receipt + external workflow is a stronger adoption surface than a dashboard alone |
| PermeantOS | Live migration, checkpoint, graph/artifact restore | No public blockchain trust anchor shown | Signed roots, provenance, two-phase migration | Signed transfer envelope and validation reports | MLX/vLLM real-runtime paths, adapters, AWS evidence | Raises the bar for real recovery/migration evidence. MemoryLineage should not compete on runtime migration breadth; it should own the public canonical recovery decision |
| Portable Agent Memory | Transfer and rehydration across agents | No public chain requirement in the paper | Capability-based disclosure and injection-resistant rehydration | Merkle-DAG provenance and cryptographic transfer | SDK and cross-model demonstrations | Defeats generic “cryptographically verified memory transfer” novelty; motivates a narrow chain-backed trust boundary |
| Runtime-Independent Persistent Agents | Quiesce/checkpoint/validate/bind/rehydrate/resume | No EVM requirement | Authorized continuation authority | Continuity evidence | 833 core tests and provider tests reported by paper | Confirms that recovery must be an explicit lifecycle decision, not only a hash comparison |
| Beyond Memory | Candidate state vs exact predecessor activation | No public chain requirement in paper | Commit, reject, quarantine, defer | Receipt as part of accepted unit | Executable model and large state exploration reported by paper | Direct pressure on our proposed decision vocabulary; MemoryLineage must provide the Web3/public-registry version and a runnable user path |
| MemLineage | Per-entry cryptographic and semantic lineage | No public EVM registry in paper | Sensitive-action gate based on lineage | Merkle log and derivation DAG | AgentDojo evaluation reported by paper | Name and concept overlap; never claim general lineage novelty. Our distinction must be recovery canonicality under independent authority |
| Forkline | External delivery queue after chain reorg | Local EVM rehearsal | Canonicality check before side effect | Evidence/replay and idempotency ledger | Public Delivery Lab and runnable demo | Strongest benchmark for clarity and completeness; MemoryLineage must make the pre-resume decision equally concrete |

The most important result is not that every adjacent project does the same thing.
They do not. The result is that each one removes one possible broad novelty
claim. The remaining differentiation is a **specific composition**:

```text
private restored snapshot
        + independently controlled public canonical head
        + authority-aware recovery classification
        + pre-resume policy decision
        + portable, independently replayable decision receipt
```

No absolute uniqueness conclusion follows from this matrix. The defensible
statement is:

> **The audit found no surveyed public project that demonstrates this exact
> five-part recovery workflow with an EVM registry, but that is a bounded survey
> result and not a universal prior-art proof.**

Sources reviewed include the [ERC-8350 draft](https://eips.ethereum.org/EIPS/eip-8350),
[Agent Memory Registry](https://github.com/b0tresch/agent-memory-registry),
[ZMem](https://github.com/zerkerlabs/zmem),
[AgentMem](https://github.com/agentmem/agentmem),
[PermeantOS](https://github.com/kabudu/permeant-os),
[Portable Agent Memory](https://arxiv.org/abs/2605.11032),
[Runtime-Independent Persistent Agents](https://arxiv.org/abs/2609.00546),
[Beyond Memory](https://arxiv.org/abs/2608.11632), and
[MemLineage](https://arxiv.org/abs/2605.14421).

## 3. The product wedge that should be frozen

The product should be framed as **MemoryLineage Recovery Integrity** while
keeping the public brand `MemoryLineage` and the tagline:

> **Verify the history, not the memory.**

The product question is:

> **Before resume, which recovery state is this snapshot, and who can
> independently verify that decision?**

The trust topology must be visible in the architecture and UI:

```text
Runtime operator
  controls the private SQLite snapshot

Independent controller / authorizer
  controls which transition may become canonical

Auditor / recovery operator
  verifies the candidate before resume
```

The Web3 justification is conditional and therefore credible:

> If one party controls the runtime, database, signing key, and audit process, a
> local signed log may be sufficient. MemoryLineage uses a public registry when
> an operator must not be the only party trusted to preserve canonical ordering.

The primary decision surface should use four outcomes:

| Decision | Meaning | Safe product action |
| --- | --- | --- |
| `CURRENT_HEAD` | Candidate commitment equals the named current evidence head | `RESUME_ALLOWED` only within the named policy and evidence scope |
| `KNOWN_HISTORICAL_CHECKPOINT` | Candidate equals a prior committed checkpoint, but not the current head | `REHEARSE_ONLY`; do not silently continue the canonical branch |
| `UNKNOWN_OR_DIVERGED` | Candidate is well formed but does not belong to the replayed canonical history | `HOLD_FOR_REVIEW` |
| `UNVERIFIED` | Evidence, schema, source, or computation is insufficient | `BLOCK_UNVERIFIED` until another evidence source is supplied |

`RESUME_ALLOWED` must not be described as a universal security guarantee. It is
the decision produced by a named evidence profile and policy. `BLOCK_UNVERIFIED`
must become an actual loader result before the product claims runtime
enforcement.

## 4. Recovery Decision Receipt is the real differentiator

A receipt is useful only if it records the exact decision inputs and can be
replayed. It must not be a decorative JSON export added after the UI renders.

### Required receipt fields

```json
{
  "schemaVersion": "memorylineage-recovery-receipt-v1",
  "decisionId": "0x...",
  "policyId": "strict-current-head-only-v1",
  "candidate": {
    "snapshotProfile": "memorylineage/private-snapshot/v1",
    "candidateCommitment": "0x...",
    "snapshotSequence": 1
  },
  "evidence": {
    "sourceClass": "DEMO_SPACE_V2_LOCAL",
    "bundleHash": "0x...",
    "specSnapshot": "v1-pinned-vector-2026-09-18",
    "registryAddress": "0x...",
    "spaceId": "0x...",
    "head": {
      "sequence": 3,
      "stateRoot": "0x..."
    },
    "blockContext": null
  },
  "decision": {
    "classification": "KNOWN_HISTORICAL_CHECKPOINT",
    "reasonCode": "CANDIDATE_IS_BEHIND_CANONICAL_HEAD",
    "recommendedAction": "REHEARSE_ONLY"
  },
  "assurance": {
    "lineageReplay": "VERIFIED",
    "authorityHistory": "TIMELINE_BOUND",
    "transitionAuthorization": "EOA_SIGNATURES_VERIFIED",
    "source": "DEMO_SPACE_V2_LOCAL"
  },
  "limitations": [
    "does_not_assess_semantic_truth",
    "does_not_prove_off_chain_availability",
    "does_not_claim_runtime_enforcement"
  ]
}
```

The actual implementation must derive `decisionId` from canonical receipt
fields. It must reject unknown critical fields, omit raw memory, omit private
locators, and never include keys or salts that would weaken the fixture's
privacy boundary.

### Receipt verification rules

The Rust verifier must independently recompute:

1. the evidence bundle hash or canonical digest used by the receipt;
2. the candidate commitment from the named snapshot profile;
3. the classification from the replayed history;
4. the expected recommendation from the declared policy;
5. the decision ID;
6. for the signed Demo Space V2 scope, the EIP-712 digest, recovered EOA
   signer, config nonce, and effective sequence window for every transition.

It must distinguish:

```text
RECEIPT_INTEGRITY_VERIFIED
```

from:

```text
AUTHORITY_SIGNATURE_VERIFIED
```

The former is implemented for the published receipt. The latter is
`EOA_SIGNATURES_VERIFIED` only for the signed Demo Space V2 bundle; the
protocol-corpus projection remains `NOT_INCLUDED`, and ERC-1271 remains an
on-chain acceptance observation rather than a historical offline proof.

### Receipt is not the runtime gate

The product now has the small framework-neutral adapter described by this flow:

```text
read snapshot
  -> compute candidate commitment
  -> load named evidence bundle
  -> run preflight
  -> emit receipt
  -> enforce policy before protected loader resumes
```

`ml-recovery-gate` implements the adapter and tests that the loader is invoked
for the current head, held for a historical checkpoint, and never reached for
diverged or invalid evidence. It does not require LangChain, LangGraph, MCP, a
hosted backend, or a new chain.

## 5. Impact and adoption audit

### Current evidence ladder

| Level | Evidence | Current status |
| --- | --- | --- |
| L0 | Narrative says the problem matters | Present, but not impact evidence |
| L1 | Own deterministic unit/integration tests | Present |
| L2 | Independent implementation or execution path agrees | Present through Rust verifier/revm corpus |
| L3 | External developer can clone and reproduce | Not yet demonstrated |
| L4 | External developer completes the recovery task and explains the limits | Not yet demonstrated |
| L5 | Generic protected loader adapter refuses or holds a candidate according to policy | Implemented locally by `ml-recovery-gate`; real agent-framework integration not yet demonstrated |
| L6 | Repeated external use, integration, issue/PR, or production usage | Not available and must not be claimed |

The current repository is strong at L1/L2. It is not yet an adoption story.
Passing tests, a large mutation matrix, and a serious problem statement do not
prove that another developer needs or can use the product.

### Who should be tested

Do not recruit “any AI user.” The first credible user is:

> **A developer or operator maintaining a long-lived agent whose private state
> may be restored or migrated, while an independent controller, protocol team,
> or auditor must retain a canonical recovery reference.**

This is narrower than “all AI agents” and matches the Web3 trust condition.
Possible test contexts:

- an agent hosted by an operator for a DAO or protocol team;
- a managed agent moved between hosts after a crash;
- a recovery operator restoring an older backup while an authorizer owns the
  canonical state policy;
- a multi-operator agent where the local database cannot be the sole source of
  truth.

Do not use a trading agent as the hero unless the project can prove a causal
link between the memory state and a financial action. Current MemoryLineage
cannot prove that link.

### Adoption surface to build

The first user-facing integration should be a CLI plus a minimal loader adapter,
not a new dashboard page:

```bash
ml recover preflight \
  --snapshot fixtures/silent-rollback-v2/snapshot-1.db \
  --evidence evidence/local/demo_space_v2_evidence.json \
  --policy recovery-policy.json
```

Expected machine-readable result:

```text
classification=KNOWN_HISTORICAL_CHECKPOINT
recommended_action=REHEARSE_ONLY
decision_id=0x...
exit_code=10
```

The loader integration test must show:

```text
snapshot-3 -> current head -> protected resume permitted by policy
snapshot-1 -> known historical -> protected resume held
tampered snapshot -> unknown/diverged -> protected resume held
malformed evidence -> unverified -> protected resume blocked
explicit override -> recorded separately, never reported as verified resume
```

The website can call the same pure Rust decision engine for the fixture, but
the CLI/adapter is what makes the product's impact claim credible.

### Human validation protocol

After the CLI and adapter exist, test three to five developers who did not
write the registry or verifier. Give them only the repository URL and README.
Do not explain the intended answer first.

Each participant should:

1. clone a clean checkout;
2. run the documented verification command;
3. run preflight against current, historical, tampered, and malformed inputs;
4. choose the action they would take before resume;
5. export the receipt and verify it independently;
6. explain what MemoryLineage proves and does not prove.

Record only reproducibility metadata and answers, never raw memory or secrets.
The report should contain:

```text
exact commit
OS and toolchain
commands
time to first successful preflight
setup failures
classification for each fixture
receipt verification result
answer to “what does it prove?”
answer to “what does it not prove?”
whether help was required
```

Suggested acceptance targets are targets, not current results:

- at least 3 independent runs complete without maintainer intervention;
- every run obtains the same four classifications as the reference verifier;
- every participant understands that a historical checkpoint is not necessarily
  malicious;
- no participant describes the system as a semantic poisoning detector;
- every participant can verify the receipt after changing one candidate field;
- setup and first preflight fit a short, documented path.

Until those results are recorded, use `NOT YET DEMONSTRATED`. Do not convert
the existing AI verifier sessions into human adoption evidence.

## 6. Lessons from winning and strong adjacent projects

The relevant lesson is not “copy their features.” It is the evidence shape that
made their products understandable and runnable.

### Forkline

Forkline succeeds as a benchmark because one failure is concrete: an external
delivery is irreversible while a blockchain event can become non-canonical. Its
public presentation ties problem, simulation, result, and limitations together.
MemoryLineage must match that clarity with one sentence:

> **A restored private snapshot can be locally valid but still be the wrong
> predecessor for the independently recorded canonical history.**

Then the user must see the recovery decision, not only a hash.

Reference: [Forkline Devpost](https://devpost.com/software/forkline-rehearse-the-rollback).

### ExitDrill: adjacent drill and explanation pattern

ExitDrill's public repository describes a short synthetic exercise that
compares a pre-export baseline with a clean or lossy SaaS export, reports the
structural dimensions separately, and ends with a concise explanation. This is
useful for the *shape* of an onboarding task: one concrete case, visible
evidence, and an understandable outcome. It is not a direct memory-lineage
competitor, a game-like tutorial, or evidence that new users understand either
product. The documented demo is a CLI workflow rather than a gamified tutorial.
ExitDrill is described as a technical alpha using synthetic data and does not
claim to prove a completed migration. Its Devpost event membership is
unverified, so it must not be counted as a 3rd-Web-Hack entrant; see the
[public project audit](./2026-09-20-3rd-web-hack-public-project-audit.md#event-membership-unresolved-not-scored-as-a-3rd-web-hack-entry).

Reference: [ExitDrill repository](https://github.com/ChelseaKR/exitdrill).

### Polkadot winners

The Polkadot Builder Party winners show a recurring pattern: a clear user pain,
a complete runnable surface, and a repository that explains the path. Fangorn
states the ownership/access problem in one sentence and links both a short
overview and a longer demo. OceanFin makes “simulate before one-click run” a
product action, not a documentation promise. The official winners update lists
Agora, Nani, Fangorn, OceanFin, and PolkaShield among the track winners.

References: [official winners update](https://polkadot.devpost.com/updates/40528-winners-announced-and-what-s-next),
[Fangorn](https://github.com/driemworks/fangorn), and
[OceanFin](https://github.com/Tizun71/OceanFin).

MemoryLineage should adopt the same structure as a recovery product:

```text
inspect -> simulate recovery -> see disposition -> choose/hold resume -> verify receipt
```

### AgentMem and ZMem

These projects raise the usability floor. AgentMem exposes a CLI, a shared
ledger, actor attribution, and offline attestation. ZMem exposes local SQLite,
review/quarantine/revoke/restore, receipts, MCP integration, a local console,
and a documented bootstrap path. Their existence means that “we have a
verifier” is not enough for impact. The verifier must sit in a workflow that a
developer can run before a consequential action.

References: [AgentMem](https://github.com/agentmem/agentmem) and
[ZMem](https://github.com/zerkerlabs/zmem).

### PermeantOS and the continuity papers

PermeantOS demonstrates a higher bar for real recovery evidence: it reports
validated source/target runtime paths, graph/artifact migration, signed roots,
provenance, and explicit experimental boundaries. The papers on
[runtime-independent agents](https://arxiv.org/abs/2609.00546) and
[portable agent memory](https://arxiv.org/abs/2605.11032) also treat
checkpoint/validate/rehydrate as a lifecycle, not a static hash.

MemoryLineage does not need their runtime breadth. It needs their discipline:
name the exact path, execute it, and show which compatibility/assurance claim
remains out of scope.

### Simulation-before-action projects

Chainlink's risk-router repository makes dry-run explicit before broadcast and
returns a structured decision from an evaluation pipeline. SentinelCRE exposes a
pre-execution simulation surface and reports exact on-chain outcomes. These are
useful patterns for Recovery Preflight: the user should see a disposition before
any state continuation or external side effect, with `eth_call`/local execution
kept distinct from broadcast.

References: [CRE Risk Router](https://github.com/lancekrogers/cre-risk-router) and
[SentinelCRE](https://github.com/ProjectWaja/SentinelCRE).

## 7. Exact work required to improve uniqueness and impact

### P0 — make the differentiated workflow real

1. Add `memorylineage-recovery-receipt-v1` to `ml-spec-types`.
2. Add a separate Rust receipt verifier that does not import the core decision
   algorithm through a shared implementation shortcut.
3. Add `ml recover preflight` to `ml-cli`.
4. Make preflight read the selected SQLite snapshot and compute its actual
   candidate commitment.
5. Add policy outcomes and exit codes for current, historical, diverged, and
   unverified states.
6. Add a minimal protected loader example that refuses or holds resume based on
   the decision.
7. Export the receipt from the browser and show the same receipt in the CLI.
8. Keep `DEMO_SPACE_V2_LOCAL`, `PROTOCOL_CORPUS_LOCAL`, and
   `SEPOLIA_REFERENCE_OBSERVATION` as separate source classes everywhere.

### P1 — prove the product to an independent developer

1. Add receipt tamper tests: candidate, bundle hash, classification, policy, and
   source class.
2. Add current/historical/diverged/unverified browser scenarios using actual
   fixture and evidence inputs.
3. Add a clean-checkout `cargo xtask adoption-smoke` command that runs the
   preflight/loader path.
4. Run the human protocol with at least three developers.
5. Publish the exact reports in a sanitized evidence directory.
6. Update README, UI, claim matrix, and Devpost wording from the same claim
   matrix.

### P2 — strengthen the trust boundary if time remains

1. Add a named production commitment profile with explicit domain separation and
   high-entropy blinding. Do not reuse the public synthetic fixture as a
   production privacy claim.
2. Add EOA signature material to a new evidence schema only if it can be
   independently replayed.
3. Keep ERC-1271 as `ON-CHAIN ACCEPTANCE OBSERVED` unless historical contract
   state can be reproduced.
4. Add one external snapshot format adapter only after the SQLite adapter is
   complete. Do not add a framework integration merely for breadth.

### Explicit stop rule

Stop after the following are true:

```text
real snapshot preflight
decision receipt
protected loader gate
independent receipt replay
three external clean-checkout users
claim/source labels aligned
```

Do not add tokens, a second chain, semantic LLM classification, generic agent
reputation, or a marketplace before those gates pass.

## 8. Forkline comparison after this audit

The realistic comparison is conditional:

| Dimension | Forkline baseline | MemoryLineage current | MemoryLineage target |
| --- | --- | --- | --- |
| Problem clarity | One concrete reorg/delivery failure | Medium; restore story is now clearer but still product-light | One pre-resume recovery decision with four visible dispositions |
| Web3 necessity | Directly tied to canonical chain event | Conditional on separated operator/controller/auditor roles | Explicit trust topology and a shared canonical recovery reference |
| Uniqueness | Narrow reorg-to-side-effect rehearsal | Limited; many memory/proof overlaps | Defensible composition: public canonicality + private snapshot + preflight + receipt + loader gate |
| Runtime consequence | Delivery worker can hold side effect | Current browser does not gate runtime | Protected loader holds stale/diverged/unverified resume |
| Evidence | Public project evidence and runnable lab | Strong local Rust/revm/evidence, separate Sepolia observation | One receipt and one reproducible recovery task with source-class labels |
| Independent verification | Evidence/replay surface | Rust verifier and revm paths | Receipt plus bundle independently replayed by CLI and browser |
| Adoption evidence | Public accessibility helps discovery | None from external developers | Three or more clean-checkout task runs; still not market adoption |
| Claim discipline | Strong published limitations | Stronger than before, but authorization proof limited | Same narrow claims, with runtime enforcement explicitly measured |

MemoryLineage can surpass Forkline on the technical/evidence dimensions listed
above after the target gates pass. It cannot honestly claim global superiority
while deployment, public accessibility, or external use evidence are absent.

## 9. Final claim language

### Use after P0, if all tests pass

> MemoryLineage is a public-registry-backed recovery preflight for private
> agent state. It classifies a restored snapshot against an independently
> replayed canonical history, distinguishes current from historical or divergent
> state, and emits a portable decision receipt before protected resume.

### Do not use

```text
the first verifiable AI memory system
prevents memory rollback everywhere
detects semantic memory poisoning
proves the agent used this memory
proves the memory is true or safe
fully offline-verifies every historical ERC-1271 signature
adopted by developers
production-ready security
```

### Current wording before P0

> MemoryLineage currently demonstrates a fixture-scoped Restore Preflight and a
> Rust/revm rejection of a stale predecessor. Runtime resume enforcement,
> portable recovery receipts, external human reproduction, and adoption have
> not yet been demonstrated.

## Final audit verdict

The original table entry is accurate today:

```text
Uniqueness       Limited but defensible
Impact/adoption  Not yet demonstrated
```

The audit does identify a credible path to improve both, but it does not turn
that path into evidence. The strongest next move is not another standards
comparison or another UI surface. It is a small, real recovery lifecycle:

```text
read actual snapshot
  -> preflight against named canonical evidence
  -> classify current/historical/diverged/unverified
  -> emit receipt
  -> enforce the decision before protected resume
  -> independently replay the receipt
```

If that vertical slice passes its external developer protocol, MemoryLineage can
make a defensible claim of differentiated technical impact. It still should say
that market adoption is unknown. Adoption is learned from repeated use or real
integration, not inferred from repository size, number of tests, or the
seriousness of the problem.
