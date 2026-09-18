# Deep research source ledger — 3rd Web Hack

Tanggal riset: 8 September 2026

## Scope

Riset ini memilih proyek hackathon yang paling dapat dibuktikan terhadap empat kriteria Devpost: Innovation, Technical Feasibility, Uniqueness, dan Design. Kandidat yang diperiksa mencakup GrantTrail, IntentWatch, AttestScope, generic outcome enforcement, EIP-8025 execution proofs, dan agent trust/escrow.

## Direct answer

Setelah loop keenam, **belum ada pilihan yang terbukti mendekati skor 5**. `ResolverCompat / FillerSafety Lab` tetap menjadi artefak yang paling jauh diuji secara lokal dengan skor bukti 3,0/5. `Warden Compromise Lab` menjadi kandidat source-backed tertinggi pada 3,8/5; `RWA StatusGuard`, `HiddenRefs Explorer`, `RWA DisclosureLens`, dan `WYRIWE Agent Provenance Gate` berada di sekitar 3,7/5. Warden masih harus melewati compromised-controller harness, invariant/fuzz report, verifier kedua, dan user test sebelum skor itu dapat naik.

Loop kelima menambahkan spike lokal untuk `SlippageTruth`: lima kasus deterministik membedakan floor statis dari floor yang dihitung ulang saat eksekusi. Artefak ini menaikkan feasibility kandidat hanya secara terbatas karena belum menjalankan EVM, router, fork, oracle feed nyata, pengukuran MEV, deployment, atau validasi eksternal.

`7702InitLock` adalah pemeriksa sempit untuk risiko front-run initializer pada delegation EIP-7702. `IntentAssumptionLens` adalah pemeriksa named assumption dan payment/step evidence pada resolver ERC-7683. `7702DelegateDiff` memeriksa perubahan capability, codehash, mutability, dan chain scope. AttestScope tetap dicatat sebagai kandidat Clear Signing yang ditahan dengan skor audit putaran kedua 2,4/5; ia bukan pemenang keseluruhan.

Tidak ada klaim bahwa konsep-konsep tersebut belum pernah dibuat sama sekali. Nilai kebaruan hanya dapat dipertahankan pada capability yang dapat dibuktikan melalui fixture terhadap tool dan proyek pembanding yang ditemukan.

## Claim and source ledger

| Claim used in recommendation | Primary source | Interpretation |
| --- | --- | --- |
| Hackathon needs working MVP, source/setup, demo or video, and pitch | https://3rd-web-hack.devpost.com/ and https://3rd-web-hack.devpost.com/rules | Direct contest requirements |
| Judging uses Innovation, Technical Feasibility, Uniqueness, Design | https://3rd-web-hack.devpost.com/ | Direct judging criteria |
| Submission deadline is 27 Sep 2026 at 12:30 IST | https://3rd-web-hack.devpost.com/details/dates | Convert to WITA only for planning |
| Eligibility wording differs between overview and rules | https://3rd-web-hack.devpost.com/ and https://3rd-web-hack.devpost.com/rules | Must be confirmed before implementation/submission |
| Gallery was not published at research time | https://3rd-web-hack.devpost.com/project-gallery | Competition comparison remains incomplete |
| Ethereum security work highlights blind signing, upgrades, tooling, and trust boundaries | https://ethereum.org/reports/trillion-dollar-security/ | Problem relevance |
| Clear Signing uses ERC-7730 descriptors and independent attestations; wallets choose trust signals | https://blog.ethereum.org/2026/05/12/clear-signing-announcement and https://ethereum.org/developers/tutorials/clear-signing | Existing ecosystem baseline |
| ERC-7730 v2 is active and v3-next is draft | https://ercs.ethereum.org/ERCS/erc-7730 | Version scope and stability limit |
| Auditor workflow already covers descriptor hash, proxy/stateRefs, and maintaining attestations | https://github.com/ethereum/clear-signing-erc7730-registry/blob/master/auditors/README.md | Prevents overclaiming |
| Wallet integration and resolution already exist in Ledger | https://developers.ledger.com/docs/clear-signing/for-wallets | AttestScope must not claim to be the first wallet integration |
| Integrity and on-chain registry work remain active issues | https://github.com/ethereum/clear-signing-erc7730-registry/issues/2882 and https://github.com/ethereum/clear-signing-erc7730-registry/issues/2881 | Proposal/roadmap status |
| ERC-8176 leaves trust policy to wallets and discusses integrity signatures | https://ethereum-magicians.org/t/erc-8176-integrity-verification-for-erc-7730/27911 | Supports a policy reference implementation, not a final standard claim |
| clearsig already implements Clear Signing translation and hashing | https://github.com/Cyfrin/clearsig | AttestScope should reuse or compare, not rebuild without reason |
| Sourcify contributes Clear Signing SDK/tooling and verification data | https://docs.sourcify.dev/blog/clear-signing-launch/ and https://github.com/sourcifyeth/clear-signing | Further reduces the novelty of generic descriptor rendering/resolution |
| Rust clear_signing library already renders ERC-7730 and exposes fallback diagnostics | https://docs.rs/clear-signing/latest/clear_signing/ | AttestScope must differentiate its policy/evidence layer |
| A deployed/open SkillProof project already exists | https://github.com/Tensored-Flow/SkillProof | SkillProof candidate is not an original blank-slate opportunity |
| EAS is a general attestation primitive and documents credential/contribution use cases | https://docs.attest.org/ and https://github.com/ethereum-attestation-service/eas-docs-site/blob/main/docs/idea--zone/use--case--examples/credentials.md | Generic credential issuance is not a sufficient differentiator |
| MetaMask and Revoke.cash provide approval inspection/revocation | https://support.metamask.io/more-web3/learn/how-to-revoke-smart-contract-allowances-token-approvals and https://revoke.cash/ | AllowanceLens candidate downgrade |
| Semaphore already provides anonymous membership proofs and anti-double-signaling | https://docs.semaphore.pse.dev/ | QuietVote candidate downgrade |
| Grant escrow lifecycle overlaps ERC-8183 and existing Milestack | https://eips.ethereum.org/EIPS/eip-8183 and https://github.com/Pauhe/milestack | GrantTrail downgrade |
| Outcome controls overlap draft EIP-7906, Safe guards, assertions.eth, and MetaMask simulation | https://eips.ethereum.org/EIPS/eip-7906, https://help.safe.global/articles/6757075087-what-is-a-transaction-guard, https://assertions.eth.limo/, and https://support.metamask.io/manage-crypto/transactions/simulations/ | Generic OutcomeGuard downgrade |
| EIP-8025 targets optional execution proofs for consensus-layer payload validation | https://eips.ethereum.org/EIPS/eip-8025 | Too protocol-level for this MVP |
| Agent attestation/action and identity/reputation primitives already have drafts | https://eips.ethereum.org/EIPS/eip-8273 and https://eips.ethereum.org/EIPS/eip-8004 | Generic agent-trust downgrade |
| ETHGlobal projects already cover Clear Signing discovery/builder, verified transaction descriptions, and on-chain curation | https://ethglobal.com/showcase/clearsign-kiw61, https://ethglobal.com/showcase/veryclear-vu8i7, and https://ethglobal.com/showcase/sign-40vt7 | Second-round uniqueness downgrade |
| A current ETHGlobal project combines policy proofs, on-chain verification, agents, and ERC-7730 | https://ethglobal.com/showcase/proof-of-claw-9006a | Further overlap for generic evidence/policy claims |
| ERC-7730 discussion already describes no-attestation fallback and dangerous-contract status | https://ethereum-magicians.org/t/eip-7730-proposal-for-a-clear-signing-standard-format-for-wallets/20403?page=2 | Status model is a documented ecosystem question, not a novel claim |
| Clear Signing governance assigns trust policy to wallets | https://clearsigning.org/governance/ | Consumer policy is an integration role, not automatic uniqueness |

