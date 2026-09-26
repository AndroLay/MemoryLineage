# Release checklist

This checklist is deliberately evidence-based. `PASS` means the repository
contains a reproducible artifact or a completed local gate. Missing external
evidence remains `NOT YET DEMONSTRATED`.

The frozen evidence baseline, source classes, and per-claim readiness statuses
are recorded in the [Top-1 readiness register](readiness-register.md).

## Repository and product

| Gate | Status | Evidence |
| --- | --- | --- |
| Project release version | TARGET `v1.0.2` | Repository release identifier for this source, Inspector, and submission package; Rust workspace crates retain their independent `0.1.0` package versions. |
| Last recorded source snapshot | LOCAL `main` = `origin/main`; remote not freshly queried; no newer tag | `497e8238915e6070c2bcb7fe3dd72a37ebc7860b` |
| Current prototype worktree | LOCAL / NOT COMMITTED | The reviewed UX, free-overview challenge entry, refreshed screenshots, PDF, and video are local changes on top of the pushed baseline. |
| Rust workspace and pinned toolchain | PASS | `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml` |
| Dioxus Inspector and first-run routes | PASS / CURRENT LOCAL RELEASE GATE | Landing → Get started tour, Explore freely → `/overview`, then its one-minute challenge link, all without enabling the tour; current release smoke passes. |
| Local preview | PASS / RUNNING | `http://127.0.0.1:8080` is served by `cargo xtask serve-web`; direct browser smoke passes with exactly one landing page. A stale earlier dev-server process was replaced. |
| Judge-facing first minute | IMPLEMENTED / NOT USER-VALIDATED | The answer is withheld until check; the result separates check status from the restore decision and exposes the exact machine reason afterward. No novice sessions or 4/4 heuristic scores are claimed. |
| Silent Rollback exact result | PASS | `BAD_PREVIOUS_STATE` in local Demo Space V2 Rust/revm evidence |
| Evidence tamper/restore | PASS | `TRANSITION_ID_MISMATCH` then `VERIFIED` in browser smoke |
| Recovery Decision Receipt | PASS | Current and historical receipts plus Rust/WASM/CLI checks |
| Protected resume example | PASS / local only | `cargo run -q -p ml-agent-runtime --example protected-resume-flow` permits a signed current head, holds a current head missing authorization and historical/divergent snapshots, and rejects invalid evidence before the loader |
| Local submission envelope | PASS / local only | `evidence/submission/manifest.json` and `ml-cli submission verify` bind source labels, hashes, incident roots, fresh Rust/revm execution, and receipts; `submissionCommit` is `null` because the exact commit must be recorded outside its own content |
| Polkadot Hub portability rehearsal | PASS / local only | Nested Ethereum and target-context bundles replayed by the independent Rust verifier; deployment and public RPC remain `NOT_PERFORMED` |
| Public package boundary | PASS | `scripts/check-public-package.sh --release` |
| Reviewer archive tooling | PASS / automated local only | `cargo xtask reviewer-package` and `cargo xtask reviewer-reproduce` passed from a clean committed candidate |
| Public static website | AUTO-DEPLOYMENT / LIVE STATUS UNVERIFIED | No manual Pages deployment was run; a Git push occurred, but whether hosting automation deployed it and what the live site serves could not be verified |
| Pitch deck | PASS / local only | Ten-page [`pitch PDF`](MemoryLineage-3rd-Web-Hack.pdf) labels The Problem, The Solution, The Innovation, The Impact, Current Limitations, and Future Scope; generated from editable HTML and visually reviewed |
| Demo video | PASS / owner-reviewed locally; upload pending | Revised 120-second [narrated cut](MemoryLineage-narrated-demo.mp4) keeps the challenge result distinct from the Lab replay, improves guided-result legibility, and gives each supporting page its own caption and narration. The 1:58 WAV uses local male Kokoro TTS at 1.08×. The project owner reviewed and accepted the cut; no independent voice-quality assessment is claimed. The public site match and Devpost upload remain to be confirmed |
| Secret-blinded commitment helper | PASS / preparation only | A separate opt-in helper binds a V2-encoded snapshot, space ID, and caller secret; no production secret lifecycle or Demo Space V2 migration is claimed |

