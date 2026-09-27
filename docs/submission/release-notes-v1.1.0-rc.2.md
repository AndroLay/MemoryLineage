# MemoryLineage v1.1.0-rc.2

This evaluation pre-release follows `v1.1.0-rc.1`. The latest stable release,
production site, and current Devpost build remain `v1.0.2`.

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
- Hosted CI for this candidate — pending. The previous rc.1 hosted attempt
  ended before workflow steps because GitHub assigned no runner.

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

The planned candidate preview alias is
<https://ml-v1-1-0-rc-2.memorylineage.pages.dev>; its deployment status is
tracked in the release checklist. Production remains on stable `v1.0.2`. The
existing demo stays on YouTube; the pitch PDF remains the only
presentation file in the repository and release. No new video or audio was
created.
