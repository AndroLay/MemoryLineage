# Contributing

Run the complete local gate before proposing a change:

```bash
npm ci
npm run verify
```

Keep protocol inputs in `contracts/`, the EVM lane in `evm/`, and the
independent replay lane in `verifier/python/`. Put exploratory work under
`research/spikes/` and explain any new public claim in
`docs/submission/claim-matrix.md`.

Never commit private keys, raw private agent memory, `.env` files, dependency
directories, Python caches, or generated output that cannot be reproduced.
