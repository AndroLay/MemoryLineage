# Evidence catalog

Evidence is grouped by how it is produced:

- `local/` — curated local verification summaries;
- `sepolia/` — public testnet deployment and readback observations;
- `silent-rollback/` — commitment-only observations of the private SQLite
  fixture used by the hero rehearsal;
- `conformance/` — vector and cross-implementation comparison outputs;
- `generated/` — reproducible machine output that should not be treated as
  hand-authored source.

Every artifact should state its generating command, source revision or pinned
vector, and whether it is a public claim or historical research output. A
private snapshot commitment never includes the underlying SQLite values.

The read-only Sepolia fixture rehearsal is recorded in
`sepolia/silent_rollback_fixture_eth_call.json`. It uses the existing deployed
registry and a commitment from snapshot 17; it does not perform a deployment
or broadcast a transaction.
