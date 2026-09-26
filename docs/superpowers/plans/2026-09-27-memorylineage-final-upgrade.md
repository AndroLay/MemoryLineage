# MemoryLineage Final Upgrade Implementation Plan

Current execution status is tracked in
[`2026-09-27-memorylineage-final-upgrade-status.md`](../../research/2026-09-27-memorylineage-final-upgrade-status.md).
This plan remains the acceptance target; implementation of the local adapter
does not mark the human, hosted CI, public-chain, or release gates complete.

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Move MemoryLineage beyond a strong local protocol prototype by proving one complete, understandable recovery decision inside a real agent checkpoint workflow, then assess it against Forkline with a fixed, evidence-based rubric. This plan targets a stronger submission and product; it cannot guarantee a judge score, win, or universal superiority.

**Architecture:** Keep v1.0.2 immutable. Build a v1.1 vertical slice in which a LangGraph checkpoint is intercepted before LangGraph receives it, converted through a strict versioned snapshot profile, assessed by the existing Rust recovery policy, and returned only when the portable recovery receipt passes independent verification. The same incident remains usable through the beginner challenge, the Rust CLI, and a clean-checkout adapter demo. Label local replay, RPC observation, corroborated RPC observation, and consensus-authenticated proof as separate assurance levels.

**Tech Stack:** Existing Rust workspace, Solidity registry and REVM harness, Recovery Decision Receipt, Dioxus/WASM Inspector, Python 3 with a pinned LangGraph integration package, standard-library process/JSON boundary to the Rust CLI, and existing `cargo xtask` gates.

**Spec:** `docs/product/product-contract.md`, `docs/product/restore-preflight-and-recovery-rehearsal.md`, `docs/research/2026-09-20-memorylineage-strict-dominance-plan-audit.md`, `docs/research/2026-09-20-memorylineage-uniqueness-impact-adoption-audit.md`, `docs/research/2026-09-27-3rd-web-hack-evidence-adjusted-scorecard.md`, `docs/submission/claim-matrix.md`, `docs/submission/readiness-register.md`, and `docs/testing.md`.

## Global Constraints

- Do not rewrite or move the published `v1.0.2` tag. The upgrade gets a new version only after its gates pass.
- Keep the central claim narrow: before an agent uses a restored checkpoint, determine whether its committed state is current, known historical, divergent, or unverified under a named authority and evidence source.
- Never claim that MemoryLineage judges whether memory is true, safe, or free of poisoning. Never claim that it prevents every local rollback or proves an external runtime obeyed a decision unless that specific boundary is demonstrated.
- Keep synthetic data in the public demo. Do not put raw checkpoint values, blinding secrets, private locators, prompts, or user data in a receipt, bundle, URL, command-line argument, log, screenshot, or chain transaction.
- A LangGraph wrapper may read and deserialize a checkpoint from its underlying saver before it withholds the value from the graph. Document this boundary: the adapter prevents the graph from receiving an unapproved checkpoint; it does not protect the saver, its storage, or its deserializer.
- A matching response from two RPCs is corroboration, not consensus proof. Do not use `CANONICAL` or equivalent wording for RPC-only evidence.
- Preserve existing receipt and snapshot profile compatibility. New profiles require explicit versioning and old fixtures must continue to replay.
- Keep the first-time journey short. Put runtime setup, cryptographic terms, chain observations, and machine codes behind an optional technical path.
- Do not create, record, render, or commit a new demo video. Retain the owner-supplied YouTube URL as an external link only; keep the pitch PDF as the repository media artifact.
- Do not add a token, DAO, semantic LLM classifier, general agent plugin marketplace, or new chain to improve scores cosmetically.
- Any score comparison is a small-sample heuristic. Publish reviewer count, exact project commits, prompts, criterion notes, and uncertainty; never present it as an official Devpost result.
- Preserve all pre-existing changes in the working tree. The current tree has unrelated local edits; implementation must not reset, overwrite, or stage them incidentally.

---

