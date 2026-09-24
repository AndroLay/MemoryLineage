# Submission claim matrix

Use this matrix as the source for the website, README, pitch, and Devpost copy.
Keep the local Demo Space V2, the earlier Sepolia deployment, and the protocol
mutation corpus separate in every explanation. The source classes and their
readiness statuses are frozen in the
[Top-1 readiness register](readiness-register.md).

| Claim | Status | Safe wording |
| --- | --- | --- |
| Sequence and predecessor continuity | Verified | “The Rust verifier replays the committed sequence and predecessor roots; the Solidity registry enforces the same transition rules.” |
| Transition and state-root derivation | Verified on the pinned corpus | “Independent Rust paths match the published ERC-8350 draft vectors and the tested Solidity implementation.” |
| Protocol-corpus authorization history | Structural only | “The protocol-corpus projection reports authority records as `STRUCTURE_ONLY`; transition signature proof is `NOT_INCLUDED` in that bundle.” |
| Demo Space V2 EOA authorization | Verified locally | “The Demo Space V2 bundle carries EIP-712 domain, digest, signature, config-nonce, and effective-sequence material for all three transitions; the independent Rust verifier recovers the active EOA authorizer and reports `EOA_SIGNATURES_VERIFIED` / `TIMELINE_BOUND`.” |
| Contract authorization behavior | Verified in separate local Rust/revm cases | “The registry accepts/rejects the tested EOA/ERC-1271 and authority-rotation cases. This does not prove each historical V2 transition signature offline.” |
| Demo Space V2 | Verified as local evidence | “Three synthetic SQLite-derived commitments were executed against the published Solidity bytecode in Rust/revm. Demo Space V2 is not deployed to Sepolia.” |
| Silent Rollback stale root | Verified in local Rust/revm evidence | “The local transition-4 attempt uses the actual state root from transition 1 while the local head is at sequence 3; the Solidity execution rejects it with `BAD_PREVIOUS_STATE`.” |
| Restore Preflight | Verified against synthetic local evidence | “The Inspector classifies a selected public fixture commitment against the replayed Demo Space V2 head/history. It is not a production commitment, live chain assessment, or external runtime gate.” |
| Recovery Decision Receipt | Verified locally and in Rust/WASM | “The receipts bind a selected snapshot commitment, named evidence source, and effective authority assurance to the versioned `strict-authorized-current-head-v2` policy. A current head with missing proof remains `CURRENT_HEAD` for lineage but receives `BLOCK_UNVERIFIED`; signed Demo Space V2 receives `RESUME_ALLOWED`. Receipt V1 remains verifiable when its resume authorization is present.” |
| Protected resume decision gate | Verified in the Rust `ml-recovery-gate` adapter and CLI fixture lane | “The adapter invokes its loader callback only when the current head, transition signatures, and authority timeline verify. Missing proof returns `BLOCK_UNVERIFIED`; historical checkpoints are held as `REHEARSE_ONLY`. This is a generic reference adapter, not an integration with a production agent framework.” |
| Reference agent runtime integration | Verified locally | “`ml-agent-runtime` loads the signed current-head snapshot into an in-memory session, holds a current head with missing authorization, historical and diverged snapshots before the loader, and fails closed on invalid evidence. The executable flow covers all five cases. This is a framework-neutral local integration; external adoption is not claimed.” |
| Bounded security assurance | Verified locally | “Rust/revm exercises the checked-in Solidity artifact across five valid transitions, six stale predecessors, two sequence failures, the 20-case mutation matrix, ERC-1271, and authority rotation. The report explicitly remains `NOT_FORMALLY_VERIFIED`.” |
| Polkadot Hub portability rehearsal | Verified locally; independently replayable; no deployment | “The same checked-in Solidity bytecode and tested invariants are rehearsed through local revm with the Polkadot Hub TestNet chain context. An independent Rust verifier replays both nested V2 observations; transition/root and stale-predecessor results match, while EIP-712 remains chain-bound. This is not a public Polkadot deployment, RPC observation, or cross-chain consensus proof.” |
| Recovery receipt privacy boundary | Verified by schema and artifact checks | “The published receipt contains commitments and evidence references only; raw SQLite values, secrets, and private locator contents are absent.” |
| Sepolia rejection probe | Separate read-only observation | “The browser can issue a read-only `eth_call` to the existing Sepolia deployment using the same stale root. That registry space is not the Demo Space V2 history.” |
| Browser Sepolia block context | Read-only single-endpoint RPC observation | “The interactive probe pins its calls to a resolved `finalized` or `safe` block number and rechecks the block hash through the same endpoint. This is one provider observation, not consensus proof. The separately published deployment reread is a distinct endpoint observation, also not a consensus proof.” |
| Authority rotation | Verified in local Rust/revm lanes | “The Demo Space V2 execution rotates authority before transition 3; the evidence records configNonce 0 and 1, effective sequence boundaries, and signer-to-sequence binding. Do not call this a public Sepolia rotation.” |
| 20-case mutation corpus | Verified as a separate protocol corpus | “The published local Rust/revm corpus rejected 20 of 20 expected mutation cases. These cases are not the Demo Space V2 incident.” |
| Evidence export and replay | Verified locally | “The website and independent Rust CLI replay the supplied portable V2 evidence bundle; the report names `verification_scope: OFFLINE_BUNDLE_REPLAY`. Changing the tested locator commitment returns `TRANSITION_ID_MISMATCH`. Replay proves the bundle's internal consistency, not its canonical chain provenance.” |
| Local submission envelope | Verified locally | “The package binds the named Demo Space V2 incident, three SQLite snapshots, local Rust/revm execution, V2 evidence, recovery receipts, and SHA-256 artifact hashes. `cargo run -q -p ml-cli -- submission verify evidence/submission/manifest.json` checks their relationships; this is a local package verdict, not a public network or external reproduction claim.” |
| V2 source label and registry address | Scope-limited | “V2 replay rejects a local bundle relabeled as a Sepolia observation or protocol corpus; `registry_identity: ADDRESS_FORMAT_ONLY` checks address syntax, not a live deployed contract. A source class remains a declaration until tied to trusted external provenance.” |
| Canonical registry and chain provenance | Not verified by offline replay | “The bundle verifier does not authenticate the declared registry address against deployed bytecode or trusted chain state. Keep local replay and RPC observation as separate results.” |
| Complete history recovery from chain data | Not implemented | “The registry exposes its head, known transition lookups, and events; it has no full-history query. Recovery requires retained event logs or bundles, followed by replay and comparison with an authenticated head. No indexer/rebuilder is included.” |
| Privacy boundary | Verified within the protocol/evidence format | “The registry and portable bundle contain commitments, not raw memory. The public repository does contain synthetic sample SQLite fixtures for deterministic reproduction.” |
| Secret-blinded snapshot helper | Preparation only | “The optional helper binds a snapshot to a space ID and caller-supplied secret. Demo Space V2 continues to use its deterministic synthetic profile; secret storage, rotation, migration, and production privacy are not demonstrated.” |
| Pitch and demo media | Local artifacts only | “The repository contains an eight-slide PDF and a 45-second silent captioned video assembled from verified local browser states. Neither is uploaded to Devpost. The last recorded hosted release predates the local source candidate, and the live Pages content was not rechecked.” |
| Static Inspector website | Verified locally | “The release WASM build and Chromium smoke cover all 11 routes, the local rollback flow, evidence tampering/restoration, and page-level overflow at 390px.” |
| Automated clean-checkout path | Verified locally | “`cargo xtask reproduce` runs the local toolchain check, complete Rust/revm/evidence gates, static website build, browser smoke, and package boundary. It does not count as external human reproduction.” |
| Reviewer source/evidence archive | Available after a clean commit | “`cargo xtask reviewer-package` creates and checks a dependency-free source/evidence archive from the exact committed `HEAD`; it does not change repository visibility or prove human reproduction.” |
| Reviewer archive runnable without Git metadata | Owner-side gate | “`cargo xtask reviewer-reproduce` extracts the exact archive and runs the automated reproduction path; it remains automated evidence, not external human reproduction.” |
| GitHub Actions browser gate | Configured; hosted runner allocation blocked | “The local equivalent passes. Final-commit runs [`35639481534`](https://github.com/AndroLay/MemoryLineage/actions/runs/35639481534), [`35639669674`](https://github.com/AndroLay/MemoryLineage/actions/runs/35639669674), [`35639841420`](https://github.com/AndroLay/MemoryLineage/actions/runs/35639841420), and [`35639973599`](https://github.com/AndroLay/MemoryLineage/actions/runs/35639973599) created a job but ended before the first step with `runner_id: 0`; no remote test result is claimed.” |
| Independent human clean-checkout reproduction | Not yet demonstrated | Do not imply external developers have reproduced the project. |
| External reviewer comprehension | Not yet demonstrated | No independent participant answers or time-to-explanation reports exist yet; the controlled protocol is only a prepared instrument. |
| Production agent adoption | Not yet demonstrated | The local reference runtime and executable example do not show use by an external production agent. |
| Public static Inspector | Last recorded release is prior candidate; current live content not rechecked | “The last recorded Rust/WASM Inspector release is `v1.0.1` at [memorylineage.pages.dev](https://memorylineage.pages.dev). The local source candidate `675c707405ac2afea1fd067be44a67890b0a35f2` was not deployed by this workflow.” This does not mean Demo Space V2 is deployed to Sepolia. |
| Staging environment | Not provided | No separate staging environment is claimed. |
| Semantic poisoning detection | Out of scope | Do not claim that MemoryLineage detects malicious meaning in otherwise valid memory. |
| Memory truthfulness or AI reasoning correctness | Unsupported | Do not claim either property. |
| Full historical offline ERC-1271 re-execution | Unsupported | The independent verifier currently validates `EOA_EIP712` proofs. Distinguish recorded on-chain ERC-1271 acceptance from replaying the signer contract's historical state. |
| Final ERC-8350 compliance | Unsupported | Say “conforms to the pinned draft/vector snapshot used by this build.” |
| Formal security audit, complete security, or first implementation | Not claimed | Do not claim an audit, total security, or first-of-kind status. The bounded assurance report is not a third-party audit or formal proof. |
| Judge score | Not applicable | The event publishes no numeric weighting. Do not publish an internally generated score as a judge result. |

## Competitive objective

Forkline remains the product-story and end-to-end coherence benchmark from the
reviewed public submissions. MemoryLineage targets a more explicit restore
decision, portable replay, evidence assurance levels, and named negative paths.
This matrix does not claim a higher jury score or superiority on hosted
availability merely because the site is live, nor does it claim video quality
or external adoption. Hosting improves access; it does not prove those outcomes.
