# MemoryLineage Submission Hardening Implementation Plan

> **For agentic workers:** This plan is executed inline in the authorized workspace. Demo video, public deployment, and staging are intentionally excluded.

**Goal:** Turn the existing Rust/Dioxus implementation into a truthful, reproducible, release-ready submission package without changing Solidity semantics or inventing external evidence.

**Architecture:** Keep the Solidity registry, Rust core/verifier, Rust/revm lane, and preserved legacy lanes as compatibility oracles. Make the local Silent Rollback fixture and published protocol corpus explicit in the UI, strengthen Cargo verification, and automate static web/package checks without requiring a backend or deployment.

**Tech Stack:** Rust 1.97.1, Dioxus 0.8.0-alpha.1, Alloy, revm, rusqlite, Solidity 0.8.x, preserved Node/Python compatibility lanes, Chromium CDP smoke script, GitHub Actions.

**Spec:** `docs/superpowers/specs/2026-09-19-memorylineage-evidence-workspace-design.md` and the final UI/product directive supplied in the task.

## Global Constraints

- Preserve exact Solidity semantics and revert reasons, including `BAD_PREVIOUS_STATE`.
- Do not publish raw memory, private locators, secrets, or private design references.
- Do not claim a new Sepolia Demo Space without an actual transaction and readback; deployment is out of scope.
- Keep the website Rust/WASM-first and retain legacy JavaScript/Python lanes as compatibility oracles until parity is proven.
- Use actual repository evidence for all UI values; never copy target-image data.
- Do not add video, deployment, staging, Polkadot, or unrelated product scope.

## Milestones

1. Baseline and evidence model: add a truthful local hero/demo record derived from the real SQLite fixture and existing protocol corpus; make the distinction visible in the UI.
2. Verification automation: strengthen `cargo xtask verify`, add release preparation commands, and add a reproducible Chromium smoke command with static SPA fallback.
3. Product consistency: configure the real GitHub link, add metadata and asset provenance, and align the 11-page design/provenance copy.
4. Release boundary: improve dependency/license/secret/package checks, produce a release manifest command, and update documentation/claim matrix.
5. Regression and integration: run Rust, legacy, browser, package, and release-preparation gates; inspect the diff, commit logically, and push to GitHub.

## Acceptance Criteria

- `cargo xtask verify` executes the Rust gates plus revm lanes, boundary checks, and targeted package checks.
- `cargo xtask smoke-web` starts an isolated static fallback server and verifies required routes plus Silent Rollback and evidence tamper interactions when Chromium is available.
- The Inspector distinguishes local private-memory demo fixture evidence from the four-transition protocol corpus and live/published Sepolia observation.
- The actual GitHub URL is used wherever a GitHub link is rendered.
- Metadata, logo provenance, design contract, and Rust-first docs agree with source behavior.
- No demo video, deployment, staging, fabricated external reproduction, or fabricated final tag is claimed.
- All changed files pass the relevant gates before commit and push.