## Why This Is the Final Upgrade

The latest evidence-adjusted comparison puts MemoryLineage and Forkline within 0.05 points of each other, below the precision of the heuristic. The useful distinction is concrete: Forkline currently shows a fuller local side-effect and reconciliation loop, while MemoryLineage has deeper signed-history and authority checks. Repeating score estimates without a common evaluation method will not close that gap.

This upgrade therefore joins the strengths that are currently separate:

```text
one simple recovery question
  -> a real saved agent checkpoint
  -> a gate before the graph consumes restored state
  -> an authority-aware decision and portable receipt
  -> independent replay from a clean checkout
```

The public-chain boundary remains explicit. A local fixture can prove local replay and gate behavior. A pinned RPC read can prove what that endpoint returned at a block. A consensus-authenticated claim requires a verified chain header and authenticated state/inclusion proofs; that is a separate future proof system, not a wording change.

## Baseline and Boundaries

### Already present

- Solidity registry with ordered transitions, predecessor checks, authorization, authority rotation, and replay protection.
- Rust reference implementation, independent evidence verifier, REVM scenario and mutation checks, browser/CLI replay, recovery receipts, and reference resume gate.
- A polished public story and guided local challenge, plus a stable `v1.0.2` source release.
- A blinded snapshot commitment helper, not yet integrated into the recovery receipt path or a real runtime adapter.

The current comparative scorecard pins MemoryLineage `v1.0.2` to
`80257f74fbb0887fd2c6c5d0fedaeb89bccabf88` and Forkline's reviewed
`forkline-outbox-20260908` source to
`a58fe2c44cc3c8d19ac3300b8b29f91dcb4fd8af`. Task 1 must confirm these are
still the intended snapshots before a new comparison; it must not silently
follow a moving branch.

### Still limiting the next rating

- The runtime loader proof is a local reference integration, not a third-party agent framework.
- The challenge is automated-smoke-tested, but first-time user comprehension has no independent report.
- Offline replay labels registry identity `ADDRESS_FORMAT_ONLY`; it does not authenticate deployed bytecode or canonical chain state.
- The blinded commitment helper has no integrated secret lifecycle and must not be used to imply production-ready privacy.
- Public Pages reachability, external clean-checkout reproduction, and a real user impact measure remain unverified in the current record.

### Product boundary for v1.1

Use LangGraph as the first real framework because its official persistence API exposes a checkpoint retrieval boundary (`BaseCheckpointSaver.get_tuple` and its async counterpart). The adapter wraps that boundary and withholds the tuple from the graph until MemoryLineage verifies the decision. Keep the adapter deliberately narrow: one pinned LangGraph release, one documented checkpoint schema, and one deterministic local demo.

This integration proves a real framework boundary in a local synthetic environment. It does not prove production adoption, protect checkpoint storage from its operator, guard deserialization inside the underlying saver, or verify consensus-finalized Ethereum state.

## File and Responsibility Map

