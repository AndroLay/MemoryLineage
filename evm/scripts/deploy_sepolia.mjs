import fs from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import {
  Contract,
  ContractFactory,
  JsonRpcProvider,
  Wallet,
  keccak256,
  toUtf8Bytes,
} from 'ethers';
import {
  buildTransition,
  compileContracts,
  computeSpaceId,
} from '../src/harness/run_tests.mjs';

const HERE = path.dirname(fileURLToPath(import.meta.url));
const ROOT = path.resolve(HERE, '../..');
const RPC_URL = process.env.SEPOLIA_RPC_URL ?? 'https://ethereum-sepolia-rpc.publicnode.com';
const SECOND_RPC_URL = process.env.SEPOLIA_RPC_URL_SECOND ?? '';
const PRIVATE_KEY = process.env.DEPLOYER_PRIVATE_KEY;
const CHAIN_ID = 11155111n;

if (process.argv.includes('--dry-run')) {
  console.log(JSON.stringify({
    status: 'DRY_RUN',
    chainId: CHAIN_ID.toString(),
    rpcUrl: RPC_URL,
    mutation: false,
    requiredEnv: 'DEPLOYER_PRIVATE_KEY',
  }, null, 2));
  process.exit(0);
}

if (!process.argv.includes('--confirm-public-testnet')) {
  throw new Error('explicit --confirm-public-testnet is required before sending Sepolia transactions');
}

if (!PRIVATE_KEY) {
  throw new Error('DEPLOYER_PRIVATE_KEY is required; no key is read from files or printed by this script');
}

const artifacts = await compileContracts();
const provider = new JsonRpcProvider(RPC_URL, {
  name: 'sepolia',
  chainId: Number(CHAIN_ID),
}, { staticNetwork: true });
const network = await provider.getNetwork();
if (network.chainId !== CHAIN_ID) throw new Error(`wrong chain: expected ${CHAIN_ID}, got ${network.chainId}`);
const wallet = new Wallet(PRIVATE_KEY, provider);
const balance = await provider.getBalance(wallet.address);
if (balance === 0n) throw new Error(`deployer ${wallet.address} has zero Sepolia ETH; fund it before deployment`);

const bytecode = `0x${artifacts.registry.evm.bytecode.object}`;
const factory = new ContractFactory(artifacts.registry.abi, bytecode, wallet);
const deployment = await factory.deploy();
const deploymentReceipt = await deployment.deploymentTransaction().wait();
const registryAddress = await deployment.getAddress();
const deployedCode = await provider.getCode(registryAddress);
if (deployedCode === '0x') throw new Error('deployment receipt exists but deployed code is empty');

const registry = new Contract(registryAddress, artifacts.registry.abi, wallet);
const salt = keccak256(toUtf8Bytes(`memory-lineage-sepolia-${deploymentReceipt.hash}`));
const spaceId = computeSpaceId(wallet.address, salt);
const registerTx = await registry.registerSpace(spaceId, wallet.address, wallet.address, salt, '0x');
const registerReceipt = await registerTx.wait();
const profileId = keccak256(toUtf8Bytes('agent-memory-state/v1'));
const transition = buildTransition({
  spaceId,
  sequence: 1,
  prevStateRoot: `0x${'00'.repeat(32)}`,
  profileId,
  payload: 'private witness omitted',
  provenance: 'private provenance omitted',
  locator: 'private locator omitted',
  deltaSalt: `delta-${deploymentReceipt.hash}`,
  provenanceSalt: `provenance-${deploymentReceipt.hash}`,
  locatorSalt: `locator-${deploymentReceipt.hash}`,
});
const domain = {
  name: 'AgentMemoryState',
  version: '1',
  chainId: CHAIN_ID,
  verifyingContract: registryAddress,
};
const types = {
  ExperienceDelta: [
    { name: 'spaceId', type: 'bytes32' },
    { name: 'sequence', type: 'uint64' },
    { name: 'prevStateRoot', type: 'bytes32' },
    { name: 'deltaCommitment', type: 'bytes32' },
    { name: 'provenanceCommitment', type: 'bytes32' },
    { name: 'profileId', type: 'bytes32' },
    { name: 'locatorCommitment', type: 'bytes32' },
  ],
};
const delta = {
  spaceId: transition.spaceId,
  sequence: BigInt(transition.sequence),
  prevStateRoot: transition.prevStateRoot,
  deltaCommitment: transition.deltaCommitment,
  provenanceCommitment: transition.provenanceCommitment,
  profileId: transition.profileId,
  locatorCommitment: transition.locatorCommitment,
};
const authorizerSignature = await wallet.signTypedData(domain, types, delta);
const commitTx = await registry.commitTransition(delta, authorizerSignature);
const commitReceipt = await commitTx.wait();