## Evidence required before score claims

1. Six deterministic fixtures cover all status outcomes.
2. A second verifier reproduces the same decision from the exported evidence bundle.
3. Proxy upgrade and stateRef changes alter status at the same proxy address.
4. Revocation, expiry, conflict, partial RPC, and modified bundle cases fail closed.
5. Three Web3 developers understand the status differences without author coaching.
6. A clean checkout runs the demo and tests using documented commands.
7. README includes a comparison matrix and says what is inherited from existing standards and tools.

## Audit rubric and result

The audit score is an evidence rating, not a jury prediction: 1 is an unsupported idea, 2 is a strong overlap or unresolved critical risk, 3 is a source-backed problem with an unimplemented bounded concept, 4 is a reproducible MVP with independent checks, and 5 requires MVP, independent verification, user validation, competitor comparison, and measured impact. The workspace contained no implementation, deployment, UI, or user-study evidence. The first-round ranking was AttestScope 2.7, IntentWatch 2.6, GrantTrail 2.5, Generic OutcomeGuard 2.3, SkillProof and AllowanceLens 2.2, QuietVote and Generic agent trust/escrow 2.1, and EIP-8025 execution proof 1.9. AttestScope was only the security-specific candidate, not an overall winner; no near-5 score is currently substantiated.

Second-round audit result: after checking closer ETHGlobal projects and the ERC-7730 status discussion, AttestScope is 2.8 for Innovation, 2.7 for Technical Feasibility, 1.9 for Uniqueness, and 2.2 for Design, averaging 2.4. GrantTrail averages 2.5 and IntentWatch has the highest feasibility at 3.1, so there is no overall winner. A conditional AttestScope post-MVP target is about 3.6, with Uniqueness capped around 2.8 unless a new capability is demonstrated against the closer comparators.

## Research limits

The Devpost gallery was unavailable for competitor comparison. Search coverage cannot establish ecosystem-wide nonexistence. ERC-8176, ERC-8283, and EIP-7906 status can change; the implementation must pin versions and label drafts. No code, deployment, browser flow, testnet transaction, or user study had been executed at the time this ledger was written.

## Third-round research and idea audit

### New primary and competitor evidence

| Claim used in section 17 | Source | Audit implication |
| --- | --- | --- |
| EIP-7702 changes EOA code through authorization tuples and says arbitrary delegate code can nearly control an account | https://eips.ethereum.org/EIPS/eip-7702 | Supports a security problem for delegation analysis; does not establish product novelty |
| Ethereum's EIP-7702 guidance warns about front-run initialization, proxies, mutable targets, and `chain_id=0` | https://ethereum.org/roadmap/pectra/7702/ | Gives concrete `7702InitLock` fixtures and fail-closed rules |
| EIP-4337 supports EIP-7702 accounts, validation simulation, and paymaster flows | https://eips.ethereum.org/EIPS/eip-4337 | Makes AA simulation relevant but raises implementation complexity |
| ERC-7579 standardizes modular smart accounts and module interoperability | https://eips.ethereum.org/EIPS/eip-7579 | Supports module audit and recovery concepts; establishes a crowded standard surface |
| ERC-7093 specifies a social recovery interface | https://eips.ethereum.org/EIPS/eip-7093 | Supports recovery rehearsal; does not prove demand for a new account implementation |
| ERC-7715 defines request, supported types, revoke, and rule-based execution permissions; MetaMask supplies a complete reference handler | https://eips.ethereum.org/EIPS/eip-7715 | Strong overlap for generic permission dashboards |
| MetaMask documents advanced permissions with exposure, conditions, expiration, and revocation | https://support.metamask.io/more-web3/dapps/advanced-permissions/ | Downgrades `PermissionScope Inspector` and `PermissionRevoke Calendar` uniqueness |
| EIP-5792 defines batch calls, wallet capabilities, atomicity, call status, and fallback behavior | https://eips.ethereum.org/EIPS/eip-5792 | Makes compatibility probe feasible but generic |
| ERC-7683 requires resolvers to expose assumptions and solvers to validate assumptions before fulfillment | https://eips.ethereum.org/EIPS/eip-7683 | Provides the strongest source-backed wedge for `IntentAssumptionLens` |
| ETHGlobal projects already implement ERC-7683 solver, auction, escrow, and settlement flows | https://ethglobal.com/showcase/intentflow-eayki and https://ethglobal.com/showcase/octointents-ciu6n | Prevents claiming a new generic intent/solver protocol |
| EIP-7906 defines transaction assertions and depends on EIP-8141 | https://eips.ethereum.org/EIPS/eip-7906 | Supports a local postcondition simulator but exposes protocol and coverage risk |
| EIP-712, ERC-7713, and ERC-6384 cover typed and human-readable signature surfaces | https://eips.ethereum.org/EIPS/eip-712, https://eips.ethereum.org/EIPS/eip-7713, and https://eips.ethereum.org/EIPS/eip-6384 | Makes a generic signature firewall overlap with Clear Signing and wallet UX |
| Safe modules and module guards can execute arbitrary transactions and a broken guard can block a Safe | https://docs.safe.global/advanced/smart-account-modules and https://docs.safe.global/reference-smart-account/guards/setModuleGuard | Supplies concrete module-risk fixtures |
| Ethereum light clients can verify RPC responses, while Nethereum and Myotis implement verified state paths | https://ethereum.org/developers/docs/nodes-and-clients/light-clients, https://docs.nethereum.com/docs/consensus-light-client/guide-verified-state/, and https://github.com/biafra23/myotis | Confirms the problem but makes a full verified RPC product too large and non-unique |
| EIP-7702 security tooling and delegation products already exist | https://ethglobal.com/showcase/aegis7702-93wwp, https://delegateguard.vercel.app/, https://www.curvegrid.com/blog/2026-02-13-a-practical-look-at-eip-7702-and-wallet-delegation, and https://github.com/codeesura/eip7702-clean-delegation | Caps uniqueness for `7702DelegateDiff` and `7702Rescue` until an exact capability gap is demonstrated |
| Simulation, transaction outcome, and wallet testing projects already exist | https://ethglobal.com/showcase/prank-wallet-cgnb3 and https://playground.thirdweb.com/account-abstraction/eip-5792 | Downgrades generic `PostconditionLab`, `WalletCapsProbe`, and `BatchReceipt` |
| EAS, Semaphore, agent identity/attestation, and Proof of Claw provide existing primitives or products | https://docs.attest.org/, https://docs.semaphore.pse.dev/, https://eips.ethereum.org/EIPS/eip-8004, https://eips.ethereum.org/EIPS/eip-8273, and https://ethglobal.com/showcase/proof-of-claw-9006a | Downgrades generic credential, privacy, and agent-policy ideas |
| Frontend asset verification has an existing on-chain proof approach | https://blog.ethstorage.io/client-side-verification-for-on-chain-frontends/ | Prevents a first-mover claim for `FrontendIntegrityStamp` |

