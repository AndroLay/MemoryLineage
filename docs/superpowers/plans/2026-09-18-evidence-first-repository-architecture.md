# Evidence-first Repository Architecture Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [x]`) syntax for tracking.

**Goal:** Reorganize the extracted 3rd-Web-Hack workspace into a reproducible evidence-first repository without changing MemoryLineage protocol behavior.

**Architecture:** Keep Solidity, the JavaScript local-EVM implementation, and the Python replay verifier in explicit boundaries. Archive research and exploratory spikes separately, add a root command surface and public-package checks, and reserve `apps/inspector` for the next product phase.

**Tech Stack:** Solidity 0.8.36, Node.js ESM, npm, ethers, EthereumJS VM, solc, Python 3 standard library, Bash, JSON fixtures.

**Spec:** `docs/superpowers/specs/2026-09-18-evidence-first-repository-architecture.md`

## Global Constraints

- Preserve protocol behavior and existing evidence while moving files.
- Keep the JavaScript EVM implementation and Python verifier independent.
- Do not add a UI framework or runtime dependency in this structural phase.
- Do not copy RevenueCat-specific mobile, cloud, billing, or Gradle architecture.
- Do not initialize or rewrite Git history; the workspace currently has no usable Git history.
- Do not publish `node_modules`, `__pycache__`, compiler output, secrets, or absolute local paths.

---

### Task 1: Establish root repository metadata and command surface

**Files:**
- Create: `README.md`
- Create: `LICENSE`
- Create: `THIRD_PARTY_NOTICES.md`
- Create: `.gitignore`
- Create: `.editorconfig`
- Create: `package.json`
- Move: `evm/package-lock.json` to `package-lock.json`
- Move: `evm/package.json` to `package.json` before replacing its scripts

**Interfaces:**
- Produces root commands `npm test`, `npm run verify`, `npm run check:boundaries`, and `npm run check:package`.
- `README.md` links to `docs/product/product-contract.md`, `docs/architecture/repository-structure.md`, and `docs/testing.md`.

- [x] **Step 1: Create repository metadata and ignore rules.**

  `.gitignore` must ignore `node_modules/`, `__pycache__/`, `*.pyc`, `evm/artifacts/`, `evidence/generated/`, `.env`, `.env.*`, and local absolute-path reports while allowing curated evidence under `evidence/`.

- [x] **Step 2: Add root npm scripts.**

  The root manifest must use the existing dependency versions and define:

  ```json
  "scripts": {
    "test:evm": "node --test evm/test/memory_lineage_registry.test.mjs",
    "test:python": "python3 -m unittest discover -s verifier/python -p 'test_*.py'",
    "test": "npm run test:evm && npm run test:python",
    "check:boundaries": "bash scripts/check-architecture-boundaries.sh",
    "check:package": "bash scripts/check-public-package.sh",
    "verify": "bash scripts/verify-local.sh"
  }
  ```

- [x] **Step 3: Write the root README.**

  It must lead with the Silent Rollback problem, explain what is and is not verified, show the repository map, document `npm ci && npm run verify`, and disclose the pinned ERC-8350 draft boundary without presenting the draft as an original invention.

- [x] **Step 4: Run the metadata smoke check.**

  Run: `node -e "const p=require('./package.json'); for (const s of ['test','verify','check:boundaries','check:package']) if (!p.scripts[s]) process.exit(1)"`

  Expected: exit code 0.

### Task 2: Create the canonical documentation hierarchy

**Files:**
- Create: `docs/architecture/repository-structure.md`
- Create: `docs/architecture/system-boundaries.md`
- Create: `docs/product/product-contract.md`
- Create: `docs/testing.md`
- Create: `docs/submission/claim-matrix.md`
- Create: `docs/research/README.md`
- Move: root research/audit Markdown files into `docs/research/`
- Move: `report-source.md` into `docs/research/report-source.md`

**Interfaces:**
- The README and scripts reference only these canonical docs.
- Research documents remain available but are no longer root-level product entry points.

- [x] **Step 1: Write the product contract.**

  Define MemoryLineage as an independent auditor for ordered, authorized, committed private-agent-memory history. Explicitly list semantic truth, memory safety, agent correctness, and off-chain availability as out of scope.

- [x] **Step 2: Write the repository map and boundary document.**

  Document allowed dependencies: `contracts` has no runtime dependency on `evidence`; `verifier/python` consumes vectors/evidence but not EVM modules; `evm` consumes Solidity and vectors; research is never imported by product code.

