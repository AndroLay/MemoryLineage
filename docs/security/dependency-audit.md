# Dependency audit record

Date: 20 September 2026

## Rust workspace

`cargo metadata --format-version 1 --locked --offline` completed against the
checked-in `Cargo.lock`. All workspace packages declare MIT licensing, and the
resolved direct Rust dependencies are recorded in
[`THIRD_PARTY_NOTICES.md`](../../THIRD_PARTY_NOTICES.md).

The RustSec advisory database was scanned with:

```bash
cargo audit
```

Result: no vulnerability, unsoundness, or yanked-package advisory was reported
for the 633 resolved dependencies. The scan reports two allowed maintenance
warnings:

| Crate | Version | Advisory | Interpretation |
| --- | ---: | --- | --- |
| `derivative` | 2.2.0 | `RUSTSEC-2024-0388` | Unmaintained transitive crate in the locked dependency graph |
| `paste` | 1.0.15 | `RUSTSEC-2024-0436` | Unmaintained transitive crate; active paths include the pinned Alloy/revm lane |

These are maintenance warnings, not known vulnerability advisories. The
workspace keeps Alloy `2.4.2`, revm `43.0.2`, Dioxus `0.8.0-alpha.1`, and the
checked-in lockfile because the current versions are part of the verified
Rust/EVM/WASM parity boundary. Replacing them only to remove an unmaintained
transitive macro crate would require a fresh parity and release audit. A future
dependency refresh must rerun the full Rust/revm, evidence, WASM, and browser
gates before it can be accepted.

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
