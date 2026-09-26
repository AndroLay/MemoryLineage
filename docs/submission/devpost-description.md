# Devpost description draft

This draft is the short judge-facing description for MemoryLineage. It keeps
the same wording as the repository and claim matrix. The submission project
version is `v1.0.2`; the preceding public Inspector release was `v1.0.1`. The
last recorded Inspector URL is [memorylineage.pages.dev](https://memorylineage.pages.dev);
current page content was not rechecked. This draft does not claim external
adoption, semantic memory safety, or authorship of ERC-8350.

## Problem

Persistent AI agents keep private memory in storage controlled by the same
operator running the agent. After a crash or restore, that operator can load an
older snapshot and present it as the next state. A normal database log gives an
external reviewer no independent checkpoint boundary: the operator still owns
the history being reviewed.

The question is precise:

> Is this restored snapshot the authorized continuation of the committed agent
> history, a known earlier checkpoint, a divergent state, or insufficiently
> evidenced?

## Solution

MemoryLineage is a local-first Restore Preflight prototype and memory-history auditor.
It commits fixed-size state evidence to a Solidity registry and keeps raw
memory off-chain. A Rust/WASM Inspector shows the canonical head, authority
history, and exact failure reason. A separate Rust verifier can replay the
portable evidence without trusting the website.

The judge-facing incident is The Silent Rollback:

```text
private snapshot 1 restored locally
canonical committed history already at state 3
attempted transition 4 uses the state root after transition 1
Solidity result: BAD_PREVIOUS_STATE
```

The local Demo Space V2 is a deterministic synthetic SQLite fixture. It is
executed against the published Solidity bytecode in Rust/revm, includes an
authority rotation, and carries EIP-712 EOA proof material for its three
transitions. Portable evidence contains commitments and proof metadata, not
the fixture's raw memory values.

## Why Web3

The operator running the agent should not be the only party trusted to preserve
the audit history. An append-only registry gives an external reviewer a
predecessor and authorization boundary while the private memory remains
off-chain. The browser and CLI label local evidence, public-chain observations,
and unsupported claims separately.

A signed local log can identify its signer, but it does not by itself give
independent parties a shared current predecessor when the operator controls the
runtime and the log storage. MemoryLineage uses the registry for that shared
checkpoint and authority boundary; it does not put the memory payload itself on
chain.

## What is original in this submission

MemoryLineage does not claim to have invented ERC-8350, EIP-712, or ERC-1271.
The submission contribution is the independently implemented audit product
around pinned semantics:

- Restore Preflight classification for current, historical, divergent, and
  unverified candidates;
- policy-bound current-head and historical recovery receipts;
- authority timeline binding for Demo Space V2 EIP-712 proofs;
- Rust reference and independent replay paths;
- Rust/revm Silent Rollback and mutation evidence;
- portable evidence import, tamper, restore, and offline verification;
- a Rust/WASM evidence workspace that exposes the security boundary directly.

## Evidence

The repository includes:

- a Solidity registry with sequence, predecessor, EIP-712, ERC-1271, and
  authority-rotation behavior;
- a 20-case protocol mutation corpus;
- deterministic SQLite fixture regeneration;
- current and historical Recovery Decision Receipts;
- a static Dioxus website with 14 evidence-workspace route paths, including a short first-run challenge;
- `cargo xtask reproduce` for the automated clean-checkout path;
- an independent Rust verifier and preserved Python/JavaScript compatibility
  lanes.

## Boundaries and future scope

MemoryLineage verifies committed ordering, predecessor continuity, configured
authorization, authority history, commitment integrity, and replayable
evidence. It does not determine whether private memory is semantically true or
safe, whether an AI reasoned correctly, whether an agent's action was caused by
that memory, or whether an external runtime obeys the reference recovery gate.

The next evidence step is external developer reproduction. The repository now
contains a ten-slide [pitch PDF](MemoryLineage-3rd-Web-Hack.pdf), a
two-minute [narrated local demo](MemoryLineage-narrated-demo.mp4), and the
original 37-second cue-only cut. The narration is English TTS generated locally;
it is not a human recording. The demo follows the guided first-run flow. The last
recorded public Inspector URL is
[memorylineage.pages.dev](https://memorylineage.pages.dev); this workflow did
not deploy source candidate commit `497e8238915e6070c2bcb7fe3dd72a37ebc7860b`; the onboarding updates in the current working tree are not part of that commit, and
current page content was not rechecked. `VERIFIED` in the CLI
report is scoped by `OFFLINE_BUNDLE_REPLAY`: the verifier checks bundle
consistency and address syntax, not canonical registry provenance. Complete
history recovery requires retained event logs or bundles; historical ERC-1271
replay is unsupported, and the interactive Sepolia probe uses one RPC endpoint.
Media upload, hosted-site update, and separate staging are not claimed here.
