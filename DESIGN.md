# MemoryLineage Inspector design

## Mode

Operate. The visitor is investigating a committed memory history and needs a
clear path from inspection to falsification to independent verification.

## Visual direction

The Inspector is a light evidence workspace carried by a stage-manager cue
rail. A memory history is a sequence of cues held in a ruled timeline; the
current canonical state is shown in cobalt, a rejected attempt in rose, and
authority or scope warnings in brass. The interface is an incident-review
instrument, not a generic crypto dashboard.

The public website has four primary operational surfaces:

- `Inspect` — current canonical state and source boundary;
- `History` — ordered transitions and authority history;
- `Tampering Lab` — falsifiable attack scenarios;
- `Verify` — portable evidence import, export, tamper, and replay.

Seven supporting surfaces complete the audit argument:

- Home;
- Transition detail;
- Public Evidence;
- Architecture;
- Security and Scope;
- Reproduce;
- Prior Work and Submission Provenance.

The primary navigation stays limited to the four operational surfaces. The
supporting surfaces are reached through contextual links and the footer.

## First viewport contract

Within the first viewport the visitor sees:

- the product sentence “Verify the history, not the memory.”;
- a real private-fixture and protocol-corpus distinction;
- the current evidence source and verdict;
- the raw-memory boundary;
- the `Run Silent Rollback` action.

Reference screenshot values are never runtime fixtures. Counts, roots,
addresses, blocks, and statuses come from repository evidence or a labeled
live observation.

## Tokens

- Navy: `#142238` for the top navigation.
- Ink: `#101a31` for headings and primary text.
- Muted: `#6d80a5` for supporting text.
- Cobalt: `#1769f5` for canonical sequence and active controls.
- Cobalt soft: `#edf4ff` for selected evidence surfaces.
- Border: `#dbe5f3` for hairlines and table rules.
- Surface: `#ffffff` for the main canvas.
- Surface 2: `#f7faff` for supporting panels.
- Success: `#18b979` for verified checks.
- Danger: `#ed5963` for rejected attempts.
- Warning: `#e4a12c` for authority and scope warnings.

A color is never the only carrier of a status. Status text and a shape or icon
remain visible in every state.

## Type and composition

Use a restrained sans-serif family for controls, labels, and body copy. Use a
strong display weight for page headings. Use monospace only for hashes,
addresses, sequences, blocks, error codes, and machine output.

Use a primary workspace, a secondary sidebar, tables, readout rows, and a
timeline. Avoid gradient text, glass decoration, repeated metric-card grids,
random crypto illustrations, and large empty marketing spaces.

## Interaction and states

- Buttons have visible focus, pressed, disabled, and keyboard states.
- Silent Rollback shows loading, then `REJECTED / BAD_PREVIOUS_STATE` from a
  real read-only contract call or an explicit `PUBLISHED EVIDENCE` fallback.
- Evidence verification shows valid, malformed, unsupported, tampered, and
  restored states.
- `OUT OF SCOPE` is reserved for semantic poisoning and similar properties the
  protocol intentionally does not evaluate.
- Reduced-motion users receive the same state changes without sweeping motion.

## Responsive behavior

The main review target is 1440px or wider; the compact target is 390px. Desktop
uses a two-column incident header and wide timeline. At narrow widths, the
timeline becomes a vertical rail or contained scroll region, sidebars stack,
three-column lab content becomes a reading sequence, and hash details wrap or
truncate with a copy affordance. There must be no page-level horizontal
overflow.

## Truth boundary

The UI may show a private snapshot commitment, but it must never expose raw
memory, private locator content, signing keys, or private prompt/context. The
website distinguishes `LIVE RPC / OBSERVED`, `PUBLISHED EVIDENCE`, `VERIFIED`,
`REJECTED`, `OUT OF SCOPE`, and `NOT YET DEMONSTRATED` consistently.

The exact Solidity machine reason `BAD_PREVIOUS_STATE` remains visible even
when the surrounding copy says “stale predecessor”.

## Release boundary

The private references under `internal/design/` are design inputs only. They
are never imported by application code or included in a release archive. The
runtime is rebuilt from Rust/WASM, HTML, CSS, SVG, and the supplied logo asset.
