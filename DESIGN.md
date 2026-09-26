# MemoryLineage Inspector design

## Mode

Operate. The visitor is investigating a committed memory history and needs a
clear path from inspection to falsification to independent verification.

## Visual direction

The Inspector is a dark evidence workspace carried by a stage-manager cue
rail. A memory history is a sequence of cues held in a ruled timeline; the
current canonical state is shown in restrained cobalt, a rejected attempt in
coral, and authority or scope warnings in amber. The interface is an
incident-review instrument, not a generic crypto dashboard.

The visual hierarchy follows one audit argument:

```text
incident → canonical history → attempted change → exact verdict → replayable proof
```

Every page keeps the same space, selected state, source class, and evidence
boundary visible where they are relevant. Supporting detail is progressively
disclosed through readout rows, event tables, and forensic links instead of
competing with the primary decision.

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

## Root landing page (`/`): Persuade

The root page uses a bolder, editorial product-story composition: oversized
plain-language headline, the synthetic Backup 1 versus Backup 3 lineage visual,
a factual proof strip, alternating problem and workflow sections, role-based
context, an Inspector preview, evidence limits, FAQ, a strong closing action,
and an open-source footer. It borrows the scale, product-preview placement,
section rhythm, and repeated calls to action from the
[Beehiiv homepage](https://www.beehiiv.com/), while keeping MemoryLineage's
dark evidence palette and its own product language. It does not borrow
Beehiiv's testimonials, customer numbers, publisher logos, pricing, or product
claims. The public page keeps synthetic data and evidence limits visible.

The footer names the actual MIT license and its copyright holders. It does not
claim exclusive rights that conflict with the repository's open-source grant.

`Get started` opens the optional guided tour at `/app`; `Explore freely` opens
the technical overview at `/overview` without the tour. The top navigation
links to landing sections and retains the GitHub icon button. The landing page uses synthetic
local examples and does not expose the challenge's machine verdict before the
visitor tries it.

## Guided first-run flow

The tour shows progress and offers Back, Next, Exit, and Finish controls. Its
first two steps stay on full-page views and spotlight their real action areas:
the Home challenge entry, then the Challenge answer and check controls. The
spotlight does not crop or zoom the page, and no highlight is fabricated when a
step has no target. It continues through the existing Inspect, History,
Tampering Lab, and Verify work areas. Challenge and Lab still require their
relevant actions before Next is enabled. The tour is optional and does not lock
technical or supporting routes.

The first-run challenge asks the visitor to predict whether the restored
snapshot is current, then checks local evidence. Before that action, it does
not show the result or machine reason. Afterward, the evidence check and
restore decision are labeled separately in plain language; the machine reason
is available in a collapsed disclosure. The complete technical overview and
audit workspace remain reachable after the challenge.

Reference screenshot values are never runtime fixtures. Counts, roots,
addresses, blocks, and statuses come from repository evidence or a labeled
live observation.

## Tokens

- Navy: `#081321` for navigation and the deepest shell surface.
- Ink: `#f4f7fb` for headings and primary text.
- Muted: `#a9b9ce` for supporting text.
- Cobalt: `#78aaff` for canonical sequence and active controls.
- Cobalt soft: `#142b47` for selected evidence surfaces.
- Border: `#263d58` for hairlines and table rules.
- Surface: `#0f1e31` for evidence panels.
- Surface 2: `#091625` for the page canvas.
- Surface 3: `#13263d` for inner readouts and tables.
- Surface deep: `#050c16` for machine output and rejection panels.
- Success: `#6bd7b4` for verified checks.
- Danger: `#f2878d` for rejected attempts.
- Warning: `#e7bf6e` for authority and scope warnings.

A color is never the only carrier of a status. Status text and a shape or icon
remain visible in every state.

## Type and composition

Use a restrained sans-serif family for controls, labels, and body copy. Use a
strong display weight for page headings. Use monospace only for hashes,
addresses, sequences, blocks, error codes, and machine output.

Use a primary workspace, a secondary sidebar, tables, readout rows, and a
timeline. Prefer a small number of meaningful surfaces over repeated cards.
Use the four-step audit path in the Tampering Lab to show restore, comparison,
replay, and verdict as one operation. Avoid gradient text, glass decoration,
random crypto illustrations, and large empty marketing spaces.

## Interaction and states

- Buttons have visible focus, pressed, disabled, and keyboard states.
- Silent Rollback replays the local Demo Space V2 bundle and shows
  `REJECTED / BAD_PREVIOUS_STATE` using transition 1's actual root against the
  reconstructed local head. An optional, separately labeled read-only Sepolia
  probe does not replace the local result.
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
SQLite values in the public demo fixture are synthetic and source-visible;
portable evidence and on-chain data contain commitments only. The website
distinguishes local Rust/revm evidence, `LIVE RPC / OBSERVED`, separate
published protocol-corpus evidence, `VERIFIED`, `REJECTED`, `OUT OF SCOPE`,
and `NOT YET DEMONSTRATED` consistently.

The exact Solidity machine reason `BAD_PREVIOUS_STATE` remains visible even
when the surrounding copy says “stale predecessor”.

## Release boundary

The private references under `internal/design/` are design inputs only. They
are never imported by application code or included in a release archive. The
runtime is rebuilt from Rust/WASM, HTML, CSS, SVG, and the supplied logo asset.
