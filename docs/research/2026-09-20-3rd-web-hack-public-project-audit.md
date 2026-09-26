# 3rd-Web-Hack Public Project Audit

**Audit date:** 20 September 2026
**Purpose:** maintain a source-backed record of publicly discoverable projects
that may compete with MemoryLineage.

**23 September status correction:** the V2 demo fixture now uses a
length-prefixed snapshot commitment and a regression test for the delimiter
ambiguity. The older serializer concern in the historical comparison below
applies to the retained V1 compatibility profile. V2 remains unsalted synthetic
evidence; see the [privacy profile note](../product/commitment-privacy-profile.md).

**25 September research addendum:** ExitDrill is recorded below as an adjacent
workflow reference with **3rd-Web-Hack membership unverified**. Its supplied
Devpost page could not be retrieved, and the official project gallery remains
unpublished. Do not count it as a same-event competitor until its submission
status is confirmed.

**27 September public-index refresh:** Devpost search results now expose two
additional pages that explicitly list 3rd-Web-Hack under “Submitted to”:
RugGuard AI and Heka. The same check reconfirmed that ArcLight AI, DEDSEC
Shadow NET, and PrithviScan list the event; their existing entries below remain
the source notes. The refresh did not independently run these applications or
audit their repositories. The official gallery page still says the organizers
have not published it, so the search results do not form a complete submission
census.

**27 September Forkline source correction:** an earlier version of this audit
linked `sauravvenkat/forkline`, which is a different agent-tracing project. The
Forkline submission source is the `forkline-outbox-20260908` branch in
`josepha-mayo/Joseph-Portfolio`. The branch ref was confirmed at commit
`a58fe2c44cc3c8d19ac3300b8b29f91dcb4fd8af`; this correction replaces the prior
description and score evidence for Forkline below. The source was inspected
read-only and its recorded checks were reviewed, but its commands were not
rerun in this assessment.

## Coverage boundary

As checked on 27 September, the official project-gallery page says the
organizers have not published the gallery. Individual Devpost project pages
and search results can confirm some event listings, but they do not establish
a complete census of submissions. Treat this document as a partial
public-index audit, not a complete ranking of entrants.

Official references:

