# Release checklist

This checklist is deliberately evidence-based. `PASS` means the repository
contains a reproducible artifact or a completed local gate. Missing external
evidence remains `NOT YET DEMONSTRATED`.

The frozen evidence baseline, source classes, and per-claim readiness statuses
are recorded in the [Top-1 readiness register](readiness-register.md).

## Repository and product

| Gate | Status | Evidence |
| --- | --- | --- |
| Project release version | PUBLISHED TAG `v1.0.2` | The annotated repository tag points to the release commit below; Rust workspace crates retain their independent `0.1.0` package versions. |
| Release source commit | PUSHED | `56ed787a678c8267dd4410db463a7ac177f90bc7`; pushed to `main` and tag `v1.0.2` on 26 September 2026. |
| Release worktree contents | INCLUDED IN `v1.0.2` | The reviewed UX, free-overview challenge entry, screenshots, PDF, and 120-second narrated video are in the tagged source commit. |
| Rust workspace and pinned toolchain | PASS | `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml` |
| Dioxus Inspector and first-run routes | IMPLEMENTED / BROWSER GATE UNSTABLE | The routes and challenge interactions are included in `v1.0.2`; see the automated verification record below for Chromium smoke results. |
| Local preview | LAST RECORDED PASS | `http://127.0.0.1:8080` was previously served by `cargo xtask serve-web`; it was not rechecked during the v1.0.2 publication step. |
| Judge-facing first minute | IMPLEMENTED / NOT USER-VALIDATED | The answer is withheld until check; the result separates check status from the restore decision and exposes the exact machine reason afterward. No novice sessions or 4/4 heuristic scores are claimed. |
| Silent Rollback exact result | PASS | `BAD_PREVIOUS_STATE` in local Demo Space V2 Rust/revm evidence |
| Evidence tamper/restore | PASS | `TRANSITION_ID_MISMATCH` then `VERIFIED` in browser smoke |
| Recovery Decision Receipt | PASS | Current and historical receipts plus Rust/WASM/CLI checks |
| Protected resume example | PASS / local only | `cargo run -q -p ml-agent-runtime --example protected-resume-flow` permits a signed current head, holds a current head missing authorization and historical/divergent snapshots, and rejects invalid evidence before the loader |
| Local submission envelope | PASS / local only | `evidence/submission/manifest.json` and `ml-cli submission verify` bind source labels, hashes, incident roots, fresh Rust/revm execution, and receipts; `submissionCommit` is `null` because the exact commit must be recorded outside its own content |
| Polkadot Hub portability rehearsal | PASS / local only | Nested Ethereum and target-context bundles replayed by the independent Rust verifier; deployment and public RPC remain `NOT_PERFORMED` |
| Public package boundary | PASS | `scripts/check-public-package.sh --release` |
| Reviewer archive tooling | PASS / automated local only | `cargo xtask reviewer-package` and `cargo xtask reviewer-reproduce` passed from a clean committed candidate |
| Public static website | DEPLOYMENT UNVERIFIED | The `main` push may trigger hosting automation. The latest request received Cloudflare HTTP 403, error 1010; this does not establish whether deployment completed or what the live site serves. |
| Pitch deck | PASS / local only | Ten-page [`pitch PDF`](MemoryLineage-3rd-Web-Hack.pdf) labels The Problem, The Solution, The Innovation, The Impact, Current Limitations, and Future Scope; generated from editable HTML and visually reviewed |
| Demo video | PASS / owner-reviewed locally; upload pending | Revised 120-second [narrated cut](MemoryLineage-narrated-demo.mp4) keeps the challenge result distinct from the Lab replay, improves guided-result legibility, and gives each supporting page its own caption and narration. The 1:58 WAV uses local male Kokoro TTS at 1.08×. The project owner reviewed and accepted the cut; no independent voice-quality assessment is claimed. The public site match and Devpost upload remain to be confirmed |
| Secret-blinded commitment helper | PASS / preparation only | A separate opt-in helper binds a V2-encoded snapshot, space ID, and caller secret; no production secret lifecycle or Demo Space V2 migration is claimed |