## Automated verification

Run from a clean checkout:

```bash
cargo xtask verify
cargo xtask release
cargo xtask reviewer-reproduce
npm run verify
```

Latest local gate after the onboarding and free-overview changes:
`TMPDIR=/var/tmp cargo xtask release --quiet` PASS, including formatting,
Clippy, workspace tests, evidence replay, WASM compile, package boundary,
static build, Chromium smoke, and release package boundary.
`TMPDIR=/var/tmp npm run verify --silent` PASS for the nine-step compatibility
lane (EVM, local audit, replay, Python, Inspector typecheck/build, and package
boundaries). The browser smoke also exported current desktop/mobile captures;
its checks cover the first-run choices, six tour steps with spotlights on steps
1–2, the one-minute challenge, the free Overview-to-challenge link without the
tour, route semantics, keyboard focus, and 390px page overflow. The ten-page
PDF was rendered and visually reviewed; the 37-second video was recorded from
current guided browser interactions, then checked for its H.264/AAC streams
and reviewed at opening, evidence-tamper, and restored-result frames. In this runner `/tmp` produces SQLite `disk I/O error`,
so the local commands use `TMPDIR=/var/tmp`; an independent SQLite write and
the runtime test both pass there.

The 120-second narrated video and separate 1:59 voice track were created after
those release gates. Their media metadata and full video decode were checked;
human listening review of TTS pronunciation and perceived naturalness remains
pending. Neither file has been uploaded to Devpost.

These checks prove the current local worktree, not a clean pushed commit or a
public deployment. They do not substitute for novice comprehension sessions,
external clean-checkout reproduction, or hosted CI.

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
| Remote CI for source candidate | NOT RECHECKED FOR THIS REVISION | This local UX review did not query GitHub Actions for `497e8238915e6070c2bcb7fe3dd72a37ebc7860b` |
| GitHub Actions for previous `v1.0.1` | BLOCKED BY HOSTED RUNNER | Runs [`35639481534`](https://github.com/AndroLay/MemoryLineage/actions/runs/35639481534), [`35639669674`](https://github.com/AndroLay/MemoryLineage/actions/runs/35639669674), [`35639841420`](https://github.com/AndroLay/MemoryLineage/actions/runs/35639841420), and [`35639973599`](https://github.com/AndroLay/MemoryLineage/actions/runs/35639973599) ended before the first step with `runner_id: 0` |
| Previous release candidate | ARCHIVED | `v1.0.1-rc.3` remains available as the preceding review candidate |
| Previous public release tag | PUBLISHED / prior candidate | `v1.0.1` points to baseline commit `44a5751`; it does not contain this source candidate |
| Previous GitHub Release | PUBLISHED / prior candidate | [MemoryLineage v1.0.1](https://github.com/AndroLay/MemoryLineage/releases/tag/v1.0.1); remote CI for that release ended before runner startup |
| New Sepolia Demo Space V2 deployment | OUT OF SCOPE | Demo Space V2 remains local Rust/revm evidence; the existing Sepolia observation is separate |
| Separate staging environment | NOT PROVIDED | Only the public static website is hosted |
| GitHub source push / new release | LAST RECORDED SNAPSHOT / NO NEW TAG OR RELEASE | Local `main` and `origin/main` both point to `497e8238915e6070c2bcb7fe3dd72a37ebc7860b`; the remote could not be freshly queried in this environment. `v1.0.1` remains the last confirmed release |
| Cloudflare Pages deployment | NO MANUAL DEPLOY / AUTO-DEPLOY UNVERIFIED | No manual deployment was attempted; the live site could not be fetched to determine whether a push-triggered build completed |
| Devpost media upload / live-demo update | NOT YET PERFORMED | Local pitch PDF and demo MP4 exist; no Devpost upload was performed, and current Pages content was not rechecked |

## Finalization rule

Do not describe the submission as externally reproduced until the human report
exists. The final release identifies the code and evidence baseline; external
reproduction remains a separate evidence gate.