### Third-round audit result

The same evidence rubric was applied: 1 is unsupported, 2 is materially overlapped or unresolved, 3 is a source-backed bounded concept, 4 requires a running MVP plus independent checks, and 5 requires MVP, independent verification, user validation, competitor comparison, and measured impact. Current averages for the 21 new ideas are: `7702InitLock` 2.8; `7702DelegateDiff` 2.6; `IntentAssumptionLens` 2.6; `RecoveryDrill` 2.5; `SolverRiskReceipt`, `TypedSignFirewall`, and `ModuleScopeAudit` 2.5; `PermissionScope Inspector` and `PostconditionLab` 2.4; `PermissionRevoke Calendar`, `WalletCapsProbe`, `BatchReceipt`, `GovernancePayloadDiff`, `7702Rescue`, `VerifiedRPCLens`, `FrontendIntegrityStamp`, `AttestationFreshnessInbox`, and `AgentActionPermit` 2.3; `ReceiptProofPack`, `PrivateCredentialGate`, and `GrantEvidenceEscrow` 2.2. Values are rounded to one decimal; they are evidence ratings, not jury forecasts.

The third-round shortlist is `7702InitLock` and `IntentAssumptionLens`. The first must prove that its initializer race fixture detects an unbound setup and accepts only a correctly bound setup. The second must prove that a resolver assumption or payment mutation changes the decision and the exported evidence can be replayed by a second verifier. `7702DelegateDiff` is a possible companion only if it produces a capability-level output distinct from the initializer check. None is allowed to claim a near-5 score before implementation and independent evidence exist.

### New audit limits

The third-round search still cannot prove ecosystem-wide nonexistence. EIP-7702 tooling, ERC-7683 projects, wallet capabilities, and security products may change before submission. No code, testnet transaction, browser flow, user study, or independent verifier was executed in this round. The score must be recomputed after the first spike rather than copied forward from this document.

## Fourth-loop adversarial re-ranking

### New primary and competitor evidence

| Claim used in section 18 | Source | Audit implication |
| --- | --- | --- |
| Aegis7702 already describes implementation audit before applying EIP-7702 delegation, including `init` and `swap` paths | https://github.com/aegis7702/core | `7702InitLock` remains a real fixture, but its basic capability is not sufficiently unique |
| WalletCheck checks malicious EIP-7702 delegations, displays full delegation history, and gives next steps; it explicitly limits its safety claim | https://www.mywalletcheck.org/ | A standalone `7702HistoryForensics` idea has direct overlap and should be downgraded |
| Etherscan exposes authority, delegated address, nonce, signature fields, current delegation, and past authorizations | https://info.etherscan.com/pectra-upgrade-whats-new/ | Explorer history is already available; a new product needs a different capability |
| ERC-7683 says resolvers must produce well-formed, solver-safe orders except named assumptions, and solvers must validate assumptions | https://github.com/ethereum/ERCs/blob/master/ERCS/erc-7683.md | Provides explicit invariants for a bounded conformance/mutation harness |
| ERC-7683 security considerations require analysis across execution and settlement, including solver assets, permissions, obligations, and payment paths | https://github.com/ethereum/ERCs/blob/master/ERCS/erc-7683.md#security-considerations | Supports evidence checks beyond a payload viewer; still does not establish full economic safety |
| OpenZeppelin's Across audit found a missing zero-address validation and noted a lack of unit tests for two ERC-7683 implementation contracts | https://www.openzeppelin.com/news/across-protocol-svm-solidity-audit | Demonstrates testability of the surface, but the published findings are not an unpatched vulnerability claim |
| ERC-7683 resolver redesign was merged into ethereum/ERCs, while the reference repository and Uniswap allocator repository provide implementations/tests | https://github.com/ethereum/ERCs/pull/1741, https://github.com/frangio/erc7683, and https://github.com/Uniswap/sc-allocators | ResolverCompat must be a bounded independent harness, not a claim to implement the standard or solver |
| ERC-8203 has a reference implementation, test vectors, and an agent metered-billing profile | https://ethereum-magicians.org/t/erc-8203-agent-off-chain-conditional-settlement-extension-interface/28041 | A generic conditional-settlement probe is already too close to existing work for the main idea |
| ERC-8244 discussion still addresses iframe isolation, CSP, provider boundary, network access, and raw HTML security | https://ethereum-magicians.org/t/erc-8244-contract-hosted-application-html/28407 | Onchain UI safety is interesting but standard/security surface is unresolved and uniqueness is weaker |

### Fourth-loop result

The repeated procedure was: search for an exact or near-exact capability, identify a reproducible failure surface, then apply a stop condition. `7702InitLock` fell from 2.8 to 2.3 after the Aegis7702 implementation audit overlap. A standalone EIP-7702 history product fell to 2.5 after WalletCheck and Etherscan evidence. `IntentAssumptionLens` remains 2.6 as a viewer, but its implementation wedge is now `ResolverCompat / FillerSafety Lab` at 3.0 after a local spike: Innovation 3.0, Technical Feasibility 3.5, Uniqueness 2.8, Design 2.8. The average stays 3.0 after rounding.

`ResolverCompat` is a source-backed bounded concept, not a completed product score. Its MVP target is three fixtures, twelve mutated payloads, eight invariant checks, one open-source fixture, a JSON evidence bundle, and a second verifier. The target can rise to about 4.0 only after those artifacts exist, an unsafe mutation is actually rejected, two external developers understand the result, and a comparison matrix confirms the exact capability gap. A score near 5 is still unsupported.

### Required implementation and audit evidence

1. Pin the ERC-7683 version/commit and state that the harness is not an economic-security guarantee.
2. Decode and validate steps, variables, payments, named assumptions, hard dependencies, and revert policies.
3. Mutate target, selector, chain, payment recipient, amount, deadline, assumption, variable index, and replay state.
4. Export deterministic evidence containing payload hash, fork/block, policy version, failed invariant, and replay command.
5. Run a second verifier that reads only the evidence bundle and reaches the same verdict.
6. Process one open-source implementation without hard-coded addresses and document unsupported semantics.
7. Stop the idea if the first 4–8 hour spike cannot produce one meaningful reject/pass difference.

