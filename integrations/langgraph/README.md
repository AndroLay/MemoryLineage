# Experimental LangGraph recovery gate

This integration is an unreleased local prototype. It wraps LangGraph's
checkpoint saver retrieval methods and returns a checkpoint to the graph only
after the Rust CLI emits a valid `RESUME_ALLOWED` receipt. A known historical
checkpoint raises `RecoveryHeld`; malformed input, unavailable evidence, and
verifier failures stop with `RecoveryGateError`.

The demo uses synthetic checkpoint data, a disposable local blinding secret,
and local REVM evidence. It does not connect an agent to Ethereum or demonstrate
production adoption, secret backup/rotation, or a production checkpoint store.
The saver may deserialize the checkpoint before the wrapper can withhold it
from graph code. This gate therefore protects delivery to graph execution; it
does not protect the saver, its database, or its serializer.

The integration test enables `LANGGRAPH_STRICT_MSGPACK` before importing
LangGraph. Applications should also configure the serializer with an explicit
safe-type allowlist when reading checkpoint storage that is not fully trusted.

## Reproduce

Use Python 3.12 and the repository's pinned Rust toolchain. Install the
integration explicitly; no project command installs Python packages for you:

```bash
python3.12 -m venv .venv
source .venv/bin/activate
python -m pip install -e integrations/langgraph
cargo xtask langgraph-verify
```

The command builds `ml-cli`, creates three synthetic LangGraph SQLite
checkpoints, closes and reopens the saver, then checks that the latest
checkpoint reaches the graph only after receipt replay while a selected
historical checkpoint is held. The same test covers synchronous and
asynchronous SQLite savers. Private fixture values and the secret are supplied
to the local Rust process over stdin. The public evidence bundle is written to
the temporary test directory; the decision receipt stays in the test process.
The directory is removed when the test exits.

The Rust package and stdlib canonicalizer tests can run without the optional
LangGraph packages:

```bash
PYTHONPATH=integrations/langgraph/src python -m unittest discover \
  -s integrations/langgraph/tests -p 'test_canonicalization.py' -q
```

## Supported checkpoint profile

`memorylineage/langgraph-checkpoint/v1` binds the full checkpoint tuple:
configuration, all checkpoint fields, metadata, parent configuration, and
pending writes. It accepts JSON-compatible values, checkpoint versions 1 and 2,
and a positive `memorylineage_sequence` channel. It rejects unknown checkpoint
fields, unsupported versions, custom Python objects, non-finite numbers,
excessive depth, and values beyond the documented size budget. A LangGraph
format change requires an explicit profile review and version bump.

The recovery secret is a caller responsibility and must remain local. The
receipt contains the blinded commitment and evidence references, not the raw
checkpoint or secret. Python cannot promise reliable erasure of immutable
temporary strings or copies made by its runtime; use synthetic data for this
prototype and do not treat this adapter as a production secret-management
solution.
