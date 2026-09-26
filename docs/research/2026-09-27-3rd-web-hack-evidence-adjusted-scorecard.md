# 3rd-Web-Hack Evidence-Adjusted Scorecard

**Assessment date:** 27 September 2026
**Purpose:** compare MemoryLineage with the public 3rd-Web-Hack submissions
identified so far using the same evidence-adjusted method as the earlier audit.

This is an independent heuristic assessment, not a Devpost judge score or a
prediction of placement. The [official event page](https://3rd-web-hack.devpost.com/)
lists Innovation, Technical Feasibility, Uniqueness, and Design; it does not
publish criterion weights. The table uses equal weights for those four
criteria.

## Method and limits

- Scores estimate the submission as a judge can inspect it now, not the
  creators' effort or the theoretical potential of the idea.
- The corrected head-to-head means are **4.375/5 for Forkline** and
  **4.325/5 for MemoryLineage**, displayed as 4.4 and 4.3 after one-decimal
  rounding. They differ by only **0.05**, far below the precision of this
  heuristic. The ranges overlap and do not support a meaningful rank claim.
- Checkable source, test, deployment, and demo evidence supports a higher
  Technical Feasibility score. A feature described only by its creators remains
  a claim until its source or behavior is inspected.
- Ranges reflect uncertainty in the publicly visible evidence; they are not
  statistical confidence intervals. The upper end assumes the public
  description is substantially accurate. The lower end discounts claims that
  could not be checked.
- Design is based on the available submission screenshots, recorded demo
  artifacts, and checked-in page source. The live project pages and applications
  could not be opened with the browsing tool in this assessment; this does not
  establish that those sites are down.
- The [official project gallery](https://3rd-web-hack.devpost.com/project-gallery)
  still says the organizers have not published it. This is a partial
  comparison of pages that identify 3rd-Web-Hack, not a complete census or
  ranking of all submissions.
- Web3 fit is a qualitative event-fit check, not an extra numeric criterion.
  A strong product concept with no visible on-chain role may still have a
  submission-fit problem; that note is based only on public materials reviewed.
- The one-decimal scores are judgment aids, not measurements. The aggregate
  ranges are not confidence intervals, and small differences between projects
  should not be treated as statistically meaningful.
- **Correction to the earlier Forkline estimate:** the competitor audit linked
  `sauravvenkat/forkline`, an unrelated agent-tracing project. The Devpost
  submission source is the `forkline-outbox-20260908` branch of
  `josepha-mayo/Joseph-Portfolio`, confirmed at
  `a58fe2c44cc3c8d19ac3300b8b29f91dcb4fd8af`. I read its source, tests,
  documentation, workflows, and recorded evidence and inspected the saved
  interface captures. I did **not** execute the Forkline tests or demo.
- MemoryLineage's local implementation scores draw on the current source and
  recorded release evidence in the [readiness register](../submission/readiness-register.md)
  and [project README](../../README.md). GitHub `main` and the annotated
  `v1.0.2` tag both resolve to `80257f74fbb0887fd2c6c5d0fedaeb89bccabf88`,
  matching this checkout. I did **not** rerun its tests in this score review.
- The Devpost pages were not fetched in this pass. The latest available owner
  screenshot shows a duplicate GitHub link and an empty contribution-description
  field; treat those two presentation observations as screenshot-bound and
  potentially stale. The latest recorded Pages check returned HTTP 403, so the
  currently served content and public reachability remain unverified.

## MemoryLineage rubric breakdown

| Official criterion | Score / 5 | Evidence-adjusted reason |
| --- | ---: | --- |
| Innovation | **4.4** | Recovery preflight asks a specific, consequential question: can a restored agent snapshot continue the latest authorized history? The authority-and-recovery decision is a meaningful product wedge, not a claim that blockchain-backed AI memory is new by itself. |
| Technical Feasibility | **4.5** | The local Rust/Solidity/revm implementation, independent bundle replay, tamper checks, and recovery gate make the prototype unusually inspectable. The same evidence does not yet establish production-agent integration, public-chain provenance for every bundle, or external reproduction. |
| Uniqueness | **4.1** | The recovery-authority and canonical-succession boundary differentiates the product. Versioned memory, provenance, replay, and rollback already have adjacent work, and the offline verifier does not authenticate canonical chain state; novelty stays bounded to the recovery-preflight use. |
| Design | **4.3** | The landing-page art direction, pitch deck, and challenge story are coherent. The Inspector screenshot still shows dense technical labels, and novice comprehension is not independently demonstrated. The latest supplied Devpost screenshot's link/form issues may be stale. |
| **Equal-weight mean** | **4.3** | (4.4 + 4.5 + 4.1 + 4.3) / 4 = 4.325, rounded to one decimal. |

## Forkline rubric breakdown after source review

| Official criterion | Score / 5 | Evidence-adjusted reason |
| --- | ---: | --- |
| Innovation | **4.3** | Forkline makes a familiar but consequential failure concrete: an observed chain event can disappear while a ticket-like effect remains. Its contribution is the connected rehearsal and explicit incident path; its own documentation does not claim reorg handling or idempotency as new inventions. |
| Technical Feasibility | **4.6** | The branch includes a local Solidity/Ganache trace, a stateful engine, transactional SQLite outbox, independent loopback HTTP/SQLite receiver, durable receipt reconciliation, and tests/evidence for process restart and lost acknowledgements. The score is capped because the lab is single-worker, SQLite is experimental in its pinned Node runtime, Ganache is archived, and no production chain or fulfillment adapter is present. |
| Uniqueness | **4.2** | The side-effect/outbox/reorg combination is sharply scoped and well demonstrated, but confirmation policy, idempotency, outboxes, and reorg handling are established engineering ideas. The defensible distinction is the integrated, inspectable rehearsal, not a new primitive. |
| Design | **4.4** | The headline and paired comparison explain the failure quickly, the controls expose the recovery sequence, and the saved desktop/mobile/browser evidence supports a coherent developer workflow. The page is still information-dense and has no independent novice-comprehension report. |
| **Equal-weight mean** | **4.4** | (4.3 + 4.6 + 4.2 + 4.4) / 4 = 4.375, rounded to one decimal. |

## Direct GitHub comparison

| Dimension | MemoryLineage `v1.0.2` | Forkline `forkline-outbox-20260908` | What the source review supports |
| --- | --- | --- | --- |
| Problem | Whether an old restored agent snapshot may continue the latest authorized history. | Whether a chain event still supports an irreversible external delivery. | Forkline's failure is easier to understand immediately; MemoryLineage addresses a more specialized recovery-authority decision. |
| Main implementation | Solidity registry; Rust protocol/types and independent verifier; local REVM harness; browser/CLI bundle replay; local reference resume runtime. | Local Solidity/Ganache trace → replay engine → SQLite outbox → loopback HTTP receiver → separate SQLite ticket ledger and receipt reconciliation. | Forkline has the more complete local side-effect workflow. MemoryLineage has deeper signed-history and authority-verification semantics. |
| Verification evidence | Recorded Rust/Solidity/revm checks, independent bundle replay, 20 mutation cases, and a local reference-runtime gate. | Checked-in release record reports 65 Node tests, 9 EVM checks, 8 independent SQLite-oracle checks, 21 local browser checks, plus 13 hosted replay checks. | Both have substantial deterministic evidence. These counts are repository records, not tests rerun in this review. |
| Chain trust | Offline bundle validation reports `ADDRESS_FORMAT_ONLY`; EOA signatures are supported for independent replay, while historical ERC-1271 replay and authenticated canonical-chain provenance remain unsupported. | Engine trusts supplied local block observations; it has no header, inclusion, or receipt-root proofs and no production chain adapter. | Neither repository proves that its sample history is canonical on a public network. The evidence models are different, so the word “verified” must stay scoped in each project. |
| Runtime boundary | Protected-resume callback and example runtime are reference integrations; no external production agent framework is demonstrated. | One local worker and a synthetic receiver; no real tickets/assets, arbitrary external API guarantee, distributed workers, or atomic chain-read/effect boundary. | Forkline is further along on local end-to-end operational integration; neither is production-integrated. |
| Repository/release | Dedicated Rust workspace and matching `main`/`v1.0.2` tag at the verified commit. Current browser/Pages reachability and external reproduction remain unverified. | Submission is a named branch of a larger portfolio repository; the branch has 255 tracked files, much of it generated evidence and retained prior work. Recorded local and hosted-replay artifacts are extensive but were not rerun here. | MemoryLineage is easier to identify as a standalone versioned project. Forkline's source path is valid but less self-contained at repository-navigation level. |
| First impression | Polished brand/landing material; Inspector carries dense protocol and machine-status detail. | Direct headline, visible controls, side-by-side outcomes; long page remains dense for non-developers. | Forkline has a small advantage in communicating its failure path; neither has independent novice testing in the reviewed evidence. |

This comparison is of the exact public source snapshots above. It is not a line-
by-line security audit, and it does not establish that the recorded test runs
can still be reproduced today.

## Results

| Project | Evidence-adjusted total /5 | Strongest public case | Main deduction | Confidence |
| --- | ---: | --- | --- | --- |
| [MemoryLineage](https://devpost.com/software/memory-lineage) | **4.2–4.4** (central estimate: 4.3) | Strong local Rust/Solidity/revm evidence, independent signed-bundle replay, and a focused snapshot-recovery boundary. | Demo Space V2 is synthetic and local; production-agent adoption, external novice testing, authenticated public-chain provenance, and current Pages reachability are not established. | Medium-high for source/local implementation; low-medium for public access |
| [Forkline](https://devpost.com/software/forkline-rehearse-the-rollback) | **4.2–4.6** (central estimate: 4.4) | Source-backed local EVM → durable outbox → separate receiver ledger; recorded failure, crash, and reconciliation coverage; clear incident story. | Single-worker local lab, experimental Node SQLite, archived Ganache, trusted supplied chain observations, synthetic tickets, and no arbitrary external delivery guarantee. Checks were not rerun. | Medium-high for inspected source and recorded evidence; medium for present reproducibility |
| [FinalityDesk](https://devpost.com/software/finalitydesk) | **3.7–4.2** | Narrow, easy-to-explain transfer check with exact recipient/amount/finality rules and a reported mutation suite. | Single-provider observation is not consensus proof; source and demo were not rerun in this assessment. | Medium from the prior source audit |
| [ProofFlow](https://devpost.com/software/proofflow) | **3.5–4.2** | Simple Create → Hash → Anchor → Verify flow; its page describes local hashing, a contract, and tests. | File anchoring is a familiar pattern; the contract/deployment and interactive flow were not independently checked here. | Medium-low |
| [TxGuard Studio](https://devpost.com/software/txguard-studio) | **3.4–4.1** | Strong hands-on premise: paste calldata or Solidity and inspect a translated result. The page reports tests and browser validation. | Three tools and automated security scoring create a broad verification burden; code and live behavior were not independently inspected. | Low-medium |
| [RugGuard AI](https://devpost.com/software/rugguard-ai-the-web3-scam-detector-jbqfwg) | **3.3–4.1** | Familiar, high-impact Web3 problem; the submission describes live trade simulation, bytecode checks, and testnet registry deployments. | Those capabilities and deployments remain submission claims: the linked repository and applications were not inspectable through this pass, and detection accuracy is unmeasured here. | Low |
| [Kinetic](https://devpost.com/software/kinetic-m9i5gv) | **3.1–4.0** | Ambitious marketplace story with Algorand registry, escrow, provider and consumer flows. | Breadth raises the proof burden. The claimed compute proof, decentralization, and current end-to-end behavior were not replayed in this assessment. | Medium-low |
| [Imvecto](https://devpost.com/software/imvecto-trust-infrastructure-for-impact-funding) | **3.2–4.0** | Direct social-impact framing and a claimed Sepolia investment lifecycle with inspectable transaction references. | Depends on an external tokenization service; transactions and user outcomes were not independently checked here. | Low-medium |
| [PRATYAKSH](https://devpost.com/software/pratyaksh-web3-fraud-cash-out-predictor) | **2.8–3.8** | Urgent fraud-prevention problem and a memorable civic-impact story. | Claims span banks, police dispatch, predictive models, and sub-second intervention; the end-to-end chain was not independently evidenced in this pass. | Low |
| [PrithviScan](https://devpost.com/software/prithviscan) | **2.7–3.6** | Clear farming workflow that translates satellite and field data into practical advice. | The listed GitHub repository describes itself as a minimal static prototype and contains only a small site surface; this does not substantiate the much broader Devpost feature claims. No on-chain role is visible in the reviewed materials. | Low-medium |
| [Heka](https://devpost.com/software/heka-edzt7q) | **2.8–3.6** | Natural-language GIS questions mapped to explainable spatial analysis; the page separates AI planning from deterministic GIS computation. | No clear blockchain role is visible in the submission, so Web3 fit is weak on the evidence reviewed. The app and repository were not run. | Low-medium |
| [ArcLight AI](https://devpost.com/software/arclight-ai) | **2.8–3.5** | Strong smart-city dashboard framing and multiple data sources. | The visible stack and story do not establish a meaningful blockchain mechanism; live behavior was not inspected. | Low-medium |
| [DEDSEC Shadow NET](https://devpost.com/software/dedsec-shadow-net) | **2.5–3.2** | Immediate temporary-messaging premise and memorable visual storytelling. | The public stack is WebSocket/web-app focused, with decentralization described as future work; no current on-chain role or linked source was verified. | Low |

## What the scores say

1. **MemoryLineage and Forkline are effectively tied in this heuristic.** The
   corrected central means differ by only 0.05 before display rounding, and
   their plausible ranges overlap. Forkline is stronger on a complete local
   operational path and immediate problem framing. MemoryLineage is stronger
   on the signed snapshot-history model, independent verifier, and standalone
   release structure. Which one a judge prefers depends on how they weigh
   demonstrable end-to-end execution against protocol depth and problem novelty.
2. **RugGuard AI is the most important newly added event comparator, though
   not a functional substitute for MemoryLineage.** The everyday scam-checking
   problem is easier to grasp, and the page claims live chain analysis and
   testnet contracts. Those claims could move its technical score substantially
   after source and deployment review; the current public text alone is
   insufficient to put it above MemoryLineage.
3. **ProofFlow and TxGuard Studio have a simpler first action in their
   submission stories.** ProofFlow starts with a file/text proof and TxGuard
   with calldata or Solidity input, while MemoryLineage needs a short setup
   story about restoring an old snapshot against a newer shared head. Their
   live interfaces were not evaluated here, so this is a comparison of
   explanation burden, not a verified UX ranking.
4. **Kinetic, Imvecto, and PRATYAKSH tell broader Web3 or impact stories.** Their
   scope also creates a larger burden to show that each claimed component works
   together.
5. **Heka, ArcLight, DEDSEC, and PrithviScan may be useful products, but the
   reviewed public materials do not clearly show a current blockchain role.**
   That is a fit concern for this event, not a claim that the products have no
   value outside it.

The simplest defensible result is: **MemoryLineage is about 4.3/5 and Forkline
about 4.4/5 on this equal-weight heuristic; the 0.1 displayed gap is not
meaningful evidence that Forkline is categorically better.** The prior Forkline
repo link was wrong, so its earlier score was not a valid source-based
comparison. After inspecting the right branch, Forkline deserves a higher
technical-feasibility estimate than the old note conveyed. MemoryLineage still
has a strong technical case; its most material relative gaps are authenticated
public-chain provenance, external runtime adoption, public reachability, and
independent first-time-user evidence. These scores are not judge results and do
not predict placement.
