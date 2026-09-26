# Product contract

## Product

MemoryLineage is an independent auditor for private AI-agent memory history.
Its product extension is Restore Preflight and Recovery Rehearsal.

> **Before an agent resumes, does this snapshot match the recorded head, a
> known earlier checkpoint, a divergent state, or insufficient evidence?**

The product tagline remains:

> **Verify the history, not the memory.**

## Real problem and Web3 condition

The complete problem statement, consequences, trust-role condition, adjacent
systems, measurable impact, and reusable submission wording are maintained in
[Problem, Importance, and Impact](./problem-and-impact.md). This contract keeps
the short normative version used by the product.

A persistent agent operator can restore or replace its off-chain memory. If an
independent controller or auditor needs to know which state transitions were
previously authorized, the operator's local database is not a shared source of
ordering. A public registry can supply that shared reference when operator,
authorizer/controller, and auditor are separate trust roles.

If the same party controls the runtime, database, authorization key, and audit
process, a local signed log may be sufficient. MemoryLineage should not claim
that blockchain is necessary for every agent or every backup workflow.

## Hero scenario: The Silent Rollback

The public synthetic fixture contains three SQLite-derived snapshots. The
local Demo Space V2 execution commits transitions `1 -> 2 -> 3`, including an
authority rotation. The fixture is then restored to snapshot `1`. A proposed
transition `4` uses the actual root derived from transition `1` while the
recorded head is transition `3`; the Solidity bytecode executed by Rust/revm
rejects it with the exact reason `BAD_PREVIOUS_STATE`.

The local restore remains possible. This demonstrates that a stale root cannot
be accepted as the next canonical transition under the tested registry rules;
it does not stop local edits, prove what an external runtime loaded, or evaluate
the meaning of memory. Demo Space V2 is local evidence and is not the separate
previously deployed Sepolia space.

## Restore Preflight classifications

| Classification | Current meaning | Product guidance |
| --- | --- | --- |
| `MATCHES DEMO EVIDENCE HEAD` | The selected fixture commitment matches the last checkpoint in the replay-verified local Demo Space V2 bundle. | Show the named local evidence source. Do not imply a live registry read or production resume authorization. |
| `KNOWN HISTORICAL CHECKPOINT` | The commitment matches an earlier checkpoint in that bundle. | Offer isolated Recovery Rehearsal. It is a legitimate older checkpoint, not automatically malicious or current. |
| `UNKNOWN / DIVERGED` | The well-formed candidate does not match a known checkpoint in the verified bundle. | Explain the mismatch; do not attribute who or why. |
| `UNVERIFIED` | Candidate or evidence is malformed, unsupported, empty, or fails independent replay. | Fail closed for the assessment and show the specific failure. |

The UI currently compares public synthetic fixture commitments to local replay
evidence. It does not recompute a production private-memory commitment, read
the browser's SQLite database, or gate an actual agent runtime. The status
`MATCHES DEMO EVIDENCE HEAD` is deliberately narrower than `CANONICAL_HEAD`.

## Recovery Decision Receipt and protected resume gate

The current Rust implementation emits
`memorylineage-recovery-receipt-v2`. The receipt binds the selected snapshot
commitment, snapshot sequence, named evidence bundle, replayed head, decision
classification, recommended action, assurance levels, and explicit limitations.
It also binds the named `strict-authorized-current-head-v2` policy. It contains no
raw memory, private locator contents, or signing material. Its `decisionId` is
recomputed from the receipt body, and the independent Rust verifier rejects
changes to the candidate, policy, evidence context, decision, assurance, or
limitations.

The generic `ml-recovery-gate` adapter runs the same receipt through independent
verification before deciding whether a protected resume may continue. A current
head is resumable only when every transition signature verifies and the
authority timeline is bound. Its loader callback is invoked only after the
decision is `RESUME_ALLOWED`:

