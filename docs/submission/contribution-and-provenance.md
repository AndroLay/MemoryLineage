# Contribution and provenance for 3rd-Web-Hack

This submission uses project version `v1.0.2`; `v1.0.1` is the preceding
public Inspector release. These repository-level version tags are independent
of the internal Cargo package versions.

This record is for a reviewer deciding what MemoryLineage contributes. It does
not infer the creation date of every source file from a Git commit or assert a
participant's eligibility. The [event rules](https://3rd-web-hack.devpost.com/rules)
ask for an original project developed for the hackathon; the entrant must be
able to attest to authorship and eligibility in the Devpost account.

## What the repository history establishes

| Recorded point | Evidence | Meaning |
| --- | --- | --- |
| 19 September 2026, 03:06 UTC+8 | Initial Git commit `2c36366` | A substantial code and research baseline entered this repository at once. The commit alone does not date when each file was written. |
| 22 September 2026, 02:48 UTC+8 | `v1.0.1` commit `44a5751` | Public release baseline for the previously hosted Inspector. |
| 24 September 2026 | Earlier source candidate `675c707405ac2afea1fd067be44a67890b0a35f2` | This candidate was recorded as pushed to `main` at the time. It was later superseded by source snapshot `497e823`; neither has a release tag newer than `v1.0.1`. |
| 26 September 2026 | Previous source baseline `497e8238915e6070c2bcb7fe3dd72a37ebc7860b` | This was the `main` tip before the final submission package was committed. |
| 26 September 2026 | Project version `v1.0.2`, commit `56ed787a678c8267dd4410db463a7ac177f90bc7` | The commit and annotated `v1.0.2` tag were pushed to GitHub. The local combined release gate did not fully pass its Chromium startup step; the hosted GitHub Actions job failed before any step ran. |
| 26 September 2026 | Current-main media cleanup | After the project owner reported uploading the demo to YouTube, the MP4, WAV, narration, and production files were removed from current `main`; the pitch PDF remains. The immutable `v1.0.2` tag still contains its original snapshot. |

The older research notes under [`docs/research/`](../research/README.md) are
historical source material. Their dates and internal scores are not hackathon
judge scores or proof that all code was written on those dates.

## Existing standards and dependencies

MemoryLineage implements a pinned ERC-8350 draft/vector snapshot and uses
EIP-712, ERC-1271, Ethereum/Sepolia, Rust, Dioxus, SQLite, and `revm`. It does
not claim authorship of these standards or tools, final ERC-8350 compliance,
or first-in-field status. Third-party license information is in
[`THIRD_PARTY_NOTICES.md`](../../THIRD_PARTY_NOTICES.md).

## Submission contribution and executable evidence

| Contribution in this repository | Where a reviewer can check it | Boundary |
| --- | --- | --- |
| Restore Preflight classifies a selected private snapshot as current, historical, divergent, or unverified | `crates/ml-recovery-gate/`, `apps/inspector/src/pages.rs`, `cargo xtask verify` | Fixture-scoped; not a live production-agent assessment |
| Recovery Decision Receipts and a loader gate hold unsafe resumes | `crates/ml-agent-runtime/`, `crates/ml-agent-runtime/examples/protected-resume-flow.rs` | Local reference integration; no external adoption claim |
| Rust and Rust/WASM replay verify ordered state, predecessor, authority, and evidence integrity | `crates/ml-verifier-independent/`, `/verify`, `ml-cli verify` | A bundle's source label is a declaration without external provenance |
| Solidity behavior and Silent Rollback are executed against checked-in bytecode | `contracts/`, `crates/ml-local-evm/`, `evidence/submission/` | Demo Space V2 is local Rust/revm, not a Sepolia transaction history |
| A separate earlier Sepolia deployment and reread demonstrate public registry observation | `evidence/sepolia/` | Different space from Demo Space V2; never combine the two as one incident |
| The Inspector presents the evidence and failure reason in a browser | `apps/inspector/`, `scripts/smoke_web.py` | The `v1.0.2` source tag is public. The Pages URL returned Cloudflare HTTP 403, error 1010, in the latest check, so the hosted build and deployment status remain unverified. |

The narrow product contribution is a verifiable recovery decision when the
runtime operator must not be the only party trusted to preserve the canonical
history. It does not judge the meaning or safety of private memory, prove what
an external agent actually loaded, or guarantee availability of off-chain data.

## How to verify the submitted revision

The final Devpost entry should name one immutable Git commit and use it for the
repository link, release artifact, presentation, demo recording, and reviewer
report. A candidate may be checked locally with:

```bash
cargo run -q -p ml-cli -- submission verify evidence/submission/manifest.json
cargo xtask release
```

The manifest's `submissionCommit: null` avoids a self-referential Git hash.
The source revision is recorded in the release checklist and reproduction
runbook; a submission should use that exact SHA. Two independent
human reports, a new public Demo Space V2 deployment, and production adoption
remain unclaimed until they occur.
