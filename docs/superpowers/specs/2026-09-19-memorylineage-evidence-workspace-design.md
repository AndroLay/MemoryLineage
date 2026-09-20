# MemoryLineage Evidence Workspace Design

Status: approved direction, awaiting spec review

Date: 2026-09-19

## Goal

Rebuild the MemoryLineage Inspector presentation as a light evidence
workspace inspired by the owner-supplied references in `the owner-supplied visual reference directory`.
The website remains the primary product. Existing Rust/WASM verification,
Sepolia read-only inspection, published evidence fallback, and exact contract
revert reasons remain the source of truth.

The first viewport must let a visitor understand four things without a
briefing: the current canonical memory history, what remains private, what the
Inspector verifies, and how to run the Silent Rollback demonstration.

## Reference boundary

The screenshots under `the owner-supplied visual reference directory` are visual references
only. They are not runtime assets and their example counts, dates, hashes,
labels, or claims must not be copied into product data.

The repository will keep owner-supplied design material under an ignored
`internal/design/` directory:

```text
internal/design/
├── README.md
├── TARGET-DESIGN.md
└── references/
    └── supplied screenshots
```

The release package must exclude `internal/`. No application code may import
from that directory.

## Product shell

The visual direction changes from the current dark stage-first composition to
a white evidence workspace:

- deep navy top navigation;
- cobalt primary actions and selected navigation;
- pale blue utility surfaces;
- rounded cards with restrained borders and shallow elevation;
- green, rose, and amber states that always include text and an icon or shape;
- compact monospace labels for hashes, sequences, and machine output;
- clear serif or display heading hierarchy with readable sans-serif body text.

The shell keeps the existing product name and tagline:

> Verify the history, not the memory.

The top navigation exposes `Inspect`, `History`, `Tampering Lab`, and `Verify`.
The Sepolia observed status remains visible. A source/GitHub action is shown
only when a real configured URL exists; the UI must not invent a repository
link.

## Surfaces

### Inspect

The Inspect surface is the default entry point. It contains:

- a space and registry readout using the bundled evidence and live observation
  when available;
- the canonical timeline from the real evidence bundle;
- current sequence, transition ID, and state root;
- authority configuration and the local authority-rotation trace;
- an explicit `ON CHAIN` versus `PRIVATE / OFF-CHAIN` boundary;
- `What MemoryLineage verifies` and `What it does not verify` panels;
- the `Run Silent Rollback` action.

The surface must preserve the current live/fallback distinction. A public RPC
failure is shown as `PUBLISHED EVIDENCE`, never as a live success.

### History

History becomes a review workspace with:

- the canonical lineage rail;
- a selected-state detail card;
- an event log derived from the evidence bundle;
- authority history and rotation markers;
- a verification summary for sequence, predecessor, authority, and head.

The displayed state count is derived from real evidence. No screenshot value is
used as a fixture.

### Tampering Lab

Tampering Lab uses a three-column desktop composition that collapses to one
column on narrow screens:

1. attack scenario list;
2. canonical chain, local restore, and attempted transition;
3. exact result and explanation.

The hero scenario is Silent Rollback. The other visible scenarios are the
published corpus cases: sequence gap, parallel history, wrong EOA signer,
locator commitment substitution, and wrong chain domain. The browser may show
published corpus evidence for non-hero cases, but it must not claim that it
broadcast an attack when it did not.

The semantic-poisoning example remains explicitly `OUT OF SCOPE` because a
validly authorized transition can contain unsafe content.

### Verify

Verify becomes an evidence workspace with:

- import and export of the public V2 evidence bundle;
- a verification summary and detailed check list;
- browser-side recomputation;
- a tamper workbench that changes one commitment and shows the exact failure;
- restore and re-verify behavior;
- a clear statement that raw memory is never exported.

The current machine reasons remain unchanged, including
`TRANSITION_ID_MISMATCH` for commitment tampering.

## Data and behavior boundaries

The redesign changes composition and styling, not protocol semantics. These
flows remain unchanged:

- Rust/WASM verifies V1 and V2 public evidence;
- the browser uses read-only Sepolia JSON-RPC for live `eth_call`;
- the app falls back to published evidence when live RPC is unavailable;
- raw memory, private locators, and signing keys never enter public evidence;
- the exact Solidity reason `BAD_PREVIOUS_STATE` remains visible in machine
  output;
- EOA and ERC-1271 claims retain their existing evidence boundaries;
- Next.js remains a migration/UX oracle until Dioxus parity is checked.

No new backend, wallet flow, token, agent model, semantic classifier, or chain
is introduced by this redesign.

## Component approach

The Dioxus app remains the primary implementation. The redesign should first
extract or consolidate shared visual primitives for:

- top navigation and status indicator;
- breadcrumbs and section headings;
- evidence cards and readout rows;
- timeline nodes and connectors;
- semantic status badges;
- check rows and result panels;
- responsive action groups.

The Next.js surface receives the same token and layout changes where practical
so it remains a useful compatibility oracle. Business and verification logic
stays outside presentational primitives.

## Responsive and accessibility contract

- Desktop target: 1440px-wide evidence workspace.
- Compact target: 390px-wide mobile layout.
- Timeline becomes vertical or horizontally scroll-safe on narrow screens.
- Cards stack without hidden critical fields or horizontal page overflow.
- Interactive controls remain keyboard operable with visible focus.
- Status is communicated with text and icon/shape, never color alone.
- Reduced-motion users receive the same state transitions without sweeping
  animation.
- File import, live RPC failure, malformed evidence, tamper failure, and
  successful verification remain understandable at every supported width.

## Verification plan

The implementation is accepted only when the following are demonstrated:

1. Dioxus native and WASM checks pass.
2. Next.js typecheck/build remain green.
3. The static Dioxus release artifact renders at desktop and mobile widths.
4. Silent Rollback still reports `BAD_PREVIOUS_STATE` or clearly labeled
   published-evidence fallback.
5. Verify still reports `VERIFIED` for the published bundle.
6. Tampering one commitment still reports `TRANSITION_ID_MISMATCH`.
7. The public package check excludes `internal/`, generated outputs, and
   reference images.
8. The exact evidence and contract verification gates remain green.

## Rollback boundary

If the visual migration causes functional or accessibility regressions, the
previous CSS and Dioxus composition remain recoverable from the current source
files. The migration must not rewrite evidence structures, contract semantics,
or published fixtures to make the new presentation pass.

