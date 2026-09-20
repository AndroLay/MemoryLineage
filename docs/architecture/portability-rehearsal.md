# Portability rehearsal

MemoryLineage keeps Ethereum Sepolia as its public trust anchor. The current
Polkadot work is a bounded preparation step that tests whether the existing
Solidity registry can be exercised under the Polkadot Hub TestNet EVM chain
context without changing protocol semantics.

```text
Demo Space V2 SQLite commitments
              |
              +--> local revm / Ethereum chain context
              |
              +--> local revm / Polkadot Hub TestNet chain context
                               |
                               v
          compare transitions, state roots, authority history,
          stale-predecessor rejection, and EIP-712 domain separation
```

The checked-in report is:

```text
evidence/local/polkadot_hub_portability_rehearsal.json
```

The report carries two nested V2 observations: the Ethereum-local execution
context and the Polkadot Hub TestNet chain context. The independent Rust path
replays both bundles with:

```bash
cargo run -q -p ml-cli -- portability verify
```

It then compares their transition/root projection, authority history, stale
predecessor result, and chain-bound EIP-712 domain. A changed nested head or
authorization proof fails closed before the report can be marked verified.

Its status is `LOCAL_REHEARSAL_PASS`. The comparison proves only that the
same local Rust/revm harness, bytecode artifact, and fixture inputs produce the
same tested lineage behavior while using chain ID `420420417`, and that the
EIP-712 domain changes with the chain context. The report deliberately records
both `deployment: NOT_PERFORMED` and `publicRpcObservation: NOT_PERFORMED`.

This is not an implementation of Polkadot consensus, a node client, a bridge,
XCM, PVM, a cross-chain identity protocol, or a two-chain canonicality claim.
The next deployment milestone, if authorized later, must add an actual
Polkadot Hub observation with contract address, code hash, block context,
transaction receipts, destination-domain signatures, and an independent
readback. Until those artifacts exist, public copy must use “local portability
rehearsal” rather than “Polkadot support”.

The target chain context and REVM compatibility boundary are based on the
[official Polkadot Hub smart-contract documentation](https://docs.polkadot.com/polkadot-protocol/smart-contract-basics/polkavm-design/)
and its [network reference](https://docs.polkadot.com/smart-contracts/dev-environments/foundry/).
