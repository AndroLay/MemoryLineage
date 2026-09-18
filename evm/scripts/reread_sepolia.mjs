import fs from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import { Contract, JsonRpcProvider, keccak256 } from 'ethers';
import { compileContracts } from '../src/harness/run_tests.mjs';

const HERE = path.dirname(fileURLToPath(import.meta.url));
const ROOT = path.resolve(HERE, '../..');
const deploymentPath = path.join(ROOT, 'evidence', 'sepolia', 'sepolia_deployment.json');
const deployment = JSON.parse(await fs.readFile(deploymentPath, 'utf8'));
const RPC_URL = process.env.SEPOLIA_RPC_URL_SECOND ?? 'https://public.1rpc.io/sepolia';
const CHAIN_ID = 11155111n;

const provider = new JsonRpcProvider(RPC_URL, {
  name: 'sepolia',
  chainId: Number(CHAIN_ID),
}, { staticNetwork: true });
const network = await provider.getNetwork();
if (network.chainId !== CHAIN_ID) throw new Error(`wrong chain: expected ${CHAIN_ID}, got ${network.chainId}`);

const code = await provider.getCode(deployment.registryAddress);
if (code === '0x') throw new Error('registry has no code on the second RPC');
const artifacts = await compileContracts();
const registry = new Contract(deployment.registryAddress, artifacts.registry.abi, provider);
const head = await registry.head(deployment.spaceId);

const receiptStatus = async (hash) => {
  const receipt = await provider.getTransactionReceipt(hash);
  return receipt ? Number(receipt.status) : null;
};
const statuses = {
  deployment: await receiptStatus(deployment.deploymentTxHash),
  registration: await receiptStatus(deployment.registerTxHash),
  validCommit: await receiptStatus(deployment.commitTxHash),
  expectedRejectedCommit: await receiptStatus(deployment.expectedRejectedTxHash),
};
const checks = {
  codeHashMatches: keccak256(code) === deployment.deployedCodeKeccak256,
  deploymentMined: statuses.deployment === 1,
  registrationMined: statuses.registration === 1,
  validCommitMined: statuses.validCommit === 1,
  expectedRejectedCommitMined: statuses.expectedRejectedCommit === 0,
  headMatches: (
    head[0] === deployment.transitionId
    && head[1] === deployment.expectedNextStateRoot
    && Number(head[2]) === 1
  ),
};
const result = {
  evidenceType: 'second_rpc_reread_workspace_owned_public_sepolia_deployment',
  chainId: CHAIN_ID.toString(),
  rpcUrl: RPC_URL,
  registryAddress: deployment.registryAddress,
  blockNumber: await provider.getBlockNumber(),
  deployedCodeKeccak256: keccak256(code),
  statuses,
  head: {
    transitionId: head[0],
    stateRoot: head[1],
    sequence: Number(head[2]),
  },
  checks,
  verdict: Object.values(checks).every(Boolean) ? 'PASS' : 'FAIL',
};
await fs.writeFile(path.join(ROOT, 'evidence', 'sepolia', 'sepolia_reread.json'), JSON.stringify(result, null, 2) + '\n');
console.log(JSON.stringify(result, null, 2));
if (result.verdict !== 'PASS') process.exitCode = 1;