| Area | Files to create or modify | Responsibility |
| --- | --- | --- |
| Snapshot profile and receipt | `crates/ml-memory-store/src/lib.rs`, `crates/ml-spec-types/src/lib.rs`, `crates/ml-verifier-independent/src/lib.rs`, `crates/ml-recovery-gate/src/lib.rs` | Add a versioned blinded checkpoint profile, carry its profile and assurance in the receipt, and fail closed on unsupported or incomplete inputs. |
| Rust process interface | `crates/ml-cli/src/main.rs`, `crates/ml-cli/tests/` or existing CLI tests, `xtask/src/main.rs` | Add a stdin JSON preflight interface so raw synthetic checkpoint data and its secret do not appear in process arguments or temporary files; output only the decision receipt and status. |
| LangGraph integration | Create `integrations/langgraph/pyproject.toml`, `integrations/langgraph/README.md`, `integrations/langgraph/src/memorylineage_langgraph/__init__.py`, `checkpointer.py`, `canonicalization.py`, and `integrations/langgraph/tests/` | Wrap sync and async checkpoint retrieval, canonicalize a supported schema, invoke the Rust decision interface, and return state only on `RESUME_ALLOWED`. |
| Runtime fixture | Create `integrations/langgraph/examples/` and a synthetic fixture manifest under `fixtures/langgraph/` | Demonstrate graph checkpoint, restart, stale restore hold, current restore, receipt export, and offline receipt replay without publishing raw state or secrets. |
| Trust and claim model | `crates/ml-spec-types/src/lib.rs`, `crates/ml-verifier-independent/src/lib.rs`, `docs/submission/claim-matrix.md`, `docs/submission/readiness-register.md`, `docs/product/product-contract.md` | Distinguish local, single-RPC, multi-RPC corroborated, and consensus-authenticated assurance. Pin known registry identity where available, while preserving the limitation for unauthenticated chain state. |
| Beginner and judge path | `apps/inspector/src/pages.rs`, `apps/inspector/src/components.rs`, `apps/inspector/assets/style.css`, `scripts/smoke_web.py`, `scripts/test_smoke_web.py` | Adjust only issues found in independent walkthroughs; keep the core challenge short and machine details optional. |
| Public reproducibility and release | `README.md`, `docs/testing.md`, `docs/reproduction/`, `.github/workflows/`, `docs/submission/release-checklist.md` | Explain the one-command demo, record independent reports, run hosted CI, and prepare a new version only after all gates pass. |
| Comparative evaluation | Create `docs/research/2026-09-27-memorylineage-final-upgrade-evaluation.md` after implementation | Freeze exact commits, evaluator instructions, blind order, criterion notes, and results without modifying scores after seeing them. |

## Implementation Tasks

### Task 1: Freeze the Benchmark and the Submission Boundary

**Files:** `docs/research/2026-09-27-3rd-web-hack-evidence-adjusted-scorecard.md`, `docs/submission/readiness-register.md`, `docs/submission/release-checklist.md`.

- [ ] Record the exact MemoryLineage commit/tag and exact Forkline branch commit used for comparison.
- [ ] Record which links and deployments were reachable on the measurement date. A failed tool access is `UNVERIFIED`, not proof of downtime.
- [ ] Keep official criteria separate from internal usability measures. The event lists Innovation, Technical Feasibility, Uniqueness, and Design; its weights are not available in the reviewed materials.
- [ ] Pre-register identical 1–5 descriptions for both projects. Require each reviewer to cite a visible interaction, source path, test, or artifact for every score.
- [ ] Do not change the existing `v1.0.2` source or claim that the v1.1 work is included in the current submission.

**Acceptance:** A third party can identify the exact versions under comparison and distinguish facts, estimates, and unknown public behavior before implementation begins.

### Task 2: Make a LangGraph Checkpoint the Actual Candidate

**Files:** the Rust snapshot/receipt/gate files and new `integrations/langgraph/` package listed above.

- [ ] Define `memorylineage/langgraph-checkpoint/v1` as an explicit, allowlisted canonical profile. Specify every persisted field that affects the restored graph state and lineage sequence. Reject unsupported serializer versions, missing fields, custom Python objects, and ambiguous values; do not silently omit them.
- [ ] Reuse and integrate `snapshot_commitment_blinded_v1` through the Rust decision path. Bind the profile version, space ID, lineage sequence, and all supported checkpoint fields. Keep the fresh 32-byte secret local and pass it only over stdin to the local Rust process; never place it in argv or receipt output.
- [ ] Add a local secret-provider interface. The demo may use a disposable synthetic secret; production secret backup, rotation, recovery, and multi-host operation remain unavailable until designed and separately validated.
- [ ] Add a typed Rust API for the same canonical snapshot representation so the CLI and adapter use one implementation of commitment and policy rules.
- [ ] Wrap both `get_tuple` and `aget_tuple`. The wrapper must await/read the underlying checkpoint, ask the Rust gate for a receipt, independently verify that receipt, and return it to LangGraph only when the action is exactly `RESUME_ALLOWED`.
- [ ] Ensure holds return a typed outcome (`REHEARSE_ONLY`, `HOLD_FOR_REVIEW`, or `BLOCK_UNVERIFIED`) with a receipt that contains no raw checkpoint values. Exceptions, missing evidence, malformed input, unknown profile, or Rust process failure must stop graph execution.
- [ ] Add a deterministic demo that writes a checkpoint, exits, restarts, and tries the stored state against current, historical, divergent, malformed, and missing-evidence cases.
- [ ] Show a protected graph node counter or equivalent observable effect. It must increment only after the allowed current-head path; held cases must not expose checkpoint values to graph nodes.
- [ ] Capture only aggregate test metadata. Do not log checkpoint payloads, blinding secret, private identifiers, or full stdin.

