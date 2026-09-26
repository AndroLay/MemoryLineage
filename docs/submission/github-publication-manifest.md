# GitHub publication record and scope

**Checked:** 26 September 2026

**Published project version:** [`v1.0.2`](https://github.com/AndroLay/MemoryLineage/tree/v1.0.2)

**Release commit:** `56ed787a678c8267dd4410db463a7ac177f90bc7`

## Publication status

- Commit `56ed787a678c8267dd4410db463a7ac177f90bc7` advanced `main` from
  `497e8238915e6070c2bcb7fe3dd72a37ebc7860b` and was pushed successfully.
- Annotated tag `v1.0.2` points to that commit and was pushed successfully.
- The release contains 49 reviewed public files. The session artifact and
  internal video-review memo listed below were excluded.
- GitHub Actions run [36248726652](https://github.com/AndroLay/MemoryLineage/actions/runs/36248726652)
  and its retry ended before any job step; the job has no runner assignment or
  step results.
- The Pages URL returned Cloudflare HTTP 403, error 1010, from the latest
  check in this environment. Deployment and live content remain unverified.
- The Devpost video and pitch deck have not been uploaded. The Git tag exists;
  no separate GitHub Release page was created.

## Intended public commit contents

When the current candidate is frozen and verified, include the changes that
belong to these public project areas:

- Repository entry points and interface source: `.gitignore`, `README.md`,
  `DESIGN.md`, `PRODUCT.md`, `apps/inspector/`, and the relevant browser
  smoke changes in `scripts/smoke_web.py`.
- Public product, reproduction, research, submission, and testing notes under
  `docs/`, including this scope note, the corrected README, claim matrix,
  release checklist, and provenance record.
- Reproducible public tooling: the pitch/video build and recording scripts.
- Final submission assets: the ten-page PDF, the two-minute narrated MP4, the
  earlier 37-second preview clearly labeled as an older cut, the voice-over
  WAV, the Devpost thumbnail PNG and its HTML/image source, the README
  landing-page capture, and the reviewed Inspector screenshots.
- The existing source, synthetic fixtures, evidence, license, and third-party
  notices already tracked by the repository.

Do not use `git add -A` as the publication decision. Review the exact staged
file list against these categories before making a release commit.

## Keep out of the public commit

| Path or class | Reason |
| --- | --- |
| `:memory:.ses` | Local session artifact, not a project deliverable; explicitly ignored |
| `docs/submission/demo-video-review.md` | Internal heuristic scores and revision notes, not judge-facing evidence; explicitly ignored |
| `target/`, `node_modules/`, `.next/`, `out/`, caches, and temporary frame/browser profiles | Rebuildable machine output |
| `.env*`, private keys, secrets, credentials, and personal session data | Sensitive local configuration; never publish |
| Any other file outside the reviewed public project and submission scope | No publication purpose established |

The human-review memo and session file remain on the local machine. The
thumbnail HTML and preview image are included because the HTML references that
image and together they make the final PNG editable. The dry WAV is retained as
the audio source for the narrated MP4; its synthetic origin and pending human
listening review are disclosed in the submission notes.

## Finalization rules

1. Freeze all source, documentation, and media edits into one clean commit.
2. Run the release and public-package gates on that exact clean commit. The
   recorded 25 September local gate predates some current media/documentation
   edits; it is not a fresh check of a clean final commit.
3. Recheck the exact staged paths, then compare the commit SHA with GitHub when
   network access is available.
4. Keep the final commit SHA consistent across the Devpost entry, PDF, video
   notes, and reproduction materials. Do not claim a Pages update or Devpost
   upload until each is observed.
5. Keep copyright wording consistent with the repository license:
   Copyright © 2026 MemoryLineage contributors; repository code and
   documentation are MIT-licensed. Do not add a blanket “All rights reserved”
   statement that contradicts the MIT permission grant. Third-party terms
   remain governed by their respective notices and licenses.
