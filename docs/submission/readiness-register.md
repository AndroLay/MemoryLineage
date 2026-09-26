# Top-1 Readiness Register

This register freezes the evidence baseline before the Top-1 readiness vertical
slice. It records the exact starting state, the source classes that must never
be merged, and a per-claim readiness status. It is a status record, not a judge
score and not a claim that any deferred work is complete.

Related plan: [`docs/superpowers/plans/2026-09-21-memorylineage-top1-readiness.md`](../superpowers/plans/2026-09-21-memorylineage-top1-readiness.md).

## Captured starting state

Captured on 22 September 2026 from the repository root.

| Field | Value |
| --- | --- |
| Commit | `44a5751c90cb66c943c8329238c1cb9802fe8ee9` |
| Commit subject | `Record final release and CI evidence` |
| Tag / describe | `v1.0.1` |
| Working tree at capture | clean (`git status --short` empty) |
| `cargo --version` | `cargo 1.97.1 (c980f4866 2026-06-30)` |
| `rustc --version` | `rustc 1.97.1 (8bab26f4f 2026-07-14)` |
| README length | 299 lines (at or below the 300-line cap) |

The current tree is a released baseline. This register does not treat any later
uncommitted edit as part of tag `v1.0.1`.

## Local gate result at capture

Recorded from an actual run at the captured commit, not copied from an earlier
session. `cargo xtask verify` ran every gate below and exited `0`.

| Gate | Command | Result |
| --- | --- | --- |
| Formatting | `cargo fmt --all -- --check` | PASS |
| Clippy | `cargo clippy --workspace --all-targets --all-features -- -D warnings` | PASS |
| Workspace tests | `cargo test --workspace` | PASS |
| Fixture manifest | `cargo xtask verify` (`verify_fixture_manifest`) | PASS |
| Demo Space V2 evidence | `cargo xtask verify` (`verify_demo_space_v2`) | PASS |
| Polkadot Hub portability rehearsal + replay | `cargo xtask verify` | PASS (local only) |
| Recovery preflight + protected resume gate | `cargo xtask verify` | PASS |
| Reference agent runtime | `cargo xtask verify` | PASS |
| Bounded security assurance | `cargo xtask verify` | PASS |
| Pinned conformance | `cargo run -q -p ml-cli -- conformance` | PASS |
| Independent evidence replay | `cargo run -q -p ml-cli -- verify …` | PASS |
| revm Silent Rollback | `cargo run -q -p ml-cli -- revm silent-rollback` | PASS |
| revm mutation lane | `cargo run -q -p ml-cli -- revm mutations` | PASS |
| revm ERC-1271 | `cargo run -q -p ml-cli -- revm erc1271` | PASS |
| revm authority rotation | `cargo run -q -p ml-cli -- revm authority-rotation` | PASS |
| WASM compile | `cargo check -p memorylineage-inspector --target wasm32-unknown-unknown` | PASS |
| Public package boundary | `scripts/check-public-package.sh` | PASS |

The full release path (`cargo xtask release`, static build, Chromium browser
smoke) and the Node compatibility lane (`npm run verify`) were not re-run for
this capture; they are covered by the release checklist and CI workflow, not by
this register.

## Source classes

Every evidence artifact belongs to exactly one class. A claim must never present
one class as another. Local Demo Space V2 evidence is never presented as a
Sepolia observation.

