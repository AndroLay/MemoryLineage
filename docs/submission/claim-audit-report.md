# Claim and evidence audit — local working tree

Reviewed on 24 September 2026. This report describes the current uncommitted
tree, not the earlier `v1.0.1` tag or the hosted site. The exact submitted
commit will be recorded outside the self-referential submission manifest when
the change is committed.

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

- `cargo xtask release`: PASS, including workspace tests, Clippy, WASM,
  submission package, static browser smoke, and release package boundary.
- `npm run verify`: PASS after `npm ci --offline` installed the pinned legacy
  compatibility dependencies; this covered the Node EVM, Python, old Inspector,
  architecture, and package lanes.
- `cargo run -q -p ml-cli -- submission verify evidence/submission/manifest.json`:
  `VERIFIED_LOCAL_PACKAGE`.
- `cargo xtask release-manifest /tmp/memorylineage-release-manifest-current.json`:
  PASS; it reports `workingTreeClean: false` and `submissionRevision: null`,
  with the local incident and protocol-corpus assurance in separate sections.
- A temporary manifest with a changed SHA-256 returned exit code 1 and
  `SUBMISSION_ARTIFACT_HASH_MISMATCH`.
- `git diff --check`: PASS; root README remains 299 lines.
- A repository search for `tamper-proof`, `unhackable`, `prevents all`,
  `detects memory poisoning`, `formally verified`, `security audited`,
  `production adoption`, and `consensus proven` found only explicit negations,
  status labels, or historical/internal discussion in current product surfaces.

`cargo xtask reviewer-package` and `reviewer-reproduce` require a clean
committed tree, so they cannot validate this uncommitted change yet. No remote
CI result, new hosted deployment, or external human report is claimed here.
