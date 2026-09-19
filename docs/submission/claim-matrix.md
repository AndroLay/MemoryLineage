# Submission claim matrix

Use this matrix as the source for the website, README, pitch, and Devpost copy.
Keep the local Demo Space V2, the earlier Sepolia deployment, and the protocol
mutation corpus separate in every explanation.

| Claim | Status | Safe wording |
| --- | --- | --- |
| Sequence and predecessor continuity | Verified | “The Rust verifier replays the committed sequence and predecessor roots; the Solidity registry enforces the same transition rules.” |
| Transition and state-root derivation | Verified on the pinned corpus | “Independent Rust paths match the published ERC-8350 draft vectors and the tested Solidity implementation.” |
| Demo Space V2 | Verified as local evidence | “Three synthetic SQLite-derived commitments were executed against the published Solidity bytecode in Rust/revm. Demo Space V2 is not deployed to Sepolia.” |
| Silent Rollback stale root | Verified in local Rust/revm evidence | “The local transition-4 attempt uses the actual state root from transition 1 while the local head is at sequence 3; the Solidity execution rejects it with `BAD_PREVIOUS_STATE`.” |
| Sepolia rejection probe | Separate read-only observation | “The browser can issue a read-only `eth_call` to the existing Sepolia deployment using the same stale root. That registry space is not the Demo Space V2 history.” |
| Authority rotation | Verified in local Rust/revm lanes | “The Demo Space V2 execution rotates authority before transition 3; the evidence records configNonce 0 and 1. Do not call this a public Sepolia rotation.” |
| 20-case mutation corpus | Verified as a separate protocol corpus | “The published local Rust/revm corpus rejected 20 of 20 expected mutation cases. These cases are not the Demo Space V2 incident.” |
| Evidence export and replay | Verified | “The website and independent Rust CLI verify the same portable V2 evidence bundle; changing the tested locator commitment returns `TRANSITION_ID_MISMATCH`.” |
| Privacy boundary | Verified within the protocol/evidence format | “The registry and portable bundle contain commitments, not raw memory. The public repository does contain synthetic sample SQLite fixtures for deterministic reproduction.” |
| Static Inspector website | Verified locally | “The release WASM build and Chromium smoke cover all 11 routes, the local rollback flow, evidence tampering/restoration, and page-level overflow at 390px.” |
| GitHub Actions browser gate | Configured; remote result pending | “CI is configured to verify Rust, build the Dioxus site, and run Chromium smoke. Report the remote run only after GitHub Actions completes successfully.” |
| Independent human clean-checkout reproduction | Not yet demonstrated | Do not imply external developers have reproduced the project. |
| Production HTTPS website or staging | Not in scope for this work | No public deployment or staging environment is claimed. |
| Semantic poisoning detection | Out of scope | Do not claim that MemoryLineage detects malicious meaning in otherwise valid memory. |
| Memory truthfulness or AI reasoning correctness | Unsupported | Do not claim either property. |
| Full historical offline ERC-1271 re-execution | Unsupported | Distinguish recorded on-chain acceptance from replaying the signer contract's historical state. |
| Final ERC-8350 compliance | Unsupported | Say “conforms to the pinned draft/vector snapshot used by this build.” |
| Formal security audit, complete security, or first implementation | Not claimed | Do not claim an audit, total security, or first-of-kind status. |
| Judge score | Not applicable | The event publishes no numeric weighting. Do not publish an internally generated score as a judge result. |
