# Third-party notices

MemoryLineage uses the following open-source components in its local EVM
verification toolchain:

- `ethers` 6.17.0 — MIT License.
- `@ethereumjs/common`, `@ethereumjs/tx`, `@ethereumjs/util`, and
  `@ethereumjs/vm` 10.1.3 — MPL-2.0.
- `solc` 0.8.36 — the npm wrapper is MIT licensed; the Solidity compiler
  distribution carries its own upstream license and notices.

The exact dependency metadata is recorded in `package-lock.json`. The
ERC-8350 draft and its published vectors are prior standards work and are
referenced as an explicitly pinned compatibility target; MemoryLineage does
not claim authorship of that standard.
