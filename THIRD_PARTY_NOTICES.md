# Third-party notices

MemoryLineage uses the following open-source components in its Rust/WASM
application and local EVM verification toolchain. Versions below are the
resolved package metadata from the checked-in `Cargo.lock` and
`package-lock.json`, not floating release claims:

## Rust application and verification lanes

- `dioxus` 0.8.0-alpha.1 — MIT OR Apache-2.0.
- `alloy` 2.4.2 and its `alloy-primitives` / `alloy-sol-types` 1.7.3 — MIT OR Apache-2.0.
- `revm` 43.0.2 — MIT.
- `rusqlite` 0.40.2 — MIT.
- `serde` 1.0.229 and `serde_json` 1.0.151 — MIT OR Apache-2.0.
- `thiserror` 2.0.20 — MIT OR Apache-2.0.
- `tiny-keccak` 2.0.2 — CC0-1.0.
- `tokio` 1.53.1 — MIT; `reqwest` 0.13.5 — MIT OR Apache-2.0.
- `wasm-bindgen` 0.2.128, `web-sys` and `js-sys` 0.3.105 — MIT OR Apache-2.0.
- `hex` 0.4.3 — MIT OR Apache-2.0.

## Preserved JavaScript compatibility lane

- `ethers` 6.17.0 — MIT License.
- `@ethereumjs/common`, `@ethereumjs/tx`, `@ethereumjs/util`, and
  `@ethereumjs/vm` 10.1.3 — MPL-2.0.
- `solc` 0.8.36 — the npm wrapper is MIT licensed; the Solidity compiler
  distribution carries its own upstream license and notices.

The exact dependency metadata is recorded in `package-lock.json`. The
ERC-8350 draft and its published vectors are prior standards work and are
referenced as an explicitly pinned compatibility target; MemoryLineage does
not claim authorship of that standard.

The repository does not claim that this inventory is a security audit. A
network-backed advisory scan is an additional release check when the registry
is reachable; offline builds remain reproducible from the lockfiles.
