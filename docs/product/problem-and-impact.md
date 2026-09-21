# Problem, Importance, and Impact

This is the canonical product note for explaining why MemoryLineage exists.
It separates the real trust problem from adjacent concerns such as semantic
memory safety, prompt injection, and agent behavior. The wording here should
be reused by the README, Inspector, Devpost description, pitch, and any future
runtime integration.

## The problem in one sentence

When a persistent AI agent restores a private memory snapshot, an independent
party cannot reliably tell whether that snapshot is the current authorized
continuation of the agent's recorded history, a legitimate older checkpoint, a
divergent state, or evidence that cannot be verified.

MemoryLineage gives that question a shared, replayable answer by committing
the succession of private states without publishing the states themselves.

## Why this problem exists

Persistent agents do not begin every session from zero. They may retain user
preferences, task progress, operating instructions, policy decisions, tool
results, and other context in a database, file store, or checkpoint system.
That storage is normally off-chain and controlled by the operator who runs the
agent.

This is useful for normal operation, but it creates a trust gap when another
party must review recovery or continuation. The local database can be restored,
copied, replaced, truncated, or presented from a different point in time. A
normal database record can describe what the operator currently has; it does
not automatically give an independent reviewer a shared answer to these
questions:

- Which state was the last authorized state?
- Did the candidate continue from that state or from an older checkpoint?
- Was a transition skipped or replayed?
- Who was authorized when that transition was accepted?
- Is this evidence complete enough to support a recovery decision?

The central issue is therefore not that an agent has memory. The issue is that
the agent's private memory history can be mutable while the runtime operator,
the authority, and the auditor belong to different trust domains.

## The concrete incident: The Silent Rollback

Consider an agent with a private SQLite memory store:

```text
Snapshot 1 -> Snapshot 2 -> Snapshot 3
                                  ^
                          recorded canonical head
```

The operator experiences a crash, migration, or incident and restores
Snapshot 1. That restore can be legitimate. The problem is that the operator
may now try to continue the agent as though Snapshot 1 were current:

```text
local restore to Snapshot 1
        |
        +--> attempt Snapshot 4 using Snapshot 1's root

shared recorded head: Snapshot 3
result: REJECTED / BAD_PREVIOUS_STATE
```

The rejection does not mean that Snapshot 1 is malicious. It means that the
candidate is not the successor of the recorded head under the registry's
linear transition rules. The operator can still inspect or rehearse from the
older checkpoint, but it cannot silently replace the shared history with it.

This is the product's central distinction:

```text
local restore is possible
canonical continuation requires evidence
```

## Why it matters

Memory can influence what an agent sees in the next session. If the history of
that memory is not independently reviewable, a recovery process can silently
reintroduce an old state or omit changes that other parties expected to remain
part of the record.

The consequences depend on the agent and its operating context, but the trust
failure is consistent:

| Failure | What becomes difficult to establish | Possible consequence |
| --- | --- | --- |
| Old snapshot presented as current | Whether the agent resumed from the latest authorized state | Stale instructions, decisions, or task context may reappear |
| Transition skipped | Which changes were omitted from the history | Reviewers cannot reconstruct the state that was actually authorized |
| Parallel successor presented | Which branch is the shared continuation | Different operators may act from incompatible histories |
| Authority changed without a usable timeline | Who could authorize a transition at that point | Recovery and governance disputes become harder to resolve |
| Evidence is altered after export | Whether a report still describes the original transition | A dashboard or report cannot be independently trusted |
| Recovery evidence is incomplete | Whether the result is safe to classify as current | Operators may resume when the correct decision should be review or hold |

These are auditability and recovery risks, not guarantees that every restore
causes harm. MemoryLineage makes the risk observable at the succession boundary
so that a host application or human reviewer can choose a policy deliberately.

## Who needs this

MemoryLineage is relevant when at least two of these roles are separated:

```text
runtime operator  !=  transition authority/controller  !=  independent auditor
```

Examples include a managed agent runtime operated by one party for a protocol
or organization whose controller retains authority over state transitions, or
a recovery process where an independent reviewer must approve continuation
after a migration or incident.

The product is not necessary for every agent. If one trusted party controls the
runtime, storage, authorization key, and audit process, a local append-only log
may be sufficient. The Web3 case begins when independent parties need a shared
ordering and authorization reference that the runtime operator cannot rewrite
locally.

## Why a blockchain registry is relevant

MemoryLineage uses Ethereum for a narrow purpose: a shared checkpoint and
authorization boundary.

```text
private state -> fixed-size commitment -> shared registry -> replayable evidence
```

