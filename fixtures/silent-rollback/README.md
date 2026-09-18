# Silent Rollback fixture

This is a synthetic, local-only SQLite fixture for the Inspector demonstration.
It contains three private snapshots:

```text
17 -> language
18 -> language + claim_policy
19 -> language + claim_policy + privacy
```

The demo restores snapshot 17 while the committed canonical head is state 19,
then attempts the next transition with the stale predecessor. The local
registry rejects it with `BAD_PREVIOUS_STATE`. The Rust memory-store lane also
derives a deterministic `snapshotCommitment` for each private file. The
Inspector may use that commitment as the stale predecessor input for a
read-only Sepolia rehearsal; it is a private-fixture commitment, not a claim
that raw SQLite content is a semantic truth root.

Regenerate and verify the snapshots with:

```bash
npm run fixtures:silent-rollback

# Refresh the commitment-only manifest with the Rust implementation.
cargo run -q -p ml-cli -- fixture manifest
```

The snapshot contents are never sent to the registry or included in portable
evidence exports.
