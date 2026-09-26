# MemoryLineage design quality contract

MemoryLineage uses a dark, evidence-first workspace. The design is judged by
how quickly a reviewer can understand one falsifiable incident and how easily
they can inspect the evidence behind the result.

## Design thesis

```text
one plain-language question
  → one visitor prediction
  → one local evidence check
  → one separate restore decision
  → optional technical detail and independent replay
```

The interface must keep those five things visually connected. A page may show
more detail, but the detail must answer the current audit question rather than
become a collection of decorative cards.

## Patterns adopted for MemoryLineage

### Simulation before action

Tenderly's product is a useful reference for presenting simulation as a
decision surface: model an action, inspect its result, then decide what to do.
MemoryLineage applies that pattern to a stale restore. The first-run challenge
asks for a prediction before revealing the result; the Lab then shows the
candidate root, canonical root, exact simulation source, and result. Neither
path broadcasts a transaction.

Reference: <https://tenderly.co/> and
<https://github.com/Tenderly/tenderly-docs>.

### Bounded drill with visible outcomes

ExitDrill is an adjacent product reference for presenting one bounded
before/after exercise, keeping the outcome of each structural dimension
visible, and ending with a concise explanation. Its public repository describes
a three-minute offline CLI demonstration over synthetic SaaS exports. That is
not the same interaction as MemoryLineage's one-minute browser challenge, and
it is not evidence that either flow has passed a novice study. Borrow the
clarity of the comparison and separate outcomes; do not describe its documented
CLI demo as a game or treat ExitDrill as a verified 3rd-Web-Hack entry. Its
event status and evidence limits are tracked in the
[public project audit](../research/2026-09-20-3rd-web-hack-public-project-audit.md#event-membership-unresolved-not-scored-as-a-3rd-web-hack-entry).

Reference: <https://github.com/ChelseaKR/exitdrill>.

### Incident header plus chronology

Sentry's issue-detail pattern separates a high-level incident summary from a
chronological trail, event context, and related actions. MemoryLineage uses the
same information architecture for Home, History, and the Lab: the first-run
screen introduces the incident without its verdict, while authority events,
transition details, and replay evidence remain available in the audit surfaces.

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

- the landing first viewport explains the backup mismatch and makes `Get started` and `Explore freely` obvious;
- the first two guided steps stay full-page and spotlight the Home challenge entry, then the Challenge answer and check controls;
- the challenge withholds its verdict until the visitor checks the evidence;
- “check passed” and “hold the older backup” have distinct meanings;
- the Lab shows restore, stale root, replay source, and exact verdict together;
- machine reasons such as `BAD_PREVIOUS_STATE` and `TRANSITION_ID_MISMATCH` remain available after the relevant action;
- source classes distinguish local evidence, published corpus, and live RPC;
- verified, rejected, warning, and out-of-scope states include text as well as
  color or shape;
- no raw memory or private locator content appears in the UI or bundle;
- desktop 1440px and mobile 390px have no critical overflow;
- keyboard focus, live result announcements, and reduced motion continue to
  work;
- the current Rust, Solidity, evidence, and browser regression gates pass.

The target is a stronger evidence workspace, not a larger product surface.
