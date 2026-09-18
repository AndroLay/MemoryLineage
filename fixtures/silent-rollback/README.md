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
EthereumJS registry rejects it with `BAD_PREVIOUS_STATE`.

Regenerate and verify the snapshots with:

```bash
npm run fixtures:silent-rollback
```

The snapshot contents are never sent to the registry or included in portable
evidence exports.
