# Inspector demo runbook

MemoryLineage Inspector is designed to be shown in a short, falsifiable flow.
The primary website is the Rust/WASM Dioxus release artifact. It reads curated
evidence, can perform a read-only Sepolia `eth_call`, and does not need a
wallet, private key, or authoritative backend.

## Start

Build the release artifact and run the repository verification gate:

```bash
cargo xtask verify
cargo xtask build-web
```

Serve `target/dx/memorylineage-inspector/release/web/public/` with a static
server configured to fall back unknown routes to `index.html`. The pinned
Dioxus development emitter currently has a known WASM exceptions-proposal
limitation; the static release artifact is the verified browser path. The
preserved Next.js surface can still be started with `npm ci && npm run dev`
when the legacy compatibility oracle is needed.

## Demo sequence

1. Start on `Inspect`. Point out the four committed states, the `VERIFIED`
   verdict, the Sepolia observation or published-evidence source label, and the
   raw-memory boundary.
2. Choose `Run Silent Rollback`. The browser reads the observed canonical head,
   constructs a stale-predecessor call using the commitment of the restored
   private fixture snapshot, and sends it to the deployed registry with
   `eth_call`; no transaction is broadcast. The local SQLite fixture and
   Rust/revm lane provide the reproducible offline counterpart.
3. Show `REJECTED / BAD_PREVIOUS_STATE`. The response comes from the existing
   Solidity registry, not a browser-only rule. If the public RPC is unavailable,
   the UI labels the result `PUBLISHED EVIDENCE`.
4. Open `History` to inspect the four public transition records. Payload,
   provenance, and locator values are not included. The three private fixture
   snapshots are shown as a separate commitment-only lane.
5. Open `Verify`, choose `Export evidence.json`, and run the independent Rust path:

  ```bash
   cargo run -q -p ml-cli -- verify memorylineage-evidence-v2.json
  ```

6. Import the file back into the page. Use `Tamper one field` to change the
   exported commitment and show that the browser verifier rejects it. The
   historical Python verifier remains available as a V1 cross-language oracle:

   ```bash
   python3 verifier/verify.py evidence/local/memory_lineage_evm_evidence.json
   ```

The result proves ordered committed history, predecessor continuity, configured
authorization, and evidence integrity. It does not prove that private memory
content is true, safe, or semantically correct.

## Fallback behavior

`Read Sepolia now` attempts a read-only `head` and `spaceAuthorization` call
through the Rust/WASM browser JSON-RPC transport. When browser access to the
public RPC is unavailable, the page labels the result `PUBLISHED EVIDENCE` and
shows the recorded second-endpoint readback instead.
