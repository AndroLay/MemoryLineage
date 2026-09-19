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

The primary user is a developer, auditor, or hackathon judge reviewing the
history of a persistent AI-agent memory space. They need to inspect a public
registry, reproduce a tampering attempt, and verify an exported evidence bundle
without receiving the raw private memory.

## Product Purpose

MemoryLineage Inspector makes committed private-agent-memory history readable
and falsifiable. Its success condition is a first-time visitor completing
Inspect → Silent Rollback → Export → Verify without a verbal briefing.

## Positioning

MemoryLineage audits continuity and authorization of committed memory history.
It is not a semantic memory-safety detector, an AI reasoning evaluator, or a
general agent wallet guard.

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
- Display an ordered transition timeline and commitment fields.
- Replay and verify the published local Silent Rollback evidence in the
  Rust/WASM browser verifier; reproduce the Solidity execution through the
  Rust/revm CLI gate.
- Export and import a portable evidence bundle.
- Verify sequence, predecessor, head, and privacy-boundary invariants in the
  Rust/WASM UI and with the independent Rust verifier.
- Offer a separate read-only Sepolia probe when the browser can reach the
  configured RPC; keep it separate from the local Demo Space V2 incident.
- Do not claim semantic truth, memory safety, causal action proof, or final
  ERC-8350 compliance.

## Evidence on Hand

- `evidence/local/memory_lineage_evm_evidence.json`
- `evidence/local/demo_space_v2_evidence.json`
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
