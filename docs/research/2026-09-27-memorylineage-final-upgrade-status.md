# MemoryLineage final upgrade status

**Scope:** the prototype upgrade aimed at a real agent-checkpoint boundary and
an evidence-based comparison with Forkline. This record tracks the evaluation
pre-release; it is not a claim that MemoryLineage outranks another project.

**Source branch:** `codex/memorylineage-final-upgrade`, fast-forwarded to
`main` at `6f0398424f5f8367f8bcf5c31006da4d922b7683`

**Latest stable baseline:** `v1.0.2`
**Evaluation pre-release:** `v1.1.0-rc.2`
**Video:** no new video, narration, or video source was created or added. The
owner-supplied YouTube URL remains an external link; the pitch PDF remains the
only presentation media stored in the repository.

## Implemented in this branch

- Added a versioned blinded snapshot path and Recovery Receipt V3. The receipt
  still declares that the secret derivation is not independently proven and
  bundle replay does not authenticate registry identity or canonical chain
  state.
- Added a bounded stdin JSON interface to `ml-cli`. Checkpoint values and the
  blinding secret are not placed in command arguments, public evidence, or the
  receipt. Rust-owned secret buffers are zeroized on drop; Python cannot promise
  reliable erasure of immutable temporary strings or runtime copies.
- Added a strict LangGraph checkpoint canonical profile v2 that binds persisted
  saver identity, the complete supported checkpoint object, metadata, parent
  config, and pending writes. Pinned checkpoint versions 1, 2, and 4 are
  accepted; version 3, unknown fields, custom values, non-finite numbers, and
  inputs beyond set limits fail closed.
- Added a LangGraph checkpointer wrapper for synchronous and asynchronous
  retrieval. It independently verifies the receipt and returns a checkpoint
  only for `RESUME_ALLOWED`; historical candidates are held. The underlying
  saver may deserialize the tuple before this wrapper sees it.
- Added deterministic fake-saver tests and real synchronous/asynchronous
  LangGraph + SQLite close/reopen resume tests. Added Python 3.12 dependency pins, a
  `cargo xtask langgraph-verify` command, and CI coverage for `codex/**` pushes.
- Extended the public-package boundary to include the new integration and
  published a separate V3 JSON schema.

## Verification on this checkout

