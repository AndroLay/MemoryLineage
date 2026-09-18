# System boundaries

## Trust boundary

The operator may control the off-chain memory store. MemoryLineage therefore
publishes commitments and authorized transitions to a public registry. An
independent verifier can replay the evidence without receiving the raw memory.

## What is verified

- transition sequence continuity;
- predecessor state-root continuity;
- configured EOA or ERC-1271 authorization;
- EIP-712 domain and field binding;
- commitment and replay consistency;
- the pinned vector and mutation corpus results.

## What is outside the boundary

- semantic truth of memory content;
- whether a memory write is safe or malicious;
- correctness of an AI model or its reasoning;
- availability of private off-chain content;
- proof that an arbitrary external action was caused by a particular memory
  state;
- compatibility with a future, unpinned ERC-8350 draft.

## Dependency directions

```text
contracts/vectors ───────┬──> evm
                         └──> verifier/python
contracts/solidity ─────────> evm compiler/harness

evidence/public ─────────────> verifier and documentation tools
research/ and docs/ ──────────> no executable product code
```

The separate Python replay implementation is an intentional independence
boundary. It must not import JavaScript output or execute the EVM harness as its
verification mechanism.
