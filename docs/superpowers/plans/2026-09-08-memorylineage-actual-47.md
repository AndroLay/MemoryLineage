# MemoryLineage Actual 4.7 Evidence Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build and verify a bounded MemoryLineage MVP far enough that its score can be recalculated from workspace-owned EVM, replay, mutation, live-flow, and impact evidence instead of external claims.

**Architecture:** Keep the existing Python commitment probe as a reference only. Add a small Solidity registry with EIP-712 EOA authorization and ERC-1271 support, execute it in a deterministic local EVM, and have a separate verifier consume exported evidence without importing the contract or reference probe. Store every pass/reject result, mutation, gas/latency measurement, and score gate in JSON.

**Tech Stack:** Solidity 0.8.x, `solc`, EthereumJS VM, ethers-compatible signing, Python 3 standard library, unittest, JSON evidence.

**Spec:** [DEEP_RESEARCH_2026-09-08.md](../../research/DEEP_RESEARCH_2026-09-08.md:1) and [AUDIT_LOOP_4_7_2026-09-08.md](../../research/AUDIT_LOOP_4_7_2026-09-08.md:1)

## Global Constraints

- The raw memory payload never enters contract storage or public evidence.
- The contract must enforce space, authorizer, sequence, predecessor root, transition commitment, and profile binding.
- Valid EOA and ERC-1271 signatures must be tested separately; signatures must bind every transition field.
- The independent verifier cannot import the contract wrapper, evaluator, or existing `spike/memory_lineage.py` decision function.
- A local EVM result is labelled local EVM evidence; it cannot be called testnet/live evidence.
- Scores use the existing four dimensions and half-up rounding; no 4.7 claim is made before all required gates pass.

---

### Task 1: Add deterministic EVM toolchain and contract test harness

**Files:**
- Create: `evm/package.json`
- Create: `evm/run_tests.mjs`
- Create: `evm/test/memory_lineage_registry.test.mjs`
- Create: `evm/contracts/MemoryLineageRegistry.sol`
- Create: `evm/contracts/Mock1271Authorizer.sol`

**Interfaces:**
- `MemoryLineageRegistry.createSpace(bytes32 spaceId, address authorizer, bytes32 profileId)`
- `MemoryLineageRegistry.commit(Transition calldata transition, bytes calldata signature)`
- `MemoryLineageRegistry.head(bytes32 spaceId) returns (uint64 sequence, bytes32 root)`
- `MemoryLineageRegistry.transitionId(Transition calldata transition) returns (bytes32)`
- `Mock1271Authorizer.isValidSignature(bytes32 digest, bytes signature) returns (bytes4)`

- [ ] Install pinned compiler/runtime dependencies without changing the Python spike.
- [ ] Write a failing test for creating a space and rejecting a commit with a wrong predecessor.
- [ ] Run the focused test and confirm it fails because the contract artifact is absent.
- [ ] Implement the minimum registry and mock authorizer needed by the failing test.
- [ ] Run the focused test and confirm it passes.

### Task 2: Prove exact ERC-8350 transition conformance

**Files:**
- Modify: `evm/test/memory_lineage_registry.test.mjs`
- Create: `evm/test_vectors/erc8350_conformance.json`
- Create: `evm/artifacts/contract_conformance.json`

- [ ] Add a failing test for the published space ID, typehashes, transition ID, and next state root.
- [ ] Run it and record the mismatch before adding compatibility code.
- [ ] Implement the Solidity encoding from the published field order, not copied output constants.
- [ ] Run the test and compare the EVM result with the independently computed expected values.
- [ ] Export ABI, bytecode hash, chain ID, block numbers, gas, and exact input fields.

### Task 3: Add authorization and mutation corpus

**Files:**
- Modify: `evm/test/memory_lineage_registry.test.mjs`
- Create: `evm/mutations.json`
- Create: `evm/artifacts/mutation_matrix.json`

- [ ] Add failing tests for invalid EOA signature, wrong authorizer, ERC-1271 rejection, wrong chain/domain, sequence gap, rollback, duplicate, profile, locator, provenance, delta, root, replay, and uninitialized space.
- [ ] Run the tests to confirm each missing guard fails for the expected reason.
- [ ] Implement fail-closed checks and typed-data domain separation.
- [ ] Add at least 20 named mutations, including payload/locator tamper and missing witness cases.
- [ ] Run the full matrix and require every mutation to reject or return `UNVERIFIED`.

### Task 4: Build an independent replay verifier

**Files:**
- Create: `spike/memory_lineage_independent.py`
- Create: `spike/test_memory_lineage_independent.py`
- Create: `spike/run_memory_lineage_evm_audit.py`
- Modify: `spike/README.md`

- [ ] Write a failing test that reads only exported JSON and reconstructs the final root and verdict.
- [ ] Run it and confirm it fails because the independent verifier does not exist.
- [ ] Implement a separate keccak/ABI/evidence parser with different function names and no import from `memory_lineage.py`.
- [ ] Re-run the same valid history and all mutations through the second verifier.
- [ ] Require zero verdict mismatches and reject tampered evidence before score aggregation.

### Task 5: Collect local EVM evidence and score gates

**Files:**
- Create: `evm/artifacts/local_evm_summary.json`
- Create: `AUDIT_MEMORYLINEAGE_ACTUAL_2026-09-08.md`
- Modify: `AUDIT_LOOP_4_7_2026-09-08.md`
- Modify: `AUDIT_KLAIM_VS_AKTUAL_2026-09-08.md`

- [ ] Run Solidity compilation, EVM tests, mutation matrix, independent replay, and existing Python suite.
- [ ] Record compiler version, bytecode hash, gas/latency, test count, mutation count, and replay mismatches.
- [ ] Recalculate four dimensions from evidence only and keep testnet/user gates explicitly separate.
- [ ] Do not promote the score to 4.7 while live/testnet and user-impact evidence remain absent.

### Task 6: External live-flow and user validation gate

**Files:**
- Modify: `AUDIT_MEMORYLINEAGE_ACTUAL_2026-09-08.md`
- Modify: `AUDIT_LOOP_4_7_2026-09-08.md`

- [ ] Deploy the exact verified bytecode to an authorized public testnet using an approved ephemeral signer or user-provided RPC/signing authority.
- [ ] Re-read the transition and rejection transactions from a second RPC/indexer and attach transaction links.
- [ ] Have two independent developers run the verifier and record comprehension, false rejects, replay latency, gas, and mismatch results.
- [ ] Recalculate actual score only if every required gate has a stored artifact; otherwise preserve the highest verified score and state the missing external gate.
