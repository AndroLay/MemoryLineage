# Evidence catalog

Evidence is grouped by how it is produced:

- `local/` — curated local verification summaries;
- `sepolia/` — public testnet deployment and readback observations;
- `silent-rollback/` — commitment-only observations of the private SQLite
  fixture used by the hero rehearsal;
- `conformance/` — vector and cross-implementation comparison outputs;
- `generated/` — reproducible machine output that should not be treated as
  hand-authored source.

The curated `local/demo_space_v2_recovery_receipt.json` is a commitment-only
decision artifact for the current head. Its sibling
`local/demo_space_v2_historical_recovery_receipt.json` records the same policy
holding snapshot 1 as `KNOWN_HISTORICAL_CHECKPOINT` / `REHEARSE_ONLY`. Both bind
the Demo Space V2 source class, effective authority timeline, and strict
current-head policy to the V2 bundle and can be replayed with the Rust verifier.
For the signed Demo Space V2 scope their assurance reports `TIMELINE_BOUND` and
`EOA_SIGNATURES_VERIFIED`; they do not contain raw memory or assert that a
production runtime enforced the decision. The schema is
`schemas/recovery-receipt-v1.schema.json`.

Every artifact should state its generating command, source revision or pinned
vector, and whether it is a public claim or historical research output. A
private snapshot commitment never includes the underlying SQLite values.

The read-only Sepolia fixture rehearsal is recorded in
`sepolia/silent_rollback_fixture_eth_call.json`. It uses the existing deployed
registry and a commitment from snapshot 17; it does not perform a deployment
or broadcast a transaction.

`local/reference_agent_runtime.json` records the framework-neutral reference
runtime loader outcomes for current-head, historical, diverged, and invalid
evidence candidates. It is local integration evidence, not external adoption.

`local/security_assurance_report.json` records the bounded Rust/revm assurance
pass over the pinned Solidity artifact. Its formal status is deliberately
`NOT_FORMALLY_VERIFIED`; it is not a third-party security audit. The structural
schema is `schemas/security-assurance-v1.schema.json`.
