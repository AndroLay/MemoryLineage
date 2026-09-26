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

## Coverage boundary

The official project gallery is currently unpublished. Devpost shows the
participant count, but states that the hackathon managers have not published
the gallery. Therefore this document is a public-index audit, not a complete
census of all submissions.

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

## Public 3rd-Web-Hack candidates

### Forkline — Rehearse the Rollback

- Devpost: <https://devpost.com/software/forkline-rehearse-the-rollback>
- Public repository audited: <https://github.com/sauravvenkat/forkline>
- Category: direct benchmark / high priority
- Publicly visible strengths:
  - replay-first local artifacts;
  - deterministic offline replay;
  - first-divergence detection;
  - CLI and CI integration;
  - schema versioning and redaction policy;
  - clear local-first developer workflow.
- Risks and limits:
  - the public GitHub README currently emphasizes agent tracing and replay and
    does not visibly expose an Ethereum or Algorand layer;
  - the Devpost page was not retrievable by the current crawler, so the exact
    relationship between the Devpost rollback story and the current GitHub
    tree must be checked again before making a definitive score;
  - replay evidence does not by itself prove blockchain canonicality.
- Audit conclusion: strongest public benchmark for problem clarity,
  reproducibility, and a compact developer workflow. It is not clearly ahead
  of MemoryLineage on Web3-specific trust anchoring unless its Devpost
  implementation contains additional evidence not visible in the repository.
- Confidence: medium for the combined Devpost/repository assessment; high for
  the public GitHub README and file structure.

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

No publicly discoverable 3rd-Web-Hack entry audited in this pass is clearly
stronger than Forkline across problem clarity, evidence discipline, Web3 fit,
and presentation at the same time.

- Forkline remains the clarity/reproducibility benchmark.
- Kinetic is the strongest additional Web3 threat by ambition and breadth.
- FinalityDesk is the strongest narrow verifier benchmark.
- ArcLight, DEDSEC, and PrithviScan are useful presentation or product
  benchmarks but have no visible current Web3 layer.
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
- the official gallery is unpublished, so hidden competitors remain possible.

This file is an audit record, not a claim that MemoryLineage will win or that
any listed project violates hackathon rules.
