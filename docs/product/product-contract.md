# Product contract

## Product

MemoryLineage is an independent auditor for private AI-agent memory history.
Its central question is:

> Is this committed state the authorized, canonical continuation of the
> previously committed history?

## Hero scenario: Silent Rollback

The agent's private memory lives in an off-chain store controlled by its
operator. The registry has accepted transitions `S1 -> S2 -> S3`. The operator
can restore the local store to `S1`; that changes the local snapshot, not the
registry history. If the operator submits transition `S4` using the root from
`S1`, the registry rejects it with `BAD_PREVIOUS_STATE` because the canonical
head is `S3`. The independent verifier can replay the evidence and show which
predecessor failed.

This demonstrates one narrow guarantee: a stale snapshot cannot be accepted as
the next canonical committed transition under the registry rules. MemoryLineage
does not prevent a local restore or evaluate the meaning of the private memory.

## Product flow

```text
Inspect -> Replay -> Attack -> Explain -> Export -> Verify offline
```

## User-visible outputs

- current sequence and state-root head;
- transition timeline with commitment prefixes;
- current authority configuration and a locally evidenced authorization
  rotation trace;
- exact rejection reason for a tested mutation;
- portable evidence bundle and offline verification verdict.

## Claims we make

- The registry enforces ordered transitions and predecessor continuity.
- Configured authorization is checked against the signed transition fields.
- Raw memory is not required on-chain.
- Published evidence can be replayed by a separate implementation.

## Claims we do not make

- Memory content is truthful or semantically safe.
- The agent's reasoning is correct.
- Every rollback or memory-poisoning attack is detected.
- External actions are causally explained by a memory state.
- The draft standard is final or immutable.