### Actual spike evidence

The throwaway spike in `spike/` ran three base fixtures and twelve mutation cases. The valid fixture returned `PASS`; a mutated payment recipient returned `REJECT` with `PAYMENT_RECIPIENT_MISMATCH`; an unknown assumption returned `UNVERIFIED` with `UNKNOWN_ASSUMPTION`. All twelve mutations returned `REJECT` or `UNVERIFIED`, all fifteen evidence files replayed to the same verdict, and a manual change to a valid evidence check was rejected by the evidence hash check. The generated summary is `evidence/generated/research-spikes/summary.json` and the runner/test source is `research/spikes/run_spike.py` plus `research/spikes/test_resolver_compat.py`.

This passes the first spike gate and raises only Technical Feasibility from 3.2 to 3.5. It does not raise Uniqueness or Design, and it does not satisfy the open-source fixture, user validation, deployment, or independent external implementation gates.

### Fourth-loop limits

The search for an exact public ERC-7683 conformance product was targeted, not exhaustive; absence of a result is not proof of nonexistence. The local ResolverCompat code and fixture execution now exist as throwaway artifacts, but no open-source resolver was executed in a fork, no user test or deployment was performed, and no independent external implementation verified the result. `PASS` must be described as “passes the selected checks,” never as a universal safety guarantee.

## Fifth-loop research and candidate audit

### New primary and competitor evidence

| Claim used in section 19 | Source | Audit implication |
| --- | --- | --- |
| ERC-8377 replaces a stale static `minAmountOut` with an execution-time reference-relative floor and its review addressed mid-price, freshness, arbitrary route data, recipient output, rounding, deadline, and adversarial sandwich behavior | https://ethereum-magicians.org/t/erc-8377-reference-relative-slippage-bounds/29292 and https://github.com/zexoverz/reference-relative-slippage-bounds | Gives `SlippageTruth` a concrete failure surface and review trail, but the product must demonstrate measured behavior rather than copy the draft |
| Spot already provides oracle protection, slippage caps, freshness, order types, audit reports, and multi-chain deployment | https://github.com/orbs-network/spot | Removes any “first protected swap” claim; the wedge must be replayable static-vs-live evidence and benchmark output |
| ERC-8392 defines token-level asset status, including market session, interruption, valuation, primary status, timestamps, and `UNKNOWN`; its PR currently has a naming/file mismatch with ERC-8391 | https://ethereum-magicians.org/t/erc-8392-asset-status-interface-for-tokenized-assets/29489 and https://github.com/ethereum/ERCs/pull/1964 | `RWA StatusGuard` addresses a real adapter gap but must pin the draft and resolve status semantics before implementation |
| Ondo exposes market status through a status site/API and Robinhood documents its own stock-token APIs and pause state | https://status.ondo.finance/market, https://docs.ondo.finance/api-reference/overview, and https://docs.robinhood.com/chain/stock-tokens/ | Supports a two-issuer adapter fixture; it does not prove that issuer status is independently truthful |
| ERC-8382 defines private NFT-to-NFT reference commitments, selective reveal, replay binding, and a graph-oriented relationship model; ERC-5521 covers public relationships | https://ethereum-magicians.org/t/erc-8382-private-referable-nfts/29442, https://github.com/ethereum/ERCs/pull/1955, and https://eips.ethereum.org/EIPS/eip-5521 | `HiddenRefs Explorer` has a clearer uniqueness/design wedge, but hidden commitment cannot be presented as proof of genuine contribution or entitlement |
| ERC-8339 has a two-phase transfer lifecycle and a reference implementation; SafeSend and Coinbase cover adjacent recipient/wrong-address protection | https://ethereum-magicians.org/t/erc-8339-two-phase-asset-transfers/29017, https://github.com/ethereum/ERCs/pull/1882, https://safesend.ch/, and https://help.coinbase.com/en/wallet/sending-and-receiving/what-happens-if-i-use-the-wrong-coinbase-wallet-address | SafeReceive has a strong pain point but direct overlap lowers uniqueness |
| ERC-8376's reference implementation went through accounting and authorization fixes and explicitly remains unaudited/not deployable as-is; GoPlus, TokenSniffer, Drydock, and LaunchProof cover adjacent launch protection | https://github.com/ethereum/ERCs/pull/1942, https://docs.gopluslabs.io/reference/api-overview, https://tokensniffer.readme.io/reference/introduction, https://drydockprotocol.org/, and https://github.com/alsaecas/launchproof | LaunchGuard needs a labeled corpus, false-positive policy, and forked escrow invariant before its impact can support a high score |
| ERC-8366 has a real Groth16 reference implementation and tests, while Circle and Tollbeam provide agent payment/spend-policy surfaces | https://github.com/fractalyze/erc-8366, https://developers.circle.com/agent-stack/agent-wallets, and https://tollbeam.com/agent-payments | PrivateSpendPolicy is technically credible but its uniqueness is constrained by existing payment products |
| ERC-8356's own discussion lists unresolved PII, self-declared usage, hash-chain, status-transition, and offchain-caller-authentication issues | https://ethereum-magicians.org/t/erc-8356-purpose-bound-third-party-data-consent/29217 | ConsentMesh remains blocked until the model and relying-party verification are repaired |
| ERC-8348, ERC-8404, ERC-8410/8409, ERC-8380, and ERC-8406 remain draft or overlap with existing lease, receipt, plan, replay, and agent-economic surfaces | https://ethereum-magicians.org/t/erc-8348-financial-lease/29076, https://ethereum-magicians.org/t/erc-8404-recomputable-verification-receipts/29521, https://ethereum-magicians.org/t/erc-8410-portable-execution-plan-artifact/29587, https://ethereum-magicians.org/t/erc-8380-unclonable-agent-execution-credentials/29274, and https://ethereum-magicians.org/t/erc-8406-fungible-agent-tokens/29220 | These remain bounded fallbacks, not current leaders |

### Fifth-loop result

The loop searched for a problem-first standard surface, checked exact or adjacent products, separated current evidence from conditional upside, and retained only candidates with an explicit next proof gate. Current evidence ratings are `SlippageTruth Lab` 3.6, `RWA StatusGuard` 3.7, `HiddenRefs Explorer` 3.7, `SafeReceive` 3.5, `ConsentMesh` 3.5, `PrivateSpendPolicy` 3.4, `LaunchGuard` 3.3, `LeaseState Lens` 3.4, `MandateConservation Auditor` 3.2, `ReceiptReplay` 3.3, `PlanSeal` 3.3, `Deactivation Exit Radar` 3.4, `CloneReplay Guard` 3.2, and `FAT Investor Console` 3.2. These are evidence ratings, not jury forecasts; no current candidate exceeds 3.7.

Conditional targets are not current scores. `SlippageTruth` can reach approximately 4.4 only after a real EVM/fork benchmark, mutation corpus, independent replay, measured adverse-output and false-reject rates, gas data, and developer testing. `RWA StatusGuard` can reach approximately 4.3 after two issuer adapters, atomic re-check behavior, status-history fixtures, and lending-integrator validation. `HiddenRefs` can reach approximately 4.3 after a contract core, independent opening verifier, graph interaction, and creator/collector testing.

