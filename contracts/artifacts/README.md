# Curated Solidity artifacts

These creation bytecode files are checked in so the Rust/revm execution lane
does not require Node.js or an online Solidity compiler at runtime.

Regenerate them from the existing compatibility compiler with:

```bash
npm ci --ignore-scripts --no-audit --no-fund
node scripts/export-solidity-artifacts.mjs
```

The legacy EthereumJS harness remains the source-compatible oracle. The
artifact manifest records the compiler version used to produce this snapshot.
