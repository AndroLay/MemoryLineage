# MemoryLineage Inspector design

## Mode

Operate. The visitor is investigating a committed memory history and needs a
clear path from inspection to falsification to independent verification.

## Visual world

The Inspector uses a stage-manager cue board as its visual carrier. A memory
history is a sequence of cues held in a ruled horizontal rail. The current
canonical state is lit in cool white and cobalt; a rollback attempt enters as a
rose warning cue and stays visible when rejected. The interface is an
instrument for an incident review, not a generic crypto dashboard.

## First viewport contract

Within the first viewport the visitor sees:

- the product sentence “Verify the history, not the memory.”;
- a real four-state evidence timeline;
- the current verdict `VERIFIED`;
- the raw-memory boundary;
- the `Run Silent Rollback` action.

## Tokens

- Ink: `#17212b` for the dark stage surface and `#f4f0e7` for paper surfaces.
- Cobalt: `#355c91` for canonical sequence and active controls.
- Rose: `#b9574f` for rejected or dangerous attempts.
- Brass: `#b08342` for warnings and authority changes.
- Sage: `#5e8b73` for verified results.
- Hairlines: `rgba(23, 33, 43, 0.16)` on paper and `rgba(244, 240, 231, 0.16)` on ink.

## Type and composition

Use a warm serif stack for the main statement and a restrained sans stack for
controls. Use a monospace stack only for hashes, sequences, and machine output.
The timeline is the main composition; panels support it with a single elevation
language and 12px corners. Avoid gradient text, glass decoration, and repeated
metric-card grids.

## Interaction and states

- Buttons have visible focus and pressed states.
- The rollback action shows `RUNNING`, then `REJECTED` with the exact contract
  reason.
- Live Sepolia inspection shows `LIVE` or an explicit `PUBLISHED EVIDENCE`
  fallback when the browser cannot reach the RPC.
- Evidence verification shows loading, valid, malformed, and tampered states.
- Reduced-motion users receive the same state changes without animated sweeps.

## Responsive behavior

Desktop uses a two-column incident header and a wide timeline. At narrow widths
the timeline becomes a vertical cue rail, controls remain full-width, and hash
details wrap without horizontal scrolling.
