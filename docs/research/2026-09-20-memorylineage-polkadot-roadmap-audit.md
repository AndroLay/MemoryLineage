# Audit of the MemoryLineage Polkadot Roadmap

Status: **critical concept review**
Date: **20 September 2026**
Scope: **semantic correctness, trust boundaries, portability claims, and fit with the current implementation**

This audit reviews the proposed Polkadot portability roadmap against the
current MemoryLineage source, evidence, and product contract. It does not audit
Polkadot or the Solidity registry formally. Its purpose is to stop the product
story from becoming broader than the evidence.

## Implementation addendum — 21 September 2026

The repository has since completed the first bounded portability milestone.
This addendum supersedes the earlier “no Polkadot crate or evidence” status for
the current checkout; the historical observations above remain useful as the
pre-implementation baseline.

Implemented:

- `crates/ml-portability` runs an explicit local chain-context rehearsal;
- `ml-local-evm` can execute the same checked-in Solidity bytecode with a
  selected EVM chain ID;
- the rehearsal compares transition IDs/state roots, authority history, stale
  predecessor rejection, and EIP-712 domain separation;
- `evidence/local/polkadot_hub_portability_rehearsal.json` records the result;
- `cargo xtask verify` regenerates and validates the report byte-for-byte.

The result is intentionally:

```text
status: LOCAL_REHEARSAL_PASS
deployment: NOT_PERFORMED
publicRpcObservation: NOT_PERFORMED
```

This resolves the encoding/chain-context preparation milestone, but it does
not resolve the later deployment milestone. The project still has no public
Polkadot address, receipt, code-hash observation, destination-domain
signature set, or cross-anchor migration proof. It must therefore continue to
describe Polkadot as local portability preparation rather than deployed
support.

## Audit verdict

The Polkadot direction is viable, but the original wording needs tightening
before implementation. The correct near-term claim is:

> **MemoryLineage can provide an anchor-aware recovery auditor with a portable
> evidence format and a separately observed Polkadot Hub execution path.**

The following stronger claims are not yet justified:

```text
the same history is canonical on Ethereum and Polkadot
the recovery receipt is an authenticated cross-chain authorization
Polkadot REVM proves chain-independent semantics
two anchors provide a stronger consensus or availability guarantee
MemoryLineage is already a production recovery gate
```

The roadmap should proceed only after these boundaries are reflected in the
documentation and acceptance tests.

## Historical pre-implementation repository audit

The pre-implementation checkout was compared with this roadmap on 20 September
2026. The results below are the historical baseline, not the current
post-portability checkout; the implementation addendum at the top supersedes
the Polkadot-status table for the current repository.

### Verification executed

```text
cargo test --workspace       PASS
cargo xtask verify           PASS
cargo xtask release          PASS
```

The successful `cargo xtask verify` run covered:

```text
format
clippy
workspace tests
fixture manifest
Demo Space V2 evidence
recovery preflight and protected resume gate
reference agent runtime
bounded security assurance
pinned conformance
independent replay
revm Silent Rollback
revm mutation lane
revm ERC-1271
revm authority rotation
WASM compile
public package boundary
```

The release gate additionally passed:

```text
static release build
static browser smoke
release package boundary
```

The first release invocation was blocked by the local browser sandbox with
`Operation not permitted`; the same gate passed when rerun with browser/port
access. This is an environment limitation, not evidence of an application
failure.

### Clean-checkout status

`cargo xtask reviewer-reproduce` did not run to completion in this checkout
because the worktree contains uncommitted and untracked research documents.
The command intentionally refuses to package a dirty worktree. This is the
correct safety behavior, but it means that a clean reviewer archive is not
claimed from this exact state.

The current source and release gates pass. The final release process still
needs a committed release state before clean-checkout reproduction can be
recorded.

### Polkadot implementation status at the historical baseline

The repository has no Polkadot runtime adapter or Polkadot evidence at this
point:

| Surface | Current status | Evidence |
| --- | --- | --- |
| Polkadot crate | Not present | Workspace contains Ethereum and local-revm crates only. |
| Polkadot RPC adapter | Not present | `ml-ethereum` remains the read-only Ethereum integration. |
| Polkadot registry deployment | Not present | No Polkadot deployment artifact is in the evidence tree. |
| Polkadot chain ID/code hash | Not present | No anchor observation exists. |
| Multi-anchor Evidence V2 | Not present | Current bundle has one network/registry context. |
| Cross-anchor migration record | Not present | No origin/destination binding exists. |
| Polkadot signatures | Not present | Existing Demo Space V2 authorization proofs are local EIP-712 proofs. |
| PVM implementation | Not present | Remains research-only. |