The local `SlippageTruth` artifact is limited but reproducible: the runner writes five cases, replay reports zero mismatches, and the full local suite reported 12/12 passing tests at that earlier loop. The current full suite is 18/18 after adding the MemoryLineage tests. It demonstrates that a static threshold of 900 can accept recipient output 930 while a live floor of 960 rejects it, that recipient balance delta defeats a route-reported output of 1000 when the recipient receives 0, and that stale oracle or expired intent fail closed. These are arithmetic fixtures, not market-loss measurements or proof that a production router is safe. See `research/spikes/slippage_truth.py`, `research/spikes/run_slippage_truth.py`, `research/spikes/test_slippage_truth.py`, and `evidence/generated/research-spikes/slippage_truth_summary.json`.

### Fifth-loop limits and decision

The Devpost gallery remained unavailable for competitor comparison, and targeted web search cannot prove ecosystem-wide nonexistence. Draft numbering and semantics can change; in particular, the ERC-8392/8391 naming inconsistency must be pinned before implementation. No browser flow, EVM fork, deployed contract, real oracle integration, MEV dollar measurement, or external user study was performed in this loop.

The next evidence order is `SlippageTruth Lab` -> `RWA StatusGuard` -> `HiddenRefs Explorer`. `SlippageTruth` is first because it now has a local failure case and a reference implementation with fork/adversarial tests. If its fork gate cannot show a measurable static-vs-live decision difference, switch to RWA StatusGuard for integration value or HiddenRefs for uniqueness/design. The research does not support a near-5 claim yet.

## Sixth-loop research and candidate audit

### New primary and competitor evidence

| Claim used in section 20 | Source | Audit implication |
| --- | --- | --- |
| ERC-8233 Warden separates controller logic from ERC-20 custody and protects time-locked, designated, and directly withdrawable balances; its reference repository includes contracts/tests and states that the implementation is unaudited | https://github.com/ethereum/ERCs/pull/1687 and https://github.com/AuHau/erc-warden | Strong bounded compromise demo, but no claim of production safety until the reference is independently tested and the lab’s own adversarial harness passes |
| OpenZeppelin TimelockController delays maintenance operations; ERC-4626, ERC-6229, ERC-7444, and ERC-1620 cover adjacent vault, lock, maturity, or streaming surfaces | https://github.com/OpenZeppelin/openzeppelin-contracts/blob/master/contracts/governance/TimelockController.sol, https://eips.ethereum.org/EIPS/eip-4626, https://eips.ethereum.org/EIPS/eip-6229, https://eips.ethereum.org/EIPS/eip-7444, and https://eips.ethereum.org/EIPS/eip-1620 | Warden’s uniqueness must be stated as the combined controller/custody, designation, account isolation, and direct-withdrawal invariant; no “first timelock” claim |
| A new RWA disclosure proposal explicitly says its ABI, profile format, reference implementation, and conformance suite are not final; an earlier RWA Disclosure Interfaces proposal defines backing, valuation, supply, audit, document anchors, attester, expiry, and hashes | https://ethereum-magicians.org/t/pre-erc-discussion-a-common-interface-for-rwa-disclosure-records-evidence-and-history/29500 and https://ethereum-magicians.org/t/erc-proposal-rwa-disclosure-interfaces/28679 | `RWA DisclosureLens` has a real interoperability gap but low protocol maturity; pin a fixture/profile rather than claim a finalized ERC |
| RWA.xyz provides issuer-contributed verified API/reference data and token-level metadata, while Securitize and Ondo provide end-to-end or issuer-specific RWA infrastructure | https://app.rwa-xyz.com/platform-overview, https://docs.rwa.xyz/, https://securitize.io/, and https://docs.ondo.finance/ | A disclosure checker must validate provenance, expiry, and policy coverage rather than recreate another RWA analytics dashboard |
| ERC-8263 anchors agent identity/model/input/output/timestamp/nonce digests; the discussion proposes the triple hash of raw input, sanitization pipeline, and actual input, with OCP and hbs-attestation-poc as adjacent/reference work | https://ethereum-magicians.org/t/erc-8263-onchain-proof-layer-for-ai-agents/28577, https://github.com/damonzwicker/observation-commitment-protocol, and https://github.com/Echo-Merlini/hbs-attestation-poc | `WYRIWE Agent Provenance Gate` can show a concrete mismatch, but it must add an independent consumer workflow beyond existing commitment and attestation components |
| ERC-8264 defines read/write/delete/export memory rights; ERC-8269 defines portable encrypted capsules/body leases and points to rmem-gateway; ERC-8350 provides sequenced memory-state commitments with public Sepolia deployment | https://ethereum-magicians.org/t/erc-8264-ai-agent-memory-access-rights/28584, https://ethereum-magicians.org/t/erc-8269-body-lease-and-credential-broker/28597, and https://ethereum-magicians.org/t/erc-8350-agent-memory-state-registry/29098 | Memory ideas have a real boundary, but PII, deletion, pagination, availability, and legal claims must be tested before a high score is defensible |
| ERC-8217 standardizes the `agent-binding` metadata key and `bindingOf` verification; Adapter8004, ENSWhois, NameWhisper, and mm-plugin already implement or expose binding/orphan behavior | https://ercs.ethereum.org/ERCS/erc-8217, https://github.com/unruggable-labs/adapter, https://docs.enswhois.com/agents, https://namewhisper.ai/agents, and https://github.com/estmcmxci/mm-plugin-ensv2 | `AgentBinding OrphanRadar` has a reproducible interoperability check but is too close to existing adapters and status displays |
| SOAC proposes a coordinated family for inventory, role naming/semantics, operation restriction, time bounds, delays, timelocks, and emergency state/response, but its reference implementation is “coming soon” | https://ethereum-magicians.org/t/introducing-soac-security-operation-and-access-control-framework/28859 | Strong problem statement, low current feasibility; OpenZeppelin Defender and Forta already cover role management, monitoring, and response |
| ERC-8287 proposes Orchard-style private fungible tokens with frozen-root compliance; ERC-8086 has a reference deployment/testnet; ERC-7984 and OpenZeppelin provide an adjacent confidential-token surface | https://ethereum-magicians.org/t/erc-8287-privacy-native-fungible-tokens/28702, https://ethereum-magicians.org/t/erc-8086-privacy-token/26623, and https://ethereum-magicians.org/t/erc-7984-confidential-fungible-token-interface/24735 | Privacy-token conformance is innovative but cryptography, circuit, verifier, and wallet dependencies cap near-term feasibility |
| ERC-8232 gives represented-RWA agents scoped permissions and has a Vyper reference implementation; ERC-8226 has a reference registry and a live Sepolia integration | https://ethereum-magicians.org/t/erc-8232-onchain-agency-for-represented-rwas/28240 and https://ethereum-magicians.org/t/erc-8226-regulated-agent-mandate/28208 | `RWA AgencyGuard` should be absorbed into RWA StatusGuard/mandate work; a separate project would duplicate the control plane |
| ERC-8126 proposes AI-agent security verification and risk scoring, while current ETHGlobal projects list AgentIndex, AgentRankr, Agentbook, and Agentic Passport | https://ethereum-magicians.org/t/erc-8126-ai-agent-verification/27445 and https://ethglobal.com/showcase?events=newyork2026 | A generic agent risk passport is directly overlapped; only a narrow verification profile could survive |

