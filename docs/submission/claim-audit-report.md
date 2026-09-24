# Claim and evidence audit — local candidate

Reviewed on 24 September 2026 against source commit
`675c707405ac2afea1fd067be44a67890b0a35f2`. This local candidate is separate
from the earlier `v1.0.1` tag; current live Pages content was not rechecked.
The submission manifest keeps `submissionCommit: null` to avoid a
self-referential hash; this report, release checklist, and reproduction runbook
record the exact source revision.

| Claim | Evidence | Source class | Recheck | Status |
| --- | --- | --- | --- | --- |
| Ordered state and predecessor continuity | `evidence/local/demo_space_v2_evidence.json` | `DEMO_SPACE_V2_LOCAL` | `cargo run -q -p ml-cli -- verify evidence/local/demo_space_v2_evidence.json` | VERIFIED_LOCAL |
| Silent Rollback exact rejection | `evidence/submission/demo-space-v2/rollback-rehearsal.json` | `DEMO_SPACE_V2_LOCAL` | `cargo run -q -p ml-cli -- submission verify evidence/submission/manifest.json` | VERIFIED_LOCAL |
| Authority timeline and EOA proofs | `evidence/local/demo_space_v2_evidence.json` | `DEMO_SPACE_V2_LOCAL` | same independent V2 replay | VERIFIED_LOCAL |
| ERC-1271 behavior | `evidence/local/rust_revm_erc1271.json` | `PROTOCOL_CORPUS_LOCAL` | `cargo xtask verify` | VERIFIED_LOCAL |
| Portable replay and tamper rejection | V2 bundle and Inspector Verify route | `DEMO_SPACE_V2_LOCAL` | `cargo xtask smoke-web` | VERIFIED_LOCAL |
| Protected loader decision, including missing authorization hold | `evidence/local/reference_agent_runtime.json` and `protected-resume-flow` example | `DEMO_SPACE_V2_LOCAL` | `cargo xtask verify` | VERIFIED_LOCAL |
| Raw private memory absent from protocol evidence | V2 privacy flag, typed bundle, and recovery receipts | `DEMO_SPACE_V2_LOCAL` | `ml-cli submission verify` | VERIFIED_LOCAL |
| Protocol mutation and bounded assurance | `evidence/local/memory_lineage_evm_evidence.json`, `security_assurance_report.json` | `PROTOCOL_CORPUS_LOCAL` | `cargo xtask verify` | BOUNDED_LOCAL |
| Semantic truth or safety of memory | No evaluation artifact | — | — | OUT_OF_SCOPE |
| Formal security verification or third-party audit | No formal proof or auditor report | — | — | NOT_CLAIMED |
| External human comprehension and clean-checkout reproduction | Protocol and blank report template only | `EXTERNAL_REPRODUCTION` | Two genuine completed reports required | NOT_YET_DEMONSTRATED |
| Production agent adoption | Local reference adapter and example only | — | External integration evidence required | NOT_YET_DEMONSTRATED |
| Public Demo Space V2 deployment | No public deployment/readback for this space | — | Separate deployment authorization and observation required | OUT_OF_SCOPE |
| Canonical registry provenance of an offline bundle | Verifier reports `ADDRESS_FORMAT_ONLY`; no trusted code/state proof is present | `DEMO_SPACE_V2_LOCAL` | Inspect `registry_identity` in the verifier report | NOT_VERIFIED |
| Rebuilding complete history from the registry | Registry exposes the head, known-ID lookups, and events; no indexer/rebuilder exists | — | No implementation or recovery evidence | NOT_IMPLEMENTED |
| Historical ERC-1271 authorization replay | On-chain accept/reject behavior only; verifier accepts EOA EIP-712 proofs | `PROTOCOL_CORPUS_LOCAL` | `cargo xtask verify` does not establish historical signer-contract state | UNSUPPORTED |
| Interactive Sepolia probe trust source | One configured endpoint, with block pinning and hash recheck | `SEPOLIA_REFERENCE_OBSERVATION` | Browser probe code and claim matrix | SINGLE_ENDPOINT_OBSERVATION |

`registry_identity: ADDRESS_FORMAT_ONLY` in the independent replay checks the
declared address's syntax. It does not authenticate deployed code or a public
RPC response. The existing Sepolia artifacts are observations of a different
space and do not extend the local Demo Space V2 incident.

The `VERIFIED` replay result means the supplied bundle passes its named
deterministic checks. It is not a proof that the bundle was fetched from the
canonical registry. Complete history recovery would need retained event logs or
bundles and a separate replay-versus-trusted-head check.

## Checks run

- `cargo xtask release --quiet`: PASS on this candidate, including workspace
  tests, Clippy, WASM, submission package, static browser smoke, and release
  package boundary.
- `npm run verify`: PASS after `npm ci --offline` installed the pinned legacy
  compatibility dependencies; this covered the Node EVM, Python, old Inspector,
  architecture, and package lanes.
- `cargo run -q -p ml-cli -- submission verify evidence/submission/manifest.json`:
  `VERIFIED_LOCAL_PACKAGE`.
- `cargo xtask release-manifest`: PASS on the clean candidate; it reports
  `workingTreeClean: true` and the exact `submissionRevision` above.
- A temporary manifest with a changed SHA-256 returned exit code 1 and
  `SUBMISSION_ARTIFACT_HASH_MISMATCH`.
- `cargo xtask reviewer-package` and `cargo xtask reviewer-reproduce`: PASS; the
  clean source archive passed the automated release path and its temporary
  checkout was removed. This is not an external human reproduction.
- `cargo fmt --all -- --check`, `git diff --check`, and the static browser smoke
  with desktop/mobile captures: PASS; all routes passed and the 390px viewport
  had no page-level horizontal overflow.
- Root README is 300 lines in the final local tree, within the 300-line cap.
- A repository search for `tamper-proof`, `unhackable`, `prevents all`,
  `detects memory poisoning`, `formally verified`, `security audited`,
  `production adoption`, and `consensus proven` found only explicit negations,
  status labels, or historical/internal discussion in current product surfaces.

No remote CI result for this candidate, new hosted deployment, Devpost upload,
or external human report is claimed here. The last recorded hosted release is
`v1.0.1`; current Pages content was not independently verified in this session.
