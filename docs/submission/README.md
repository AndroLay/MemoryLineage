# Submission packet

This directory contains the judge-facing source material for MemoryLineage.
Every statement in these files follows the repository claim matrix and keeps
local Demo Space V2 evidence separate from the existing Sepolia observation.

| Artifact | Purpose |
| --- | --- |
| [Devpost description](devpost-description.md) | Short problem, solution, evidence, and boundary text |
| [Claim matrix](claim-matrix.md) | Source of truth for supported and unsupported claims |
| [Pitch deck source](pitch-deck.md) | Eight-slide presentation outline based on current evidence |
| [Pitch PDF](MemoryLineage-3rd-Web-Hack.pdf) | Eight-slide judge presentation generated from editable HTML |
| [Local demo video](demo-video.md) | 45-second captioned video of verified local browser states and regeneration steps |
| [Contribution and provenance](contribution-and-provenance.md) | Repository timeline, standards, original product work, and claim limits |
| [Release checklist](release-checklist.md) | Finalization state and remaining external gates |
| [Local incident envelope](../../evidence/submission/README.md) | Hashed Demo Space V2 artifacts and offline verification command |
| [Current claim audit](claim-audit-report.md) | Local evidence results and unresolved external gates |
| [Previous public release](https://github.com/AndroLay/MemoryLineage/releases/tag/v1.0.1) | The hosted `v1.0.1` baseline; it predates this source candidate |

The pitch deck is a source document. The last recorded public site URL is
[memorylineage.pages.dev](https://memorylineage.pages.dev). The locally verified
source candidate is `675c707405ac2afea1fd067be44a67890b0a35f2`; it has been
pushed to GitHub `main` and remains untagged. The prior `v1.0.1` release
remains the last confirmed release. No manual Pages deployment was run; whether
the push triggered automatic hosting is unverified because the live Pages URL
could not be fetched here. The local video and pitch PDF were built from the
candidate source revision and are not uploaded to Devpost. A separate staging
environment and external human reproduction are not claimed.
