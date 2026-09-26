# MemoryLineage v1.1.0-rc.1

This is an evaluation pre-release. The latest stable release and current
Devpost submission remain `v1.0.2`.

## What is included

- An experimental LangGraph checkpointer gate for synchronous and asynchronous
  checkpoint retrieval, using a locally verified Recovery Receipt V3 before a
  checkpoint is returned to graph execution.
- A strict, versioned checkpoint profile and blinded snapshot commitment path,
  with bounded JSON input passed to the Rust CLI over stdin.
- A V3 recovery receipt schema, Rust verifier changes, focused Python tests,
  and a CI workflow intended to run the real LangGraph + SQLite close/reopen
  integration checks.
- Documentation that separates the local prototype from production use and
  records the limits around secret lifecycle, storage deserialization, and
  canonical chain identity.

## Verification status

The local Rust release gate and existing repository verification passed on the
candidate checkout. The Python unit tests that do not require LangGraph passed.
The real LangGraph + SQLite integration tests could not run locally because
this environment lacks Python 3.12 and the pinned packages. The first hosted
workflow attempt and its retry failed before any workflow step started because
GitHub assigned no runner (`runner_id: 0`); this is an unavailable CI result,
not a passing test result. Treat this as a pre-release until the hosted gate
completes successfully.

Reproduce the integration gate with Python 3.12 and the pinned dependencies as
described in [`integrations/langgraph/README.md`](../../integrations/langgraph/README.md):

```bash
python3.12 -m venv .venv
source .venv/bin/activate
python -m pip install -e integrations/langgraph
cargo xtask langgraph-verify
```

## Limits

This adapter uses synthetic local checkpoint data and local REVM evidence. It
does not provide production secret backup or rotation, protect a saver before
deserialization, establish adoption by a production agent, or authenticate a
canonical Ethereum registry state. Do not use real user memory with this
prototype.

The candidate is previewed separately from production at
<https://ml-v1-1-0-rc-1.memorylineage.pages.dev>. Production remains on the
stable `v1.0.2` build. The existing YouTube demo remains external; no new video,
audio, or narration assets are part of this release.
