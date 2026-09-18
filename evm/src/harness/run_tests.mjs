import fs from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import { createCustomCommon, Hardfork, Mainnet } from '@ethereumjs/common';
import { createLegacyTx } from '@ethereumjs/tx';
import { createVM, runTx } from '@ethereumjs/vm';
import {
  bytesToHex,
  createAccount,
  createAddressFromPrivateKey,
  createAddressFromString,
  hexToBytes,
} from '@ethereumjs/util';
import {
  AbiCoder,
  Interface,
  Wallet,
  keccak256,
  toUtf8Bytes,
} from 'ethers';
import solc from 'solc';

const HERE = path.dirname(fileURLToPath(import.meta.url));
const REPOSITORY_ROOT = path.resolve(HERE, '../../..');
const CONTRACTS_ROOT = path.join(REPOSITORY_ROOT, 'contracts', 'solidity');
const abiCoder = AbiCoder.defaultAbiCoder();
const errorInterface = new Interface([
  'error Error(string)',
  'error Panic(uint256)',
]);

const CHAIN_ID = 31337n;
const GAS_LIMIT = 12_000_000n;
const GAS_PRICE = 10n;
const ZERO_ROOT = `0x${'00'.repeat(32)}`;
const DEPLOYER_KEY = `0x${'11'.repeat(32)}`;
const RELAYER_KEY = `0x${'22'.repeat(32)}`;
const SIGNER_KEY = `0x${'33'.repeat(32)}`;

// The vector type string is kept in one place so the JS oracle is visibly
// independent from the Solidity artifact. The Python probe has its own copy.
const ERC8350_EXPERIENCE_DELTA_TYPEHASH = keccak256(
  toUtf8Bytes(
    'ExperienceDelta(bytes32 spaceId,uint64 sequence,bytes32 prevStateRoot,bytes32 deltaCommitment,bytes32 provenanceCommitment,bytes32 profileId,bytes32 locatorCommitment)',
  ),
);
const MEMORY_STATE_TYPEHASH = keccak256(
  toUtf8Bytes('MemoryState(bytes32 prevStateRoot,bytes32 transitionId)'),
);
const MEMORY_SPACE_TYPEHASH = keccak256(
  toUtf8Bytes('MemorySpace(address initialController,bytes32 salt)'),
);

