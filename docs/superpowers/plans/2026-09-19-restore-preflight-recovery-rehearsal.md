# Restore Preflight & Recovery Rehearsal Implementation Plan

> **For agentic workers:** This plan is being executed inline in the current authorized repository workspace; do not push, deploy, stage, or record a demo video.

**Goal:** Add an honest, reproducible Restore Preflight prototype that classifies the existing synthetic snapshots against independently replayed lineage, reports authority assurance without overclaiming, and pins live Sepolia checks to one observed block context.

**Architecture:** Keep the current Solidity contract and V1/V2 evidence formats. Make evidence assurance explicit in the independent verifier, put fixture classification in the Inspector's normalized data layer, and keep live RPC parsing/state selection in the browser layer with pure helpers covered by native tests.

**Tech Stack:** Rust 1.97.1, Dioxus 0.8.0-alpha.1, serde/serde_json, current Ethereum JSON-RPC transport, existing Rust/revm and `cargo xtask verify` gates.

**Spec:** `docs/product/restore-preflight-and-recovery-rehearsal.md`

## Global Constraints

- Preserve current Solidity, evidence files, vectors, fixture bytes, and exact `BAD_PREVIOUS_STATE` semantics.
- Keep the V1 and V2 evidence readers backward compatible; do not silently treat absent authorization signatures as verified.
- Label the current preflight as synthetic Demo Space V2 evidence assessment; it does not gate a real agent runtime.
- Do not treat RPC reads as consensus proof; record one resolved block context and fail visibly if it cannot be confirmed.
- Raw fixture values, private locators, keys, and blinding material must not enter exported evidence.
- Do not push, deploy, stage, or create the demo video.

---

### Task 1: Correct the verifier's authorization assurance status

**Files:**
- Modify: `crates/ml-verifier-independent/src/lib.rs`
- Test: `crates/ml-verifier-independent/src/lib.rs` unit tests
- Modify: `apps/inspector/src/pages.rs` Verify check labels/status tone
- Modify: `docs/testing.md` and `docs/submission/claim-matrix.md`

**Interfaces:**
- Preserve the existing `VerificationReport.authority_history` field for consumers, but return `STRUCTURE_ONLY` for V1/V2 bundles whose authorization rows contain only addresses and nonces.
- Add `VerificationReport.authorization_proof` and report `NOT_INCLUDED` when the imported evidence has no transition signature/proof material.
- Keep `verdict == "VERIFIED"` scoped to the replayed checks; the report must show the authorization limitation alongside that verdict.

- [x] **Step 1: Add regression assertions first.** In `v2_projection_is_verified_independently`, assert `authority_history == "STRUCTURE_ONLY"` and `authorization_proof == "NOT_INCLUDED"`. Add the same assertions to a V1 `current_bundle_is_verified` test.
- [x] **Step 2: Run the focused test and confirm the expected failure.** The new assertions failed against the old `PASS`/missing-field report as expected.
- [x] **Step 3: Implement the truthful report fields.** Both report constructors now return `STRUCTURE_ONLY` and `NOT_INCLUDED`; structural rejection checks remain.
- [x] **Step 4: Update the Verify surface.** It shows `Authority record structure` as `STRUCTURE_ONLY` and `Transition authorization proof` as `NOT_INCLUDED`, without failing the independent lineage verdict.
- [x] **Step 5: Run the focused tests and inspect JSON output.** The verifier tests pass and the CLI JSON reports `VERIFIED` for replay while explicitly marking authority proof absent.

### Task 2: Add deterministic Restore Preflight classification

**Files:**
- Modify: `apps/inspector/src/data.rs`
- Test: `apps/inspector/src/data.rs` unit tests
- Modify: `apps/inspector/src/pages.rs`
- Modify: `apps/inspector/assets/main.css` or the currently active Dioxus stylesheet

**Interfaces:**
- Add `RestoreAssessment` with variants for `EvidenceHeadMatch { sequence }`, `KnownHistoricalCheckpoint { sequence, head_sequence }`, `UnknownOrDiverged`, and `Unverified`.
- Add `classify_restore_candidate(candidate_commitment: &str, bundle: &EvidenceBundleV2) -> RestoreAssessment`; it must call the independent V2 replay before trusting any checkpoint list.
- Add `UiData::assess_snapshot(sequence)` to locate an actual fixture snapshot commitment and pass it through the classifier.