**Acceptance:** A reviewer runs the LangGraph example from a clean checkout, sees a current checkpoint reach the protected node, sees historical/divergent/invalid checkpoints held before graph consumption, exports a receipt, and verifies it independently with the Rust CLI. A test must demonstrate that failure to launch the Rust gate fails closed.

### Task 3: Strengthen Evidence Identity Without Overstating Chain Trust

**Files:** `crates/ml-spec-types/src/lib.rs`, `crates/ml-verifier-independent/src/lib.rs`, `crates/ml-ethereum/src/lib.rs`, `apps/inspector/src/lib/chain.ts`, claim/readiness docs, and relevant tests.

- [ ] Bind the expected chain ID, registry address, registry runtime bytecode hash, space ID, and observed block number/hash in a versioned trust profile.
- [ ] Verify bytecode against a pinned known artifact and reject a code mismatch. Preserve separate results for “the bundle replays” and “this RPC endpoint reported this registry at this block.”
- [ ] If two RPC providers are used, compare chain ID, block hash, registry code hash, head, and required reads at the same pinned block. Label agreement `MULTI_RPC_CORROBORATED`, not consensus verified.
- [ ] Add negative cases for wrong chain, wrong registry, wrong code, inconsistent block hash, provider disagreement, stale block, and provider unavailability. Unavailable evidence must not become a pass.
- [ ] Keep `CONSENSUS_AUTHENTICATED` unavailable unless a header/finality trust anchor and authenticated state/inclusion proof are implemented and independently checked. Do not scope a full light client into v1.1 by implication.
- [ ] Keep ERC-1271 historical offline verification status explicit. Existing contract execution tests are not the same as independent historical signature replay.

**Acceptance:** Every result has a machine-readable source and assurance label; a valid local replay cannot be relabeled as a live or canonical-chain result; all trust-profile tampering tests fail closed.

### Task 4: Make the First Minute Clear and Demonstrably Useful

**Files:** `apps/inspector/src/pages.rs`, `apps/inspector/src/components.rs`, `apps/inspector/assets/style.css`, browser smoke files, `docs/reproduction/comprehension-protocol.md`.

- [ ] Before changing copy or layout, run the existing one-minute challenge with five people unfamiliar with MemoryLineage. Give no live explanation; record first action, completion time, confusion, and the participant's explanation of the result.
- [ ] Fix only observed blockers. Keep the user-facing question concrete: “Can this restored backup continue the latest shared history?” Keep the story's backup, latest shared state, and allowed/held outcome visible together.
- [ ] Separate “the evidence check passed” from “the old checkpoint is held.” Show technical result codes only on request and state plainly that the demo is synthetic/local.
- [ ] Keep advanced controls, RPC, signatures, and CLI as optional next steps. No first-run user should need to understand blockchain terms to complete the challenge.
- [ ] Verify keyboard-only completion, visible focus, status announcements, 390px layout, desktop layout, and 200% zoom. Preserve a clear direct route for technical reviewers.
- [ ] Repeat the walkthrough with at least five new participants after changes. Record whether each target is met: four of five find the first action within five seconds, four of five complete within one minute without help, and all five understand that MemoryLineage verifies continuity evidence rather than memory truth.