Therefore the repository currently supports an Ethereum/local-revm product,
not Polkadot portability. This is consistent with the roadmap's proposed
status, but the website and README must not imply otherwise.

### Current recovery implementation status

The recovery gate is stronger than a mock UI but narrower than a generic agent
platform:

```text
SQLite snapshot
    ↓
fixture-profile commitment
    ↓
local Demo Space V2 evidence replay
    ↓
RecoveryDecisionReceipt
    ↓
reference loader gate
```

The tests prove that the reference loader is invoked for the current fixture
head and not invoked for historical, divergent, or invalid fixture evidence.
They do not prove that a third-party agent runtime, remote snapshot store, or
Polkadot observation obeys the gate.

### Current receipt status

The current receipt is useful and independently tamper-checkable. Its verified
properties are:

```text
receipt schema
policy ID
candidate commitment
evidence bundle hash
registry/space/head context
classification
recommended action
assurance values
decision digest
```

Its current limitations are:

```text
no issuer signature over the receipt
no independent proof that the bundle came from an RPC source
no mandatory freshness/finality requirement for every source class
no cross-anchor identity
no post-load runtime receipt
```

That is why it should currently be described as an integrity-bound local
recovery decision receipt. A future Polkadot receipt should add explicit anchor
observation metadata before it is called anchor-authenticated.

### Current data-model gaps relevant to portability

The current implementation also has the following gaps before a second anchor
can be added safely:

1. `snapshot_commitment()` still uses delimiter-based serialization. The
   commitment profile needs an unambiguous encoding and a version migration
   plan.
2. The public action vocabulary distinguishes `BLOCK_UNVERIFIED` from
   `HOLD_FOR_REVIEW`; the roadmap must not introduce `FAIL_CLOSED` as a new
   machine value without a schema decision.
3. The registry is linear. Branch authorization and merge semantics are not
   available.
4. Evidence contains one network/registry context rather than a list of
   anchor observations.
5. Local recovery evidence does not prove snapshot availability or post-load
   state identity.
6. Existing EIP-712 authorization is bound to its current chain ID and
   verifying contract; destination anchors need fresh domain-bound signatures
   or an explicit migration proof.

## Corrected roadmap status matrix

| Claim or capability | Current project | Roadmap status | Correct public wording |
| --- | --- | --- | --- |
| Ordered Solidity registry | Demonstrated | Complete for current Ethereum path | `IMPLEMENTED / EVIDENCED` |
| Local stale-predecessor rejection | Demonstrated with Rust/revm | Complete for local Demo Space V2 | `REJECTED / BAD_PREVIOUS_STATE` |
| Ethereum Sepolia observation | Demonstrated separately | Existing public anchor | `OBSERVED`, not consensus proof |
| Recovery preflight | Demonstrated on supplied fixture | Needs external adapter | `REFERENCE GATE` |
| Recovery receipt | Demonstrated and tamper-checked | Needs anchor authenticity/freshness | `INTEGRITY-BOUND RECEIPT` |
| Polkadot deployment | Not present | Future Phase C | `NOT YET DEMONSTRATED` |
| Polkadot semantic conformance | Not present | Future, after deployment | `NOT YET DEMONSTRATED` |
| Cross-anchor history continuity | Not designed | Separate migration protocol | `OUT OF SCOPE` |
| PVM implementation | Not present | Optional research | `RESEARCH ONLY` |
| External agent adoption | Not demonstrated | Future validation gate | `NOT YET DEMONSTRATED` |
| Formal security verification | Not performed | Separate assurance track | `NOT_FORMALLY_VERIFIED` |

## Required corrections before Polkadot implementation

The roadmap is safe to continue only after these P0 decisions are made:

```text
P0.1  Freeze and version snapshot commitment encoding.
P0.2  Normalize classification/action vocabulary.
P0.3  Define source policy for local, Ethereum, and future Polkadot evidence.
P0.4  Define what a receipt authenticates and what it only hashes.
P0.5  Keep local fixture recovery separate from production runtime claims.
P0.6  Decide whether Polkadot is an independent observation or a migration
      destination. Do not implement both meanings under one field.
```