- [x] **Step 3: Write the testing and claim matrix docs.**

  Record the 8 EVM, 16 core Python, and 12 archived research baseline gates, the
  20 mutation corpus, Sepolia evidence, and safe claim wording such as
  “conforms to the pinned draft snapshot” instead of “final ERC-8350 compliant.”

- [x] **Step 4: Move existing research without deleting it.**

  Place the candidate/audit/deep-research files under `docs/research/` and add an index describing which files are historical research, implementation evidence, and stale scoring artifacts.

### Task 3: Separate protocol sources, vectors, and mutations

**Files:**
- Create: `contracts/solidity/`
- Create: `contracts/vectors/`
- Create: `contracts/mutations/`
- Move: `evm/contracts/MemoryLineageRegistry.sol` to `contracts/solidity/MemoryLineageRegistry.sol`
- Move: `evm/contracts/Mock1271Authorizer.sol` to `contracts/solidity/Mock1271Authorizer.sol`
- Move: `evm/test_vectors/erc8350_conformance.json` to `contracts/vectors/erc8350_conformance.json`
- Move: `evm/mutations.json` to `contracts/mutations/mutations.json`

**Interfaces:**
- The EVM compiler reads Solidity from `contracts/solidity/`.
- EVM tests and Python verifier read the canonical vector from `contracts/vectors/`.

- [x] **Step 1: Move the four protocol inputs.**

  Use `mv` only; do not edit contract contents during the move.

- [x] **Step 2: Update all path consumers.**

  Update `evm/src/harness/run_tests.mjs`, `evm/test/memory_lineage_registry.test.mjs`, `evm/audit_local.mjs`, and deployment/readback scripts to resolve repository root paths with `path.resolve`, never hard-coded `/home/andro/...` paths.

- [x] **Step 3: Check path completeness.**

  Run: `rg -n "evm/contracts|evm/test_vectors|evm/mutations|/home/andro" --glob '!node_modules/**' --glob '!*.md' .`

  Expected: no executable source matches.

### Task 4: Isolate the EVM harness and tests

**Files:**
- Create: `evm/src/harness/`
- Move: `evm/run_tests.mjs` to `evm/src/harness/run_tests.mjs`
- Move: `evm/test/memory_lineage_registry.test.mjs` to the same test path
- Modify: `evm/src/harness/run_tests.mjs`
- Modify: `evm/test/memory_lineage_registry.test.mjs`
- Modify: `evm/audit_local.mjs`, `evm/deploy_sepolia.mjs`, `evm/reread_sepolia.mjs`, and `evm/probe_sepolia_reference.mjs`

**Interfaces:**
- `evm/src/harness/run_tests.mjs` continues exporting `RegistryHarness`, `buildTransition`, `computeTransitionId`, `computeNextStateRoot`, `createRegistryHarness`, and the existing constants.
- The test file remains the sole Node test entrypoint.

- [x] **Step 1: Move the harness and update its repository-root calculation.**

  Derive the root from `path.resolve(HERE, '../../..')` after the move, then use `contracts/solidity` and `contracts/vectors` from that root.

- [x] **Step 2: Update test imports and vector path.**

  Import from `../src/harness/run_tests.mjs` and load `contracts/vectors/erc8350_conformance.json`.

- [x] **Step 3: Update script imports and artifact paths.**

  Keep generated outputs under `evidence/generated/` or `evidence/sepolia/` and make every script create its destination directory before writing.

- [x] **Step 4: Run the focused EVM suite.**

  Run: `npm run test:evm`

  Expected: 8 tests pass.

### Task 5: Establish the independent Python verifier boundary

**Files:**
- Create: `verifier/python/memory_lineage/`
- Create: `verifier/python/tests/`
- Move: MemoryLineage Python modules from `spike/` into `verifier/python/memory_lineage/`
- Move: MemoryLineage Python tests into `verifier/python/tests/`
- Modify: imports and artifact paths in the moved modules/tests

**Interfaces:**
- `python3 -m unittest discover -s verifier/python -p 'test_*.py'` remains the canonical Python gate.
- The verifier reads canonical vectors and evidence by path, never by importing JavaScript/EVM code.

