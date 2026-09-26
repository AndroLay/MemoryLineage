# MemoryLineage final upgrade status

**Scope:** the prototype upgrade aimed at a real agent-checkpoint boundary and
an evidence-based comparison with Forkline. This is a work-status record, not
a new public release or a claim that MemoryLineage outranks another project.

**Branch:** `codex/memorylineage-final-upgrade`

**Published baseline:** `v1.0.2`, unchanged
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
- Added a strict LangGraph checkpoint tuple profile that binds config, the
  complete supported checkpoint object, metadata, parent config, and pending
  writes. Unsupported versions, fields, custom values, non-finite numbers, and
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
| Python canonicalization tests | PASS: 5 tests | Python 3.14.7; stdlib profile only |
| Python adapter/CLI tests | PASS: 4 tests | Python 3.14.7 and fake saver; async subprocess path exercised |
| Full integration unittest discovery | PASS: 9 passed, 2 skipped | The two real-framework methods skip because pinned packages are absent |
| `cargo xtask verify` | PASS | Local Rust, REVM, evidence, WASM, and package gates |
| `cargo xtask release --quiet` | PASS | Static build, Chromium route/keyboard/responsive smoke, and release package gate |
| `npm run verify --silent` | PASS | All nine existing EVM, Python, Inspector, audit, replay, and package checks |
| Real LangGraph/SQLite integration tests | NOT RUN locally | Python 3.12 and pinned packages are absent; package download was blocked by network DNS |
| `cargo xtask langgraph-verify` | BLOCKED AT PREREQUISITE CHECK | Correctly requires Python 3.12 and installed LangGraph packages; does not report skipped framework tests as a pass |
| Hosted CI for this branch | PENDING | The workflow is configured to run after push; no hosted result is claimed before it runs |

## Still required before calling the upgrade complete

1. A successful hosted run of the pinned LangGraph tests and repository CI on
   this exact branch commit.
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
6. A full release gate, clean-checkout reproduction, release metadata, and a
   separate version/tag. The changes in this branch are not part of v1.0.2 and
   do not silently update the Devpost submission.

These remaining gates limit the product and evaluation claims; they do not
prevent publishing this implementation branch for review. Push of the branch
does not merge to `main`, change the v1.0.2 tag, deploy Pages, or publish a new
video.