The first Polkadot milestone should be deliberately narrow:

```text
fresh Polkadot deployment
one dedicated space
one valid transition sequence
one authority rotation
one stale-predecessor eth_call
one code-hash observation
one independently verified bundle
```

It should not include bridges, XCM, branch merge, PVM, or cross-chain
canonicality.

## Critical findings

### C1 — Polkadot Hub REVM is deployment portability, not independent semantics

Deploying the same Solidity bytecode to Polkadot Hub REVM can demonstrate that
the registry is portable to a second EVM-compatible execution environment. It
does not independently validate the protocol semantics in the same way as a
native PVM or separately implemented state machine.

The distinction is:

```text
Ethereum Solidity bytecode
        ↓
Polkadot Hub REVM
        = EVM deployment portability
```

versus:

```text
Solidity registry
        ↓
independent Rust/PVM implementation
        = independent semantic implementation
```

Required correction:

- call the first result `Polkadot Hub REVM deployment portability`;
- reserve `cross-execution semantic conformance` for a separately implemented
  or independently executed semantic path;
- do not treat REVM and Ethereum as two independent protocol implementations;
- do not add PVM merely to make the claim sound larger.

### C2 — There is no cross-chain identity for one memory history

If Ethereum and Polkadot each contain a registry, both can independently accept
the same snapshot commitment. That does not establish that they represent one
shared history. They may be two unrelated histories with equal-looking data.

Before calling a receipt portable across anchors, the protocol needs an
explicit relationship such as:

```text
origin anchor
destination anchor
origin head
destination head
migration epoch
authorized migration record
destination registry identity
```

Without this, a user can receive:

```text
Ethereum: RESUME_ALLOWED
Polkadot: RESUME_ALLOWED
```

while the two chains disagree about which history is authoritative. The product
must therefore distinguish:

```text
independently valid anchor observation
```

from:

```text
cross-anchor continuity
```

The latter requires a migration or bridge protocol and is out of scope for the
first Polkadot milestone.

### C3 — A recovery receipt is currently integrity-bound, not externally authenticated

The current receipt contains a recomputed `decisionId` and binds the decision
to an evidence bundle. This proves internal consistency when the verifier has
the same bundle. It does not by itself prove:

- who produced the receipt;
- that the bundle was fetched from the named chain;
- that the observation was fresh;
- that the observed block was finalized;
- that the registry code at that block matched the expected artifact;
- that an external operator accepted the decision.

Anyone who can construct a structurally valid local bundle can recompute a
hash-based receipt. This is not automatically a vulnerability in the local
fixture, but it is a hard boundary for a portable production receipt.

Required design split:

```text
Receipt integrity
    hash and schema consistency

Anchor observation
    block, code hash, event/head read, finality label

Receipt authenticity
    optional signer or authority attestation

Cross-anchor continuity
    separate migration proof
```

The first Polkadot receipt should be labeled `OBSERVATION_RECEIPT` or
`ANCHOR_BOUND_RECEIPT`, not `cross-chain authorization receipt`.

### C4 — The current recovery gate is a fixture adapter, not a generic agent gate

The current runtime proves that the supplied SQLite reference loader is not
called for held or invalid fixture evidence. That is valuable and real. It does
not prove that an arbitrary LangGraph, OpenClaw, custom Python agent, or
production loader will obey the decision.

The roadmap must keep three claims separate:

```text
Reference loader gate       demonstrated locally
Framework adapter           not demonstrated until built and tested
Production runtime control  not claimed without external integration
```

Polkadot does not solve this gap. A second chain can provide another anchor,
but it cannot enforce what a process does with a private snapshot.

### C5 — The decision vocabulary is inconsistent with the current code

The roadmap uses `FAIL_CLOSED` as an action, while the current implementation
uses `BLOCK_UNVERIFIED` for the unverified case and `HOLD_FOR_REVIEW` for a
divergent case. The product contract and runtime evidence also distinguish
`UNVERIFIED` from `UNKNOWN_OR_DIVERGED`.

This must be normalized before Polkadot support:

