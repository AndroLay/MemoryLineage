# Rust parity ledger

This ledger records what has been compared against the preserved JavaScript,
Python, Solidity, and Sepolia evidence lanes. It is evidence for the migration,
not a judge score.

| Boundary | Result | Reproduction |
| --- | --- | --- |
| Pinned space/transition/root vectors | PASS | `cargo test -p ml-core` |
| Rust independent evidence replay | PASS | `cargo run -q -p ml-cli -- verify evidence/local/memory_lineage_evm_evidence.json` |
| Structured conformance report | MATCH | `cargo run -q -p ml-cli -- conformance` |
| SQLite snapshot read/restore | PASS | `cargo test -p ml-memory-store` |
| Alloy host ABI and Sepolia read | PASS | `cargo run -q -p ml-cli -- live inspect` |
| Real Sepolia stale predecessor `eth_call` | PASS | `cargo run -q -p ml-cli -- live rollback` |
| Dioxus native compile | PASS | `cargo check -p memorylineage-inspector` |
| Dioxus WASM compile | PASS | `cargo check -p memorylineage-inspector --target wasm32-unknown-unknown` |
| Dioxus release artifact DOM smoke | PASS | Chromium loaded the static release artifact and rendered the hero, case board, tabs, and Verify view |
| Dioxus release interaction smoke | PASS | Chromium/CDP clicked Silent Rollback, Verify, and Tamper; observed `BAD_PREVIOUS_STATE`, `VERIFIED`, and `TRANSITION_ID_MISMATCH` |
| Dioxus public route smoke | PASS | Chromium/CDP loaded `/`, `/inspect`, `/history`, `/history/4`, `/lab`, `/verify`, `/evidence`, `/architecture`, `/security`, `/reproduce`, and `/prior-work` through a static SPA fallback |
| Dioxus responsive smoke | PASS | Chromium rendered the Home page at the compact 390px viewport without critical clipping; full dev-server responsive parity remains separately qualified below |
| Evidence decoder regression | PASS | `cargo test -p memorylineage-inspector` loads the bundled deployment, reread, replay, mutation, and conformance artifacts |
| Legacy nine-step gate | PASS | `npm run verify` |
| Dioxus dev-server route and full responsive parity | NOT YET DEMONSTRATED | `dx serve --web` currently fails in the dev emitter on the generated WASM exceptions proposal; release static artifact smoke is separate |
| Rust/revm registry execution slice | PASS | `cargo run -q -p ml-cli -- revm silent-rollback` deploys the curated Solidity artifact, commits three transitions, and decodes `BAD_PREVIOUS_STATE` |
| Rust/revm core mutation lane | PASS | `cargo run -q -p ml-cli -- revm mutations` rejects 20/20 ordering, predecessor, zero-value, unknown-space, EOA signature/domain, replay, and branch cases with expected reasons |
| Rust/revm ERC-1271 lane | PASS | `cargo run -q -p ml-cli -- revm erc1271` deploys the mock authorizer, accepts its owner signature, then rejects after acceptance is disabled |
| Rust/revm authority rotation | PASS | `cargo run -q -p ml-cli -- revm authority-rotation` advances `configNonce`, rejects the old authorizer, and accepts the new authorizer |

The exact contract revert reason remains `BAD_PREVIOUS_STATE`. Product copy may
describe that result as a stale predecessor or predecessor mismatch, but the
machine reason is preserved.

Machine-readable outputs for these lanes are stored in
`evidence/local/rust_revm_*.json`; they are deterministic outputs of the pinned
artifact and contain no raw memory or signing keys.

The current browser target uses Rust/WASM verification and a browser JSON-RPC
transport for read-only Sepolia `eth_call`. If the public RPC cannot be read,
the UI must label the published evidence fallback rather than presenting it as
a live result.

The Rust/revm lane currently covers the registry deployment, registration,
three valid direct-authorized commits, Rust reference root comparison, the
stale-predecessor rejection, a 20-case core mutation matrix, and the mock
ERC-1271 accept/reject path. EthereumJS remains the broader complete execution
oracle until every legacy scenario is compared field by field.
