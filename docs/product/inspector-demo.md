# Inspector demo runbook

MemoryLineage Inspector is a Rust/WASM Dioxus website over curated evidence,
independent replay, and an optional read-only Sepolia probe. It does not require
a wallet, private key, transaction broadcast, or authoritative backend.

## Start

Run the deterministic verification gate and build the static website:

```bash
cargo xtask verify
cargo xtask build-web
```

Serve `target/dx/memorylineage-inspector/release/web/public/` with an SPA
fallback to `index.html`. The pinned Dioxus development emitter still has a
WASM exceptions-proposal limitation, so the documented local path uses the
static release artifact. The preserved Next.js surface is a compatibility
oracle only.

## Demo Space V2

The unified local incident starts from three synthetic SQLite snapshots that
model private agent state. The sample database is included with the public
repository for deterministic reproduction; its values are omitted from the
portable evidence and are never written to the registry:

```text
Snapshot 1 → Snapshot 2 → Snapshot 3
Transition 1 → Transition 2 → authority rotation → Transition 3
```

The Rust/revm harness executes the published Solidity bytecode. Transition 4
then uses the actual state root produced by transition 1 while the local
registry head is transition 3. The expected exact result is
`REJECTED / BAD_PREVIOUS_STATE`. The simulation does not write to a network.

Recreate and verify the same incident from the CLI:

```bash
cargo run -q -p ml-cli -- fixture demo-create /tmp/memorylineage-demo-fixture
cargo run -q -p ml-cli -- evidence demo-v2 /tmp/memorylineage-demo-fixture /tmp/memorylineage-demo-evidence.json
cargo run -q -p ml-cli -- verify /tmp/memorylineage-demo-evidence.json
```

The primary Tampering Lab action independently verifies the published local
Demo Space V2 bundle in Rust/WASM. A separate control can issue a read-only
`eth_call` against the existing Sepolia deployment, which uses a different
space and is labeled as a separate observation. An unavailable or unexpected
RPC result does not replace or change the local result. No transaction is
broadcast.

## Judge flow

1. Open `/inspect` to see the local Demo Space V2 head and the separate
   Sepolia observation.
2. Open `/history` to trace transitions 1–3 and the authority rotation before
   transition 3.
3. Open `/lab/silent-rollback` and run the local evidence replay. The separate
   Sepolia probe is optional and visibly labeled as a different observation.
4. Open `/verify`, export the bundle, tamper with `locatorCommitment`, and
   restore the original. Expected results are `TRANSITION_ID_MISMATCH` and
   `VERIFIED`.
5. Verify the same exported bundle with the independent Rust CLI.

The separate protocol corpus still contains four committed transitions and
the 20-case mutation lane. It is not presented as the Demo Space V2 incident.
The result proves ordered committed history, predecessor continuity, the
configured registry rules, and the integrity of this evidence bundle. It does
not prove that private memory is truthful or semantically safe.

## Existing Sepolia observation

The published deployment, second-endpoint reread, and read-only rejection
artifacts describe the existing Sepolia space. No new public Demo Space V2
deployment is part of this work. This boundary keeps the website truthful about
which history is local and which observation is public-chain evidence.
