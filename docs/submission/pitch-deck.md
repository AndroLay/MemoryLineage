# MemoryLineage — judge presentation

This ten-slide deck marks every part of the requested pitch brief: **The
Problem, The Solution, The Innovation, The Impact, and Future Scope**. It also
states the current prototype limits before presenting future work. The
submission version is `v1.0.2`. The
[printable PDF](MemoryLineage-3rd-Web-Hack.pdf) is generated from the editable
[HTML slides](pitch-deck.html). Claims follow the repository
[claim matrix](claim-matrix.md).

## 1 — MemoryLineage

**Section:** Opening

**On-slide:** Can this backup continue the agent's history? Verify the history,
not the memory.

**Talk track:** Persistent agents can return from a backup after an outage. The
hard question is whether the restored state is still allowed to continue the
history recorded before the outage. MemoryLineage makes that recovery check
visible and reviewable.

## 2 — The Problem

**Section marker:** `01 · THE PROBLEM`

**On-slide:** Snapshot 1 → Snapshot 2 → Snapshot 3 (head); restore Snapshot 1.

**Talk track:** A database restore can succeed while the restored history is
out of date. If the same operator controls the runtime, local storage, and the
log used to review that restore, another controller needs a shared checkpoint
to compare against. The problem is deciding what may continue after recovery.

## 3 — The Solution: concept

**Section marker:** `02 · THE SOLUTION · CONCEPT`

**On-slide:** Private snapshots. Shared order. Reviewable decisions. Can this
backup continue from the latest approved state?

**Talk track:** Raw snapshots stay in the agent’s own storage. A fixed-size
snapshot fingerprint gives the shared record something small to compare, while
ordered records show where a snapshot sits in the history and who approved
each step. A reviewer can check the sequence without reading the memory.
MemoryLineage does not judge whether that memory is true, safe, or useful, and
privacy safeguards for real memory still need validation.

## 4 — The Solution: architecture

**Section marker:** `02 · THE SOLUTION · ARCHITECTURE`

**On-slide:**

```text
WRITE PATH
Private agent store → Rust evidence builder → ┬→ Solidity registry
                                                └→ portable evidence bundle

RESTORE PATH
Restored backup + evidence + latest recorded state
→ Rust replay → Inspector explanation → local reference gate → agent loader
```

**Talk track:** The architecture has two paths. When recording a state, the
agent keeps the raw snapshot locally. Rust creates a fixed-size fingerprint
and transition records; the registry stores the shared order and approval
metadata, while a portable bundle carries evidence that can be replayed later.
When storage is restored, Rust checks that evidence, the Inspector explains
whether the backup is current, older, different, or unverified, and a local
reference gate makes the pre-load decision. The checked-in Solidity bytecode
runs locally through Rust/revm with synthetic Demo Space V2 data; there is no
public deployment of this demo space.

## 5 — The Solution: recovery flow

**Section marker:** `02 · THE SOLUTION · RECOVERY FLOW`

**On-slide:** Choose a backup → check its evidence → compare with the latest
recorded state and approval → explain the result → decide before loading.

**Talk track:** First select the backup that returned. The verifier checks that
the supplied records fit together; the preflight then compares the backup with
the named latest state and its approval history. A latest, approved state can
be eligible in the local demo gate. A known older backup is for rehearsal, and
a different or unverified state is held for review. The protected loader is a
framework-neutral reference adapter, not a production agent integration.

## 6 — The Solution: Silent Rollback example

**Section marker:** `02 · THE SOLUTION · SILENT ROLLBACK EXAMPLE`

**On-slide:** The recorded latest state is 3; the restored backup is state 1.
A new step from that old state returns `BAD_PREVIOUS_STATE`.

**Talk track:** The local registry execution rejects a new step that starts at
state 1 when the recorded latest state is 3. The recovery gate separately
recognizes backup 1 as a known older snapshot, marks it `REHEARSE_ONLY`, and
records that the local reference loader was not called. “Check passed” means
the check caught the mismatch and held the backup; it does not mean the backup
was approved.

## 7 — The Innovation

**Section marker:** `03 · THE INNOVATION`

**On-slide:** Restore-only reopens a file. A local signed log records a signer.
MemoryLineage adds shared ordering, portable replay, and a reviewable decision
before the local loader.

**Talk track:** The distinction is the point where the checks meet. Restoring a
file can reopen bytes without showing that they are current. A signed local
log can show who signed a record, but if the same operator controls the log
and runtime, it may not give another party an independent latest checkpoint.
MemoryLineage combines an ordered shared record, a portable evidence bundle,
and a policy decision before the local loader. This is a workflow contribution
built around existing standards, not a new cryptographic primitive or a claim
to authorship of ERC-8350, EIP-712, or ERC-1271. The local tamper example
returns `TRANSITION_ID_MISMATCH` for changed evidence and
`BUNDLE REPLAY VERIFIED` for the restored bundle. Replay establishes bundle
consistency; it does not authenticate the registry’s public-chain origin.

## 8 — The Impact

**Section marker:** `04 · THE IMPACT`

**On-slide:** Operators compare what came back. Controllers apply one visible
resume rule. Auditors replay the same evidence independently.

**Talk track:** This is most useful when the person restoring storage is not
the only person deciding whether the agent may resume. The operator can see
whether the backup matches the named latest state before handing control back.
The controller gets a visible rule: latest and approved may pass the local
demo gate, a known older backup is practice-only, and different or unverified
states are held. The auditor can replay the supplied bundle and inspect a
specific mismatch instead of trusting a dashboard label. The prototype shows
this behavior in a synthetic local case, including that the reference loader
was not called. Production outcomes and user impact have not been measured.

## 9 — Current Limitations

**Section marker:** `CURRENT LIMITATIONS`

**On-slide:** Offline replay checks bundle consistency, not which contract or
network produced the bundle. Full history needs retained logs or bundles.
Real-memory privacy, production integration, historical smart-wallet checks,
multi-source network confirmation, and semantic memory safety are not
demonstrated.

**Talk track:** The independent verifier reports `ADDRESS_FORMAT_ONLY` for
registry identity. It checks address syntax, not deployed bytecode or
authenticated chain state. The registry has events and known transition
lookups but no complete-history indexer, so reconstruction depends on retained
logs or bundles. Historical smart-wallet approval replay and a second network
source are not shown. Demo Space V2 is local and synthetic; extra privacy
protection for real memories is not integrated or audited. The runtime gate is
a reference adapter, and no external production integration or formal security
audit is claimed. MemoryLineage checks lineage—not truth, safety, reasoning
quality, or whether memory caused an action.

## 10 — Future Scope

**Section marker:** `05 · FUTURE SCOPE`

**On-slide:** Verify the source; rebuild and match the history; gate one real
agent loader; validate with outside reviewers and a real-memory privacy plan.

**Talk track:** Future work has four checkable exits. First, verify deployed
contract code and registry state against a block checked through a trusted
source. Second, rebuild from saved logs and bundles, replay the records to the
checked registry head, and stop if anything is missing. Third, integrate one
real agent framework and show that a current, approved state can reach its
loader while an old, diverged, or unknown backup cannot. Before using real
memory, outside reviewers should complete the recovery task unaided, and the
team must test privacy-key backup and recovery without exposing memory data.

**Reproduction:** `cargo xtask reproduce`

**License line:** © 2026 MemoryLineage contributors. Repository code is
open-source under the MIT License; third-party terms are listed in the
repository notices. The deck avoids “all rights reserved” so its wording does
not contradict the source license.
