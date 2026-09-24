# Hackathon Submission Finalization Plan

**Goal:** Give a first-time 3rd-Web-Hack reviewer one coherent, truthful, runnable MemoryLineage submission from a named source revision.

**Baseline:** `44a5751` (`v1.0.1`) plus the existing uncommitted submission-envelope and Inspector work. Preserve every existing edit. The prior Top-1 readiness plan remains the technical implementation record; this plan covers the final submission package.

**Evidence boundary:** Demo Space V2 is a synthetic local Rust/revm incident. The earlier Sepolia space is a separate public observation. A public Demo Space V2 deployment, human reproduction, external adoption, remote CI pass, and formal audit require their own evidence.

## Milestone 1 — Stable contribution and privacy story

- [x] Add a concise provenance record showing the repository baseline, the work created for the hackathon, third-party standards, and exact evidence for each claim.
- [x] Update stale research status that still lists the fixed V2 snapshot encoding ambiguity as current.
- [x] Add a versioned secret-blinded commitment API and focused regression tests, without changing the pinned V2 fixture or claiming production privacy.
- [x] Make the README, Devpost copy, claim matrix, and Inspector language consistent with the resulting scope.

**Files:** `crates/ml-memory-store/src/lib.rs`, its focused tests, `docs/product/`, `docs/research/`, `docs/submission/`, `README.md`.

**Acceptance:** focused Rust tests; `cargo xtask verify`; no fixture or evidence drift.

## Milestone 2 — Judge-ready presentation and demo

- [x] Export a concise presentation from `docs/submission/pitch-deck.md` as a viewable slide artifact with no unsupported claims.
- [x] Produce a short local-screen demo or a deterministic live-demo package showing restore selection, `BAD_PREVIOUS_STATE`, evidence tamper/restore, and CLI replay.
- [x] Record the exact artifact source revision and state whether the public site contains it.

**Files:** `docs/submission/` and a bounded demo script under `scripts/` only if existing browser smoke cannot produce the needed screen evidence.

**Acceptance:** inspect every slide and video segment; verify that on-screen roots, labels, and command outcomes match the submission manifest.

## Milestone 3 — Reviewer and publication handoff

- [x] Produce a clean, reproducible release candidate without swallowing unrelated workspace changes.
- [x] Run the repository release, legacy compatibility, submission-bundle, and reviewer-archive gates on the exact candidate revision.
- [x] Prepare an exact publication checklist for GitHub, hosted Inspector, and any optional same-space testnet transaction. Do not present a local replay as a public transaction.
- [x] Keep human comprehension/reproduction and Devpost account submission pending until real external evidence exists.

**Files:** `docs/submission/release-checklist.md`, `docs/submission/README.md`, evidence metadata, and existing release tooling if a concrete gap is found.

**Acceptance:** `cargo xtask release`, `npm run verify`, `ml-cli submission verify`, `git diff --check`, plus clean reviewer archive reproduction if a commit is made.

## Current decision

The official event accepts a short video **or** a live demonstration and asks for a brief presentation. The presentation and demo are submission deliverables; new protocol features are secondary. Publication and public-chain transactions remain distinct external operations after local artifacts are reviewable.

## Local completion record

The committed local candidate is `1c15dc89d434943319bca1234c783ab421636906`.
`cargo xtask release --quiet`, `npm run verify --silent`,
`cargo xtask reviewer-package`, and `cargo xtask reviewer-reproduce` passed.
The eight-page PDF and 45-second video were inspected and are reproducible from
this source revision. The candidate has not been pushed or tagged; the last
recorded hosted release is `v1.0.1`, and current Pages content could not be
rechecked in this session. Devpost upload and external human
comprehension/reproduction remain pending.