```text
classification: UNVERIFIED
recommendedAction: BLOCK_UNVERIFIED
display policy: FAIL_CLOSED
```

If the product wants `FAIL_CLOSED` as the public action, it must be a deliberate
schema migration with compatibility tests. It must not be introduced only in
the roadmap or UI copy.

## High-severity findings

### H1 — Snapshot commitment encoding is not yet unambiguous

The current memory-store commitment serializes key/value fields with delimiter
characters. This creates an ambiguity class for keys or values containing the
same delimiters. The roadmap mentions this, but portability cannot be built on
top of it.

Required gate:

```text
length-prefixed or ABI-defined encoding
collision regression corpus
old fixture migration decision
versioned snapshot profile
```

Do not silently change existing published fixture roots. Either preserve the
old profile as a historical fixture and introduce a new profile, or regenerate
the bundle with an explicitly documented version change.

### H2 — “No raw memory on-chain” does not equal confidentiality

The current fixture commitment is deterministic. If the private state has low
entropy, an observer may guess candidate values and compare hashes. A second
chain does not improve this.

The product should say:

> **Raw memory is not included in the registry or portable evidence.**

It should not say:

> **The commitment provides complete privacy.**

A future production privacy profile needs a deliberate decision about random
nonces, keyed commitments, selective disclosure, or a proof system. Each choice
changes what an independent verifier can verify. This is a separate design
problem, not an automatic consequence of Polkadot portability.

### H3 — The current registry is linear; authorized branches are not implemented

The roadmap lists an “authorized branch” as a possible recovery classification.
The current registry enforces one canonical sequence and predecessor. It does
not provide branch registration, branch authority, merge semantics, or branch
finalization.

Until those semantics exist, the supported classifications should be:

```text
CURRENT_HEAD
KNOWN_HISTORICAL_CHECKPOINT
UNKNOWN_OR_DIVERGED
UNVERIFIED
```

An authorized branch must remain future scope or be described as a separate
protocol extension. A historical checkpoint is not automatically a branch.

### H4 — A commitment does not prove availability or successful loading

The registry can show that a candidate commitment matches a known commitment.
It does not prove that:

- the encrypted snapshot can be retrieved;
- the decryption key is available;
- the loader accepted the snapshot;
- the loaded runtime state equals the preflighted bytes;
- the runtime did not transform the state after verification.

The recovery design needs two receipts if it becomes operational:

```text
Preflight receipt
    candidate was permitted or held before loading

Post-load receipt
    the runtime loaded and reported the same committed state
```

The post-load receipt should be an optional future phase, not implied by the
current preflight result.

### H5 — Chain-bound EIP-712 signatures cannot simply move to Polkadot

The current evidence includes chain ID and verifying contract in the EIP-712
domain. A signature produced for Ethereum is therefore not automatically a
valid authorization on Polkadot.

The migration options are:

```text
fresh authorization on the destination anchor
verified bridge/migration authorization
explicit independent histories
```

The first Polkadot milestone should use fresh destination-domain signatures and
label the histories as independent observations. Reusing Ethereum signatures
would weaken the security model.

### H6 — Authority rotation can diverge independently on two chains

`configNonce` and authorizer history are local to each registry. If Ethereum
rotates to authorizer B while Polkadot remains on authorizer A, the same
candidate can receive different decisions for legitimate reasons.

The UI and receipt must therefore include:

```text
anchor-local authorizer
anchor-local configNonce
effective sequence
authority source
```

A future cross-anchor governance rule may reconcile them, but that is not part
of simple deployment portability.

### H7 — “Same decision” is not enough for cross-anchor conformance

Two implementations can return `BAD_PREVIOUS_STATE` for different reasons or
accept different commitments while producing a similar UI label. Conformance
must compare:

```text
calldata
chain/domain inputs
expected state root
expected transition ID
revert selector/reason
authority state
block observation
```

The roadmap should not use only a `PASS/PASS` result.

## Medium-severity findings

### M1 — Polkadot may increase product complexity faster than user value

A chain selector, two histories, two RPC sources, and two authority timelines
can make the first-time experience harder to understand. The hero story should
remain one recovery incident. Polkadot belongs in the evidence or architecture
surface until it has a concrete user decision to add.

### M2 — Polkadot Hub testnet identity must be pinned before implementation

The roadmap currently names Polkadot Hub generically. Before code is written,
record:

