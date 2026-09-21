# Submission claim matrix

Use this matrix as the source for the website, README, pitch, and Devpost copy.
Keep the local Demo Space V2, the earlier Sepolia deployment, and the protocol
mutation corpus separate in every explanation.

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
| Recovery Decision Receipt | Verified locally and in Rust/WASM | “The receipts bind a selected snapshot commitment, named evidence source, effective authority assurance, and `strict-current-head-only-v1` policy to the replayed V2 bundle; published current-head and historical-hold artifacts cover `CURRENT_HEAD` and `KNOWN_HISTORICAL_CHECKPOINT`, while the browser/CLI also exercise divergent and unverified paths. They do not claim runtime enforcement.” |
| Protected resume decision gate | Verified in the Rust `ml-recovery-gate` adapter and CLI fixture lane | “The adapter verifies the receipt and invokes its loader callback only for `RESUME_ALLOWED`; a known historical checkpoint is held as `REHEARSE_ONLY`. This is a generic reference adapter, not an integration with a production agent framework.” |
| Reference agent runtime integration | Verified locally | “`ml-agent-runtime` loads the current-head snapshot into an in-memory session, holds historical and diverged snapshots before the loader, and fails closed on invalid evidence. This is a framework-neutral local integration; external adoption is not claimed.” |
| Bounded security assurance | Verified locally | “Rust/revm exercises the checked-in Solidity artifact across five valid transitions, six stale predecessors, two sequence failures, the 20-case mutation matrix, ERC-1271, and authority rotation. The report explicitly remains `NOT_FORMALLY_VERIFIED`.” |
| Polkadot Hub portability rehearsal | Verified locally; independently replayable; no deployment | “The same checked-in Solidity bytecode and tested invariants are rehearsed through local revm with the Polkadot Hub TestNet chain context. An independent Rust verifier replays both nested V2 observations; transition/root and stale-predecessor results match, while EIP-712 remains chain-bound. This is not a public Polkadot deployment, RPC observation, or cross-chain consensus proof.” |
| Recovery receipt privacy boundary | Verified by schema and artifact checks | “The published receipt contains commitments and evidence references only; raw SQLite values, secrets, and private locator contents are absent.” |
| Sepolia rejection probe | Separate read-only observation | “The browser can issue a read-only `eth_call` to the existing Sepolia deployment using the same stale root. That registry space is not the Demo Space V2 history.” |
| Browser Sepolia block context | Read-only RPC observation | “Head and probe calls are pinned to a resolved `finalized` or `safe` block number and the block hash is rechecked. This is one endpoint observation, not consensus proof.” |
| Authority rotation | Verified in local Rust/revm lanes | “The Demo Space V2 execution rotates authority before transition 3; the evidence records configNonce 0 and 1, effective sequence boundaries, and signer-to-sequence binding. Do not call this a public Sepolia rotation.” |
| 20-case mutation corpus | Verified as a separate protocol corpus | “The published local Rust/revm corpus rejected 20 of 20 expected mutation cases. These cases are not the Demo Space V2 incident.” |
| Evidence export and replay | Verified | “The website and independent Rust CLI verify the same portable V2 evidence bundle; changing the tested locator commitment returns `TRANSITION_ID_MISMATCH`.” |
| Privacy boundary | Verified within the protocol/evidence format | “The registry and portable bundle contain commitments, not raw memory. The public repository does contain synthetic sample SQLite fixtures for deterministic reproduction.” |
| Static Inspector website | Verified locally | “The release WASM build and Chromium smoke cover all 11 routes, the local rollback flow, evidence tampering/restoration, and page-level overflow at 390px.” |
| Automated clean-checkout path | Verified locally | “`cargo xtask reproduce` runs the local toolchain check, complete Rust/revm/evidence gates, static website build, browser smoke, and package boundary. It does not count as external human reproduction.” |
| Reviewer source/evidence archive | Available after a clean commit | “`cargo xtask reviewer-package` creates and checks a dependency-free source/evidence archive from the exact committed `HEAD`; it does not change repository visibility or prove human reproduction.” |
| Reviewer archive runnable without Git metadata | Owner-side gate | “`cargo xtask reviewer-reproduce` extracts the exact archive and runs the automated reproduction path; it remains automated evidence, not external human reproduction.” |
| GitHub Actions browser gate | Configured; platform startup blocked | “The local equivalent passes. Push run [`35537419699`](https://github.com/AndroLay/MemoryLineage/actions/runs/35537419699) and manual dispatch [`35537502288`](https://github.com/AndroLay/MemoryLineage/actions/runs/35537502288) ended before job startup with `jobs: []`; no remote test result is claimed.” |
| Independent human clean-checkout reproduction | Not yet demonstrated | Do not imply external developers have reproduced the project. |
| Public static Inspector | Deployed | “The Rust/WASM Inspector is available at [memorylineage.pages.dev](https://memorylineage.pages.dev) on Cloudflare Pages.” This does not mean Demo Space V2 is deployed to Sepolia. |
| Staging environment | Not provided | No separate staging environment is claimed. |
| Semantic poisoning detection | Out of scope | Do not claim that MemoryLineage detects malicious meaning in otherwise valid memory. |
| Memory truthfulness or AI reasoning correctness | Unsupported | Do not claim either property. |
| Full historical offline ERC-1271 re-execution | Unsupported | Distinguish recorded on-chain acceptance from replaying the signer contract's historical state. |
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
