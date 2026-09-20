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
| Protected resume adapter | PASS | `cargo test -p ml-recovery-gate` holds snapshot 1 before the loader callback and permits snapshot 3 |
| SQLite fixture commitment manifest | PASS | `cargo xtask verify` regenerates the manifest in `/tmp` and compares it byte-for-byte |
| Alloy host ABI and Sepolia read | PASS | `cargo run -q -p ml-cli -- live inspect` |
| Real Sepolia stale predecessor `eth_call` | PASS | `cargo run -q -p ml-cli -- live rollback` |
| Dioxus native compile | PASS | `cargo check -p memorylineage-inspector` |
| Dioxus WASM compile | PASS | `cargo check -p memorylineage-inspector --target wasm32-unknown-unknown` |
| Dioxus release artifact DOM smoke | PASS | Chromium loaded the static release artifact and rendered the hero, case board, tabs, and Verify view |
| Dioxus release interaction smoke | PASS | Chromium/CDP replayed local Demo Space V2 in Silent Rollback, then tampered/restored the V2 bundle and the Recovery Decision Receipt; observed `BAD_PREVIOUS_STATE`, `TRANSITION_ID_MISMATCH`, `RECOVERY_DECISION_MISMATCH`, and `VERIFIED` |
| Dioxus public route smoke | PASS | `cargo xtask smoke-web` loads `/`, `/inspect`, `/history`, `/history/3`, `/lab`, `/verify`, `/evidence`, `/architecture`, `/security`, `/reproduce`, and `/prior-work` through a static SPA fallback |
| History ledger | PASS | Browser and Rust tests confirm three Demo Space V2 commits plus the stale-root rejection; the separate 20-case protocol corpus is explicitly labeled |
| Dioxus responsive smoke | PASS | `cargo xtask smoke-web` checks every route at 390px for page-level horizontal overflow; full dev-server parity remains separately qualified below |
| GitHub Actions browser gate | CONFIGURED | CI installs the pinned Dioxus CLI, builds the static Inspector, and runs `cargo xtask smoke-web`; report a remote PASS only after the run completes |
| Evidence decoder regression | PASS | `cargo test -p memorylineage-inspector` loads the bundled deployment, reread, replay, mutation, and conformance artifacts |
| Legacy nine-step gate | PASS | `npm run verify` |
| Dioxus dev-server route and full responsive parity | NOT YET DEMONSTRATED | `dx serve --web` starts, but the browser build fails with `Failed to parse import section` in the pinned emitter; release static artifact smoke is separate |
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

The primary Silent Rollback action replays the local Demo Space V2 bundle in
Rust/WASM. A separate browser JSON-RPC control performs an optional read-only
Sepolia `eth_call`; if it fails, the UI preserves the local result and reports
the probe as unavailable. The static release smoke is the reproducible browser
gate. The pinned `dx serve --web` server starts, but its browser emitter still
fails while parsing the generated WASM module, so full dev-server parity remains
separately qualified.

The Rust/revm lane currently covers the registry deployment, registration,
three valid direct-authorized commits, Rust reference root comparison, the
stale-predecessor rejection, a 20-case core mutation matrix, and the mock
ERC-1271 accept/reject path. EthereumJS remains the broader complete execution
oracle until every legacy scenario is compared field by field.