- [x] **Step 1: Add four classifier tests first.** Tests cover evidence-head match, historical checkpoint, unknown/tampered commitment, and invalid replay.
- [x] **Step 2: Run the focused Inspector tests and confirm they fail because the classifier is absent.** The pre-implementation test run failed at the missing classifier API, as expected.
- [x] **Step 3: Implement the pure classifier.** It replays V2 evidence before matching a snapshot commitment to a transition and returns the four bounded classifications.
- [x] **Step 4: Add the Restore Preflight panel to `/inspect`.** Keyboard-operable selection, explicit local synthetic source/limits, tamper/restore copy, and historical rehearsal link are implemented.
- [x] **Step 5: Verify native tests and Dioxus builds.** Inspector tests and the pinned WASM target check pass.

### Task 3: Pin the browser's Sepolia observation to a single block context

**Files:**
- Modify: `apps/inspector/src/browser.rs`
- Modify: `apps/inspector/src/pages.rs` where `LiveRollbackOutcome` is rendered
- Test: `apps/inspector/src/browser.rs` native unit tests

**Interfaces:**
- Add a pure `PinnedBlock` helper containing the selected tag, numeric block number, and block hash.
- Add pure parsers for an RPC block object, a canonical hex quantity, and a block-identity equality check.
- Extend `LiveRollbackOutcome::Rejected` with the observed block number/hash and selected `safe` or `finalized` tag.

- [x] **Step 1: Add pure helper tests first.** Tests cover valid and malformed block metadata, quantity encoding, and block-hash change.
- [x] **Step 2: Run the focused Inspector tests and confirm the helper tests fail to compile because the types/functions are absent.** The pre-implementation run failed on the missing helpers, as expected.
- [x] **Step 3: Implement block selection.** The probe prefers `finalized`, falls back to `safe`, and does not use `latest`.
- [x] **Step 4: Pin every related `eth_call` to the resolved block number.** Head and stale attempt use one number; the block identity is re-read before reporting rejection.
- [x] **Step 5: Display the observation honestly.** The UI names PublicNode RPC, tag, block number/hash, exact `BAD_PREVIOUS_STATE`, and unavailable state separately from local evidence.
- [x] **Step 6: Run native tests and the WASM check.** Tests/build pass; the browser flow is read-only and does not broadcast a transaction. Revert reason acceptance requires decoded Solidity `Error(string)` data, not free-form text.

### Task 4: Align product claims and run the full regression gate

**Files:**
- Modify: `README.md`, `PRODUCT.md`, `docs/product/product-contract.md`, `docs/testing.md`, `docs/submission/claim-matrix.md`
- Review: `apps/inspector/src/pages.rs`, `crates/ml-verifier-independent/src/lib.rs`, `apps/inspector/src/browser.rs`

**Interfaces:**
- Product documentation must distinguish lineage replay, authority proof, RPC observation/finality, and runtime loading.
- Preserve the current local Demo Space V2 versus separate Sepolia observation boundary.

- [x] **Step 1: Add one source-of-truth wording for the preflight statuses and evidence limits.** Product contract, README, testing guide, claim matrix, UI, and full concept document now agree.
- [x] **Step 2: Run formatting and Clippy through the existing xtask gate.** `cargo xtask verify` passes both.
- [x] **Step 3: Run `cargo xtask verify`.** EVM/revm lanes, V1/V2 replay, fixture/evidence reproducibility, WASM compile, and package-boundary checks pass.
- [x] **Step 4: Run browser/static-release smoke.** `cargo xtask release` passes static build, all 11 routes, preflight/tamper/rollback interactions, 390px overflow checks, and release package check. The known `dx serve` limitation remains documented separately.
- [x] **Step 5: Review the final diff.** `git diff --check` passes; no Solidity, vector, fixture, or evidence files changed; release package check excludes private/internal design references.
- [x] **Step 6: Record exact completed and incomplete goals.** Runtime integration, production commitment profile, external human reproduction, hosting/deployment, staging, and video remain explicitly incomplete or out of scope.

## Execution record — 2026-09-19

Latest consolidated result:

```text
cargo xtask release                         PASS
  format / clippy / workspace tests         PASS
  fixture and Demo Space V2 reproducibility  PASS
  pinned conformance / independent replay    PASS
  Rust/revm rollback / mutations             PASS
  Rust/revm ERC-1271 / rotation              PASS
  WASM compile / package boundary            PASS
  static Dioxus build / Chromium smoke       PASS
  all 11 routes / preflight interactions     PASS
  rollback reason / evidence tamper restore  PASS
  responsive 390px page overflow              PASS
```

The browser smoke required loopback permission in this environment. It used a
temporary local static server and Chromium only; it did not deploy or contact a
staging service. Remaining product limits are the fixture-only preflight, no
agent-runtime gate, no production hiding-commitment profile, no transition
signatures in current V1/V2 bundles, single-provider RPC observation, and no
external human clean-checkout reproduction. Hosting, staging, and demo video
remain excluded by scope.