## Automated verification

The v1.0.2 release commit is `56ed787a678c8267dd4410db463a7ac177f90bc7`.
Run these commands from a clean checkout for a fresh reproduction:

```bash
cargo xtask verify
cargo xtask release
cargo xtask reviewer-reproduce
npm run verify
```

On the release commit, `cargo xtask release --quiet` passed formatting, Clippy,
workspace tests, fixture and evidence checks, portability replay, recovery and
runtime gates, Solidity/revm scenarios, WASM compilation, package boundary, and
static build. It then failed at the Chromium step with
`Chromium page debugging target did not start`; therefore the combined release
gate did not exit successfully. A standalone browser smoke passed once with
Chromium stderr captured to a temporary file, while other standard smoke
attempts could not start the page target. Treat the browser smoke as unstable,
not as a consistently passing gate.

`python3 -m unittest scripts.test_smoke_web -q` passed all 8 tests, including
the regression check that keeps Chromium's Linux singleton socket path below
the operating system limit. The static browser checks cover first-run choices,
six tour steps with spotlights on steps 1–2, the one-minute challenge, the free
Overview-to-challenge link without the tour, route semantics, keyboard focus,
and 390px page overflow. The ten-page PDF was rendered and visually reviewed.
The narrated video was checked for H.264/AAC streams and full decode; the owner
reviewed and accepted the synthetic narration and final cut. Neither file has
been uploaded to Devpost.

These results document the tagged local source and evidence. They do not
substitute for novice comprehension sessions, external clean-checkout
reproduction, a successful hosted CI run, or confirmation of the public Pages
deployment.

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
| Remote CI for v1.0.2 source commit | FAILED BEFORE ANY STEP | Run [`36248726652`](https://github.com/AndroLay/MemoryLineage/actions/runs/36248726652) and its retry ended with no steps or assigned runner; they provide no CI test result for the commit. |
| GitHub Actions for previous `v1.0.1` | BLOCKED BY HOSTED RUNNER | Runs [`35639481534`](https://github.com/AndroLay/MemoryLineage/actions/runs/35639481534), [`35639669674`](https://github.com/AndroLay/MemoryLineage/actions/runs/35639669674), [`35639841420`](https://github.com/AndroLay/MemoryLineage/actions/runs/35639841420), and [`35639973599`](https://github.com/AndroLay/MemoryLineage/actions/runs/35639973599) ended before the first step with `runner_id: 0` |
| Previous release candidate | ARCHIVED | `v1.0.1-rc.3` remains available as the preceding review candidate |
| Previous public release tag | PUBLISHED / prior candidate | `v1.0.1` points to baseline commit `44a5751`; it does not contain this source candidate |
| Previous GitHub Release | PUBLISHED / prior candidate | [MemoryLineage v1.0.1](https://github.com/AndroLay/MemoryLineage/releases/tag/v1.0.1); remote CI for that release ended before runner startup |
| New Sepolia Demo Space V2 deployment | OUT OF SCOPE | Demo Space V2 remains local Rust/revm evidence; the existing Sepolia observation is separate |
| Separate staging environment | NOT PROVIDED | Only the public static website is hosted |
| GitHub source push and version tag | PUBLISHED | Commit `56ed787a678c8267dd4410db463a7ac177f90bc7` and annotated tag `v1.0.2` were pushed; no separate GitHub Release page was created |
| Cloudflare Pages deployment | UNVERIFIED | The latest request received Cloudflare HTTP 403, error 1010; deployment completion and served version cannot be inferred from that response |
| Devpost media upload / live-demo update | NOT YET PERFORMED | Local pitch PDF and demo MP4 exist; no Devpost upload was performed, and current Pages content was not rechecked |

## Finalization rule

Do not describe the submission as externally reproduced until the human report
exists. The final release identifies the code and evidence baseline; external
reproduction remains a separate evidence gate.
