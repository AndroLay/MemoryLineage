# Dependency audit record

Date: 20 September 2026

## Rust workspace

`cargo metadata --format-version 1 --locked --offline` completed against the
checked-in `Cargo.lock`. All workspace packages declare MIT licensing, and the
resolved direct Rust dependencies are recorded in
[`THIRD_PARTY_NOTICES.md`](../../THIRD_PARTY_NOTICES.md). `cargo-audit` is not
installed in this environment, so this is license/lockfile evidence rather than
a Rust advisory scan.

## Preserved Node compatibility lane

Commands:

```bash
npm ci --ignore-scripts
npm audit --omit=dev --audit-level=high
```

Result: `found 0 vulnerabilities` in the installed production dependency
graph. The compatibility lane keeps the evidence-bound Next.js `15.5.25` and
solc `0.8.36` versions. The root package manifest applies scoped npm overrides
to PostCSS `8.5.23` and tmp `0.2.7`, the respective patch releases, because
the npm advisory database flags the locked transitive versions. Keeping solc
itself pinned preserves the published compiler and bytecode baseline.
`npm ci --ignore-scripts` and the complete `npm run verify` lane must be rerun
whenever either override changes.

This is a compatibility-lane dependency remediation, not a claim that the
Solidity contract has undergone a formal security audit.

The Rust/Dioxus website does not execute the legacy Next.js server or compile
the Solidity contract in the browser. The contract trust anchor remains the
already-published artifact and its evidence-bound compiler lane.

The exact dependency metadata and override are recorded in `package-lock.json`
and `package.json`.