- [3rd-Web-Hack overview](https://3rd-web-hack.devpost.com/)
- [3rd-Web-Hack project gallery](https://3rd-web-hack.devpost.com/project-gallery)

The entries below are separated into:

1. public projects whose Devpost pages currently state that they were
   submitted to 3rd-Web-Hack;
2. the two projects previously supplied and audited by the team;
3. adjacent projects from other hackathons that are useful benchmarks but are
   not 3rd-Web-Hack competitors.

Projects whose event membership cannot be verified are listed separately and
are not included in the competitor count.

The evidence-adjusted comparison and criterion-by-criterion MemoryLineage
rating are in the [27 September scorecard](./2026-09-27-3rd-web-hack-evidence-adjusted-scorecard.md).

## Public 3rd-Web-Hack candidates

### Forkline — Rehearse the Rollback

- Devpost: <https://devpost.com/software/forkline-rehearse-the-rollback>
- Submission source branch: <https://github.com/josepha-mayo/Joseph-Portfolio/tree/forkline-outbox-20260908>
- Audited branch commit: `a58fe2c44cc3c8d19ac3300b8b29f91dcb4fd8af`
- Category: direct benchmark / high priority
- Source-backed strengths:
  - a Solidity `DemoOrders` fixture is compiled and exercised in local Ganache;
  - the engine checks observed block ancestry, event identity, confirmation
    policy, duplicates, and reorg effects;
  - the Delivery Lab connects that engine to a durable SQLite outbox and a
    separate loopback HTTP receiver with its own SQLite ticket ledger;
  - dispatch rechecks eligibility, persists send intent, binds receipts to
    payload hashes, pauses on uncertain acknowledgements, and reconciles by
    receipt lookup instead of blindly reposting;
  - tests and recorded evidence cover stale queued work, duplicate/idempotent
    requests, reorgs before and after dispatch, lost acknowledgements, process
    restart, and database integrity;
  - screenshots and a recorded browser run show the side-by-side failure case
    and the guarded result.
- Risks and limits:
  - this is a single-worker local prototype; Node 22's built-in SQLite is
    documented as experimental, and the pinned Ganache version is archived;
  - its EVM observations are supplied by a local fixture, not authenticated
    headers, inclusion proofs, or a production RPC/consensus client;
  - a chain reorg can still occur after the last observation or after an
    external action; the prototype cannot make a chain read and an external
    effect atomic;
  - receiver idempotency is specific to the local receiver and does not prove
    exactly-once behavior for arbitrary external APIs;
  - tickets and orders are synthetic local rows, not real admissions or assets;
  - the branch is part of a larger portfolio repository and contains substantial
    generated evidence and retained prior work; reviewers need the branch link
    and its README to find the submitted slice;
  - the recorded release/browser evidence was not rerun in this comparison.
- Audit conclusion: a stronger source-backed implementation than the prior
  note recorded. Forkline currently shows a more complete local path from a
  chain event observation through an irreversible-effect boundary than
  MemoryLineage's reference resume adapter. MemoryLineage has deeper signed
  snapshot and independent-verifier semantics. Neither project proves public
  chain canonicality or production adoption.
- Confidence: high that the inspected branch source and evidence exist at the
  recorded commit; medium that the archived runs reproduce today, because they
  were not rerun; low for comparative user adoption, which neither artifact set
  establishes.

### FinalityDesk

- Devpost: <https://devpost.com/software/finalitydesk>
- GitHub: not resolved in the current public-index pass; retain the repository
  URL from the earlier local audit when available.
- Category: direct benchmark / evidence-quality competitor
- Publicly audited strengths:
  - verifies exact ERC-20 transfer semantics;
  - checks recipient, token, amount, canonical block, and finalized head;
  - narrow question with a low explanation burden;
  - mutation coverage and invalid RPC-envelope handling;
  - careful treatment of integer precision and write-RPC restrictions.
- Risks and limits:
  - single-provider observation is not a light-client or consensus proof;
  - the underlying payment-verification pattern is less novel than the
    engineering discipline;
  - public repository details need a fresh direct check before release.
- Audit conclusion: a serious small-scope competitor. MemoryLineage should
  match its setup clarity and exact failure reporting.
- Confidence: high from the earlier repository audit; current public-page
  refresh was limited.

### Kinetic

- Devpost: <https://devpost.com/software/kinetic-m9i5gv>
- Repository: <https://github.com/Shivanikinagi/KINETIC>
- Live interface: <https://kinetic-pink.vercel.app/>
- Category: strongest newly surfaced Web3 competitor
- Publicly visible strengths:
  - Algorand TestNet provider registry, escrow, and badge contracts;
  - documented TestNet application IDs;
  - public repository with `contracts`, `api`, `provider_node`, `web`, and
    `tests` directories;
  - marketplace, wallet, provider discovery, autonomous agent, and payment
    narrative;
  - the repository describes real SHA-256 workloads through subprocess/Docker
    and on-chain proof hashes.
- Risks and limits:
  - the repository README says it was built for AlgoBharat Hack Series 3.0,
    while the Devpost page lists several hackathons including 3rd-Web-Hack;
    this requires a careful originality/provenance explanation;
  - a chained hash of reported execution steps does not, by itself, prove that
    the claimed GPU computation occurred. This is an architectural inference,
    not a claim that the implementation is fabricated;
  - the FastAPI backend remains a significant orchestration component despite
    the fully decentralized wording;
  - the surface area is broad, so the gap between described and independently
    demonstrated behavior is larger than in FinalityDesk or MemoryLineage.
- Audit conclusion: Kinetic is the closest new threat to Forkline on Web3
  ambition and product breadth. It is not clearly stronger overall because its
  proof-of-compute, decentralization, and originality claims need tighter
  evidence.
- Confidence: high for repository structure and published deployment records;
  medium for execution and decentralization claims.

### RugGuard AI — The Web3 Scam Detector

- Devpost: <https://devpost.com/software/rugguard-ai-the-web3-scam-detector-jbqfwg>
- Public interfaces listed on Devpost: <https://rugguard-ai.ai.studio> and
  <https://rugguard-ai-88if.onrender.com>
- Event membership: Devpost lists 3rd-Web-Hack under “Submitted to.”
- Publicly described strengths:
  - addresses a familiar user problem: checking token and project scam risk;
  - combines AI summaries and market/on-chain data, including trade simulation
    and bytecode checks as described by the submission;
  - describes a community scam registry whose contract was deployed to
    Ethereum Sepolia and Base Sepolia.
- Risks and limits:
  - the repository link could not be resolved from the indexed page in this
    pass, and neither application nor contract deployment was independently
    tested;
  - risk-classification accuracy, false-positive/negative behavior, and the
    claimed testnet deployments therefore remain project-published claims;
  - the problem is directly relevant to Web3 security but is not a close
    functional substitute for snapshot recovery or agent-memory continuity.
- Audit conclusion: a significant additional competitor for immediate
  problem clarity and visible Web3 activity. Do not score its implementation
  above MemoryLineage without inspecting the source, live behavior, and
  deployment evidence.
- Confidence: high for the public description and event listing; low for
  implementation and deployment claims not independently checked.

### Heka

- Devpost: <https://devpost.com/software/heka-edzt7q>
- Live interface listed on Devpost:
  <https://heka-web.ulofeuduokhai.workers.dev>
- Event membership: Devpost lists 3rd-Web-Hack under “Submitted to,” alongside
  several other events.
- Publicly described strengths:
  - turns a spatial question into an explainable map and evidence;
  - lists GIS-oriented components including Cesium, GeoJSON, OpenStreetMap,
    and Cloudflare Workers;
  - offers a concrete product workflow that is easier to picture than
    MemoryLineage’s recovery trust problem.
- Risks and limits:
  - the indexed submission material does not show a clear blockchain or
    on-chain component;
  - the app and repository were not independently inspected in this refresh.
- Audit conclusion: useful benchmark for making a technical product’s first
  user task concrete, but not currently a direct Web3 or memory-lineage
  competitor based on the public material reviewed.
- Confidence: high for the indexed description and event listing; low for
  implementation details not inspected.

### ArcLight AI

- Devpost: <https://devpost.com/software/arclight-ai>
- Repository listed by the submission: <https://github.com/Rishabhv16/arclight-ai>
- Live interface: <https://arclight-ai.vercel.app/>
- Category: visual/product benchmark, weak Web3 competitor
- Publicly visible strengths:
  - polished smart-city dashboard;
  - multiple modules, charts, AI assistant, and real-data APIs;
  - strong first impression and visual packaging.
- Risks and limits:
  - the public stack lists Next.js, TypeScript, AI APIs, Open-Meteo, WAQI,
    and simulators;
  - no visible blockchain, wallet, smart contract, on-chain state, or
    attestation is described;
  - the project therefore has a major Web3-fit problem for this event.
- Audit conclusion: useful visual benchmark, not a stronger 3rd-Web-Hack
  submission than MemoryLineage.
- Confidence: high for the published stack; repository implementation should
  still be checked before making claims about unlisted modules.

### DEDSEC Shadow NET

- Devpost: <https://devpost.com/software/dedsec-shadow-net>
- Live interface: <https://dedsec-shadow-net-14425316810.asia-southeast1.run.app/>
- Category: storytelling/UI benchmark, weak Web3 competitor
- Publicly visible strengths:
  - immediately understandable ephemeral-room story;
  - real-time WebSocket interaction;
  - memorable cyberpunk presentation;
  - explicit RAM-only and expiration behavior.
- Risks and limits:
  - current stack is React, Node, Express, Vite, and WebSockets;
  - decentralized communication is listed as future work;
  - JavaScript reference deletion is not equivalent to a guaranteed secure
    physical-memory wipe.
- Audit conclusion: valuable lesson for opening narrative and demo pacing, but
  not a Web3 implementation that threatens MemoryLineage.
- Confidence: high for the public Devpost description and stack.

### PrithviScan

- Devpost: <https://devpost.com/software/prithviscan>
- Repository: <https://github.com/kushal-narkhede/PrithviScan>
- Live interface: <https://prithviscan.web.app/>
- Category: newly surfaced public entry; weak Web3 competitor
- Publicly visible strengths:
  - live product framing around satellite data and farming decisions;
  - Firebase authentication and Firestore data flows;
  - NASA/weather data, machine learning, alerts, field insights, and AI
    assistance;
  - broad practical impact story.
- Risks and limits:
  - the listed stack contains Firebase, Firestore, Gemini, machine learning,
    GeoJSON, OpenCV, and REST APIs but no visible blockchain layer;
  - the submission is listed across many hackathons, so event-specific
    originality needs explanation;
  - it is a strong application concept but does not visibly solve a Web3
    problem.
- Audit conclusion: new public candidate found in this pass, but not a direct
  threat to Forkline or MemoryLineage under the stated Web3 requirement.
- Confidence: high for the Devpost description; repository execution was not
  rerun in this pass.

## Event membership unresolved; not scored as a 3rd-Web-Hack entry

### ExitDrill — structural recovery drill for SaaS exports

- Devpost page supplied for this audit: <https://devpost.com/software/exitdrill>
- Public repository: <https://github.com/ChelseaKR/exitdrill>
- Event status: **unverified**. The Devpost page was inaccessible to this
  review, and the official gallery is unpublished. No accessible source in this
  pass confirms that ExitDrill was submitted to 3rd-Web-Hack; do not treat it as
  an event entrant or use it in event scoring.
- The repository describes a technical alpha that compares a pre-export
  baseline with a SaaS export across entity identity, relationships, attachment
  bytes, permissions, and audit history.
- Its README describes a three-minute offline CLI demo using clean and
  adversarial synthetic CRM exports. The adversarial case keeps row count the
  same while showing separate structural-loss signals, then gives a short
  human-readable summary and replayable aggregate receipts.
- Product relevance to MemoryLineage: a useful adjacent example of a bounded
  drill, concrete comparison, explicit per-dimension outcome, and concise
  explanation. Its documented three-minute demo is a CLI walkthrough; the
  sources reviewed do not establish a game-like onboarding flow or novice
  comprehension.
- Claim boundary: the repository labels the project a technical alpha with
  synthetic fixtures; it explicitly does not prove a completed migration or
  operational exit.
- Confidence: high for the repository README as reviewed on 25 September 2026;
  low for any claim about Devpost submission or 3rd-Web-Hack membership.

## Adjacent projects and research, not 3rd-Web-Hack entries

These projects appeared during the search but their Devpost pages identify
other hackathons. They must not be counted as 3rd-Web-Hack competitors.

### MemLineage — direct conceptual neighbor

- Paper: <https://arxiv.org/abs/2605.14421>
- Repository: <https://github.com/amurlaniakea/memlineage>
- Submitted to 3rd-Web-Hack: no evidence found.
- Publicly visible strengths:
  - signed memory entries;
  - Merkle-log provenance;
  - derivation DAG and untrusted-ancestor propagation;
  - sensitive-action gate;
  - deterministic memory-poisoning benchmarks.
- Difference from MemoryLineage:
  - MemLineage focuses on semantic/provenance enforcement and stopping
    sensitive actions derived from untrusted memory;
  - MemoryLineage focuses on canonical succession, predecessor continuity,
    authorization history, and public Web3 auditability;
  - MemLineage does not make MemoryLineage unnecessary, but its existence
    makes the product boundary and name distinction essential.

### Rehearsal

- Devpost: <https://devpost.com/software/rehearsal-h6fxmu>
- Submitted to: OpenAI Build Week.
- Useful patterns:
  - measured future-state preview;
  - bounded outcome contract;
  - exact preview-to-apply binding;
  - offline execution receipt;
  - verified rollback and honest scope disclosure.

### ForkTrace

- Devpost: <https://devpost.com/software/forktrace>
- Live interface: <https://forktrace.vercel.app/>
- Submitted to: OpenAI Build Week.
- Useful patterns:
  - immutable trace fork;
  - first divergence boundary;
  - replay only after the fork point;
  - explicit `DIVERGED` status;
  - honest distinction between hosted walkthrough and local live replay.

### Branchline

- Devpost: <https://devpost.com/software/branchline>
- Submitted to: OpenAI Build Week.
- Useful patterns:
  - evidence-linked release rehearsal;
  - deterministic scenario rules;
  - specialist reports bound to an evidence hash;
  - human decision ledger;
  - publish-or-block outcome.

## Comparative conclusion

Within this partial public set, Forkline remains the clearest combined
benchmark for problem explanation, reproducibility, and demo packaging. This
is not a claim that it outranks every entrant. RugGuard AI is a notable
challenger on familiar problem framing and claimed live testnet activity, but
its implementation and deployment evidence need independent review before a
technical comparison is justified.

- Forkline remains the clarity/reproducibility benchmark.
- Kinetic is the strongest additional Web3 threat by ambition and breadth.
- FinalityDesk is the strongest narrow verifier benchmark.
- RugGuard AI is an additional direct Web3-security competitor with a more
  immediately familiar user problem; its deployment and detection claims need
  source-level verification before technical comparison.
- Heka, ArcLight, DEDSEC, and PrithviScan are useful user-workflow or
  presentation benchmarks, but their reviewed public material does not show a
  clear current on-chain role.
- MemLineage is the most important adjacent research risk for uniqueness and
  naming, but it is not a 3rd-Web-Hack submission.

## MemoryLineage comparison record

Current MemoryLineage evidence is recorded in:

- [README product flows](../../README.md)
- [product contract](../product/product-contract.md)
- [strict dominance audit](./2026-09-20-memorylineage-strict-dominance-plan-audit.md)
- [potential expansion map](./2026-09-20-memorylineage-potential-expansion.md)

Current limitations that affect competitive confidence:

- Demo Space V2 is local evidence and is separate from the existing Sepolia
  observation;
- external human reproduction is not yet demonstrated;
- the reference agent runtime is fixture-scoped, not production adoption;
- V2 encoding is unambiguous, while a production privacy profile still needs
  private blinding-secret management and an independently reviewed migration;
- the official gallery remained unpublished on the 27 September check, so
  additional public or unindexed competitors may remain.

This file is an audit record, not a claim that MemoryLineage will win or that
any listed project violates hackathon rules.
