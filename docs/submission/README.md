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
| [Devpost thumbnail](assets/devpost-thumbnail.png) | 3:2 image with the project logo and a website preview |
| [Thumbnail source](assets/devpost-thumbnail.html) | Editable source using [the website preview image](assets/devpost-website-preview.png) |
| [Contribution and provenance](contribution-and-provenance.md) | Project contribution, repository history, standards, and claim limits |
| [Release checklist](release-checklist.md) | Local verification record and outstanding external gates |
| [Local incident envelope](../../evidence/submission/README.md) | Demo Space V2 artifacts and offline verification command |
| [Historical claim audit](claim-audit-report.md) | Audit of the 24 September source candidate; it does not describe v1.0.2 |
| [GitHub publication record](github-publication-manifest.md) | Source snapshot, included files, exclusions, and publication limits |
| [Previous public release](https://github.com/AndroLay/MemoryLineage/releases/tag/v1.0.1) | `v1.0.1`; superseded by this `v1.0.2` submission build |

## Publication status

The `v1.0.2` source release is tagged at commit
`56ed787a678c8267dd4410db463a7ac177f90bc7`; both `main` and the annotated tag
were pushed to GitHub on 26 September 2026. It follows the previous public
release, `v1.0.1`.

GitHub Actions run [36248726652](https://github.com/AndroLay/MemoryLineage/actions/runs/36248726652)
and its retry failed before any job step started; the API reported no steps and
no assigned runner, so they provide no test result for this commit. See the
[release checklist](release-checklist.md) for the local gate evidence.

The public Pages URL is [memorylineage.pages.dev](https://memorylineage.pages.dev).
The latest check from this environment received Cloudflare HTTP 403, error
1010; that response does not establish whether the new deployment completed or
what the site serves. The project owner reports that the two-minute demo video
is uploaded to YouTube. Its URL is not recorded in this repository. The video,
voice track, script, and production sources are removed from the current
`main` branch; the immutable `v1.0.2` tag retains the earlier snapshot. The
pitch PDF remains in the repository. The Devpost entry's video link has not
been independently checked. There is no separate staging environment or
external human reproduction report.

The local session file `:memory:.ses` is not a project artifact and must never
be published.