**Acceptance:** The independent report contains the task script, anonymized outcomes, screen dimensions, and exact build commit. If a target misses, the report says so; do not translate an automated smoke pass into a usability claim.

### Task 5: Make Reproduction and Failure Recovery One Command

**Files:** `xtask/src/main.rs`, `.github/workflows/`, `README.md`, `docs/testing.md`, `docs/reproduction/`, package boundary checks.

- [ ] Add a repository-owned command that builds the pinned Rust CLI, creates the synthetic LangGraph checkpoint, runs allowed and held cases, verifies the resulting receipt, and cleans its temporary data on success or failure.
- [ ] Include clear prerequisites, expected terminal output, offline behavior, and a short troubleshooting section. Pin Python and LangGraph dependency versions; do not install packages implicitly in a user environment.
- [ ] Add fixture tests for process interruption, missing secret, incorrect secret, absent evidence, stale state, malformed checkpoint, corrupted receipt, and cleanup after failure.
- [ ] Run the same command from a packaged clean checkout without personal paths, developer-only files, network access, or undocumented manual steps.
- [ ] Require CI to run Rust verification, Python adapter tests, the browser smoke, and package-boundary checks on the same commit. Record hosted CI status; local success alone does not satisfy this gate.
- [ ] Ask two independent developers to reproduce the bounded flow from the package and attach their unedited reports, command transcript, OS/tool versions, and commit hash.

**Acceptance:** One documented command reproduces every local demo result from a clean package; two independent reports succeed without verbal help; hosted CI is green for the release commit.

### Task 6: Reassess the Four Criteria Against Forkline Fairly

**Files:** Create `docs/research/2026-09-27-memorylineage-final-upgrade-evaluation.md`; update the scorecard only after observations are complete.

- [ ] Recruit at least five independent reviewers with a mix of developer and first-time-user backgrounds. Do not use the implementation team as scorers.
- [ ] Give both projects the same time limit, task prompt, device class, network conditions, and access to public materials. Randomize project order and use the exact frozen commits recorded in Task 1.
- [ ] Ask the same comprehension and task-completion questions before asking reviewers to score Innovation, Technical Feasibility, Uniqueness, and Design.
- [ ] Require a short evidence note with every score. Preserve individual ratings and the median; do not remove outliers after seeing results.
- [ ] Use a pre-declared internal target: median at least 4.0/5 in every official criterion, higher median than Forkline in at least three of four criteria, and no criterion more than 0.25 lower. Report failure against the target honestly; do not adjust the threshold retroactively.
- [ ] Separately record demo completion, incorrect interpretations, reproduction success, gate outcomes, and measured local preflight overhead. These are product evidence, not extra official contest criteria.
- [ ] Publish uncertainty and evaluator limits. Five reviewers can expose usability problems but cannot prove a market-wide preference or predict the official outcome.

**Acceptance:** The evaluation is reproducible from its frozen protocol and contains both passing and failing observations. Any “better than Forkline” statement cites the same-panel result and its limitations.

### Task 7: Release v1.1 Only After the Evidence Package Is Complete

**Files:** `README.md`, `docs/submission/claim-matrix.md`, `docs/submission/readiness-register.md`, `docs/submission/release-checklist.md`, `docs/testing.md`, package manifest and release metadata.

- [ ] Keep every current `v1.0.2` artifact and tag intact. Give the new release its own version and changelog entry.
- [ ] Update the landing/demo copy, pitch deck, and public claims only after the adapter and evidence are reproducible. Show the actual LangGraph scenario and make source class and assurance visible.
- [ ] Mark production deployment, real customer memory, market adoption, formal security audit, consensus-authenticated chain state, and historical ERC-1271 offline verification according to their real status.
- [ ] Run the exact repository gate from a clean tree, including `cargo xtask reproduce`, Python integration tests, `cargo xtask release`, and hosted CI. Archive command results with the commit.
- [ ] Check the public website and GitHub release from an unauthenticated session after publishing. Record status, served commit/version, and checks performed; do not infer deployment from a successful upload.
- [ ] Create the new tag only after the release checklist and reviewer evidence agree with the built artifacts.

