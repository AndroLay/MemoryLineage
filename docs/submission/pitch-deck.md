# MemoryLineage — pitch deck source

This is the judge-facing eight-slide source for a future presentation export.
It intentionally uses only evidence already present in this repository. The
local Demo Space V2 is synthetic and deterministic; it is not presented as a
new Sepolia deployment.

## Slide 1 — The Silent Rollback

### The problem

An AI agent has private memory that changes over time:

```text
snapshot 1  →  snapshot 2  →  snapshot 3
```

After a crash, the operator restores snapshot 1 and tries to continue as if
the newer history never existed.

The question is not whether the operator can restore a local database. The
question is whether the restored snapshot is the authorized continuation of
the committed history.

### Judge takeaway

An operator-controlled local log cannot be the only checkpoint an external
reviewer trusts when the operator also controls the runtime and storage.

## Slide 2 — Why a shared registry?

Signed local records prove that a key signed a record. They do not, by
themselves, give independent parties a shared current predecessor when the
same operator controls the runtime, log, and review process.

MemoryLineage uses an Ethereum registry for the narrow shared boundary:

```text
ordered predecessor
        +
configured authority
        +
publicly replayable commitment
```

Raw memory remains outside the registry. Blockchain is used for the shared
ordering and authorization boundary, not as a database for prompts or private
documents.

## Slide 3 — MemoryLineage

> **Verify the history, not the memory.**

MemoryLineage is an independent auditor for private AI-agent memory history.

It classifies a supplied snapshot against a replay-verified evidence bundle as:

```text
current head
known historical checkpoint
unknown / diverged
unverified
```

It also exposes a Restore Preflight and Recovery Decision Receipt surface so a
future runtime adapter can distinguish a permitted current-head resume from a
historical rehearsal or a review hold.

The current adapter is a reference integration boundary over the supplied
fixture. It is not a production agent loader.

## Slide 4 — One coherent evidence path

```text
synthetic private SQLite snapshots
              ↓
Rust commitment and evidence paths
              ↓
Solidity MemoryLineage registry semantics
              ↓
Rust/revm execution evidence
              ↓
Rust/WASM Inspector
              ↓
portable evidence + independent Rust verifier
```

The portable bundle contains fixed-size commitments, transition identifiers,
authority metadata, and verification observations. It does not contain the
fixture's raw values, private locators, signing keys, or prompts.

## Slide 5 — The falsifiable result

Demo Space V2 has three committed transitions and an authority rotation before
transition 3. The local registry head is transition 3.

The restored candidate uses the actual root derived from transition 1 as the
predecessor for attempted transition 4.

The published Solidity bytecode executed in Rust/revm returns:

```text
REJECTED
BAD_PREVIOUS_STATE
```

This proves the tested stale predecessor cannot be accepted as the next
canonical transition. It does not prevent a local restore or determine what a
separate production runtime loaded.

## Slide 6 — Do not trust the dashboard

The Inspector lets a reviewer:

1. inspect the canonical local evidence head;
2. trace the transition and authority history;
3. run Silent Rollback;
4. export the portable evidence;
5. change one commitment;
6. observe `TRANSITION_ID_MISMATCH`;
7. restore the bundle and observe `VERIFIED`;
8. run the independent Rust CLI verifier.

The static release gate covers all 11 routes, the 390px no-overflow check,
the rollback result, evidence tamper/restore, and Recovery Decision Receipt
tamper/restore.

## Slide 7 — What is original here?

MemoryLineage does not claim to have invented ERC-8350, EIP-712, or ERC-1271.
The submission contribution is the evidence-backed audit product around pinned
semantics:

- Restore Preflight classification;
- policy-bound recovery receipts;
- authority timeline binding;
- independent Rust replay;
- Rust/revm Solidity execution evidence;
- portable evidence import, tamper, restore, and verification;
- a Rust/WASM evidence workspace with explicit scope boundaries.

The project targets canonical succession and authorization of committed private
states. It is not a semantic memory-poisoning detector or a generic AI wallet
guard.

## Slide 8 — Honest boundary and reproduction

### MemoryLineage verifies

```text
sequence continuity
predecessor continuity
authorization rules and authority history
commitment integrity
deterministic replay
```

### MemoryLineage does not verify

```text
semantic truth or safety of memory
AI reasoning correctness
causal memory → action linkage
off-chain availability
every form of memory poisoning
```

The repository provides:

```bash
cargo xtask reproduce
cargo xtask reviewer-package /tmp/memorylineage-reviewer-package.tar.gz
cargo xtask reviewer-reproduce
```

These are automated local/owner-side gates. The public static Inspector is
already hosted; external human reproduction, staging, and the demo video remain
separate submission operations until their evidence is recorded.
