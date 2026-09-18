# Dependency audit record

Date: 19 September 2026

## Rust workspace

`cargo metadata --format-version 1 --locked --offline` completed against the
checked-in `Cargo.lock`. All workspace packages declare MIT licensing, and the
resolved direct Rust dependencies are recorded in
[`THIRD_PARTY_NOTICES.md`](../../THIRD_PARTY_NOTICES.md). `cargo-audit` is not
installed in this environment, so this is license/lockfile evidence rather than
a Rust advisory scan.

## Preserved Node compatibility lane

Command:

```bash
npm audit --omit=dev --audit-level=high --package-lock-only
```

Result: the audit reports four advisories in the preserved Next.js/Solidity
compatibility lane:

- `postcss` is pulled by the locked Next.js lane; the available remediation
  requires a breaking Next.js 16 upgrade;
- `tmp` is pulled by `solc` 0.8.36; the available remediation requires moving
  to `solc` 0.8.37 outside the pinned compiler range.

`npm audit fix --dry-run --package-lock-only --omit=dev` confirmed those are
the available upgrade paths and made no lockfile change. They are not silently
forced during this submission hardening because the Next.js lane is retained as
a compatibility oracle and changing `solc` can change compiled bytecode and
invalidate the published contract evidence. Revisit both dependencies before a
future production deployment; this repository does not claim the legacy lane is
advisory-clean.

The Rust/Dioxus website does not execute the legacy Next.js server or compile
the Solidity contract in the browser. The contract trust anchor remains the
already-published artifact and its evidence-bound compiler lane.
