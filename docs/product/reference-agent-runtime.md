# Reference agent runtime integration

MemoryLineage now includes a small, framework-neutral reference runtime in
`crates/ml-agent-runtime`. It is the first executable integration boundary
between the recovery decision and a stateful agent process in this repository.

The runtime loads actual private values from the deterministic SQLite fixture
only after the recovery gate permits the current canonical head. It keeps those
values in an in-memory session and does not place them in the evidence bundle,
the registry, or the runtime report.

## Reproduce it

```bash
cargo run -q -p ml-cli -- agent reference-demo \
  fixtures/silent-rollback-v2 \
  evidence/local/demo_space_v2_evidence.json \
  /tmp/reference-agent-runtime.json
```

The committed output is
`evidence/local/reference_agent_runtime.json`. The verification gate regenerates
the file byte-for-byte with:

```bash
cargo xtask verify
```

The report covers four outcomes:

| Candidate | Runtime decision | Loader |
| --- | --- | --- |
| Current head / snapshot 3 | `RESUMED` / `RESUME_ALLOWED` | Invoked |
| Known historical checkpoint / snapshot 1 | `HELD` / `REHEARSE_ONLY` | Not invoked |
| Unknown or diverged snapshot | `HELD` / `HOLD_FOR_REVIEW` | Not invoked |
| Invalid evidence | `FAIL_CLOSED` | Not invoked |

This is a real local runtime path: the loader receives the private snapshot
only for the permitted current-head case. The other cases are held before the
loader boundary. The fixture values are synthetic and public for reproducible
testing, so this artifact does not demonstrate deployment adoption or a
third-party agent framework integration.

## What this changes in the product claim

The project can now claim:

> A reference agent runtime can enforce the verified recovery decision at its
> loader boundary for the supplied local evidence profile.

The project still cannot claim:

- an external developer has adopted the adapter;
- a production agent framework has integrated it;
- all agent runtimes will enforce the policy;
- semantic safety or truthfulness of the loaded memory.

Those claims require an independently maintained integration or external human
reproduction record. They remain explicitly `NOT YET DEMONSTRATED` in the
Inspector and submission claim matrix.