**Acceptance:** A clean clone of the new tag reproduces the browser flow, LangGraph gate, and independent receipt verification; the public page matches the tagged source and does not make stronger claims than the evidence.

## Rubric-to-Evidence Map

| Criterion | What must become visible | Evidence that can justify a higher internal rating |
| --- | --- | --- |
| Innovation | A specific recovery-authority problem in which operator, authorizer, and auditor can be different roles | The same actual checkpoint decision crosses a public-history policy, independent verifier, and runtime gate; novelty claims stay bounded to that combination |
| Technical Feasibility | A real framework resumes only after a verified decision | Reproduction across restart, allowed/held failure matrix, portable receipt, independent CLI verification, clean checkout, and hosted CI |
| Uniqueness | Clear distinction from generic agent memory, checkpointing, backup, and rollback tools | Exact prior-art boundaries plus evidence that the recovery decision is the key product operation; no “first/only” claim |
| Design | A new user understands the challenge and result without reading technical docs | Independent first-use results, short task completion, accessible control path, legible responsive UI, and no ambiguous “rejected” status |

## Stop Rules and Deferred Work

- If the LangGraph adapter can return a held checkpoint to graph code on any error path, stop release work until that is fixed.
- If canonicalization omits a field that can change resumed behavior, the profile is invalid. Version a complete schema or fail closed; do not silently hash only convenient fields.
- If the blinding secret is unavailable, corrupted, or mismatched, do not compare against a different profile or fall back to an unblinded production claim.
- If RPCs disagree or are unavailable, report the exact state and hold any decision that requires that observation. Never relabel local fixture replay as chain confirmation.
- If external user sessions do not meet the pre-registered targets, revise the flow and rerun; do not mark “intuitive” from team opinion.
- Consensus light client, production secret lifecycle, ERC-1271 historical proofs, arbitrary agent frameworks, production customer data, formal third-party audit, and measured market impact stay future work unless separately scoped with evidence and ownership.
- Do not claim that a high heuristic median guarantees podium placement. Official weights, reviewer preferences, and competitors' complete evidence may be unknown.

## Definition of Done

- [ ] One actual LangGraph checkpoint is the candidate input to the recovery decision.
- [ ] The graph receives restored state only after an independently verified `RESUME_ALLOWED` receipt.
- [ ] Historical, divergent, malformed, unsupported, missing-evidence, and gate-process-failure cases stop before graph consumption.
- [ ] The versioned snapshot profile deterministically binds all supported resume-relevant data and fails closed for unsupported data.
- [ ] Raw synthetic checkpoint data and blinding secrets do not appear in receipts, bundles, logs, URLs, command arguments, or public artifacts.
- [ ] The receipt distinguishes local replay from RPC observation and never asserts consensus without authenticated proofs.
- [ ] A novice can complete the challenge within the pre-registered usability target, or the remaining gap is documented honestly.
- [ ] Two external clean-checkout reproductions and hosted CI pass on the release commit.
- [ ] The same-panel Forkline comparison uses the frozen protocol; results are published even when the target is missed.
- [ ] `v1.0.2` remains immutable and the new release's public claims match its exact artifacts.

## Reference Material

- [3rd-Web-Hack official event page](https://3rd-web-hack.devpost.com/) — rubric labels; no published criterion weights were found in the reviewed page.
- [LangGraph persistence guide](https://docs.langchain.com/oss/python/langgraph/persistence) — checkpoint and persistence concepts.
- [LangGraph `BaseCheckpointSaver.get_tuple` API](https://reference.langchain.com/python/langgraph.checkpoint/base/BaseCheckpointSaver/get_tuple) — synchronous checkpoint retrieval boundary. Implement and verify the async counterpart against the pinned release before supporting it.