```text
CURRENT_HEAD + EOA proof + bound authority timeline -> RESUME_ALLOWED
CURRENT_HEAD + missing/unbound authorization       -> BLOCK_UNVERIFIED
KNOWN_HISTORICAL_CHECKPOINT                        -> REHEARSE_ONLY
UNKNOWN_OR_DIVERGED                                -> HOLD_FOR_REVIEW
UNVERIFIED                                         -> BLOCK_UNVERIFIED
```

Receipt V1 remains readable for compatibility when its resume authorization is
present and verifiable. A V1 receipt cannot authorize resume when the proof is
missing; the protected gate creates and requires the V2 policy.

This is an executable recovery-decision surface and a useful integration
boundary. It is not yet a production agent loader, does not read an arbitrary
runtime's private database, and does not prove that a separate runtime obeyed
the decision. The website labels the source as local Demo Space V2 evidence;
it does not promote the receipt to a live-chain or adoption claim.

## Product flow

```text
present the Backup 1 / Backup 3 incident in plain language
  -> ask for a prediction before revealing any verdict
  -> verify the local evidence and explain the restore decision
  -> disclose the machine reason on request
  -> allow the reviewer to continue into the full audit
```

```text
Inspect evidence source
  -> select restored checkpoint
  -> replay the named history independently
  -> classify current / historical / diverged / unverified
  -> rehearse a historical restore in an isolated path
  -> inspect exact contract/evidence result
  -> emit and verify a recovery decision receipt
  -> export and independently verify the portable bundle
```

## Judge path

The first-time review path is intentionally shorter than the full architecture
description:

```text
Inspect the canonical head
  -> run the old restore rehearsal
  -> reject the stale predecessor with BAD_PREVIOUS_STATE
  -> tamper one evidence commitment
  -> reject with TRANSITION_ID_MISMATCH
  -> restore the original bundle
  -> verify it independently
  -> reproduce the result with the Rust CLI
```

The browser, CLI, and evidence pages use the same Demo Space V2 bundle for
this path. The static release and automated `cargo xtask reproduce` command
make it reviewable without a hosted service; external human reproduction is
still a separate evidence requirement.

## Evidence assurance vocabulary

- `VERIFIED` applies only to the named deterministic checks that ran.
- `OBSERVED` means a named source returned a value; an RPC response is not a
  consensus proof.
- `STRUCTURE_ONLY` means authority records have structurally plausible
  addresses and nonce ordering; it is not historical signature verification.
- `TIMELINE_BOUND` means the signed Demo Space V2 proof set is mapped to an
  explicit effective sequence and config nonce in the published authority
  timeline. It is a bundle-bound consistency result, not a public-chain
  consensus proof.
- `NOT_INCLUDED` means the portable bundle does not contain transition-level
  authorization proof.
- `REJECTED` means a real verifier or execution path rejected the tested input.
- `OUT OF SCOPE` means MemoryLineage did not assess that property.
- `NOT YET DEMONSTRATED` means no evidence is being implied.

The portable verifier independently recomputes commitment IDs, roots, sequence,
predecessor, head, and privacy-boundary checks available in the bundle. The
V2 report labels its scope `OFFLINE_BUNDLE_REPLAY`; its
`registry_identity: ADDRESS_FORMAT_ONLY` result checks address syntax, not
deployed bytecode, authenticated chain state, or the bundle's provenance. The
protocol-corpus projection reports authority history as `STRUCTURE_ONLY` and
transition authorization proof as `NOT_INCLUDED`. Demo Space V2 additionally
contains the typed-data domain, digest, and EOA signature for every transition;
the independent verifier recovers the declared signer and reports
`EOA_SIGNATURES_VERIFIED`. ERC-1271 remains execution evidence rather than
historical offline signer-contract reexecution.

The registry can report its current head, look up a transition when its ID is
known, and emit transition events. It cannot enumerate a space's full history
from the head. Complete-history recovery therefore depends on retained event
logs or evidence bundles; this release has no indexer or history rebuilder.