const EXPERIENCE_DELTA_TYPES = {
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

const SPACE_REGISTRATION_TYPES = {
  SpaceRegistration: [
    { name: 'spaceId', type: 'bytes32' },
    { name: 'controller', type: 'address' },
    { name: 'authorizer', type: 'address' },
  ],
};

const SPACE_AUTHORIZATION_TYPES = {
  SpaceAuthorization: [
    { name: 'spaceId', type: 'bytes32' },
    { name: 'newController', type: 'address' },
    { name: 'newAuthorizer', type: 'address' },
    { name: 'nonce', type: 'uint64' },
  ],
};

function readSource(fileName) {
  return fs.readFile(path.join(CONTRACTS_ROOT, fileName), 'utf8');
}

async function compileContracts() {
  const input = {
    language: 'Solidity',
    sources: {
      'MemoryLineageRegistry.sol': { content: await readSource('MemoryLineageRegistry.sol') },
      'Mock1271Authorizer.sol': { content: await readSource('Mock1271Authorizer.sol') },
    },
    settings: {
      optimizer: { enabled: true, runs: 200 },
      evmVersion: 'shanghai',
      outputSelection: { '*': { '*': ['abi', 'evm.bytecode.object', 'metadata'] } },
    },
  };
  const output = JSON.parse(solc.compile(JSON.stringify(input)));
  const errors = (output.errors ?? []).filter((entry) => entry.severity === 'error');
  if (errors.length > 0) {
    throw new Error(errors.map((entry) => entry.formattedMessage).join('\n'));
  }
  const registry = output.contracts['MemoryLineageRegistry.sol'].MemoryLineageRegistry;
  const mock1271 = output.contracts['Mock1271Authorizer.sol'].Mock1271Authorizer;
  return {
    compilerVersion: solc.version(),
    registry,
    mock1271,
  };
}

function abiWordEncode(types, values) {
  return abiCoder.encode(types, values);
}

function commitment(value, salt) {
  return keccak256(toUtf8Bytes(`${salt}${value}`));
}

function computeSpaceId(initialController, salt) {
  return keccak256(
    abiWordEncode(['bytes32', 'address', 'bytes32'], [MEMORY_SPACE_TYPEHASH, initialController, salt]),
  );
}

function computeTransitionId(transition) {
  return keccak256(
    abiWordEncode(
      ['bytes32', 'bytes32', 'uint64', 'bytes32', 'bytes32', 'bytes32', 'bytes32', 'bytes32'],
      [
        ERC8350_EXPERIENCE_DELTA_TYPEHASH,
        transition.spaceId,
        transition.sequence,
        transition.prevStateRoot,
        transition.deltaCommitment,
        transition.provenanceCommitment,
        transition.profileId,
        transition.locatorCommitment,
      ],
    ),
  );
}

function computeNextStateRoot(previousRoot, transitionId) {
  return keccak256(abiWordEncode(['bytes32', 'bytes32', 'bytes32'], [MEMORY_STATE_TYPEHASH, previousRoot, transitionId]));
}

function buildTransition({
  spaceId,
  sequence,
  prevStateRoot,
  profileId,
  payload = 'memory delta',
  provenance = 'agent://local/trace/1',
  locator = 'cid://memory/1',
  deltaSalt = 'delta-salt',
  provenanceSalt = 'provenance-salt',
  locatorSalt = 'locator-salt',
}) {
  const transition = {
    spaceId,
    sequence,
    prevStateRoot,
    deltaCommitment: commitment(payload, deltaSalt),
    provenanceCommitment: commitment(provenance, provenanceSalt),
    profileId,
    locatorCommitment: commitment(locator, locatorSalt),
  };
  transition.transitionId = computeTransitionId(transition);
  transition.nextStateRoot = computeNextStateRoot(prevStateRoot, transition.transitionId);
  transition.nonce = BigInt(sequence);
  return {
    ...transition,
    payload,
    provenance,
    locator,
    deltaSalt,
    provenanceSalt,
    locatorSalt,
  };
}

function transitionTuple(transition) {
  return [
    transition.spaceId,
    transition.sequence,
    transition.prevStateRoot,
    transition.deltaCommitment,
    transition.provenanceCommitment,
    transition.profileId,
    transition.locatorCommitment,
  ];
}

function asHex(value) {
  return typeof value === 'string' ? value : bytesToHex(value);
}

function revertReason(result) {
  const raw = asHex(result.execResult.returnValue ?? new Uint8Array());
  if (raw === '0x') return result.execResult.exceptionError?.error ?? 'EVM_REVERT';
  try {
    const parsed = errorInterface.parseError(raw);
    if (parsed?.name === 'Error') return parsed.args[0];
    if (parsed?.name === 'Panic') return `PANIC_${parsed.args[0].toString()}`;
  } catch {
    // Keep the raw return bytes in the thrown error for diagnostics.
  }
  return `EVM_REVERT_${raw}`;
}

export class RegistryHarness {
  constructor({ vm, common, artifacts, deployerWallet, relayerWallet, signerWallet, registryAddress, registryInterface, registryBytecode }) {
    this.vm = vm;
    this.common = common;
    this.artifacts = artifacts;
    this.deployerWallet = deployerWallet;
    this.relayerWallet = relayerWallet;
    this.signerWallet = signerWallet;
    this.registryAddress = registryAddress;
    this.registryInterface = registryInterface;
    this.registryBytecode = registryBytecode;
    this.spaceSalt = `0x${'dd'.repeat(32)}`;
    this.spaceId = computeSpaceId(this.signerWallet.address, this.spaceSalt);
    this.profileId = keccak256(toUtf8Bytes('profile/local/1'));
  }

  async send({ fromKey = DEPLOYER_KEY, to, data = '0x', value = 0n, gasLimit = GAS_LIMIT }) {
    const privateKey = hexToBytes(fromKey);
    const sender = createAddressFromPrivateKey(privateKey);
    const account = await this.vm.stateManager.getAccount(sender);
    const nonce = account?.nonce ?? 0n;
    const txData = {
      nonce,
      gasLimit,
      gasPrice: GAS_PRICE,
      value,
      data: hexToBytes(data),
    };
    if (to !== undefined && to !== null) txData.to = createAddressFromString(to);
    const tx = createLegacyTx(txData, { common: this.common }).sign(privateKey);
    const result = await runTx(this.vm, { tx, skipBlockGasLimitValidation: true });
    if (result.execResult.exceptionError !== undefined) {
      throw new Error(revertReason(result));
    }
    return result;
  }

  async call({ to = this.registryAddress, data = '0x', caller = this.deployerWallet.address }) {
    const result = await this.vm.evm.runCall({
      to: createAddressFromString(to),
      caller: createAddressFromString(caller),
      data: hexToBytes(data),
      gasLimit: GAS_LIMIT,
      value: 0n,
    });
    if (result.execResult.exceptionError !== undefined) throw new Error(revertReason(result));
    return asHex(result.execResult.returnValue);
  }

  domain() {
    return {
      name: 'AgentMemoryState',
      version: '1',
      chainId: CHAIN_ID,
      verifyingContract: this.registryAddress,
    };
  }

  async signTransition(transition, wallet = this.signerWallet) {
    const signature = await wallet.signTypedData(this.domain(), EXPERIENCE_DELTA_TYPES, {
      ...transition,
      sequence: BigInt(transition.sequence),
    });
    return signature;
  }

  async signRegistration({ spaceId = this.spaceId, controller = this.signerWallet.address, authorizer = this.signerWallet.address } = {}, wallet = this.signerWallet) {
    return wallet.signTypedData(this.domain(), SPACE_REGISTRATION_TYPES, { spaceId, controller, authorizer });
  }

  async signAuthorization({ spaceId = this.spaceId, newController, newAuthorizer, nonce }, wallet = this.signerWallet) {
    return wallet.signTypedData(this.domain(), SPACE_AUTHORIZATION_TYPES, {
      spaceId,
      newController,
      newAuthorizer,
      nonce: BigInt(nonce),
    });
  }

  async createSpace({ spaceId = this.spaceId, controller = this.signerWallet.address, authorizer = this.signerWallet.address, salt = this.spaceSalt, signature } = {}) {
    const controllerSignature = signature ?? (await this.signRegistration({ spaceId, controller, authorizer }));
    const data = this.registryInterface.encodeFunctionData('registerSpace', [spaceId, controller, authorizer, salt, controllerSignature]);
    return this.send({ to: this.registryAddress, data, fromKey: DEPLOYER_KEY });
  }

  async updateAuthorization({ spaceId = this.spaceId, newController, newAuthorizer, nonce = 1, signature } = {}) {
    const controllerSignature = signature ?? (await this.signAuthorization({ spaceId, newController, newAuthorizer, nonce }));
    const data = this.registryInterface.encodeFunctionData('updateSpaceAuthorization', [spaceId, newController, newAuthorizer, controllerSignature]);
    return this.send({ to: this.registryAddress, data, fromKey: RELAYER_KEY });
  }

  async deploy1271Authorizer(owner = this.signerWallet.address) {
    const artifact = this.artifacts.mock1271;
    const constructorInterface = new Interface(artifact.abi);
    const constructorData = constructorInterface.encodeDeploy([owner]);
    const bytecode = `0x${artifact.evm.bytecode.object}${constructorData.slice(2)}`;
    const result = await this.send({ data: bytecode, fromKey: DEPLOYER_KEY });
    if (!result.createdAddress) throw new Error('ERC1271 deployment did not return an address');
    return result.createdAddress.toString();
  }

  async set1271Accept(authorizerAddress, accept) {
    const iface = new Interface(this.artifacts.mock1271.abi);
    const data = iface.encodeFunctionData('setAccept', [accept]);
    return this.send({
      to: authorizerAddress,
      data,
      fromKey: SIGNER_KEY,
    });
  }

  async nextTransition({
    spaceId = this.spaceId,
    profileId = this.profileId,
    payload = 'memory delta next',
    provenance = 'agent://local/trace/next',
    locator = 'cid://memory/next',
    sequence,
    prevStateRoot,
  } = {}) {
    const head = await this.readHead(spaceId);
    return buildTransition({
      spaceId,
      sequence: sequence ?? head.sequence + 1,
      prevStateRoot: prevStateRoot ?? head.root,
      profileId,
      payload,
      provenance,
      locator,
      deltaSalt: 'delta-salt-next',
      provenanceSalt: 'provenance-salt-next',
      locatorSalt: 'locator-salt-next',
    });
  }

  async commit(transition, { signature, fromKey = RELAYER_KEY } = {}) {
    const signed = signature ?? (await this.signTransition(transition));
    const data = this.registryInterface.encodeFunctionData('commitTransition', [transitionTuple(transition), signed]);
    return this.send({ to: this.registryAddress, data, fromKey });
  }

  async readHead(spaceId = this.spaceId) {
    const data = this.registryInterface.encodeFunctionData('head', [spaceId]);
    const raw = await this.call({ data });
    const decoded = this.registryInterface.decodeFunctionResult('head', raw);
    return { transitionId: decoded[0], root: decoded[1], sequence: Number(decoded[2]) };
  }

  async commitValidFirstTransition() {
    const transition = buildTransition({
      spaceId: this.spaceId,
      sequence: 1,
      prevStateRoot: ZERO_ROOT,
      profileId: this.profileId,
    });
    const signature = await this.signTransition(transition);
    const result = await this.commit(transition, { signature });
    const head = await this.readHead();
    return {
      verdict: 'PASS',
      sequence: head.sequence,
      root: head.root,
      transition,
      gasUsed: result.totalGasSpent.toString(),
      logs: result.receipt.logs?.length ?? 0,
    };
  }

  async commitWithWrongPredecessor() {
    const transition = buildTransition({
      spaceId: this.spaceId,
      sequence: 2,
      prevStateRoot: ZERO_ROOT,
      profileId: this.profileId,
    });
    const signature = await this.signTransition(transition);
    await this.commit(transition, { signature });
  }
}

export async function createRegistryHarness() {
  const artifacts = await compileContracts();
  const common = createCustomCommon({ chainId: Number(CHAIN_ID) }, Mainnet, {
    hardfork: Hardfork.Shanghai,
  });
  const vm = await createVM({ common, activatePrecompiles: true });
  const deployerWallet = new Wallet(DEPLOYER_KEY);
  const relayerWallet = new Wallet(RELAYER_KEY);
  const signerWallet = new Wallet(SIGNER_KEY);
  for (const wallet of [deployerWallet, relayerWallet, signerWallet]) {
    const address = createAddressFromString(wallet.address);
    await vm.stateManager.putAccount(
      address,
      createAccount({ nonce: 0n, balance: 10n ** 24n }),
    );
  }

  const registryInterface = new Interface(artifacts.registry.abi);
  const registryBytecode = `0x${artifacts.registry.evm.bytecode.object}`;
  const deployment = createLegacyTx(
    {
      nonce: 0n,
      gasLimit: GAS_LIMIT,
      gasPrice: GAS_PRICE,
      value: 0n,
      data: hexToBytes(registryBytecode),
    },
    { common },
  ).sign(hexToBytes(DEPLOYER_KEY));
  const deploymentResult = await runTx(vm, { tx: deployment, skipBlockGasLimitValidation: true });
  if (deploymentResult.execResult.exceptionError !== undefined || !deploymentResult.createdAddress) {
    throw new Error(`registry deployment failed: ${revertReason(deploymentResult)}`);
  }
  const registryAddress = deploymentResult.createdAddress.toString();
  const harness = new RegistryHarness({
    vm,
    common,
    artifacts,
    deployerWallet,
    relayerWallet,
    signerWallet,
    registryAddress,
    registryInterface,
    registryBytecode,
  });
  await harness.createSpace();
  return harness;
}

export {
  CHAIN_ID,
  DEPLOYER_KEY,
  ERC8350_EXPERIENCE_DELTA_TYPEHASH,
  MEMORY_SPACE_TYPEHASH,
  MEMORY_STATE_TYPEHASH,
  RELAYER_KEY,
  SIGNER_KEY,
  EXPERIENCE_DELTA_TYPES,
  SPACE_AUTHORIZATION_TYPES,
  SPACE_REGISTRATION_TYPES,
  ZERO_ROOT,
  buildTransition,
  commitment,
  compileContracts,
  computeNextStateRoot,
  computeSpaceId,
  computeTransitionId,
  transitionTuple,
};