```text
exact network/environment
chain ID
RPC endpoint policy
finality terminology
registry deployment address
code hash
compiler/toolchain
faucet or funding assumption
```

Without this, “Polkadot support” is not reproducible.

### M3 — Multi-anchor observations are not multi-provider consensus

Reading Ethereum and Polkadot from two RPC endpoints gives two observations. It
does not establish consensus between the chains or even independent consensus
inside each chain. The UI must preserve the distinction.

### M4 — Portable evidence needs a trust policy, not only a schema

The evidence format can list multiple anchors, but the verifier must know which
one is authoritative for a decision. Add an explicit policy:

```text
source policy:
  ethereum-primary
  polkadot-primary
  independent-observations
  migration-required
```

Without it, a consumer may accidentally select the first valid anchor in the
bundle.

### M5 — Impact is still unproven

Polkadot portability can improve technical reach and ecosystem relevance. It
does not prove developer adoption, production use, or market demand. Those
require an external runtime integration or independent user reproduction.

## Corrected product wording

Use this wording before the second anchor exists:

> **MemoryLineage is an Ethereum-backed recovery-integrity auditor for private
> agent snapshots. It is designed around an anchor-aware evidence format so the
> same recovery checks can be reproduced in other execution environments.**

Use this wording after Polkadot Hub REVM evidence passes:

> **MemoryLineage provides independently replayable recovery evidence observed
> from Ethereum Sepolia and Polkadot Hub REVM. The observations are
> anchor-specific; they do not by themselves establish one shared cross-chain
> canonical history.**

Use this wording only after a migration protocol exists:

> **MemoryLineage verifies an authorized migration of a committed history
> between anchors.**

## Corrected implementation order

The original order should be revised to:

```text
1. Fix snapshot canonical encoding and version the profile.
2. Normalize decision vocabulary and policy identifiers.
3. Separate receipt integrity, anchor observation, and receipt authenticity.
4. Freeze Ethereum Demo Space V2 evidence.
5. Add chain-neutral anchor types without changing Ethereum output.
6. Add Polkadot Hub REVM as an independently labeled observation path.
7. Use fresh Polkadot EIP-712 signatures.
8. Compare exact calldata, roots, IDs, errors, authority, and block data.
9. Add an anchor-selection policy to evidence and recovery receipts.
10. Test offline/freshness failure as fail-closed behavior.
11. Keep branch/merge, cross-chain migration, bridge proofs, and PVM out of v1.
12. Add one external agent adapter only after the anchor path is stable.
```

## Acceptance gates for Polkadot support

The word `supported` may be used only when all of these pass:

```text
Ethereum behavior unchanged
exact Polkadot environment recorded
registry bytecode and code hash recorded
fresh destination-domain authorization works
valid transition accepted
stale predecessor rejected
authority rotation tested
exact revert reason decoded
event history replayed
head read at a named block/finality context
bundle includes anchor policy
receipt distinguishes integrity from authenticity
offline and stale-observation paths fail closed
independent Rust verifier reproduces the result
no raw memory appears in output
```

If only the bytecode deployment and a head read work, the status is:

```text
POLKADOT DEPLOYMENT OBSERVED
```

If the same corpus also passes, the status is:

```text
POLKADOT REVM CONFORMANCE DEMONSTRATED
```

If a migration proof is added, the status may become:

```text
CROSS-ANCHOR MIGRATION VERIFIED
```

These statuses must not be collapsed into one badge.

## Final assessment

Polkadot remains a sensible second anchor, but it is not automatically the
solution to MemoryLineage's hardest problem. The hardest problems are:

```text
what exactly is canonical?
who authorizes a recovery?
how is a candidate bound to that history?
how fresh and authentic is the observation?
how does a real loader obey the decision?
how are two anchors related?
```

The strongest safe strategy is:

```text
Ethereum-first recovery gate
        ↓
chain-neutral evidence and policy
        ↓
Polkadot Hub REVM observation
        ↓
exact cross-anchor conformance
        ↓
external runtime adapter
        ↓
only then migration or PVM research
```

If implementation stops at the second box, MemoryLineage is still a strong
Ethereum product. If it reaches the fourth box, it has demonstrated meaningful
portability. It should not claim cross-chain recovery until it reaches the
migration-proof stage.
