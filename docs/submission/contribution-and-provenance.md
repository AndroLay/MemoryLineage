# Contribution and provenance for 3rd-Web-Hack

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
| 23 September 2026 | Current working tree and `evidence/submission/manifest.json` | The local incident envelope, direct restore action, and protected-resume example are later work. They have no public revision identity until committed and published. |

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
| The Inspector presents the evidence and failure reason in a browser | `apps/inspector/`, `scripts/smoke_web.py` | Hosted `v1.0.1` predates the current working-tree updates |

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
The actual submitted commit belongs in the release metadata and external
reviewer report. Two independent human reports, a new public Demo Space V2
deployment, and production adoption remain unclaimed until they occur.