| Source class | Meaning | Representative artifacts |
| --- | --- | --- |
| `DEMO_SPACE_V2_LOCAL` | The deterministic synthetic SQLite incident executed against published Solidity bytecode in Rust/revm. No transaction is broadcast. | `evidence/local/demo_space_v2_evidence.json`, `evidence/local/demo_space_v2_recovery_receipt.json`, `fixtures/silent-rollback-v2/manifest.json` |
| `SEPOLIA_REFERENCE_OBSERVATION` | Read-only observations of the earlier public Sepolia deployment. A single-endpoint read, not consensus. | `evidence/sepolia/sepolia_deployment.json`, `evidence/sepolia/sepolia_reread.json`, `evidence/sepolia/reference_live_probe_*.json` |
| `PROTOCOL_CORPUS_LOCAL` | The separate 4-transition protocol corpus and 20-case mutation matrix. Distinct from the 3-transition Demo Space V2 incident. | `evidence/local/rust_revm_mutation_matrix.json`, `contracts/vectors/erc8350_conformance.json`, `contracts/mutations/mutations.json` |
| `EXTERNAL_REPRODUCTION` | Independent clean-checkout reports authored by other developers. | None yet; see status below. |

## Readiness statuses

Status vocabulary matches the claim matrix: `VERIFIED` only after the named
deterministic check runs; `OBSERVED` when a source returned a value without a
consensus claim; `NOT_YET_DEMONSTRATED` when the required evidence does not yet
exist; `VERIFIED_LOCAL_PACKAGE` when a checked local submission bundle binds
the incident across the site, CLI, and documentation; `OUT_OF_SCOPE` when the
protocol intentionally does not evaluate it.

| Readiness claim | Source class | Status | Status-changing condition |
| --- | --- | --- | --- |
| Stale-predecessor rejection (`BAD_PREVIOUS_STATE`) | `DEMO_SPACE_V2_LOCAL` | VERIFIED | Snapshot 1's recorded root stops being rejected against canonical head 3 |
| Portable evidence tamper + restore (`TRANSITION_ID_MISMATCH` → `VERIFIED`) | `DEMO_SPACE_V2_LOCAL` | VERIFIED | Browser/WASM and CLI stop agreeing on the tamper reason |
| Independent replay (browser + Rust CLI agree) | `DEMO_SPACE_V2_LOCAL` | VERIFIED | Any published vector diverges between paths |
| Demo Space V2 EOA authorization (EIP-712) | `DEMO_SPACE_V2_LOCAL` | VERIFIED | Independent verifier stops recovering the active authorizer |
| Protected resume gate (verified current head + signatures + bound timeline required) | `DEMO_SPACE_V2_LOCAL` | VERIFIED (fixture-scoped) | Loader runs for missing/incomplete authorization, historical/divergent, or invalid input |
| Bounded security assurance | `DEMO_SPACE_V2_LOCAL` + `PROTOCOL_CORPUS_LOCAL` | VERIFIED (`NOT_FORMALLY_VERIFIED`) | Presented as a formal or third-party audit |
| Protocol-corpus authorization history | `PROTOCOL_CORPUS_LOCAL` | STRUCTURE_ONLY | Transition signature proof is independently included and checked |
| 20-case mutation corpus (20/20 rejected) | `PROTOCOL_CORPUS_LOCAL` | VERIFIED | Any expected mutation case stops being rejected |
| Polkadot Hub portability rehearsal | `DEMO_SPACE_V2_LOCAL` (chain context) | OBSERVED (`LOCAL_REHEARSAL_PASS`) | A public Polkadot deployment or RPC observation is claimed |
| Sepolia rejection probe | `SEPOLIA_REFERENCE_OBSERVATION` | OBSERVED | Presented as consensus or as the Demo Space V2 history |
| Public incident coherence (one bundle drives site + CLI + docs) | `DEMO_SPACE_V2_LOCAL` | VERIFIED_LOCAL_PACKAGE | The checked submission bundle, Inspector, CLI, and documentation stop agreeing on incident IDs, roots, or artifact hashes |
| First-time reviewer comprehension | — | NOT_YET_DEMONSTRATED | Recorded comprehension reports from independent developers exist |
| External human clean-checkout reproduction | `EXTERNAL_REPRODUCTION` | NOT_YET_DEMONSTRATED | Two independent reports at a recorded commit are attached and re-run |
| Remote CI green | — | NOT_YET_DEMONSTRATED | A hosted runner executes the workflow past its first step |
| Production agent-runtime adoption | — | NOT_YET_DEMONSTRATED | A real external runtime is integrated and recorded |
| New Sepolia deployment of Demo Space V2 | — | OUT_OF_SCOPE | Only if separately authorized; not part of this slice |
| Pitch PDF and external demo link | PDF in repository; YouTube upload owner-reported | PDF VERIFIED / LINK UNCHECKED | The PDF fails to render, or a supplied YouTube URL fails to show the described demo |
| Public Devpost upload / staging environment | — | NOT_YET_DEMONSTRATED / NOT_PROVIDED | A real upload or separately hosted staging environment is provided |
| Semantic memory truth / poisoning detection | — | OUT_OF_SCOPE | The protocol does not evaluate semantic meaning |
| Formal third-party security audit | — | OUT_OF_SCOPE | Not performed; bounded assurance is not an audit |