| Check | Result | Limit |
| --- | --- | --- |
| `cargo test -q -p ml-recovery-gate blinded_` | PASS: 2 tests | Rust local fixture only |
| `cargo test -q -p ml-cli --test recover_blinded_stdin` | PASS: 1 test | Rust CLI/local REVM fixture |
| Python canonicalization, adapter, and LangGraph tests | PASS: 13 tests | Python 3.12 with pinned LangGraph 1.2.12 and SQLite checkpoint 3.1.1; includes sync and async SQLite close/reopen graph gates |
| `cargo xtask verify` | PASS | Local Rust, REVM, evidence, WASM, and package gates |
| `cargo xtask release --quiet` | PASS | Formatting, Clippy, workspace tests, evidence and revm gates, WASM, Chromium route/keyboard/responsive smoke, and release package gate |
| `npm run verify --silent` | PASS | All nine existing EVM, Python, Inspector, audit, replay, and package checks |
| `cargo xtask langgraph-verify` | PASS | 13 Python integration tests with the pinned Python 3.12 dependencies; a temporary 10 ms event-loop pulse was needed only for this sandbox runtime's thread callback wakeup |
| Hosted CI for `v1.1.0-rc.1` | UNAVAILABLE BEFORE WORKFLOW STEPS | Run [`36278888260`](https://github.com/AndroLay/MemoryLineage/actions/runs/36278888260) and its retry for code commit `e87be2e` ended with `runner_id: 0` and zero steps; they provide no test result. |
| Hosted CI for `v1.1.0-rc.2` | UNAVAILABLE BEFORE WORKFLOW STEPS | Run [`36287384859`](https://github.com/AndroLay/MemoryLineage/actions/runs/36287384859) and its retry for code commit `9ecc54d` ended with `runner_id: 0` and zero steps; they provide no test result. |

## Still required before calling the upgrade stable or complete

1. A successful hosted run of the pinned LangGraph tests and repository CI on
   the `v1.1.0-rc.2` candidate commit.
2. Two independent developer clean-checkout reproductions and at least five
   first-time user sessions using the pre-registered protocol. These results
   cannot be authored on behalf of participants.
3. A fair same-panel MemoryLineage/Forkline evaluation on frozen commits.
   Until that exists, no score target or claim of overall superiority is met.
4. A production secret lifecycle, an integration that protects the storage and
   deserialization boundary, a supported production saver, and independent
   runtime adoption.
5. Authenticated chain-state evidence for any canonical Ethereum claim. The
   offline verifier continues to prove bundle replay only; RPC agreement would
   be corroboration, not consensus authentication.
6. A successful hosted release gate and clean-checkout reproduction for the
   candidate. The candidate is separate from v1.0.2 and does not silently
   update the Devpost submission.

These remaining gates limit stable-release, production-adoption, and evaluation
claims. The candidate remains a pre-release: the `v1.0.2` tag and Devpost entry
are unchanged. Its static Inspector is now deployed from `main` to Pages
production; this website deployment does not make the candidate a stable
release. No new video was created.

## v1.1.0-rc.1 candidate publication — 27 September 2026

The candidate is published as an annotated Git tag and GitHub pre-release from
`codex/memorylineage-final-upgrade`. The changelog and README keep the stable
`v1.0.2` baseline distinct. The static Inspector candidate is deployed to a
Cloudflare Pages preview branch; production remains on `main` at `v1.0.2`.

Hosted workflow run [`36278888260`](https://github.com/AndroLay/MemoryLineage/actions/runs/36278888260)
and its retry for code commit `e87be2e` both ended before any workflow step
with `runner_id: 0`. Since no remote test ran, this pre-release must not be
treated as fully CI-verified or stable. Local release-gate results are recorded
above; the real LangGraph + SQLite close/reopen gate still needs a successful
hosted or equivalent Python 3.12 run. The Pages preview alias returned HTTP
200; production remains on `main` at stable `v1.0.2`.

## v1.1.0-rc.2 verification update — 27 September 2026

The rc.2 candidate revises the LangGraph canonical profile to v2, based on the
persisted saver configuration, and supports the v4 checkpoint format emitted
by pinned LangGraph 1.2.12 while continuing to reject the unreviewed v3 format.
It also fixes the `BaseCheckpointSaver` clone and default-serializer paths found
by the real integration suite.

On this checkout, all 13 pinned Python integration tests passed, including
real synchronous and asynchronous LangGraph + SQLite close/reopen resume tests.
`cargo xtask langgraph-verify`, `cargo xtask release --quiet`, and
`npm run verify --silent` passed. The isolated Python 3.12 environment needed
a temporary event-loop pulse in its test runner; that diagnostic workaround is
outside the repository. GitHub Actions run [`36287384859`](https://github.com/AndroLay/MemoryLineage/actions/runs/36287384859)
and its retry both ended before test steps (`runner_id: 0`). Cloudflare Pages
deployed code commit `9ecc54d` to the RC2 preview branch; the alias returned
HTTP 200. At the time of this preview publication, the stable `v1.0.2`
production and Devpost references remained unchanged. Independent user
sessions and a fair same-panel comparison remain unmeasured.

## Main merge and Pages production update — 27 September 2026

The candidate branch was fast-forwarded and pushed to `main` at
`6f0398424f5f8367f8bcf5c31006da4d922b7683`. The merged checkout passed
`cargo xtask release --quiet`, `npm run verify --silent`, and the pinned
LangGraph sync/async checkpoint gate. Cloudflare Pages project `memorylineage`
now serves the static Inspector from production branch `main`, source
`6f03984`, deployment ID `2ad1796d-6fac-44ff-b644-37351d6be2a4` at
[`2ad1796d.memorylineage.pages.dev`](https://2ad1796d.memorylineage.pages.dev).
The direct URL and custom domain returned HTTP 200, and the production browser
smoke passed. Previous deployment `d001a7dc` from source `80257f74` remains
available and returned HTTP 200.

GitHub Actions run
[`36288452284`](https://github.com/AndroLay/MemoryLineage/actions/runs/36288452284)
for the pushed commit ended before its first step with `runner_id: 0`; it has
no remote test result. Local gates passed independently. `v1.0.2` remains the
stable tag and Devpost baseline, and `v1.1.0-rc.2` remains an evaluation
pre-release. No stable `v1.1.0` release or Devpost update was made.
