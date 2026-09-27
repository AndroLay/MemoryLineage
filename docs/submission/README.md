# Submission packet

This directory contains the public submission material and evidence notes.
Every claim follows the [claim matrix](claim-matrix.md), keeping local Demo
Space V2 evidence separate from the earlier Sepolia observation.

**Stable project and submission version: `v1.0.2`.** The evaluation
pre-release is [`v1.1.0-rc.2`](release-notes-v1.1.0-rc.2.md); it is not the
current Devpost build. The preceding stable release is
[`v1.0.1`](https://github.com/AndroLay/MemoryLineage/releases/tag/v1.0.1).

| Artifact | Purpose |
| --- | --- |
| [Devpost description](devpost-description.md) | Judge-facing problem, solution, evidence, and limitations |
| [Devpost submission copy](devpost-submission.md) | Copy-ready project overview, story, tags, links, media, and publication notes |
| [Claim matrix](claim-matrix.md) | Source of truth for supported and unsupported claims |
| [Pitch deck source](pitch-deck.md) | Ten-slide outline with talk track, architecture, recovery flow, impact, limitations, and future scope |
| [Pitch PDF](MemoryLineage-3rd-Web-Hack.pdf) | Ten-slide presentation generated from editable HTML |
| [Devpost thumbnail](assets/devpost-thumbnail.png) | 3:2 image with the project logo and a website preview |
| [Thumbnail source](assets/devpost-thumbnail.html) | Editable source using [the website preview image](assets/devpost-website-preview.png) |
| [Contribution and provenance](contribution-and-provenance.md) | Project contribution, repository history, standards, and claim limits |
| [Release checklist](release-checklist.md) | Local verification record and outstanding external gates |
| [v1.1.0-rc.2 release notes](release-notes-v1.1.0-rc.2.md) | Evaluation candidate changes, verification status, and limits |
| [Local incident envelope](../../evidence/submission/README.md) | Demo Space V2 artifacts and offline verification command |
| [Historical claim audit](claim-audit-report.md) | Audit of the 24 September source candidate; it does not describe v1.0.2 |
| [GitHub publication record](github-publication-manifest.md) | Source snapshot, included files, exclusions, and publication limits |
| [Previous public release](https://github.com/AndroLay/MemoryLineage/releases/tag/v1.0.1) | `v1.0.1`; superseded by this `v1.0.2` submission build |

## Publication status

The annotated [`v1.0.2` source tag](https://github.com/AndroLay/MemoryLineage/tree/v1.0.2)
is the final submission snapshot and follows the previous public release,
`v1.0.1`. The matching [GitHub Release](https://github.com/AndroLay/MemoryLineage/releases/tag/v1.0.2)
contains the pitch PDF; video, audio, narration, and production files are not
reachable through the current `main` branch or `v1.0.2` tag history.

GitHub Actions run [36248726652](https://github.com/AndroLay/MemoryLineage/actions/runs/36248726652)
and its retry refer to the original pre-cleanup source revision and failed
before any job step started; they provide no CI result for the final rewritten
tag. See the [release checklist](release-checklist.md) for the local gate
evidence and its limits.

The public Pages URL is [memorylineage.pages.dev](https://memorylineage.pages.dev).
On 27 September 2026, Wrangler 4.141.0 deployed the static Inspector to
production from branch `main`, source commit `6f03984`, deployment ID
`2ad1796d-6fac-44ff-b644-37351d6be2a4`; its direct URL is
[2ad1796d.memorylineage.pages.dev](https://2ad1796d.memorylineage.pages.dev).
Both the direct URL and custom domain returned HTTP 200, and the production
browser smoke passed. This production site uses the `v1.1.0-rc.2` evaluation
candidate; the stable GitHub release and Devpost submission remain `v1.0.2`.
The earlier production deployment from source `80257f74` remains available at
[d001a7dc.memorylineage.pages.dev](https://d001a7dc.memorylineage.pages.dev)
and returned HTTP 200 as a rollback target. The
project owner supplied the two-minute [YouTube demo](https://youtu.be/K2QUHm4lJCo),
and an owner-provided screenshot shows the Devpost project page with its
embedded player. Playback was not independently checked. The pitch PDF remains
in the repository and is attached to the GitHub Release. Video, audio, scripts,
and production sources were purged from
the reachable history of `main` and `v1.0.2`. There is no separate staging
environment or external human reproduction report.

The local session file `:memory:.ses` is not a project artifact and must never
be published.

The `v1.1.0-rc.2` evaluation candidate is available as a GitHub pre-release and
is now merged into `main`. Its static Inspector is deployed to Pages production;
the stable release and Devpost entry remain at `v1.0.2`. The pinned local
LangGraph sync/async integration gate passes, while hosted CI did not start a
workflow step and independent human reproduction remains unproven. See the
current release notes and checklist before treating the candidate as stable.