- [x] **Step 1: Move only MemoryLineage implementation and tests.**

  Move `memory_lineage.py`, `memory_lineage_independent.py`, `memory_lineage_impact.py`, `audit_memory_lineage_gates.py`, `run_memory_lineage.py`, `run_memory_lineage_evm_audit.py`, and their three MemoryLineage test files. Keep ResolverCompat, SlippageTruth, candidate loops, and generic spike runners for Task 6.

- [x] **Step 2: Update package imports.**

  Replace `from spike...` imports with `from memory_lineage...` or package-relative imports that work when the test discovery root is `verifier/python`.

- [x] **Step 3: Update canonical fixture paths.**

  Resolve repository root from `Path(__file__).resolve()` and read `contracts/vectors/` and `evidence/` from there.

- [x] **Step 4: Run the focused Python suite.**

  Run: `npm run test:python`

  Expected: the existing 24 tests pass.

### Task 6: Archive exploratory spikes and curate evidence

**Files:**
- Create: `research/spikes/`
- Move: ResolverCompat and SlippageTruth implementation/tests into `research/spikes/`
- Move: candidate audit loop and generic spike runner into `docs/research/` or `research/spikes/` according to whether the file is executable
- Move: `evm/artifacts/` outputs into `evidence/generated/` or `evidence/sepolia/`
- Move: retained `spike/artifacts/` outputs into `evidence/generated/`
- Create: `evidence/README.md`

**Interfaces:**
- Generated evidence is labeled as generated and reproducible.
- No executable product code imports `research/` or `docs/research/`.

- [x] **Step 1: Archive executable non-MemoryLineage spikes.**

  Move `resolver_compat.py`, `slippage_truth.py`, their tests, and their runners under `research/spikes/`; preserve their README context under `docs/research/`.

- [x] **Step 2: Curate existing JSON evidence.**

  Preserve deployment, readback, local EVM, mutation, and replay outputs under named evidence directories. Keep historical score-gate artifacts but label them as internal historical evidence and remove them from product claims.

- [x] **Step 3: Add the evidence index.**

  Document each retained artifact, the command that regenerates it, and whether it is public, generated, or historical.

### Task 7: Add repository-wide architecture and public-package gates

**Files:**
- Create: `scripts/verify-local.sh`
- Create: `scripts/check-architecture-boundaries.sh`
- Create: `scripts/check-public-package.sh`
- Modify: `package.json`

**Interfaces:**
- All scripts exit nonzero on a failed required check.
- `verify-local.sh` runs tests, boundary checks, package checks, and the local EVM audit in a deterministic order.

- [x] **Step 1: Write the boundary check.**

  Fail if executable files import `research`, `docs`, or `evidence/generated`, or if source still references removed `spike` and old vector paths.

- [x] **Step 2: Write the public-package check.**

  Require `README.md`, `LICENSE`, `package.json`, `contracts/`, `verifier/`, `evidence/`, `docs/`, and `scripts/`; reject `node_modules`, `__pycache__`, `.env*`, private-key-shaped files, and `/home/andro/` in executable source.

- [x] **Step 3: Write the aggregate gate.**

  Run `npm test`, `npm run check:boundaries`, `npm run check:package`, and `node evm/audit_local.mjs`; print only concise pass/fail diagnostics.

- [x] **Step 4: Run each gate independently.**

  Run: `npm run check:boundaries`, `npm run check:package`, and `npm run verify`.

  Expected: all exit 0 after path and fixture migration.

### Task 8: Review the migrated repository and verify the public shape

**Files:**
- Modify: any path/import/doc files exposed by the verification results
- Create: `docs/architecture/migration-notes.md`

**Interfaces:**
- Migration notes identify moved paths and confirm that no protocol behavior was intentionally changed.

- [x] **Step 1: Inspect the final tree.**

  Run: `find . -maxdepth 3 -type f ...` excluding generated dependency/cache directories and confirm the target layout.

- [x] **Step 2: Search for stale public claims and absolute paths.**

  Run: `rg -n "actualScore|memory poisoning|final ERC-8350|/home/andro/|node_modules|__pycache__" README.md docs contracts verifier evm scripts`

  Expected: no unsupported claim in the root README or product docs; historical research may retain the terms with context.

- [x] **Step 3: Run the final verification gate.**

  Run: `npm run verify`

  Expected: all required checks pass.

- [x] **Step 4: Record residual limitations.**

  Document that the Inspector UI, real snapshot/restore demo, human external rerun, and final submission video are subsequent product tasks, not silently represented as complete by this cleanup.