## Deferred work not authorized by this register

Consistent with the readiness plan and the strategy notes, this slice does not
authorize: a new chain, a public Demo Space V2 deployment, a large external
agent-framework integration, or any global-victory / adoption claim. The local
The v1.0.2 tag included the pitch deck and demo video files. Current `main`
retains the PDF but removes video, audio, and production sources after the
project owner reported uploading the demo to YouTube. These materials do not
imply public deployment or real agent adoption.

## Progress after the frozen baseline

On 23 September 2026, the current working tree added the source-labeled
[`evidence/submission/manifest.json`](../../evidence/submission/manifest.json)
and the local incident card. `ml-cli submission verify` now checks the exact
fixture commitments, transition-1 stale root, canonical head, regenerated
Rust/revm V2 evidence, two recovery receipts, and SHA-256 artifact hashes.
The Inspector displays the same incident ID and package context. Home and
Inspect now open the local restore check directly. The executable protected
resume example checks five loader outcomes: signed current head, missing
authorization, historical, diverged, and invalid evidence. The current local
`cargo xtask release` and `npm run verify` pass, including the 11-route
Chromium smoke and legacy compatibility lane.
The source tree is committed as candidate
`675c707405ac2afea1fd067be44a67890b0a35f2` and has been pushed to GitHub
`main`. This moves **public incident coherence** to `VERIFIED_LOCAL_PACKAGE`
for that source revision; no new tag or release was created.
`submissionCommit` remains `null` because the manifest cannot contain its own
final Git SHA; the exact revision is recorded in the release checklist and
reproduction runbook. The prior `v1.0.1` tag and last recorded hosted release
predate this update. A Git push occurred, but whether Pages automatically
deployed it and what the live site serves could not be verified here. External
human reproduction remains `NOT_YET_DEMONSTRATED`; no remote CI result for the
candidate was retrieved.

The initial finalization work added a printable eight-slide pitch PDF, a
contribution and provenance record, and an opt-in secret-blinded commitment
helper. At that stage the PDF was reproducible from candidate commit
`675c707405ac2afea1fd067be44a67890b0a35f2`; the later UX refresh and
owner-reported YouTube upload are recorded below. At that checkpoint, no
Devpost upload or manual public-site deployment had been performed; current
Pages content was not independently verified. No same-space Sepolia deployment,
production secret lifecycle, or external human report is implied by their
presence.

## Local onboarding UX update — 25 September 2026

The local judge path is now implementation-complete: the landing offers a
guided start or a direct technical overview; the free overview has its own
one-minute challenge link and does not activate the tour. The guided path
highlights real controls on steps 1 and 2, asks the visitor to predict before
showing a result, and then carries the incident into Inspect, History,
Tampering Lab, and Verify. The local decision says the evidence check passed
while Backup 1 is held against shared head 3; `BAD_PREVIOUS_STATE` appears only
after the visitor opens the technical reason. All copy labels this as a
synthetic local example with no agent resume or transaction.

