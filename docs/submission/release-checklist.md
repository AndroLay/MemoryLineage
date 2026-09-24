# Release checklist

This checklist is deliberately evidence-based. `PASS` means the repository
contains a reproducible artifact or a completed local gate. Missing external
evidence remains `NOT YET DEMONSTRATED`.

The frozen evidence baseline, source classes, and per-claim readiness statuses
are recorded in the [Top-1 readiness register](readiness-register.md).

## Repository and product

| Gate | Status | Evidence |
| --- | --- | --- |
| Source candidate | PUSHED TO `main` / NOT TAGGED | `675c707405ac2afea1fd067be44a67890b0a35f2` |
| Rust workspace and pinned toolchain | PASS | `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml` |
| Dioxus Inspector routes | PASS | 11 routes in `apps/inspector/src/main.rs` and static browser smoke |
| Silent Rollback exact result | PASS | `BAD_PREVIOUS_STATE` in local Demo Space V2 Rust/revm evidence |
| Evidence tamper/restore | PASS | `TRANSITION_ID_MISMATCH` then `VERIFIED` in browser smoke |
| Recovery Decision Receipt | PASS | Current and historical receipts plus Rust/WASM/CLI checks |
| Protected resume example | PASS / local only | `cargo run -q -p ml-agent-runtime --example protected-resume-flow` permits a signed current head, holds a current head missing authorization and historical/divergent snapshots, and rejects invalid evidence before the loader |
| Local submission envelope | PASS / local only | `evidence/submission/manifest.json` and `ml-cli submission verify` bind source labels, hashes, incident roots, fresh Rust/revm execution, and receipts; `submissionCommit` is `null` because the exact commit must be recorded outside its own content |
| Polkadot Hub portability rehearsal | PASS / local only | Nested Ethereum and target-context bundles replayed by the independent Rust verifier; deployment and public RPC remain `NOT_PERFORMED` |
| Public package boundary | PASS | `scripts/check-public-package.sh --release` |
| Reviewer archive tooling | PASS / automated local only | `cargo xtask reviewer-package` and `cargo xtask reviewer-reproduce` passed from a clean committed candidate |
| Public static website | AUTO-DEPLOYMENT / LIVE STATUS UNVERIFIED | No manual Pages deployment was run; a Git push occurred, but whether hosting automation deployed it and what the live site serves could not be verified |
| Pitch deck | PASS / local only | Eight-page [`pitch PDF`](MemoryLineage-3rd-Web-Hack.pdf), editable [HTML source](pitch-deck.html), and visual page inspection |
| Demo video | PASS / local only | 45-second [captioned video](demo-video.md) from five static-browser states; not uploaded to Devpost |
| Secret-blinded commitment helper | PASS / preparation only | A separate opt-in helper binds a V2-encoded snapshot, space ID, and caller secret; no production secret lifecycle or Demo Space V2 migration is claimed |

## Automated verification

Run from a clean checkout:

```bash
cargo xtask verify
cargo xtask release
cargo xtask reviewer-reproduce
npm run verify
```

The static Dioxus release path and Chromium smoke are the supported browser
release path. Local development is supported through `cargo xtask serve-web`,
which disables the pinned Dioxus alpha's incompatible Rust hot-patch path;
`cargo xtask smoke-dev-web` repeats that browser check. Direct
`dx serve --web` must include `--hot-patch false`.

Dependency checks currently report no known RustSec or npm production
vulnerabilities. RustSec still reports two transitive maintenance warnings:
`derivative` and `paste`.

## External and submission gates

| Gate | Status | Why |
| --- | --- | --- |
| External human clean-checkout report | NOT YET DEMONSTRATED | `evidence/reproduction/` contains no self-authored report |
| Remote CI for source candidate | NO RESULT RETRIEVED | Candidate `675c707405ac2afea1fd067be44a67890b0a35f2` was pushed to `main`; no CI result for this push could be retrieved |
| GitHub Actions for previous `v1.0.1` | BLOCKED BY HOSTED RUNNER | Runs [`35639481534`](https://github.com/AndroLay/MemoryLineage/actions/runs/35639481534), [`35639669674`](https://github.com/AndroLay/MemoryLineage/actions/runs/35639669674), [`35639841420`](https://github.com/AndroLay/MemoryLineage/actions/runs/35639841420), and [`35639973599`](https://github.com/AndroLay/MemoryLineage/actions/runs/35639973599) ended before the first step with `runner_id: 0` |
| Previous release candidate | ARCHIVED | `v1.0.1-rc.3` remains available as the preceding review candidate |
| Previous public release tag | PUBLISHED / prior candidate | `v1.0.1` points to baseline commit `44a5751`; it does not contain this source candidate |
| Previous GitHub Release | PUBLISHED / prior candidate | [MemoryLineage v1.0.1](https://github.com/AndroLay/MemoryLineage/releases/tag/v1.0.1); remote CI for that release ended before runner startup |
| New Sepolia Demo Space V2 deployment | OUT OF SCOPE | Demo Space V2 remains local Rust/revm evidence; the existing Sepolia observation is separate |
| Separate staging environment | NOT PROVIDED | Only the public static website is hosted |
| GitHub source push / new release | PUSHED / NO NEW TAG OR RELEASE | The source candidate is on `main`; `v1.0.1` remains the last confirmed release |
| Cloudflare Pages deployment | NO MANUAL DEPLOY / AUTO-DEPLOY UNVERIFIED | No manual deployment was attempted; the live site could not be fetched to determine whether a push-triggered build completed |
| Devpost media upload / live-demo update | NOT YET PERFORMED | Local pitch PDF and demo MP4 exist; no Devpost upload was performed, and current Pages content was not rechecked |

## Finalization rule

Do not describe the submission as externally reproduced until the human report
exists. The final release identifies the code and evidence baseline; external
reproduction remains a separate evidence gate.
