# Submission packet

This directory contains the public submission material and evidence notes.
Every claim follows the [claim matrix](claim-matrix.md), keeping local Demo
Space V2 evidence separate from the earlier Sepolia observation.

**Project and submission version: `v1.0.2`.** The preceding public release is
[`v1.0.1`](https://github.com/AndroLay/MemoryLineage/releases/tag/v1.0.1).

| Artifact | Purpose |
| --- | --- |
| [Devpost description](devpost-description.md) | Judge-facing problem, solution, evidence, and limitations |
| [Claim matrix](claim-matrix.md) | Source of truth for supported and unsupported claims |
| [Pitch deck source](pitch-deck.md) | Ten-slide outline with talk track, architecture, recovery flow, impact, limitations, and future scope |
| [Pitch PDF](MemoryLineage-3rd-Web-Hack.pdf) | Ten-slide presentation generated from editable HTML |
| [Two-minute narrated demo](MemoryLineage-narrated-demo.mp4) | 120-second local browser recording with cursor, captions, interface cues, and synthesized male English narration |
| [Voice-over track](MemoryLineage-voiceover-en.wav) | Dry 1:58, 24 kHz mono Kokoro `am_michael` track at 1.08×; synthetic and owner-reviewed as part of the final video |
| [Devpost thumbnail](assets/devpost-thumbnail.png) | 3:2 image with the project logo and a website preview |
| [Thumbnail source](assets/devpost-thumbnail.html) | Editable source using [the website preview image](assets/devpost-website-preview.png) |
| [Video notes](demo-video.md) | Evidence scope, attribution, and regeneration details |
| [Two-minute video script](memorylineage-video-script-en.md) | Timed narration and evidence-gated storyboard |
| [Contribution and provenance](contribution-and-provenance.md) | Project contribution, repository history, standards, and claim limits |
| [Release checklist](release-checklist.md) | Local verification record and outstanding external gates |
| [Local incident envelope](../../evidence/submission/README.md) | Demo Space V2 artifacts and offline verification command |
| [Historical claim audit](claim-audit-report.md) | Audit of the 24 September source candidate, not the current dirty worktree |
| [GitHub publication scope](github-publication-manifest.md) | Source snapshot, intended public files, exclusions, and publication limits |
| [Previous public release](https://github.com/AndroLay/MemoryLineage/releases/tag/v1.0.1) | `v1.0.1`; superseded by this `v1.0.2` submission build |

The earlier [37-second cue-only cut](MemoryLineage-local-demo.mp4) is retained as
a short preview. Use the two-minute narrated cut as the submission video.

## Publication status

The last recorded public site is [memorylineage.pages.dev](https://memorylineage.pages.dev);
its current contents have not been verified. The last recorded source snapshot
is `497e8238915e6070c2bcb7fe3dd72a37ebc7860b`. Local `main` and
`origin/main` refs match it, but this environment could not freshly query
GitHub. Current UX and media edits remain uncommitted and are not part of that
snapshot.

The previous `v1.0.1` release remains the last confirmed public release. No
new tag, Pages deployment, or Devpost upload was made during this documentation
review. The narrated video and pitch PDF have not been uploaded to Devpost.
There is no separate staging environment or external human reproduction report.

The internal heuristic score memo `demo-video-review.md` is intentionally
excluded from the public packet. The local session file `:memory:.ses` is not
a project artifact and must never be published.
