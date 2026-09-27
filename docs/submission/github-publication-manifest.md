# GitHub publication record and scope

**Checked:** 27 September 2026

**Published project version:** [`v1.0.2`](https://github.com/AndroLay/MemoryLineage/tree/v1.0.2)

**Release source tag:** [`v1.0.2`](https://github.com/AndroLay/MemoryLineage/tree/v1.0.2)

## Publication status

- The annotated `v1.0.2` tag points to the sanitized stable release snapshot.
  After that release, `main` advanced to the `v1.1.0-rc.2` evaluation
  candidate at `6f03984`; the stable tag intentionally remains pinned to its
  original release commit.
- The GitHub Release uses tag `v1.0.2` and attaches the ten-page pitch PDF.
  The local session artifact was excluded.
- GitHub Actions run [36248726652](https://github.com/AndroLay/MemoryLineage/actions/runs/36248726652)
  and its retry refer to the original pre-cleanup revision and ended before any
  job step; the rewritten tag has no hosted CI result.
- Wrangler deployed Pages production from branch `main`, source `6f03984`,
  deployment ID `2ad1796d-6fac-44ff-b644-37351d6be2a4`. The production domain
  and deployment URL returned HTTP 200, and the production browser smoke
  passed. The previous production deployment at source `80257f74` remains
  available at `d001a7dc.memorylineage.pages.dev` and returned HTTP 200.
- The project owner reports uploading the two-minute demo to YouTube; its URL
  is not recorded here. The Devpost video field was not independently checked.
- The video, audio, narration, and production files are absent from the
  reachable history of `main` and the rewritten `v1.0.2` tag. The ten-page
  pitch PDF remains in the repository and is attached to the Release.

## Intended public commit contents

Current `main` and the published `v1.0.2` stable tag contain the public source
and review materials without video or audio production history. `main` includes
the newer evaluation candidate; the stable tag and Devpost submission remain
at `v1.0.2`.

- Repository entry points and interface source: `.gitignore`, `README.md`,
  `DESIGN.md`, `PRODUCT.md`, `apps/inspector/`, and the relevant browser
  smoke changes in `scripts/smoke_web.py`.
- Public product, reproduction, research, submission, and testing notes under
  `docs/`, including this scope note, the corrected README, claim matrix,
  release checklist, and provenance record.
- Reproducible public tooling: the editable pitch source and PDF build path.
- Final submission assets: the ten-page PDF, the Devpost thumbnail PNG and its
  HTML/image source, the README landing-page capture, and reviewed Inspector
  screenshots. Video and audio files are hosted outside this repository and
  are not part of the GitHub Release.
- The existing source, synthetic fixtures, evidence, license, and third-party
  notices already tracked by the repository.

Do not use `git add -A` as the publication decision. Review the exact staged
file list against these categories before making a release commit.

## Keep out of the public commit

| Path or class | Reason |
| --- | --- |
| `:memory:.ses` | Local session artifact, not a project deliverable; explicitly ignored |
| Demo MP4/WAV, narration script, and video build/recording scripts | The project owner reports hosting the demo on YouTube; these files are intentionally absent from the reachable `main` and `v1.0.2` histories |
| `target/`, `node_modules/`, `.next/`, `out/`, caches, and temporary frame/browser profiles | Rebuildable machine output |
| `.env*`, private keys, secrets, credentials, and personal session data | Sensitive local configuration; never publish |
| Any other file outside the reviewed public project and submission scope | No publication purpose established |

The session file remains local. The thumbnail HTML and preview image are
included because the HTML references that image and together they make the
final PNG editable. The `v1.0.2` annotated tag is rewritten to the sanitized
source snapshot; older version tags are unchanged.

## Finalization rules

1. Keep source and documentation changes in a clean commit. Pitch PDF remains
   a repository artifact; the demo video is hosted externally.
2. Run the release and public-package gates on that exact clean commit.
3. Recheck the exact staged paths, then compare the commit SHA with GitHub when
   network access is available.
4. Keep the v1.0.2 source tag consistent in the Devpost entry, PDF, and
   reproduction materials. Do not claim a Pages update or Devpost
   link verification until each is observed.
5. Keep copyright wording consistent with the repository license:
   Copyright © 2026 MemoryLineage contributors; repository code and
   documentation are MIT-licensed. Do not add a blanket “All rights reserved”
   statement that contradicts the MIT permission grant. Third-party terms
   remain governed by their respective notices and licenses.
