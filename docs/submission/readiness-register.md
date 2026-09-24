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
| Demo video / staging environment | — | OUT_OF_SCOPE | Explicitly deferred submission work |
| Semantic memory truth / poisoning detection | — | OUT_OF_SCOPE | The protocol does not evaluate semantic meaning |
| Formal third-party security audit | — | OUT_OF_SCOPE | Not performed; bounded assurance is not an audit |

## Deferred work not authorized by this register

Consistent with the readiness plan and the strategy notes, this slice does not
authorize: a new chain, a public Demo Space V2 deployment, a large external
agent-framework integration, a demo video, or any global-victory / adoption
claim. Those remain separate, evidence-gated operations.

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
This moves **public incident coherence** to `VERIFIED_LOCAL_PACKAGE` for the
working tree. `submissionCommit` is `null` because a commit cannot include its
own final SHA; the exact submitted SHA must be recorded in release metadata
and reviewer reports after commit. The prior `v1.0.1` tag and hosted site do
not contain this update. External
human reproduction and remote CI remain `NOT_YET_DEMONSTRATED`.

The later finalization work adds a printable eight-slide pitch PDF, a 45-second
captioned video assembled from actual static-browser states, a contribution and
provenance record, and an opt-in secret-blinded commitment helper. These are
local candidate artifacts. No Devpost upload, public site refresh, same-space
Sepolia deployment, production secret lifecycle, or external human report is
implied by their presence.
