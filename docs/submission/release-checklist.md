# Release checklist

This checklist is deliberately evidence-based. `PASS` means the repository
contains a reproducible artifact or a completed local gate. Missing external
evidence remains `NOT YET DEMONSTRATED`.

The frozen evidence baseline, source classes, and per-claim readiness statuses
are recorded in the [Top-1 readiness register](readiness-register.md).

## Repository and product

| Gate | Status | Evidence |
| --- | --- | --- |
| Project release version | PUBLISHED TAG AND RELEASE `v1.0.2` | The annotated tag and GitHub Release use the same sanitized source snapshot; Rust workspace crates retain their independent `0.1.0` package versions. |
| Current upgrade candidate | PRE-RELEASE `v1.1.0-rc.2`, MERGED TO `main` | The annotated tag and GitHub pre-release remain evaluation-only; the source was fast-forwarded to `main` at `6f03984` and its static Inspector is deployed to Pages production. `v1.0.2` remains the stable GitHub release and Devpost baseline. See the [candidate release notes](release-notes-v1.1.0-rc.2.md). |
| Release source history | PURGED AND PUBLISHED | The reachable `main` and `v1.0.2` histories exclude the MP4, WAV, narration, and video production files. Older version tags are unchanged. |
| Release artifacts | PUBLISHED | The ten-page pitch PDF remains in the repository and is attached to the GitHub Release. No video or audio files are attached. |
| Rust workspace and pinned toolchain | PASS | `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml` |
| Dioxus Inspector and first-run routes | IMPLEMENTED / CURRENT CHROMIUM GATE PASS | The routes and challenge interactions are included in `v1.0.2`; the current release gate also rebuilt and smoke-tested the candidate. |
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
| Public static website | DEPLOYED FROM `main`; HTTP 200; BROWSER SMOKE PASS | Wrangler 4.141.0 deployed Pages production from branch `main`, source commit `6f03984`, deployment [`2ad1796d.memorylineage.pages.dev`](https://2ad1796d.memorylineage.pages.dev), ID `2ad1796d-6fac-44ff-b644-37351d6be2a4`. Both this URL and [`memorylineage.pages.dev`](https://memorylineage.pages.dev) returned HTTP 200; `scripts/smoke_web.py --base-url https://memorylineage.pages.dev` passed the route, challenge, accessibility, responsive, tamper, and restore checks. Previous production deployment [`d001a7dc.memorylineage.pages.dev`](https://d001a7dc.memorylineage.pages.dev), source `80257f74`, remains available and returns HTTP 200. |
| Candidate website preview | DEPLOYED; HTTP 200 | Wrangler 4.141.0 deployed earlier candidate source commit `9ecc54d` to branch `ml-v1-1-0-rc-2`; direct URL [`10856758.memorylineage.pages.dev`](https://10856758.memorylineage.pages.dev) and branch alias [`ml-v1-1-0-rc-2.memorylineage.pages.dev`](https://ml-v1-1-0-rc-2.memorylineage.pages.dev) returned HTTP 200. The newer candidate build is now the production deployment from `main`. |
| Pitch deck | PASS / local only | Ten-page [`pitch PDF`](MemoryLineage-3rd-Web-Hack.pdf) labels The Problem, The Solution, The Innovation, The Impact, Current Limitations, and Future Scope; generated from editable HTML and visually reviewed |
| Demo video | URL SUPPLIED; EMBED VISIBLE IN OWNER SCREENSHOT | The project owner supplied <https://youtu.be/K2QUHm4lJCo>; a screenshot shows the Devpost page with an embedded player. Playback is not independently verified. The video and production files are absent from the reachable `main` and `v1.0.2` histories. |
| Secret-blinded commitment helper | PASS / preparation only | A separate opt-in helper binds a V2-encoded snapshot, space ID, and caller secret; no production secret lifecycle or Demo Space V2 migration is claimed |

## Automated verification

Use the annotated `v1.0.2` tag for a clean checkout and fresh reproduction:

```bash
cargo xtask verify
cargo xtask release
cargo xtask reviewer-reproduce
npm run verify
```

The `v1.1.0-rc.2` source passed `cargo xtask release --quiet`, including
formatting, Clippy, workspace and evidence tests, Solidity/revm scenarios,
WASM compilation, public-package checks, static build, Chromium browser smoke,
and release-package boundary. The gate required permission to bind local
Chromium/HTTP sockets in this environment. `npm run verify --silent` also
passed all nine compatibility, Inspector, evidence, and package checks.

`PYTHON=/path/to/python3.12 cargo xtask langgraph-verify` passed the pinned
LangGraph 1.2.12 synchronous and asynchronous SQLite boundary tests (13 tests
total across the integration suite). This sandbox's isolated Python 3.12
selector needed a temporary 10 ms event-loop pulse during test execution and
shutdown; the pulse was installed only in the temporary test environment and
is not a repository dependency or adapter change. A normal Python 3.12 runtime
should run the documented gate directly.

`python3 -m unittest scripts.test_smoke_web -q` passed all 8 tests, including
the regression check that keeps Chromium's Linux singleton socket path below
the operating system limit. The static browser checks cover first-run choices,
six tour steps with spotlights on steps 1–2, the one-minute challenge, the free
Overview-to-challenge link without the tour, route semantics, keyboard focus,
and 390px page overflow. The ten-page PDF was rendered and visually reviewed.
The project owner supplied <https://youtu.be/K2QUHm4lJCo>. The owner-provided
screenshot shows an embedded player on the Devpost page; playback has not been
verified here.

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
| Hosted CI for `v1.1.0-rc.2` | UNAVAILABLE BEFORE WORKFLOW STEPS | The post-push run [`36288452284`](https://github.com/AndroLay/MemoryLineage/actions/runs/36288452284) for `main` at `6f03984` ended with `runner_id: 0` and no steps. Earlier runs [`36287384859`](https://github.com/AndroLay/MemoryLineage/actions/runs/36287384859) and its retry for `9ecc54d` had the same outcome. These provide no code test result; the local gates passed independently on `main`. |
| GitHub Actions for previous `v1.0.1` | BLOCKED BY HOSTED RUNNER | Runs [`35639481534`](https://github.com/AndroLay/MemoryLineage/actions/runs/35639481534), [`35639669674`](https://github.com/AndroLay/MemoryLineage/actions/runs/35639669674), [`35639841420`](https://github.com/AndroLay/MemoryLineage/actions/runs/35639841420), and [`35639973599`](https://github.com/AndroLay/MemoryLineage/actions/runs/35639973599) ended before the first step with `runner_id: 0` |
| Previous release candidate | ARCHIVED | `v1.0.1-rc.3` remains available as the preceding review candidate |
| Previous public release tag | PUBLISHED / prior candidate | `v1.0.1` points to baseline commit `44a5751`; it does not contain this source candidate |
| Previous GitHub Release | PUBLISHED / prior candidate | [MemoryLineage v1.0.1](https://github.com/AndroLay/MemoryLineage/releases/tag/v1.0.1); remote CI for that release ended before runner startup |
| New Sepolia Demo Space V2 deployment | OUT OF SCOPE | Demo Space V2 remains local Rust/revm evidence; the existing Sepolia observation is separate |
| Separate staging environment | NOT PROVIDED | Only the public static website is hosted |
| GitHub source tag and Release page | PUBLISHED | The sanitized annotated `v1.0.2` tag and matching GitHub Release are published; the PDF is the only attached project artifact. |
| Cloudflare Pages deployment | CONFIRMED VIA WRANGLER; HTTP 200; BROWSER SMOKE PASS | Current production is branch `main`, source commit `6f03984`, deployment [`2ad1796d.memorylineage.pages.dev`](https://2ad1796d.memorylineage.pages.dev). The main domain and deployment URL returned HTTP 200, and the production browser smoke passed. The previous production artifact `d001a7dc` from source `80257f74` remains present and reachable (HTTP 200) as the rollback target. |
| Devpost page and demo video | SCREENSHOT PROVIDED; PLAYBACK UNVERIFIED | The owner-provided screenshot shows the project page and embedded video player, and the owner supplied <https://youtu.be/K2QUHm4lJCo>. Video playback and the current Devpost contents remain unverified. Pages reachability and browser flow are verified in the deployment row above. The pitch PDF remains in the repository. |

## Finalization rule

Do not describe the submission as externally reproduced until the human report
exists. The final release identifies the code and evidence baseline; external
reproduction remains a separate evidence gate.