Current verification passed on 25 September 2026:

- `TMPDIR=/var/tmp cargo xtask release --quiet` — formatting, Clippy, all
  workspace tests, fixtures, Demo Space V2 evidence, portability rehearsal,
  recovery gate, runtime example, bounded assurance, submission bundle,
  conformance, independent replay, revm lanes, WASM, package boundary, static
  build, Chromium smoke, and release package boundary.
- `TMPDIR=/var/tmp npm run verify --silent` — all nine EVM, Python, Inspector,
  audit, replay, and package checks.
- The Chromium smoke covers all 14 stable route patterns plus dynamic Lab
  paths, guided and free entry, challenge answers, unknown Lab slug, keyboard
  focus, and 390px no-overflow behavior. It confirms the landing scroll reveal
  is attached and the refreshed captures are from the current static build.
  The active `http://127.0.0.1:8080` preview was also checked directly with the
  same browser smoke. It now serves one landing page; the duplicate previously
  seen there came from a stale Dioxus process, which was replaced with the
  repository-supported launcher.
- At this 25 September checkpoint, the eight-page PDF was rendered and visually
  reviewed. The source and current CSS/viewport screenshots remained uncommitted on local HEAD
  `497e8238915e6070c2bcb7fe3dd72a37ebc7860b`. This describes the local
  deltas, not the earlier source snapshot: local `main` and `origin/main`
  match that SHA. No additional commit, tag, deployment, or Devpost upload of
  those deltas occurred.

This completes the local prototype implementation and its available automated
gates. It does not establish first-time reviewer comprehension or heuristic
4/4 scores: independent novice sessions remain `NOT_YET_DEMONSTRATED`. External
clean-checkout reproduction, hosted CI, current Pages content, and verification
of the Devpost video link remain open. The prototype demonstrates
a local reference-runtime hold and replayable stale-root rejection; it does not
claim production runtime integration, measured incident reduction, canonical
public-chain provenance from offline replay, or semantic memory safety.

## Judge pitch brief alignment — 26 September 2026

The local pitch deck has been revised to ten 16:9 slides with explicit
section labels for **The Problem**, **The Solution**, **The Innovation**,
**The Impact**, **Current Limitations**, and **Future Scope**. Its architecture
diagram separates the private-memory write path and its two outputs from the
candidate-evidence recovery path through preflight and the local reference
gate. The impact section says outcomes have not been measured; current trust
limits appear before the future-work roadmap.

The PDF was regenerated from `docs/submission/pitch-deck.html`, reports ten
pages at 16:9, and was rendered and visually reviewed. The talk track is in
`docs/submission/pitch-deck.md`. This is still a local submission artifact: no
Devpost upload, public-site update, or external reviewer assessment is claimed.
The project and submission version assigned to this final candidate is
`v1.0.2`; `v1.0.1` is the preceding confirmed public release.
The project owner reports the demo video is uploaded to YouTube. Its URL is not
stored in this repository and has not been independently checked.

## v1.0.2 publication status — 26 September 2026

The v1.0.2 commit `56ed787a678c8267dd4410db463a7ac177f90bc7` and annotated tag
were pushed to GitHub. The GitHub Actions job and its retry failed before any
step started and had no assigned runner, so they produced no CI test result. The
local release gate passed its code, evidence, contract, WASM, package, and static
build checks, but the combined gate did not complete its Chromium smoke step
reliably. A standalone browser smoke passed once with Chromium output captured;
other attempts could not start the page debugging target.

The public Pages URL returned Cloudflare HTTP 403, error 1010, from the latest
check in this environment. That does not confirm whether the v1.0.2 deployment
completed. The project owner reports that the demo video is hosted on YouTube;
its URL and the Devpost video field are not recorded here. Current `main` keeps
the pitch PDF and no video/audio production assets. The v1.0.2 tag retains the
original media snapshot.
