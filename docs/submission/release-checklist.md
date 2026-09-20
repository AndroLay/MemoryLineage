# Release checklist

This checklist is deliberately evidence-based. `PASS` means the repository
contains a reproducible artifact or a completed local gate. Missing external
evidence remains `NOT YET DEMONSTRATED`.

## Repository and product

| Gate | Status | Evidence |
| --- | --- | --- |
| Rust workspace and pinned toolchain | PASS | `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml` |
| Dioxus Inspector routes | PASS | 11 routes in `apps/inspector/src/main.rs` and static browser smoke |
| Silent Rollback exact result | PASS | `BAD_PREVIOUS_STATE` in local Demo Space V2 Rust/revm evidence |
| Evidence tamper/restore | PASS | `TRANSITION_ID_MISMATCH` then `VERIFIED` in browser smoke |
| Recovery Decision Receipt | PASS | Current and historical receipts plus Rust/WASM/CLI checks |
| Public package boundary | PASS | `scripts/check-public-package.sh --release` |
| Reviewer archive tooling | PASS | `cargo xtask reviewer-package` and `cargo xtask reviewer-reproduce` |
| Public static website | LIVE | [memorylineage.pages.dev](https://memorylineage.pages.dev), hosted on Cloudflare Pages |
| Pitch deck source | PASS | [`pitch-deck.md`](pitch-deck.md) |

## Automated verification

Run from a clean checkout:

```bash
cargo xtask verify
cargo xtask release
cargo xtask reviewer-reproduce
npm run verify
```

The static Dioxus release path and Chromium smoke are the supported browser
release path. Local development is supported through `cargo xtask serve-web`,
which disables the pinned Dioxus alpha's incompatible Rust hot-patch path;
`cargo xtask smoke-dev-web` repeats that browser check. Direct
`dx serve --web` must include `--hot-patch false`.

## External and submission gates

| Gate | Status | Why |
| --- | --- | --- |
| External human clean-checkout report | NOT YET DEMONSTRATED | `evidence/reproduction/` contains no self-authored report |
| GitHub Actions remote PASS | PENDING | Recent runs ended in platform `startup_failure` with no jobs |
| Final submission tag | NOT YET CREATED | Create only after the release candidate and external evidence are frozen |
| New Sepolia Demo Space V2 deployment | OUT OF SCOPE | Demo Space V2 remains local Rust/revm evidence; the existing Sepolia observation is separate |
| Separate staging environment | NOT PROVIDED | Only the public static website is hosted |
| Demo video | OUT OF SCOPE | Explicitly excluded from the current work |

## Finalization rule

Do not create `v1.0-3rd-web-hack` or describe the submission as externally
reproduced until the human report and final remote CI result exist. The local
repository can be released as a review candidate before those external gates,
but the distinction must remain visible in the website, README, and Devpost
copy.