const rejectedTransition = buildTransition({
  spaceId,
  sequence: 3,
  prevStateRoot: transition.nextStateRoot,
  profileId,
  payload: 'expected rejection witness omitted',
  provenance: 'expected rejection provenance omitted',
  locator: 'expected rejection locator omitted',
  deltaSalt: `rejected-delta-${deploymentReceipt.hash}`,
  provenanceSalt: `rejected-provenance-${deploymentReceipt.hash}`,
  locatorSalt: `rejected-locator-${deploymentReceipt.hash}`,
});
const rejectedDelta = {
  spaceId: rejectedTransition.spaceId,
  sequence: BigInt(rejectedTransition.sequence),
  prevStateRoot: rejectedTransition.prevStateRoot,
  deltaCommitment: rejectedTransition.deltaCommitment,
  provenanceCommitment: rejectedTransition.provenanceCommitment,
  profileId: rejectedTransition.profileId,
  locatorCommitment: rejectedTransition.locatorCommitment,
};
const rejectedSignature = await wallet.signTypedData(domain, types, rejectedDelta);
const rejectedData = registry.interface.encodeFunctionData('commitTransition', [
  rejectedDelta,
  rejectedSignature,
]);
const rejectedTx = await wallet.sendTransaction({
  to: registryAddress,
  data: rejectedData,
  gasLimit: 300000n,
});
let rejectedReceipt;
let rejectedReason = null;
try {
  rejectedReceipt = await rejectedTx.wait();
} catch (error) {
  rejectedReason = error.shortMessage ?? 'expected transaction reverted';
  rejectedReceipt = error.receipt ?? await provider.getTransactionReceipt(rejectedTx.hash);
}
if (!rejectedReceipt || Number(rejectedReceipt.status) !== 0) {
  throw new Error('expected invalid sequence transaction to be mined with status 0');
}
const head = await registry.head(spaceId);
const result = {
  evidenceType: 'workspace_owned_public_sepolia_deployment',
  chainId: CHAIN_ID.toString(),
  rpcUrl: RPC_URL,
  secondRpcUrlConfigured: Boolean(SECOND_RPC_URL),
  registryAddress,
  deployerAddress: wallet.address,
  deploymentTxHash: deploymentReceipt.hash,
  registerTxHash: registerReceipt.hash,
  commitTxHash: commitReceipt.hash,
  expectedRejectedTxHash: rejectedTx.hash,
  expectedRejectedReceiptStatus: Number(rejectedReceipt.status),
  expectedRejectedReason: rejectedReason,
  bytecodeKeccak256: keccak256(bytecode),
  deployedCodeKeccak256: keccak256(deployedCode),
  spaceId,
  transitionId: transition.transitionId,
  expectedNextStateRoot: transition.nextStateRoot,
  onchainHead: {
    transitionId: head[0],
    stateRoot: head[1],
    sequence: Number(head[2]),
  },
  rawPayloadStored: false,
  note: 'Run evm/reread_sepolia.mjs through a second RPC before treating this as the live gate.',
};
await fs.mkdir(path.join(ROOT, 'evidence', 'sepolia'), { recursive: true });
await fs.writeFile(path.join(ROOT, 'evidence', 'sepolia', 'sepolia_deployment.json'), JSON.stringify(result, null, 2) + '\n');
console.log(JSON.stringify(result, null, 2));