### Sixth-loop result

The sixth sweep added `Warden Compromise Lab` at 3.8, `RWA DisclosureLens` at 3.7, `WYRIWE Agent Provenance Gate` at 3.7, `AgentBinding OrphanRadar` at 3.5, `SOAC PrivilegeOps Drill` at 3.4, `PrivacyToken Interop Lab` at 3.3, `MemoryRights Capsule` at 3.4, `RWA AgencyGuard` at 3.4, and `AgentRisk Passport` at 3.2. The dimension ratings and target gates are recorded in section 20 of the recommendation document. These are evidence ratings, not jury forecasts, and no current candidate is near 5.

`Warden Compromise Lab` leads on paper because the failure mode is directly measurable: compare how much a malicious controller can redirect in a direct-custody baseline and under Warden rules, then verify designated-balance, expiry, direct-withdrawal, and controller-isolation invariants. The reference repository has a concrete implementation and tests, but it explicitly says it is unaudited. The local workspace has not run that repository and has no Forge/Anvil binary, so the score must not be described as locally proven.

`RWA DisclosureLens` and `WYRIWE Agent Provenance Gate` have credible source-backed wedges but remain draft/integration concepts. `AgentBinding OrphanRadar`, SOAC, privacy-token, memory, and generic agent-risk ideas were downgraded after exact or adjacent products and standards were found.

### Sixth-loop limits and decision

The search still cannot prove ecosystem-wide nonexistence. ETHGlobal showcase pages are useful competitor evidence, not a complete market census. Draft standards, numbers, deployed addresses, and product pages can change before the Devpost deadline. No Warden contract execution, EVM fork, fuzz run, deployment, browser flow, or external user study was performed in this loop.

The next evidence order is `Warden Compromise Lab` if Solidity/Hardhat is available, followed by the already executable `SlippageTruth Lab`. If Warden cannot produce an independently replayable compromised-controller failure/pass matrix, keep SlippageTruth as the implementation path and retain RWA StatusGuard/HiddenRefs as fallbacks. The evidence still supports no near-5 claim.

## Seventh-loop source ledger and audit

| Claim | Primary source | Audit consequence |
| --- | --- | --- |
| ERC-8410 proposes a JSON portable execution plan for one sender and one chain, a digest over broadcast-determining fields, an artifact-reference envelope, canonicalization rules, JSON schemas, four digest vectors, and one Rust/JavaScript cross-check; the PR names Ekubo Wallet as its reference implementation and remains open/draft | https://github.com/ethereum/ERCs/pull/1992 and https://github.com/EkuboProtocol/wallet | `PlanSeal` rises from 3.3 to 3.8, but the external vectors and wallet are not our MVP. The target 4.7 requires our own implementation, independent verifier, mutation corpus, wallet/testnet flow, comparison, user tests, and measured false-reject/overhead data. |
| ERC-8313 frames Protocol Interaction Manifest as a machine-readable intent/lookup/execution workflow that supplements an ABI; the official PR and discussion describe safety checks, simulation, trust levels, and risk disclosures | https://github.com/ethereum/ERCs/pull/1836, https://ethereum-magicians.org/t/providing-protocol-interaction-knowledge-in-machine-readable-files-translating-intent-into-transactions/28663, and https://medium.com/coinmonks/introducing-erc-8313-protocol-interaction-manifests-1ef8bfc63040 | `ManifestTruth` is a credible second route with current evidence 3.8 and conditional target 4.7, but exact-product absence is only a targeted-search result, not proof of ecosystem nonexistence. |
| ERC-8238 describes a coercion-resistant vault and the discussion reports a 71/71 Foundry suite, Sepolia fork integration, verified deployment, and live demo; reference repo is https://github.com/DeFiRe-business/eip-proposal-5wrench | https://ethereum-magicians.org/t/erc-8238-coercion-resistant-vault/28130 and https://github.com/DeFiRe-business/eip-proposal-5wrench | `CoercionProof Scenario Lab` becomes a 3.8–3.9 challenger, but the direct reference demo plus Safe spending limits, Cedar Wallet, and Edge duress mode prevent a 4.7 target until a distinct audit-lab wedge is independently implemented. |
| Safe supports per-token spending limits and Cedar/Edge expose adjacent wallet safety or duress flows | https://help.safe.global/articles/3961440620-set-up-and-use-spending-limits, https://cedarwallet.io/, and https://edge.app/blog/crypto-basics/duress-mode/ | Do not claim coercion protection is a new wallet capability; compare invariant, scenario corpus, and residual-risk evidence instead. |
| ERC-8312 explicitly separates bounded-action metering from enforcement and discusses overlap with ERC-8226; ERC-8316 defines a programmable settlement-lock lifecycle but leaves value-system validation to implementations | https://ethereum-magicians.org/t/erc-8312-bounded-agent-actions/28851, https://ethereum-magicians.org/t/erc-xxxx-programmable-settlement-locks/28861, and https://github.com/ethereum/ERCs/pull/1840 | `BoundedSpend Cursor Lab` and `SettlementLock Explorer` remain below the 4.7 path because the standard boundary and enforcement layer are incomplete or overlapping. |
| ERC-8239 includes SkillRegistry/SkillAttestation deployments and an `asrpm` CLI with manifest/content checksum verification; the thread distinguishes artifact integrity from behavior | https://ethereum-magicians.org/t/erc-8239-agent-skill-registry/28335 | `SkillPackage Integrity Lab` is a possible 4.2 fallback, but it cannot claim that checksum equality proves a skill behaves safely. |
| ERC-8409 is a very recent EIP-712 signed service payment quote proposal binding issuer, payer, chain, asset, recipient, amount, request, quote ID, and validity window | https://ethereum-magicians.org/t/erc-8409-signed-service-payment-quotes/29577 | `QuoteTruth` remains 3.2 until a reference implementation, replay vectors, settlement binding, and user evidence exist. |

### Loop-seven score decision

The only score increase is `PlanSeal` because the new evidence adds concrete digest vectors and an independent-language cross-check to an already scoped capability. `Warden` remains 3.8: reference evidence is stronger, but it is not evidence from this workspace and its uniqueness is reduced by the reference product and adjacent wallets. `RWA AgencyGuard` falls to 3.3 after the bounded-metering and mandate overlap is clearer. All other previous candidates remain at their last audited value; full row-by-row evidence is in `AUDIT_IDE_KANONIK_2026-09-08.md`.

