# Evidence-first Repository Architecture

**Status:** Approved for implementation on 2026-09-18

## Goal

Turn the extracted 3rd-Web-Hack workspace into a small, source-first repository that makes MemoryLineage's protocol, independent verifier, evidence, tests, and future Inspector product easy to understand and reproduce.

## Design decision

Use the useful principles from the RevenueCat Shipaton repository—canonical root documentation, explicit boundaries, reproducible verification, and public-package checks—without copying its mobile, API, worker, billing, Gradle, or cloud-platform breadth.

The repository remains deliberately split between a JavaScript/EVM implementation and a Python verifier. The separate implementations are evidence of independent replay and must not be collapsed into one shared runtime package.

## Boundaries

- `contracts/` owns Solidity sources, conformance vectors, and the adversarial mutation corpus.
- `evm/` owns the local EVM harness, EVM tests, deployment, and Sepolia readback scripts.
- `verifier/python/` owns the independent replay model and its tests.
- `evidence/` owns curated and generated verification outputs; it never becomes an input to protocol code.
- `docs/` owns product, architecture, testing, submission, and research documentation.
- `research/spikes/` owns abandoned or exploratory executable spikes.
- `scripts/` owns repository-wide verification and packaging checks.
- `apps/inspector/` is a future product boundary and is not scaffolded during this structural cleanup.

## Target layout

```text
README.md
LICENSE
THIRD_PARTY_NOTICES.md
.gitignore
.editorconfig
package.json
package-lock.json
contracts/{solidity,vectors,mutations}
evm/{src,test,scripts}
verifier/python/{memory_lineage,tests}
evidence/{local,sepolia,conformance,schemas}
docs/{architecture,product,research,submission,testing.md}
research/spikes
scripts/{verify-local.sh,check-architecture-boundaries.sh,check-public-package.sh}
```

## Canonical commands

- `npm ci` installs the JavaScript/EVM dependencies from the repository root.
- `npm test` runs the EVM and Python test suites.
- `npm run verify` runs the complete local verification gate.
- `npm run check:boundaries` checks that executable code does not depend on research or generated evidence.
- `npm run check:package` checks that the public package contains the required entry points and no generated dependency trees or credential-shaped files.

## Preservation rules

- Move source and documentation; do not rewrite protocol behavior during the structural migration.
- Preserve existing evidence and research history, while labeling machine-generated artifacts and stale/abandoned spikes.
- Do not claim ERC-8350 final compliance, semantic memory safety, or a 4.7 judge score.
- Do not initialize or rewrite Git history automatically; the supplied workspace has no usable Git history.
- Generated `node_modules`, Python caches, and compiler output must be ignored and excluded from the public package after rebuildability is established.

## Acceptance criteria

1. A reader can understand the product, proof boundary, and local commands from the root README.
2. Solidity, EVM execution, Python replay, evidence, research, and scripts have separate paths.
3. The existing 8 EVM tests, 16 core Python tests, and 12 archived research
   tests still pass.
4. `npm run verify` passes from the repository root.
5. The public-package check rejects generated dependency/cache directories and absolute local paths.
6. No protocol behavior or existing evidence is silently discarded.