The registry provides:

- an ordered predecessor rule that is visible to independent parties;
- a canonical head for the registry's configured space;
- transition identifiers and derived roots that bind the committed fields;
- configured EOA/ERC-1271 authorization behavior;
- an authority-rotation record with sequence/configuration context;
- a public reference against which a local restore can be compared.

This is not a claim that a blockchain is automatically better than a database.
It is a conditional Web3 claim:

> When the operator must not be the only party trusted to preserve the
> history, a shared registry can provide a common ordering and authorization
> reference while the memory payload stays off-chain.

A database, signed file, or local append-only log may be the better choice when
there is only one trust domain. MemoryLineage should not be used to add a chain
where that condition does not exist.

## What happens without MemoryLineage

Without a shared commitment boundary, the recovery process may still work, but
its trust model is narrower:

1. The operator restores a private snapshot.
2. The operator's local system decides that the snapshot is current.
3. An auditor receives a database, export, or report produced by that system.
4. The auditor can check the supplied contents and signatures, but may have no
   independent reference for the expected predecessor and canonical head.
5. A disagreement becomes a dispute about which operator-controlled record to
   trust.

This does not make the database wrong. It means the reviewer cannot establish
the history independently of the party presenting it.

With MemoryLineage, the recovery question becomes explicit:

```text
candidate snapshot
    -> derive the candidate commitment under the named profile
    -> replay the named history
    -> compare the candidate with the recorded head/checkpoints
    -> inspect authority and observation assurance
    -> classify the recovery decision
```

The result can be:

| Classification | Meaning | Appropriate action |
| --- | --- | --- |
| `MATCHES DEMO EVIDENCE HEAD` / current head | Candidate matches the last checkpoint in the identified, replay-verified history | Continue only under the host policy and stated evidence assumptions |
| `KNOWN HISTORICAL CHECKPOINT` | Candidate is a real earlier checkpoint but not the current head | Rehearse or recover through an explicitly authorized process |
| `UNKNOWN / DIVERGED` | Candidate matches no known checkpoint or conflicts with the replay | Hold normal continuation and investigate |
| `UNVERIFIED` | Evidence is malformed, incomplete, unsupported, or cannot establish the required context | Fail closed for protected workflows |

These classifications describe evidence state. They do not label a historical
checkpoint as malicious and do not prove the contents of any private snapshot.

## What MemoryLineage actually changes

MemoryLineage changes the recovery decision from an operator assertion into an
auditable comparison:

| Before | With MemoryLineage |
| --- | --- |
| “The restored database is current.” | “This candidate matches or does not match the named committed history.” |
| “The report says the transition followed the previous one.” | “The verifier recomputes sequence, predecessor, transition ID, and state-root relationships.” |
| “The operator controls the audit log.” | “Independent parties can compare against a shared registry head and authority record.” |
| “The dashboard shows a green result.” | “The evidence bundle can be exported, tampered with, and replayed by a separate verifier.” |
| “A historical restore is either safe or malicious.” | “The product distinguishes current, historical, divergent, and unverified evidence.” |

The product therefore protects a narrow but useful boundary: the integrity and
authority of committed state succession.

## What is public and what remains private

The public verification domain contains only the data needed to audit the
lineage:

```text
PUBLIC / ON CHAIN
  sequence, predecessor root, fixed-size commitments, transition ID,
  derived state root, authorization metadata, and named observations

PRIVATE / LOCAL
  raw memory, documents, prompts, model context, private locator contents,
  secrets, and any blinding value used by a production commitment profile
```

Raw memory is not required to be published on-chain. This is a data-boundary
property, not a claim of absolute privacy. Commitment profiles can still leak
equality or metadata if they are poorly designed, and a production adapter
needs reviewed canonicalization, domain separation, and suitable hiding
parameters.

## Difference from adjacent problems

MemoryLineage is deliberately not a general AI safety product.

| System or approach | Question it answers | MemoryLineage's relationship |
| --- | --- | --- |
| Database backup and audit log | What records does the operator retain? | Useful inside one trust domain; it does not necessarily provide a shared external predecessor |
| Signed local record | Who signed this record? | Adds authorship evidence; it does not alone establish a shared canonical succession when the storage is operator-controlled |
| Memory/content safety guard | Is this write or memory semantically suspicious? | Complementary; MemoryLineage does not inspect natural-language meaning |
| Checkpoint/time-travel tool | Can a developer inspect or replay an older checkpoint? | Legitimate workflow; MemoryLineage distinguishes rehearsal from claiming that the checkpoint is current |
| Forkline | Is a blockchain event still canonical before an irreversible off-chain delivery? | Adjacent canonicality preflight; MemoryLineage audits private-agent state succession before recovery/resume |

