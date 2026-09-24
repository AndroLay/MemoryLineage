# Demo Space V2: Silent Rollback

Three synthetic SQLite snapshots produce private commitments that were used
for three accepted local Rust/revm transitions. Sequence 3 is the canonical
head. Snapshot 1 is then restored locally. The attempted transition 4 uses
the **state root produced by transition 1** as its predecessor, so the local
Solidity execution rejects it with `BAD_PREVIOUS_STATE`.

The snapshot-1 commitment and the transition-1 state root are different
values. [`manifest.json`](manifest.json) records both; the
[`rollback-rehearsal.json`](rollback-rehearsal.json) records the exact attempt.
[`revm-observation.json`](revm-observation.json) is a duplicate of the
deterministic V2 Rust/revm output for package comparison. No transaction was
broadcast. The separate Sepolia observations are not this incident.

Replay the incident and package from the repository root:

```bash
cargo run -q -p ml-cli -- verify evidence/local/demo_space_v2_evidence.json
cargo run -q -p ml-cli -- submission verify evidence/submission/manifest.json
```
