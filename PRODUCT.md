# Product

<!-- impeccable:product-schema 1 -->

## Platform

web

## Stack

Rust 1.97.1, Dioxus Web/WASM, the Rust evidence and verification crates,
read-only Alloy host access plus a small Rust/WASM browser JSON-RPC transport,
and the Solidity Ethereum registry. The
preserved Next.js/React Inspector and Python replay lane are compatibility and
migration oracles, not the primary product path.

## Users

The primary user is an operator restoring a persistent AI-agent snapshot and an
independent controller or auditor who must review that recovery decision. They
need to determine whether the candidate matches the current evidence head, a
known earlier checkpoint, a divergent state, or insufficient evidence—without
receiving raw private memory.

## Problem

The operator may control the agent's off-chain memory database. Restoring an
older backup can move that local snapshot behind states already committed to a
registry. The operator's local database alone does not establish which
checkpoint is the shared canonical continuation when another party controls
authorization or audits recovery.

## Product Purpose

MemoryLineage Inspector makes committed private-agent-memory history readable
and falsifiable. Restore Preflight classifies a candidate against a named,
replay-verified history; Recovery Rehearsal explains how a known older snapshot
differs from the recorded head; and a Recovery Decision Receipt makes the
classification portable. The Rust CLI includes a reference protected-resume
gate that permits only a current-head decision and holds historical or
unverified candidates. The current prototype is still limited to public
synthetic fixture/evidence and does not gate an external agent runtime. Its
success condition is a first-time visitor understanding the restore mismatch,
its evidence source, and the exact limit of the result without a verbal
briefing.

## Judge path

The root page introduces the problem, the recovery-check approach, the limits
of the local evidence, and the Inspector before offering two clear ways in:
`Get started` opens the guided flow at `/app`; `Explore freely` opens the
technical overview at `/overview` without starting the tour. The tour is
optional and can be exited at any point. Its first two
steps show the incident and prediction challenge as full-page views with a
spotlight on the existing action area: the challenge entry on Home, then the
answer and check controls on Challenge. Spotlight continues through Inspect,
History, Tampering Lab, and Verify; it never invents a target when a page has
none.
Supporting technical pages stay available through normal navigation after the
tour or through direct use. The tour asks the visitor to make and check the
incident decision, then replay the local rollback before advancing through the
evidence review. The free overview also links directly to the one-minute
challenge without starting the tour.

The review should still fit one short incident rather than a tour of every
technical surface:

```text
Open the MemoryLineage landing page
  -> choose Get started or Explore freely
  -> if exploring freely, open the one-minute challenge from the overview
  -> predict whether restored Backup 1 is current
  -> check the local evidence
  -> learn that Backup 1 is older than the shared Backup 3 head
  -> open the exact machine reason if desired
  -> continue into the technical review

Restore snapshot 1 locally
  -> compare it with the committed state-3 head
  -> receive BAD_PREVIOUS_STATE
  -> tamper one portable commitment
  -> receive TRANSITION_ID_MISMATCH
  -> restore the bundle and receive VERIFIED
```

Home starts with a one-minute predict-then-check challenge and withholds its
verdict until the visitor checks the local evidence. The Overview preserves the
full technical explanation. Inspect identifies the evidence head, History
shows the authority-bound succession, Tampering Lab executes the negative
path, and Verify lets a reviewer replay the same bundle independently. The automated
`cargo xtask reproduce` command covers the repository path; external human
reproduction remains a separately recorded evidence item.

## Positioning

MemoryLineage audits continuity of committed memory history and describes
authorization assurance at the level actually supported by each evidence
source. The local Demo Space V2 includes independently recovered EOA EIP-712
signatures for all three transitions; the separate protocol-corpus projection
remains `STRUCTURE_ONLY` with signature proof `NOT_INCLUDED`. The product does
not prevent local restores, gate an external runtime, assess semantic
truth/safety, explain agent reasoning, or prove that an external action was
caused by a particular memory state.

## Operating Context

The unified local hero workflow uses an off-chain SQLite snapshot fixture with
states 1, 2, and 3, Rust memory-store tooling, three Solidity transitions,
authority rotation, and a Rust/revm stale-predecessor rejection. Its attempted
transition uses the actual root produced by transition 1 while the local
registry head remains at transition 3. The previously deployed Sepolia space
and its read-only observations are separate evidence; no claim is made that
Demo Space V2 was written to Sepolia. Raw fixture contents stay outside the
evidence bundle.

## Capabilities and Constraints

- Inspect current sequence, state root, registry, authority, and evidence source.
- Classify the selected synthetic checkpoint as a match to the Demo Space V2
  evidence head, a known historical checkpoint, unknown/diverged, or unverified.
- Offer an explicit local Recovery Rehearsal path for a historical checkpoint;
  never describe it as automatically malicious.
- Display an ordered transition timeline and commitment fields.
- Replay and verify the published local Silent Rollback evidence in the
  Rust/WASM browser verifier; reproduce the Solidity execution through the
  Rust/revm CLI gate.
- Export and import a portable evidence bundle.
- Export, import, tamper, and independently replay a Recovery Decision Receipt
  bound to the named V2 evidence bundle.
- Run the generic `ml-recovery-gate` protected-resume adapter through the CLI:
  current head is `RESUME_ALLOWED`; a historical checkpoint is
  `REHEARSE_ONLY`; unknown or unverified input is held or blocked before the
  loader callback is invoked.
- Verify sequence, predecessor, head, and privacy-boundary invariants in the
  Rust/WASM UI and with the independent Rust verifier.
- Keep authority record shape separate from offline cryptographic
  authorization proof. Demo Space V2 includes EIP-712 EOA proof material;
  protocol-corpus projections without that material remain structural-only.
- Offer a separate read-only Sepolia probe when the browser can reach the
  configured RPC; keep it separate from the local Demo Space V2 incident.
- Do not claim semantic truth, memory safety, causal action proof, or final
  ERC-8350 compliance.
- Do not claim production commitment privacy or runtime prevention from the
  fixture-scoped preflight.

## Evidence on Hand

- `evidence/local/memory_lineage_evm_evidence.json`
- `evidence/local/demo_space_v2_evidence.json`
- `evidence/local/demo_space_v2_recovery_receipt.json`
- `fixtures/silent-rollback-v2/manifest.json`
- `evidence/sepolia/sepolia_deployment.json`
- `evidence/sepolia/sepolia_reread.json`
- `contracts/vectors/erc8350_conformance.json`
- `contracts/mutations/mutations.json`
- Existing Solidity, Rust/revm, and historical Python verification lanes.
- A separate 4-transition protocol corpus and 20-case mutation matrix; these
  remain distinct from the 3-transition Demo Space V2 incident.

The external human rerun and demo video remain separate submission work; the
Inspector product surface and its release artifact are present in the current
repository.

## Product Principles

- Demonstrate the failure mode before explaining the protocol.
- Make every visible claim traceable to evidence or label it as a fixture.
- Keep raw memory private and show that boundary explicitly.
- Prefer one complete audit journey over a wide feature list.
- Make rejection reasons understandable without hiding their exact code.

## Accessibility & Inclusion

The Inspector must support keyboard operation, visible focus, semantic buttons
and headings, sufficient contrast, reduced-motion preferences, responsive
layouts, and status announcements for simulation and verification results.