The 4.7 figures are conditional targets only. `PlanSeal` uses target dimensions 4.7, 4.8, 4.6, and 4.8, averaging 4.725 and rounding to 4.7. `ManifestTruth` uses 4.7, 4.7, 4.6, and 4.8, averaging exactly 4.7. Neither target is an achieved score until the evidence gates in the recommendation document pass.

## Eighth-loop claim-versus-actual audit

The strict audit separates three things that had previously been adjacent in the tables:

| Classification | Actual evidence in this workspace |
| --- | --- |
| Actual local | ResolverCompat and SlippageTruth source, tests, generated JSON, and replay output |
| External source-backed | ERC-8410 and ERC-8313 open/draft PR pages; ERC-8238’s reference-project report of 71/71, Sepolia, deployment, and demo |
| Conditional target | PlanSeal 4.7 and ManifestTruth 4.7; no required gate has passed |

The ERC-8410 PR page is real and describes schemas, vectors, cross-checking, and Ekubo Wallet, but it is still Open/Draft, requests additional review, and records a commit with CI errors. The ERC-8313 PR is also Open/Draft, requests review, and records a commit with errors. The workspace contains no PlanSeal or ManifestTruth source, vector, schema runner, wallet adapter, testnet transaction, or independent verifier. Therefore their 3.8 values are research ratings, not actual project scores.

The local claim at that earlier audit was narrower and reproducible: the unittest suite was 12/12, ResolverCompat reported three fixtures and twelve mutations, and SlippageTruth reported five cases with zero replay mismatches. The current suite is 18/18 and also covers the MemoryLineage spike. All artifacts remain arithmetic/local spikes; they do not establish production, EVM-fork, economic, deployment, or user evidence.

## Ninth-loop deep research: MemoryLineage Auditor / ERC-8350

| Claim used in the deep-research report | Primary source | Audit consequence |
| --- | --- | --- |
| ERC-8350 defines a sequenced memory-state registry with a space identifier, authorizer, predecessor, sequence number, state root, and transition metadata; the discussion reports a public Sepolia deployment and publishes the state-transition invariants and nonclaims | https://ethereum-magicians.org/t/erc-8350-agent-memory-state-registry/29098 | This is a concrete protocol-shaped wedge for an agent-memory audit, but a draft discussion and deployment report are external evidence. They do not prove our contract, verifier, or product flow. |
| The ERC-8350 repository contains a Solidity registry, two isolated TypeScript implementations, a golden vector, a fixture Space, and a check command; it also states that an external implementation and security review are still needed | https://github.com/AwareLiquid/ERC-8350 | Feasibility is materially stronger than an idea-only candidate. Same-repository implementations do not count as independent verification, and the repository’s own open work remains a gate. |
| The published vector specifies exact `spaceId`, typehashes, `transitionId`, and `nextStateRoot` values | https://github.com/AwareLiquid/ERC-8350/blob/main/test-vectors/v1.json | The workspace can test byte-level commitment compatibility rather than merely assert that a hash exists. This supports a research rating, not a live-chain score. |
| ERC-8350’s official ERC PR remains Open/Draft and discusses assignment, immutability, and review status | https://github.com/ethereum/ERCs/pull/1910 | Do not present ERC-8350 as an accepted/final standard. Pin the commit and surface the draft status in any demo. |
| OWASP describes agent-memory poisoning as an attack surface; its Agent Memory Guard provides hashing, protected-key checks, anomaly/size checks, snapshots, rollback, and middleware | https://genai.owasp.org/2026/05/13/memory-is-a-feature-it-is-also-an-attack-surface/, https://owasp.org/www-project-agent-memory-guard/ | The problem is independently grounded. It also exposes the product bar: the demo must show a concrete memory mutation and recovery/audit decision, not only a blockchain hash. |
| AgentMem, Timechain Agent, Engram, and Daryl already cover grounded or append-only agent memory, hash-chain/timestamping, encrypted user-owned memory, or signed replay/audit behavior | https://github.com/agentmem/agentmem, https://github.com/FrostedFlaming0/timechain-agent, https://github.com/rogerdemello/engram, and https://github.com/daryl-labs-ai/daryl | MemoryLineage’s uniqueness must be the verifiable linear state-transition and authority audit layer with an independent verifier and attack lab. “Blockchain memory” alone is not a defensible novelty claim. |
| ERC-8281/OCP, ERC-8404/RVR, and ERC-8354/CAPV provide adjacent observation, execution-receipt, and policy-verdict surfaces | https://ethereum-magicians.org/t/erc-8281-observation-commitment-protocol-ocp/28399, https://ethereum-magicians.org/t/erc-8404-recomputable-verification-receipts/29521, and https://eips.ethereum.org/EIPS/eip-8354 | These are comparison baselines. A MemoryLineage demo should explain why memory-state continuity and rollback detection are the narrow user job, rather than repackage generic receipts or policy proofs. |

### Ninth-loop local evidence

The workspace spike is `verifier/python/memory_lineage/model.py`, with runner `verifier/python/memory_lineage/demo.py` and tests in `spike/test_memory_lineage.py`. It implements a dependency-free Keccak-256 commitment model and checks the published ERC-8350 vector fields. The runner exercises one valid four-transition history and six mutations: rollback, sequence gap, parallel history, unauthorized authorizer, locator substitution, and payload tamper. The latest fresh run reports 18/18 unit tests passing, seven runner cases, six mutation cases rejected/non-pass, all published-vector checks true, and zero replay mismatches. The generated summary is `evidence/generated/memory_lineage_summary.json`.

This local evidence proves only the selected commitment and linear-history rules. It does not prove Solidity equivalence, EIP-712 EOA/ERC-1271 authorization, event indexing, proxy immutability, testnet behavior, data availability, semantic truth of memory contents, poisoning resistance in a real agent, or user value. The spike is an exploratory artifact, not production code.

### Ninth-loop score decision

`MemoryLineage Auditor / ERC-8350` receives a current research rating of **4.2/5** because it has an independently grounded problem, a public draft protocol surface, published vectors, a reference repository, and a reproducible local mutation audit. It does **not** receive 4.7: the local implementation is not an EVM implementation, the external implementations are not independent from one another, the contract and authorization boundary have not been run here, and no user or testnet evidence exists.

The conditional 4.7 gate requires: an EVM contract or faithful fork fixture; EOA and ERC-1271 authorization tests; an independent verifier separate from the publisher; valid and invalid transition vectors; mutation tests for rollback, sequence gaps, forked histories, unauthorized writers, locator substitution, and payload tampering; an agent-memory poisoning fixture with before/after audit output; a testnet transaction; gas/latency measurements; and developer/user testing. Any missing required gate keeps the candidate below 4.7.

The **ninth-loop snapshot** ordered `MemoryLineage Auditor / ERC-8350` as the strongest research challenger at 4.2, with `PlanSeal` and `ManifestTruth` at 3.8. The tenth-loop recalibration below supersedes that snapshot. No current candidate has an actual, independently verified 4.7 score.

## Tenth-loop score recalibration