The precise positioning is:

> MemoryLineage audits the canonical, authorized succession of committed
> private-agent memory states across separate trust domains.

It does not claim to detect semantic poisoning, prove a model's reasoning, or
prove that a later action was caused by a particular memory state.

## Current evidence and honest status

The repository currently demonstrates the problem with a deterministic local
Demo Space V2:

- three synthetic SQLite-derived snapshots;
- three local Solidity transitions executed against published bytecode in
  Rust/revm;
- an authority rotation;
- a transition-4 attempt using the actual earlier snapshot root;
- exact `BAD_PREVIOUS_STATE` rejection;
- portable evidence that can be tampered with and restored;
- an independent Rust verifier and CLI replay path.

The earlier Sepolia deployment and its read-only observations are separate
evidence. Demo Space V2 is not claimed as a Sepolia deployment. The synthetic
fixture is not production agent data, and the reference runtime adapter does
not prove external adoption.

| Question | Current status |
| --- | --- |
| Can a stale committed predecessor be rejected? | `VERIFIED` in the local Rust/revm evidence |
| Can a portable commitment be tampered with and detected? | `VERIFIED` with `TRANSITION_ID_MISMATCH` |
| Can the evidence be replayed independently? | `VERIFIED` by the Rust verifier/CLI |
| Is raw fixture memory included in the portable bundle? | `NOT INCLUDED` |
| Does this prove semantic memory safety? | `OUT OF SCOPE` |
| Does this prove an external runtime obeys the decision? | `NOT YET DEMONSTRATED` |
| Is this a formal verification or third-party security audit? | `NOT_FORMALLY_VERIFIED` |
| Does this prove production adoption or market impact? | `NOT YET DEMONSTRATED` |

These boundaries are part of the product. A verified lineage result is useful
because it states exactly what was checked and does not silently expand into a
claim about the memory's meaning.

## How success should be measured

Technical success is not the number of hashes or pages. It is whether an
independent reviewer can understand and reproduce the decision:

1. A first-time reviewer can explain the restore mismatch without learning the
   underlying draft standard first.
2. The Inspector identifies whether its source is live observation, local
   evidence, or a fallback.
3. The Silent Rollback shows the exact machine reason and a human explanation.
4. Evidence export and import produce the same verification result.
5. Changing one commitment fails closed with the expected mismatch.
6. A separate Rust CLI reaches the same result without trusting the dashboard.
7. The product exposes who was authorized and under which configuration
   context when that evidence is available.
8. The security page makes the non-goals as visible as the verified claims.
9. External human reproduction and real agent-runtime integration are recorded
   separately before either is claimed.

The first eight are product and technical acceptance criteria. The last item
is an adoption and integration gate, not something a fixture can establish.

## Short reusable explanations

### Fifteen-second explanation

> AI agents keep private memory in storage controlled by an operator. If that
> operator restores an old snapshot, other parties cannot tell whether it is
> current or stale. MemoryLineage records the authorized succession of memory
> states without publishing the memory, so a stale restore can be detected and
> independently verified.

### Problem statement for a submission

> Persistent AI-agent memory is commonly mutable off-chain storage controlled
> by the runtime operator. After restore, migration, or failover, an
> independent controller or auditor needs to know whether the candidate state
> is the authorized continuation of the previously committed history. Existing
> local storage does not provide a shared, replayable predecessor boundary
> across trust domains. MemoryLineage commits fixed-size state evidence to an
> Ethereum registry and verifies the succession without publishing raw memory.

### Why it is a Web3 problem

> The Web3 requirement appears when the runtime operator must not be the only
> party trusted to preserve the history. Ethereum supplies a shared registry
> for ordered commitments and configured authorization; the private memory
> remains off-chain and the evidence can be replayed independently.

## Non-goals

MemoryLineage does not claim to:

- determine whether memory content is true, useful, or safe;
- detect every form of memory poisoning or prompt injection;
- prove that an agent reasoned correctly or behaved harmlessly;
- prove which bytes a separate runtime actually loaded without a runtime
  adapter and attestation evidence;
- prove the availability or recoverability of off-chain memory;
- prove that a later tool call or transaction was caused by a memory state;
- provide a light-client or full consensus proof from an ordinary RPC response;
- replace a content-safety guard, backup system, runtime policy engine, or
  formal security audit;
- claim production adoption from a synthetic fixture.

The product is successful when it makes the narrower history question clear,
checkable, and independently falsifiable.
