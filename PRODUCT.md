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

The hero workflow uses an off-chain SQLite snapshot fixture with states 17, 18,
and 19, Rust memory-store tooling, the Rust/revm execution lane, curated local
EVM evidence, and an observed Ethereum Sepolia deployment. The browser uses a
read-only Sepolia `eth_call` for the live Silent Rollback rehearsal when the RPC
is available, with published evidence shown explicitly when it is not. Raw
fixture contents stay outside the on-chain evidence path.

## Capabilities and Constraints

- Inspect current sequence, state root, registry, authority, and evidence source.
- Display an ordered transition timeline and commitment fields.
- Run a real local Silent Rollback contract simulation.
- Export and import a portable evidence bundle.
- Verify sequence, predecessor, head, and privacy-boundary invariants in the
  Rust/WASM UI and with the independent Rust verifier.
- Use a public Sepolia read when the browser can reach the configured RPC, with
  published evidence as an explicit fallback.
- Do not claim semantic truth, memory safety, causal action proof, or final
  ERC-8350 compliance.

## Evidence on Hand

- `evidence/local/memory_lineage_evm_evidence.json`
- `evidence/sepolia/sepolia_deployment.json`
- `evidence/sepolia/sepolia_reread.json`
- `contracts/vectors/erc8350_conformance.json`
- `contracts/mutations/mutations.json`
- Existing Solidity, Rust/revm, and historical Python verification lanes.

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
