# MemoryLineage v1.1.0-rc.2

This evaluation pre-release follows `v1.1.0-rc.1`. The stable GitHub release
and current Devpost submission remain `v1.0.2`. The candidate was fast-forwarded
to `main` at `6f03984`, and that source now serves the static Inspector in Pages
production. This website deployment does not promote the GitHub pre-release to
a stable release.

## Changes since rc.1

- Reviewed the checkpoint format emitted by pinned LangGraph 1.2.12 and added
  support for version 4. Versions 1 and 2 remain supported; version 3 and all
  unknown versions still fail closed.
- Introduced checkpoint canonical profile
  `memorylineage/langgraph-checkpoint/v2`. It binds the persisted `configurable`
  saver identity and the full checkpoint, metadata, parent, and pending-write
  data. It omits LangGraph's per-run `__pregel_runtime` object and top-level
  invocation options, which are not persisted by the saver. The new profile
  name prevents its commitments from being confused with rc.1's v1 projection.
- Fixed `BaseCheckpointSaver` cloning during graph compilation and supplied the
  framework's default serializer when a duck-typed test saver has no `serde`
  attribute.

## Verification

- `cargo xtask langgraph-verify` — PASS with the pinned Python 3.12,
  LangGraph 1.2.12, and SQLite checkpoint 3.1.1 dependencies; all 13 Python
  integration tests passed, including synchronous and asynchronous graph
  resume gates.
- `cargo xtask release --quiet` — PASS, including the Chromium browser smoke
  and public package boundary.
- `npm run verify --silent` — PASS across all nine compatibility and package
  checks.
- Hosted CI for source commit `9ecc54d` — unavailable before workflow steps.
  Run [`36287384859`](https://github.com/AndroLay/MemoryLineage/actions/runs/36287384859)
  and its retry both had `runner_id: 0` and zero steps. They provide no code
  test result.
- The post-push `main` run for `6f03984`,
  [`36288452284`](https://github.com/AndroLay/MemoryLineage/actions/runs/36288452284),
  also ended with `runner_id: 0` and no steps; it provides no test result.

The sandbox's isolated Python 3.12 runtime did not wake its selector reliably
for thread callbacks. A temporary 10 ms test-runner pulse was used only in the
temporary Python environment for the local integration run; no repository
dependency or adapter workaround was added.

## Limits and release status

This is an evaluation candidate, not a production-agent integration. It uses
synthetic local checkpoint data and local REVM evidence. The wrapped saver may
deserialize before the guard sees a checkpoint. The prototype does not provide
a production secret lifecycle or authenticate canonical Ethereum chain state.
Independent developer reproduction, novice comprehension sessions, and a
same-panel comparison with Forkline remain unmeasured; no superiority score is
claimed.

The earlier candidate preview alias is
<https://ml-v1-1-0-rc-2.memorylineage.pages.dev>; it returned HTTP 200 after
deployment of source commit `9ecc54d`. The current production deployment uses
branch `main`, source commit `6f03984`, deployment ID
`2ad1796d-6fac-44ff-b644-37351d6be2a4`, and URL
<https://2ad1796d.memorylineage.pages.dev>. Both it and
<https://memorylineage.pages.dev> return HTTP 200; the production browser smoke
passed. The previous production deployment from `80257f74` remains available
at <https://d001a7dc.memorylineage.pages.dev> and returns HTTP 200. The stable
GitHub release and Devpost submission remain `v1.0.2`; the existing demo stays
on YouTube; the pitch PDF remains the only
presentation file in the repository and release. No new video or audio was
created.
