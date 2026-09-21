# MemoryLineage design quality contract

MemoryLineage uses a dark, evidence-first workspace. The design is judged by
how quickly a reviewer can understand one falsifiable incident and how easily
they can inspect the evidence behind the result.

## Design thesis

```text
one incident
  → one canonical lineage
  → one attempted continuation
  → one exact verdict
  → one independently replayable bundle
```

The interface must keep those five things visually connected. A page may show
more detail, but the detail must answer the current audit question rather than
become a collection of decorative cards.

## Patterns adopted for MemoryLineage

### Simulation before action

Tenderly's product is a useful reference for presenting simulation as a
decision surface: model an action, inspect its result, then decide what to do.
MemoryLineage applies that pattern to a stale restore. The browser does not
broadcast a transaction; it shows the candidate root, canonical root, exact
simulation source, and result. This keeps the product faithful to its
read-only and evidence-backed boundary.

Reference: <https://tenderly.co/> and
<https://github.com/Tenderly/tenderly-docs>.

### Incident header plus chronology

Sentry's issue-detail pattern separates a high-level incident summary from a
chronological trail, event context, and related actions. MemoryLineage uses the
same information architecture for Home, History, and the Lab: the current
verdict is visible first, while authority events, transition details, and
replay evidence remain available below it.

Reference: <https://docs.sentry.io/product/issues/issue-details/>.

### Summary lists and explicit error states

GOV.UK's summary-list guidance is a good fit for registry facts: a key, a
value, and a short contextual action. Its error-summary guidance also supports
the rule that a failure must be named clearly and linked to the relevant
detail. MemoryLineage therefore uses readout rows for roots, IDs, authority,
and source, while machine failures remain visible beside plain-language
explanations.

References:

- <https://design-system.service.gov.uk/components/summary-list/>
- <https://design-system.service.gov.uk/components/error-summary/>

### Evidence tables with progressive disclosure

Carbon's data-table guidance recommends consistent row sizing, dedicated table
controls, and expandable rows when detail would otherwise overwhelm the main
view. MemoryLineage applies that rule to event logs, mutation corpus rows, and
evidence artifacts: the overview remains compact, while the transition or
artifact detail is opened deliberately.

Reference: <https://carbondesignsystem.com/components/data-table/usage/>.

### Local submission references

The local Flowline submission contributes the mission → three-step loop →
central workspace structure. The local Withheld submission contributes a
strong authority boundary, an agent-side inspection rail, and an audit ledger
that records actions in order. MemoryLineage uses those patterns only as
composition references; its data, wording, and protocol behavior remain
MemoryLineage-specific.

References:

- `webmcp-challenge/submissions/flowline/src/ui/BriefPage.tsx`
- `webmcp-challenge/submissions/flowline/src/ui/ArenaPage.tsx`
- `webmcp-challenge/submissions/withheld/src/ui/TopBar.tsx`
- `webmcp-challenge/submissions/withheld/src/ui/Audit.tsx`

## Acceptance gates

The design pass is accepted only when all of these remain true:

- a first-time reviewer understands the Silent Rollback in the first viewport;
- the Lab shows restore, stale root, replay source, and exact verdict together;
- `BAD_PREVIOUS_STATE` and `TRANSITION_ID_MISMATCH` remain visible;
- source classes distinguish local evidence, published corpus, and live RPC;
- verified, rejected, warning, and out-of-scope states include text as well as
  color or shape;
- no raw memory or private locator content appears in the UI or bundle;
- desktop 1440px and mobile 390px have no critical overflow;
- keyboard focus, live result announcements, and reduced motion continue to
  work;
- the current Rust, Solidity, evidence, and browser regression gates pass.

The target is a stronger evidence workspace, not a larger product surface.
