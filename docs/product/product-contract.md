# Product contract

## Product

MemoryLineage is an independent auditor for private AI-agent memory history.
Its central question is:

> Is this committed state the authorized, canonical continuation of the
> previously committed history?

## Hero scenario: Silent Rollback

An agent progresses through private states `S1 -> S2 -> S3`. An operator
restores `S1` and tries to continue the history as if `S2` and `S3` never
existed. The registry rejects the stale predecessor, and the independent
verifier explains the exact invariant that failed.

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
