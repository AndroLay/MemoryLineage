# MemoryLineage Inspector Product Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Ship a functional Inspector that demonstrates a real SQLite Silent Rollback fixture, runs the existing EthereumJS contract simulation, exports portable evidence, and verifies that evidence independently.

**Architecture:** A Next.js workspace app reads curated repository evidence on the server, uses viem for an optional live Sepolia head read, and calls a Node local-EVM simulation route for the hero attack. A portable JSON schema is verified both in TypeScript and by a separate Python CLI.

**Tech Stack:** Next.js, React, TypeScript, viem, Zod, Tailwind CSS, shadcn-style local primitives, SQLite via Python standard library, EthereumJS VM, Solidity, Python unittest.

**Spec:** `PRODUCT.md` and `DESIGN.md`

## Global Constraints

- Do not rewrite the Solidity/EthereumJS core or migrate to Foundry.
- Do not use mock values where curated evidence or the local simulation can be used.
- Keep raw fixture memory off-chain and label the fixture as synthetic demonstration data.
- Do not claim semantic memory safety, causal action proof, or final ERC-8350 compliance.
- Preserve keyboard operation, visible focus, responsive layout, reduced motion, and explicit loading/error/success states.
- Keep the independent Python verifier separate from the JavaScript/EVM implementation.

---

### Task 1: Correct claims and establish portable evidence

**Files:**
- Modify: `README.md`
- Modify: `docs/testing.md`
- Modify: `scripts/check-public-package.sh`
- Modify: `package.json`
- Create: `evidence/schemas/evidence.schema.json`
- Create: `verifier/python/memory_lineage/portable.py`
- Create: `verifier/verify.py`
- Create: `verifier/python/tests/test_portable.py`

- [x] Update Python test wording so research-spike tests are not described as MemoryLineage verifier tests.
- [x] Add a release-mode package check that rejects generated contents.
- [x] Define portable evidence invariants and implement the Python CLI.
- [x] Test valid, missing predecessor, malformed schema, privacy leak, and head mismatch cases.

### Task 2: Build the Silent Rollback fixture

**Files:**
- Create: `fixtures/silent-rollback/create_fixture.py`
- Create: `fixtures/silent-rollback/README.md`
- Create: `fixtures/silent-rollback/manifest.json`
- Create: `fixtures/silent-rollback/snapshot-17.db`
- Create: `fixtures/silent-rollback/snapshot-18.db`
- Create: `fixtures/silent-rollback/snapshot-19.db`

- [x] Create deterministic SQLite snapshots with the three private memory states.
- [x] Keep raw values inside the fixture and expose only state labels/commitments to the Inspector.
- [x] Add a regeneration command and a check that each snapshot contains exactly one expected state.

### Task 3: Add the Inspector workspace and server data boundary

**Files:**
- Modify: `package.json`, `package-lock.json`
- Create: `apps/inspector/package.json`
- Create: `apps/inspector/tsconfig.json`
- Create: `apps/inspector/next.config.ts`
- Create: `apps/inspector/postcss.config.mjs`
- Create: `apps/inspector/app/layout.tsx`
- Create: `apps/inspector/app/page.tsx`
- Create: `apps/inspector/app/globals.css`
- Create: `apps/inspector/src/lib/server-data.ts`
- Create: `apps/inspector/src/lib/types.ts`

- [x] Add the workspace dependency boundary and root `dev`, `build:inspector`, and `test:inspector` scripts.
- [x] Load local and Sepolia evidence from repository paths on the server.
- [x] Keep the page data serializable and make the app buildable without a wallet or private key.

### Task 4: Implement the four Inspector surfaces

**Files:**
- Create: `apps/inspector/app/components/inspector-shell.tsx`
- Create: `apps/inspector/app/components/ui/button.tsx`
- Create: `apps/inspector/app/components/ui/status-badge.tsx`
- Create: `apps/inspector/app/components/ui/cue-rail.tsx`
- Create: `apps/inspector/src/lib/portable-evidence.ts`
- Create: `apps/inspector/src/lib/chain.ts`

- [x] Implement Inspect with current head, authority, network, privacy boundary, and evidence provenance.
- [x] Implement History with actual transition evidence and state labels from the SQLite fixture.
- [x] Implement Tampering Lab with a real Silent Rollback action and corpus-backed mutation rows.
- [x] Implement Verify with export, import, client-side structural validation, and one-field tamper demonstration.
- [x] Add authority history as a clearly labeled current configuration plus tested rotation evidence.

### Task 5: Connect the local contract simulation and live Sepolia read

**Files:**
- Create: `evm/scripts/simulate_silent_rollback.mjs`
- Create: `apps/inspector/app/api/simulate/rollback/route.ts`
- Modify: `apps/inspector/src/lib/chain.ts`

- [x] Commit canonical transitions in the existing EthereumJS harness.
- [x] Attempt a correctly sequenced transition with a stale predecessor and return `BAD_PREVIOUS_STATE`.
- [x] Read the published Sepolia `head` and `spaceAuthorization` with viem when reachable.
- [x] Return explicit fallback status when RPC access fails.

### Task 6: Verify product boundary and rendered behavior

**Files:**
- Modify: `scripts/verify-local.sh`
- Modify: `.github/workflows/verify.yml`
- Modify: `README.md`
- Create: `docs/product/inspector-demo.md`
- Create: `.impeccable/review/desktop.png`
- Create: `.impeccable/review/mobile.png`

- [x] Add fixture generation, portable verifier tests, Inspector typecheck, and Inspector build to local verification.
- [x] Run the Impeccable detector once over the changed UI targets.
- [x] Run desktop and mobile render checks with installed headless Chromium; API and CLI checks cover Silent Rollback and evidence verification.
- [x] Remove generated evidence from the release package and run the strict release check.
