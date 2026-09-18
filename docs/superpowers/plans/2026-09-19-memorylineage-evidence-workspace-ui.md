# MemoryLineage Evidence Workspace UI Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use `superpowers:subagent-driven-development` (recommended) or `superpowers:executing-plans` to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Turn the existing Rust/WASM Inspector into a truthful, multi-page evidence workspace whose core flows—Inspect, History, Tampering Lab, and Verify—remain backed by the current Rust/Solidity evidence while matching the approved internal visual contract.

**Architecture:** Keep the existing protocol/evidence crates and Dioxus version unchanged. Add a thin Inspector presentation/data layer: normalized view models are built once from the existing published replay/evidence and runtime observations; reusable Dioxus shell/components render those models; route selection is explicit and browser-history aware. Keep live Sepolia reads read-only and visibly distinguish them from published fallback evidence. Preserve the current single-page implementation as the behavioral oracle until route and feature parity are verified.

**Tech Stack:** Rust 1.97.1, Dioxus 0.8.0-alpha.1, WebAssembly, `web-sys`/`wasm-bindgen`, existing `ml-spec-types`, `ml-core`, `ml-evidence`, `ml-ethereum`, `ml-local-evm`, CSS, existing Solidity registry, existing fixture/evidence artifacts.

**Spec:** `docs/superpowers/specs/2026-09-19-memorylineage-evidence-workspace-design.md` and the user-provided final UI/product directive.

## Global Constraints

- Do not change Solidity semantics, conformance vectors, fixture contents, revert reasons, mutation expectations, or published evidence to match visual references.
- Do not import or package `internal/design/*.png`; use them only as private visual references.
- Do not fabricate GitHub, Devpost, Etherscan, provider, tag, human-reproduction, timestamp, hash, count, or test-result data.
- Keep primary navigation limited to Inspect, History, Tampering Lab, and Verify; expose supporting pages contextually.
- Label live RPC, published evidence, observed, rejected, out-of-scope, and not-yet-demonstrated states precisely.
- Preserve the current Next.js Inspector as a compatibility/visual oracle until Dioxus parity is checked.
- Keep the release gate and internal-boundary checks intact.

---

## Phase 0: Establish the UI migration baseline

- [x] Inspect the current Dioxus entrypoint, static data types, evidence projection, browser RPC helpers, and CSS before editing.
- [x] Inspect the current visual tokens and component composition and compare the private target images without copying their example data.
- [x] Confirm the existing baseline outputs are recorded in the task notes: `cargo xtask verify` and `npm run verify` both pass.
- [x] Resolve the compatible routing approach for pinned Dioxus 0.8 alpha with a small explicit path adapter that preserves static hosting.

## Phase 1: Build the shared evidence-workspace foundation

- [x] Add the Inspector view-model layer for memory-space, canonical-state, timeline, authority, verification, source, mutation, and artifact/conformance records.
- [x] Normalize current replay, V1/V2 evidence, Sepolia observation/fallback, mutation corpus, conformance output, and authority trace without hardcoding screenshot values.
- [x] Extract reusable Dioxus presentation primitives for the shell, navigation, readouts, hashes, lineage, checks, tables, attack results, evidence workbench, commands, scope, and footer.
- [x] Replace the old dark stage-first tokens with the approved light evidence-workspace tokens while preserving semantic status colors and responsive behavior.
- [x] Add route-aware shell navigation and contextual support links without inventing external URLs.

## Phase 2: Implement the core audit surfaces

- [x] Implement `/inspect` as the current-state workspace with actual network, registry, space, canonical head, source, authority, privacy boundary, and actions.
- [x] Implement `/history` as a temporal review workspace with a canonical rail, authority markers, event log, statistics, selected-state sidebar, and source label.
- [x] Implement `/history/:sequence` as a forensic transition detail page using only fields present in the loaded evidence; unavailable fields are explicit.
- [x] Implement `/lab` and `/lab/:scenario` with the six evidence-backed attack families, live Silent Rollback `eth_call` when available, exact `BAD_PREVIOUS_STATE`, published-corpus fallback, and semantic-poisoning `OUT OF SCOPE` handling.
- [x] Implement `/verify` with published/imported evidence loading, Rust/WASM browser checks, export, one-field tamper workbench, exact `TRANSITION_ID_MISMATCH`, and restore/re-verify behavior.

## Phase 3: Implement supporting argument pages

- [x] Implement `/` as a dense but concise home page that explains the problem, private boundary, canonical overview, Silent Rollback CTA, verification scope, and supporting links.
- [x] Implement `/evidence` from actual deployment, observations, mutation, conformance, reproduction, and artifact records; unfinished external evidence is marked honestly.
- [x] Implement `/architecture`, `/security`, and `/reproduce` from current Rust/Solidity/toolchain behavior and existing documentation, marking historical paths accurately.
- [x] Implement `/prior-work` using the established evidence/provenance grammar; separate prior standards from contributions supported by repository evidence and avoid unsupported “first” claims.

## Phase 4: Verify interaction and presentation quality

- [x] Add loading, RPC failure/fallback, malformed/unsupported evidence, empty/unknown-space, simulation, copy, import/export, and unexpected-result states.
- [x] Add keyboard/focus semantics, readable status text, live verification announcements, reduced-motion support, and mobile layout behavior for route families.
- [x] Compare the implemented workspace composition against the private target grammar and correct hierarchy, density, spacing, and panel relationships without copying reference data.
- [x] Check compact Home/Inspect layouts for page overflow, contained hash/table overflow, vertical timeline fallback, and usable touch targets.
- [x] Keep the `dx serve` limitation accurately documented and verify the static release behavior separately.

## Phase 5: Documentation and regression gate

- [x] Update product/architecture/demo documentation to describe Dioxus/Rust/WASM as the primary website while preserving historical implementation notes.
- [x] Run `cargo fmt --check`, `cargo clippy --workspace --all-targets --all-features`, `cargo test --workspace`, `cargo xtask verify`, and `npm run verify` as applicable.
- [x] Build the Dioxus web release and run the static interaction smoke path for Silent Rollback, Verify, and tampered evidence.
- [x] Run the public-package check on a release-shaped copy and confirm `internal/`, reference PNGs, build outputs, and generated garbage remain excluded.
- [x] Review changed files for fabricated claims, stale routes, protocol changes, leaked private references, and mismatched source labels.

## Self-review checklist

- [x] Every visible value comes from actual source/evidence or is clearly labeled as unavailable.
- [x] Exact machine failures remain visible where the contract produces them.
- [x] Live and published evidence states cannot be confused.
- [x] The website can be understood and exercised without a terminal.
- [x] The Rust/WASM app remains a read-only inspection client; no authoritative backend was introduced.
- [x] Existing protocol and verification gates remain green after UI changes.