The latest audit rechecked source status and recalculated all currently decision-relevant averages with decimal values retained until the final step. The prior score table mixed truncation and ordinary rounding: MemoryLineage dimensions `4.6 + 4.0 + 3.9 + 4.5 = 17.0`, or exactly `4.25`, and Execution Evidence Chain dimensions `4.2 + 3.4 + 3.4 + 4.0 = 15.0`, or exactly `3.75`. Applying one consistent half-up rule changes them to 4.3 and 3.8 respectively.

| Claim used in the recalibration | Current primary evidence | Score implication |
| --- | --- | --- |
| ERC-8350 remains a draft with a public Sepolia registry and external recomputation evidence; the official PR is still Open/Draft and awaits editor review | https://ethereum-magicians.org/t/erc-8350-agent-memory-state-registry/29098, https://github.com/AwareLiquid/ERC-8350, and https://github.com/ethereum/ERCs/pull/1910 | No new product dimension is granted. The research average is corrected from 4.2 to 4.3; the workspace still has no EVM MVP. |
| ERC-8410 still has a concrete schema, four digest vectors, Rust/JavaScript cross-check, and Ekubo reference flow, while its PR is Open/Draft and requires another editor review | https://github.com/ethereum/ERCs/pull/1992 | PlanSeal remains 3.8. External artifact quality does not become a local conformance MVP. |
| ERC-8313 remains an Open/Draft Protocol Interaction Manifest proposal requiring editor review and carrying no equivalent local conformance artifact | https://github.com/ethereum/ERCs/pull/1836 | ManifestTruth remains 3.8. |
| ERC-8238 reports 71/71 Foundry tests, Sepolia fork integration, verified deployment, and live demo, while direct wallet/reference overlap remains | https://ethereum-magicians.org/t/erc-8238-coercion-resistant-vault/28130 | CoercionProof remains 3.8: feasibility evidence is strong but uniqueness is capped and the implementation is not ours. |
| ERC-8354 explicitly says a valid proof establishes faithful evaluation of the committed ruleset, not policy correctness or interpreter fidelity | https://eips.ethereum.org/EIPS/eip-8354 | CAPV remains 3.9; its upside does not remove the fidelity and proving blockers. |
| OCP remains a generic observation/evidence primitive and ERC-8330 remains a status/freshness interface whose staleness flags only protect consumers that check them | https://github.com/damonzwicker/observation-commitment-protocol and https://eips.ethereum.org/EIPS/eip-8330 | OCP remains 3.8 and NAVFreshness remains 3.7. |

The recalibrated numeric research values are `MemoryLineage` 4.3, `CAPV` 3.9, `PlanSeal` 3.8, `ManifestTruth` 3.8, `Execution Evidence Chain` 3.8, `OCP` 3.8, and `CoercionProof`/`Warden` 3.8. Numeric research rank is separate from implementation priority: CAPV has a higher paper score but its ZK/fidelity/liveness gates are heavier, so local evidence and bounded feasibility still make MemoryLineage the selected next probe.

No score decrease is supported by this loop. All other candidates remain at their prior values, including the earlier `RWA AgencyGuard` decrease from 3.4 to 3.3. No candidate has an actual 4.7 score.

## Eleventh-loop source ledger: external benchmarks and memory rights

| Claim | Primary source | Audit consequence |
| --- | --- | --- |
| FlexGov reports a live governance-observability product with deterministic metrics, four counterfactual weighting rules, Graph-backed pagination, canonical hashes, and 34 tests | https://ethglobal.com/showcase/flexgov-ooe2m and https://github.com/abrown1564/flexgov | Use it as an upper-bound benchmark. Rebuilding the same core report does not preserve uniqueness for our submission. |
| KSwap-VM reports bytecode-level formal verification, negative controls, a reusable K semantics layer, and 281 properties; its repository says 20/52 opcodes are modelled and some layer-two theorems remain `ADMITTED` | https://ethglobal.com/showcase/kswap-vm-aix5n and https://github.com/vovunku/swap-vm-verified | Strong proof discipline but direct overlap and explicit coverage limits prevent a new generic formal-verification product from claiming 4.7. |
| Doca reports Base mainnet deployment, 11 Hardhat tests, paired control measurements, and a known single-fill limitation | https://ethglobal.com/showcase/doca-finance-rjm24 and https://github.com/ottodevs/doca | Concrete DeFi benchmark; not our MVP and no originality credit for duplicating its inventory-budget controller. |
| Assay reports live ENS/Hedera/Graph challenge and slash behavior, while explicitly listing missing atomic settlement and amount/memo checks | https://ethglobal.com/showcase/assay-26egq and https://github.com/fiorelorenzo/assay | Useful evidence of measurable agent-reputation flow and an honest limitation list; generic agent service reputation is already occupied. |
| Commitment Issues reports a real SSH-agent proxy, World ID selfie check, signal-bound commit approvals, and a remaining phone diff-display hole | https://ethglobal.com/showcase/commitment-issues-y1t2h | Strong human-accountability benchmark; signing gate and identity dependencies reduce feasibility and uniqueness for a duplicate. |
| ERC-8264 defines four subject-memory operations; its companion Capsule/Body Lease proposal and reference repository report self-tests, 21/21 Foundry tests, EVM testnet deployments, Bitcoin/Solana anchors, and live verification | https://ethereum-magicians.org/t/erc-8264-ai-agent-memory-access-rights/28584, https://ethereum-magicians.org/t/erc-8269-body-lease-and-credential-broker/28597, and https://github.com/clavote-boop/rmem-gateway | MemoryRights becomes a strong 4.1 challenger/benchmark, but its reference implementation is already close to the proposed product and is not independent evidence for our workspace. |
| ERC-8263, ERC-8273, and ERC-8257 cover inference anchoring, per-operation agent attestation, and predicate-gated tool registration | https://ethereum-magicians.org/t/erc-8263-onchain-proof-layer-for-ai-agents/28577, https://ethereum-magicians.org/t/erc-8273-attestation-gated-agentic-actions/28617, and https://ethereum-magicians.org/t/erc-8257-agent-tool-registry/28457 | Absorb them as comparison/composition layers. A generic proof, attestation, or tool-registry submission has direct overlap. |

### Eleventh-loop decision

The machine-readable audit loop at `research/spikes/candidate_audit_loop.py` ran three rounds and returned no actual 4.7 hit. `MemoryLineage Auditor / ERC-8350` remains 4.3 as a research rating and retains a conditional 4.7 path only after local contract conformance, independent replay, testnet, mutation, and user-impact gates. `MemoryRights / ERC-8264 + Capsule` is 4.1 and remains a benchmark/challenger, not a reason to raise MemoryLineage. The search scope is targeted and cannot prove ecosystem-wide nonexistence.

Addendum: the local contract-conformance and independent-replay gates now pass.
The remaining missing evidence is workspace-owned public testnet deployment,
second-RPC or indexer reread, and measured developer/user impact. The ledger
therefore still reports no actual 4.7 and keeps the conditional target separate
from the 4.3 research rating.
