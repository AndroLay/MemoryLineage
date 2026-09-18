import fs from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { compileContracts } from '../evm/src/harness/run_tests.mjs';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const artifacts = await compileContracts();
const output = path.join(root, 'contracts', 'artifacts');
await fs.mkdir(output, { recursive: true });

await fs.writeFile(
  path.join(output, 'memory_lineage_registry_creation.hex'),
  `0x${artifacts.registry.evm.bytecode.object}\n`,
);
await fs.writeFile(
  path.join(output, 'mock_1271_authorizer_creation.hex'),
  `0x${artifacts.mock1271.evm.bytecode.object}\n`,
);
await fs.writeFile(
  path.join(output, 'manifest.json'),
  `${JSON.stringify(
    {
      compiler: artifacts.compilerVersion,
      contracts: {
        MemoryLineageRegistry: 'memory_lineage_registry_creation.hex',
        Mock1271Authorizer: 'mock_1271_authorizer_creation.hex',
      },
      source: 'contracts/solidity',
      generatedBy: 'scripts/export-solidity-artifacts.mjs',
    },
    null,
  )}\n`,
);