For ERC-1271, registry acceptance at execution time is distinct from a full
historical offline re-execution of the signer contract. A standalone bundle
must not claim the latter without the relevant historical code/state or an
equivalent supported proof.

## Live Ethereum observation

The optional browser probe is a read-only observation of the existing Sepolia
deployment and a different memory space. It resolves `finalized`, falling back
to `safe` only when allowed by implementation, pins the head read and stale-root
`eth_call` to that block number, then checks the block hash again. The observed
tag, number, and hash are shown with the result. If the tag or identity cannot
be established, no expected rejection is claimed. This is still a single-RPC
observation, not consensus or light-client verification.

The probe accepts `BAD_PREVIOUS_STATE` only when the RPC error includes a
decodable Solidity `Error(string)` payload with that exact reason. Free-form
provider text containing the phrase is insufficient.

## Polkadot Hub portability boundary

The repository now contains a local Polkadot Hub TestNet portability rehearsal
at `evidence/local/polkadot_hub_portability_rehearsal.json`. It executes the
same checked-in Solidity creation bytecode through the Rust/revm lane with the
documented Polkadot Hub TestNet chain ID, compares transition roots, authority
history, and stale-predecessor behavior with the Ethereum-local context, and
confirms that EIP-712 domains remain chain-bound.

This is deliberately labeled `LOCAL_REHEARSAL_PASS`. It is not a Polkadot
deployment, public RPC observation, consensus proof, cross-chain identity, or
claim that one history is canonical on two networks. The official Polkadot
documentation describes the Hub REVM path as EVM-compatible execution for
existing Solidity bytecode; the project therefore treats the current result as
deployment portability preparation, not independent consensus evidence.

## Claims we make

- The Solidity registry enforces ordered transitions, predecessor continuity,
  and configured authorization for a submitted transition.
- The tested stale predecessor in local Demo Space V2 is the actual root
  recorded after snapshot 1; Solidity execution rejects the attempt with
  `BAD_PREVIOUS_STATE`.
- The Restore Preflight classifies only the supplied public synthetic fixture
  against the replay-verified Demo Space V2 history.
- Current V1/V2 offline replay verifies the listed lineage checks and exposes
  its structural-only authorization limitation.
- Raw memory is not required to be published on-chain or included in portable
  evidence.
- A current-head Demo Space V2 receipt is reproduced byte-for-byte by the
  tracked artifact and independently verified by the Rust/WASM and CLI paths.
- The reference CLI gate holds the supplied historical fixture as
  `REHEARSE_ONLY` instead of permitting a protected resume.
- The local Polkadot Hub rehearsal preserves the tested transition, root,
  authority, and stale-predecessor invariants under a separate chain context.

## Claims we do not make

- The browser preflight or reference CLI gate prevents a local restore or
  proves that a production agent runtime was blocked. The CLI gate is a
  reference integration boundary over the supplied fixture, not a production
  runtime integration.
- The private memory is truthful, semantically safe, or available.
- The agent's reasoning is correct or an external action was caused by a
  specific memory state.
- Every rollback or memory-poisoning attack is detected.
- A match against local Demo Space V2 means the current public Sepolia head.
- A single RPC response proves Ethereum consensus.
- The local Polkadot Hub rehearsal proves a public Polkadot deployment,
  network state, cross-chain finality, or cross-chain recovery authorization.
- The current portable evidence independently proves every historical
  authorization signature.
- The ERC-8350 draft is final or immutable.

## Related layers

Content/runtime memory screening and lineage auditing are complementary. A
memory guard can assess whether a write is suspicious; MemoryLineage can assess
whether a committed candidate belongs to an ordered history. Neither result
substitutes for the other. A future runtime adapter may compose them, but only
after its actual resume hook and failure policy are integration-tested.
