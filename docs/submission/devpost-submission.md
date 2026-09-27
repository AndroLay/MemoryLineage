# Devpost submission copy — MemoryLineage v1.0.2

This document is the copy-ready project entry for Devpost. The story below is
written in English to match the submission form. The project owner supplied a
screenshot showing the public Devpost page and its embedded video player, and
provided the YouTube link below. The current production Inspector now returns
HTTP 200 and passes the production browser smoke after deployment from `main`
at `6f03984`. The video player was visible in the supplied screenshot, but
playback has not been independently verified. The stable Devpost source
baseline remains `v1.0.2`.

## Project overview

**Project name**

Memory Lineage

**Elevator pitch**

> Before an AI agent resumes, MemoryLineage checks whether a restored backup follows its team's latest approved history—and holds stale restores for review.

Character count: 154 of 200.

## Project story

### Inspiration

**Problem.** After an outage, an AI agent can restart from a backup that opens
normally but is behind the history its team treats as current. The operator can
see that the files loaded; a reviewer still needs to know whether that snapshot
is an authorized point to continue from. We started with this recovery
question, not with the idea of putting private memory on a blockchain.

### What it does

**Solution.** MemoryLineage checks a restored snapshot before an agent
resumes. It compares the snapshot's fingerprint and previous step with the
recorded history and approval evidence. It says whether the snapshot is
current, an older known checkpoint, different from the recorded history, or
unsupported by enough evidence, and explains why before anyone continues.

The demo follows **The Silent Rollback**: Backup 1 is restored after the
synthetic history has reached state 3. A new step tries to continue from the
old state, and local execution of the Solidity registry rejects it with
`BAD_PREVIOUS_STATE`. The Inspector asks the visitor to predict the outcome,
then reveals the evidence and reason. A separate Rust verifier can replay the
bundle without relying on the website.

This is a local synthetic prototype, not a connection to a production agent or
a live-chain run of this history. Offline replay checks that the bundle is
internally consistent; it does not prove which live chain produced it.

**Innovation.** Our focus is snapshot-level recovery: whether a backup is an
authorized continuation before an agent resumes. MemoryLineage does not trace
the origin or derivation of individual entries, detect memory poisoning, or
claim general novelty for memory lineage. Its contribution is bringing a
recovery-time decision, authority context, and portable evidence replay
together in one prototype workflow.

### How we built it

We built the protocol logic, CLI, and independent evidence verifier in Rust.
The browser Inspector uses Dioxus and WebAssembly. A Solidity registry defines
the sequence, predecessor, and authorization rules. In the demo,
Rust/`revm` runs the checked-in Solidity bytecode against a deterministic
fixture derived from synthetic SQLite snapshots. Raw memory stays off-chain;
the portable evidence contains commitments and proof metadata. We build on
existing standards and do not claim to have invented ERC-8350, EIP-712, or
ERC-1271.

### Challenges we ran into

We had to make the contract, Rust tools, browser, and CLI agree on the same
sequence and approval rules. Another challenge was explaining the boundary of
a successful replay: a bundle can be consistent without proving which chain
produced it, and a commitment cannot tell us whether the memory is true or
safe. The Inspector spells this out instead of hiding it behind one green
check.

### Accomplishments that we're proud of

**Impact (intended, not measured).** The person restoring a backup, the person
approving it, and an auditor can inspect the same recovery decision before a
snapshot reaches the reference loader. In the prototype, the stale snapshot is
held before that loader. We have not measured incident reduction, user impact,
or production outcomes.

- Built a complete local recovery-preflight walkthrough around a concrete
  stale-snapshot incident.
- Replayed the Silent Rollback against the checked-in Solidity behavior through
  Rust/`revm`, with the stale predecessor rejected as `BAD_PREVIOUS_STATE`.
- Added an independent Rust verifier and CLI for portable evidence, so review
  does not depend only on the website.
- Kept raw fixture values out of the registry and portable evidence while
  making commitments, proof metadata, and decision receipts inspectable.
- Published the `v1.0.2` source snapshot and a ten-page pitch PDF under the
  repository's MIT license.

### What we learned

We learned to say exactly what a successful replay means. It shows that the
supplied evidence fits together; it does not establish that the evidence came
from the canonical contract. We also saw that privacy and recovery have to be
planned together: raw memory can stay off-chain, but reviewers still need
retained bundles or event logs to reconstruct the history.

### What's next for MemoryLineage

**Future scope.** We want to authenticate the registry and chain behind the
evidence, rebuild history from retained events or bundles, and integrate the
gate with one real agent runtime. We also need uncoached developers to try the
flow and measure whether they understand and complete it. Historical ERC-1271
replay, privacy and secret recovery for real memory, formal security review,
and measured user impact remain future work.

## Project details

### Built with

The owner screenshot shows these tags already selected. Keep them aligned with
the technologies used in the repository:

`cli`, `cryptography`, `dioxus`, `eip-712`, `ethereum`, `revm`, `rust`,
`solidity`, `sqlite`, `webassembly`.

### Try it out

Add these links:

1. **Live Inspector** — <https://memorylineage.pages.dev> *(Current Pages production is from `main` at `6f03984`; HTTP 200 and production browser smoke passed. The stable GitHub release and Devpost baseline remain `v1.0.2`.)*
2. **Source code** — <https://github.com/AndroLay/MemoryLineage>
3. **v1.0.2 release and pitch PDF** — <https://github.com/AndroLay/MemoryLineage/releases/tag/v1.0.2>

The screenshot currently shows the GitHub repository twice. Replace the third
duplicate link with the v1.0.2 Release link above.

The `127.0.0.1:8080` local address is for your own machine and must not be used
as a public Devpost try-it link.

### Project media

**Thumbnail**

Upload [`assets/devpost-thumbnail.png`](assets/devpost-thumbnail.png). It is a
1500 × 1000 PNG (3:2) with the project logo and website preview.

**Image gallery**

Recommended order:

1. [`assets/lab-local.png`](assets/lab-local.png) — the recovery scenario and
   decision flow.
2. [`assets/devpost-website-preview.png`](assets/devpost-website-preview.png)
   — the landing-page design and product context.
3. [`assets/verify-local.png`](assets/verify-local.png) — evidence verification
   and the result details.

The screenshots are local prototype captures; do not describe them as a live
public deployment. The four selected image files are each below Devpost's
5 MB limit. Check that text remains legible in Devpost's upload preview.

### Video demo link

**Two-minute demo video:** <https://youtu.be/K2QUHm4lJCo>

The project owner supplied this link, and the provided screenshot shows an
embedded player on the Devpost page. Open the link in a signed-out/private
browser window to confirm playback before treating video access as verified.

### Contribution field

The screenshot shows the contribution field empty. Suggested text, if it
accurately reflects the entrant's work:

> I designed and built the MemoryLineage prototype, including its recovery
> workflow, Rust and Solidity implementation, Inspector, evidence verifier,
> local demo, documentation, and presentation. The demo uses synthetic data
> and is not connected to a production AI agent.

Adjust the wording to credit any teammates and match the entrant's actual
contribution before saving it on Devpost.

## Copyright and license note

Suggested optional footer for the public story or project page:

> © 2026 MemoryLineage contributors. The project source is open source under the MIT License; see the repository for third-party notices.

Do not add “All rights reserved” as a blanket statement for the source code:
that would conflict with the repository's MIT license. Third-party materials
remain subject to their respective notices and licenses.
