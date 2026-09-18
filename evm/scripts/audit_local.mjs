import fs from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import {
  EXPERIENCE_DELTA_TYPES,
  ZERO_ROOT,
  buildTransition,
  computeNextStateRoot,
  computeSpaceId,
  computeTransitionId,
  createRegistryHarness,
} from '../src/harness/run_tests.mjs';
import { keccak256, toUtf8Bytes } from 'ethers';

const HERE = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(HERE, '../..');
const vector = JSON.parse(
  await fs.readFile(path.join(root, 'contracts', 'vectors', 'erc8350_conformance.json'), 'utf8'),
);
const artifactDir = path.join(root, 'evidence', 'generated');
const localEvidenceDir = path.join(root, 'evidence', 'local');
await fs.mkdir(artifactDir, { recursive: true });
await fs.mkdir(localEvidenceDir, { recursive: true });

const asJson = (value) => JSON.stringify(value, (_, candidate) => (
  typeof candidate === 'bigint' ? candidate.toString() : candidate
), 2) + '\n';

function publicTransition(transition) {
  return {
    spaceId: transition.spaceId,
    sequence: Number(transition.sequence),
    prevStateRoot: transition.prevStateRoot,
    deltaCommitment: transition.deltaCommitment,
    provenanceCommitment: transition.provenanceCommitment,
    profileId: transition.profileId,
    locatorCommitment: transition.locatorCommitment,
    transitionId: computeTransitionId(transition),
    nextStateRoot: computeNextStateRoot(transition.prevStateRoot, computeTransitionId(transition)),
  };
}

function tuple(transition) {
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

async function contractVectorEvidence(harness) {
  const spaceId = computeSpaceId(vector.initial_controller, vector.space_salt);
  const transition = {
    spaceId,
    sequence: 1,
    prevStateRoot: ZERO_ROOT,
    deltaCommitment: vector.delta_commitment,
    provenanceCommitment: vector.provenance_commitment,
    profileId: vector.profile_id,
    locatorCommitment: vector.locator_commitment,
  };
  const transitionId = computeTransitionId(transition);
  const nextStateRoot = computeNextStateRoot(ZERO_ROOT, transitionId);
  const full = { ...transition, transitionId, nextStateRoot };
  const idRaw = await harness.call({
    data: harness.registryInterface.encodeFunctionData('hashExperienceDelta', [tuple(full)]),
  });
  const rootRaw = await harness.call({
    data: harness.registryInterface.encodeFunctionData('computeNextStateRoot', [ZERO_ROOT, transitionId]),
  });
  const contractTransitionId = harness.registryInterface.decodeFunctionResult('hashExperienceDelta', idRaw)[0];
  const contractNextRoot = harness.registryInterface.decodeFunctionResult('computeNextStateRoot', rootRaw)[0];
  const typehashes = {};
  for (const name of ['EXPERIENCE_DELTA_TYPEHASH', 'MEMORY_STATE_TYPEHASH', 'MEMORY_SPACE_TYPEHASH']) {
    const raw = await harness.call({ data: harness.registryInterface.encodeFunctionData(name) });
    typehashes[name] = harness.registryInterface.decodeFunctionResult(name, raw)[0];
  }
  return {
    source: vector.source,
    inputs: {
      initialController: vector.initial_controller,
      spaceSalt: vector.space_salt,
    },
    expected: {
      spaceId: vector.expected_space_id,
      experienceDeltaTypehash: vector.expected_experience_delta_typehash,
      memoryStateTypehash: vector.expected_memory_state_typehash,
      memorySpaceTypehash: vector.expected_memory_space_typehash,
      transitionId: vector.expected_transition_id,
      nextStateRoot: vector.expected_next_state_root,
    },
    independentlyComputed: {
      spaceId,
      transitionId,
      nextStateRoot,
    },
    contract: {
      typehashes,
      transitionId: contractTransitionId,
      nextStateRoot: contractNextRoot,
    },
    allMatch: [
      spaceId === vector.expected_space_id,
      transitionId === vector.expected_transition_id,
      nextStateRoot === vector.expected_next_state_root,
      typehashes.EXPERIENCE_DELTA_TYPEHASH === vector.expected_experience_delta_typehash,
      typehashes.MEMORY_STATE_TYPEHASH === vector.expected_memory_state_typehash,
      typehashes.MEMORY_SPACE_TYPEHASH === vector.expected_memory_space_typehash,
      contractTransitionId === vector.expected_transition_id,
      contractNextRoot === vector.expected_next_state_root,
    ].every(Boolean),
    fullTransition: publicTransition(full),
  };
}

async function attempt(harness, transition, signature) {
  try {
    const result = await harness.commit(transition, { signature });
    return {
      status: 'PASS',
      reason: null,
      gasUsed: result.totalGasSpent.toString(),
      receiptStatus: result.receipt.status,
    };
  } catch (error) {
    return {
      status: 'REJECT',
      reason: error.message,
      gasUsed: null,
      receiptStatus: 0,
    };
  }
}

async function mutationCases(harness, firstResult) {
  const first = firstResult.transition;
  const validNext = await harness.nextTransition();
  const validSignature = await harness.signTransition(validNext);
  const cases = [];
  const add = async (name, transition, signature) => {
    const result = await attempt(harness, transition, signature);
    cases.push({ name, expected: 'REJECT', observed: result, transition: publicTransition(transition) });
  };

  const sequenceGap = buildTransition({ spaceId: harness.spaceId, sequence: 3, prevStateRoot: firstResult.root, profileId: harness.profileId });
  await add('sequence_gap', sequenceGap, await harness.signTransition(sequenceGap));
  const sequenceZero = buildTransition({ spaceId: harness.spaceId, sequence: 0, prevStateRoot: firstResult.root, profileId: harness.profileId });
  await add('sequence_zero', sequenceZero, await harness.signTransition(sequenceZero));
  const rollback = buildTransition({ spaceId: harness.spaceId, sequence: 2, prevStateRoot: ZERO_ROOT, profileId: harness.profileId });
  await add('rollback_predecessor', rollback, await harness.signTransition(rollback));
  const randomPredecessor = buildTransition({ spaceId: harness.spaceId, sequence: 2, prevStateRoot: `0x${'12'.repeat(32)}`, profileId: harness.profileId });
  await add('wrong_predecessor_random', randomPredecessor, await harness.signTransition(randomPredecessor));
  await add('zero_delta_commitment', { ...validNext, deltaCommitment: ZERO_ROOT }, validSignature);
  await add('zero_profile_id', { ...validNext, profileId: ZERO_ROOT }, validSignature);
  for (const [name, id] of [['unknown_space_a', 'unknown-space-a'], ['unknown_space_b', 'unknown-space-b']]) {
    const unknown = buildTransition({ spaceId: keccak256(toUtf8Bytes(id)), sequence: 1, prevStateRoot: ZERO_ROOT, profileId: harness.profileId });
    await add(name, unknown, await harness.signTransition(unknown));
  }
  await add('wrong_eoa_signer', validNext, await harness.signTransition(validNext, harness.relayerWallet));
  await add('missing_signature', validNext, '0x');
  await add('truncated_signature', validNext, validSignature.slice(0, -4));
  const flipped = `${validSignature.slice(0, -4)}${validSignature.slice(-4, -2) === '00' ? '01' : '00'}${validSignature.slice(-2)}`;
  await add('signature_byte_tamper', validNext, flipped);
  const invalidV = `${validSignature.slice(0, -2)}1d`;
  await add('signature_invalid_v', validNext, invalidV);
  await add('wrong_chain_domain', validNext, await harness.signerWallet.signTypedData({ ...harness.domain(), chainId: 1 }, EXPERIENCE_DELTA_TYPES, validNext));
  await add('wrong_contract_domain', validNext, await harness.signerWallet.signTypedData({ ...harness.domain(), verifyingContract: `0x${'98'.repeat(20)}` }, EXPERIENCE_DELTA_TYPES, validNext));
  for (const field of ['deltaCommitment', 'provenanceCommitment', 'locatorCommitment']) {
    await add(`${field}_signature_binding`, { ...validNext, [field]: keccak256(toUtf8Bytes(`bound-${field}`)) }, validSignature);
  }
  await add('duplicate_replay', first, await harness.signTransition(first));
  const branchA = await harness.nextTransition({ payload: 'branch-a' });
  const branchB = await harness.nextTransition({ payload: 'branch-b' });
  const branchAResult = await attempt(harness, branchA, await harness.signTransition(branchA));
  cases.push({ name: 'branch_setup', expected: 'PASS', observed: branchAResult, transition: publicTransition(branchA) });
  await add('parallel_history', branchB, await harness.signTransition(branchB));
  return cases;
}

async function validHistory(harness) {
  const transitions = [];
  const gas = [];
  const first = await harness.commitValidFirstTransition();
  transitions.push(publicTransition(first.transition));
  gas.push({ sequence: first.sequence, totalGasSpent: first.gasUsed });
  for (let i = 0; i < 3; i += 1) {
    const transition = await harness.nextTransition({
      payload: `private-memory-delta-${i}`,
      provenance: `agent://private/provenance/${i}`,
      locator: `cid://private/${i}`,
    });
    const result = await harness.commit(transition, { signature: await harness.signTransition(transition) });
    const head = await harness.readHead();
    transitions.push(publicTransition(transition));
    gas.push({ sequence: head.sequence, totalGasSpent: result.totalGasSpent.toString() });
  }
  return { transitions, gas, finalHead: await harness.readHead() };
}

async function readAuthorization(harness) {
  const raw = await harness.call({
    data: harness.registryInterface.encodeFunctionData('spaceAuthorization', [harness.spaceId]),
  });
  const [controller, authorizer, configNonce] = harness.registryInterface.decodeFunctionResult('spaceAuthorization', raw);
  return { controller, authorizer, configNonce: Number(configNonce) };
}

async function authorityHistory(harness) {
  const initial = await readAuthorization(harness);
  await harness.updateAuthorization({
    newController: harness.relayerWallet.address,
    newAuthorizer: harness.relayerWallet.address,
    nonce: 1,
  });
  const rotated = await readAuthorization(harness);
  return [
    { ...initial, label: 'initial authority' },
    { ...rotated, label: 'rotated authority' },
  ];
}

const vectorHarness = await createRegistryHarness();
const conformance = await contractVectorEvidence(vectorHarness);
const historyHarness = await createRegistryHarness();
const history = await validHistory(historyHarness);
const authorityHarness = await createRegistryHarness();
const authority = await authorityHistory(authorityHarness);
const mutationHarness = await createRegistryHarness();
const mutationFirst = await mutationHarness.commitValidFirstTransition();
const mutations = await mutationCases(mutationHarness, mutationFirst);

const unexpected = mutations.filter((entry) => entry.name !== 'branch_setup' && entry.observed.status !== 'REJECT');
if (!conformance.allMatch || unexpected.length > 0) {
  throw new Error(`local audit failed: conformance=${conformance.allMatch} unexpected=${unexpected.length}`);
}

const registryBytecodeHash = keccak256(historyHarness.registryBytecode);
const summary = {
  evidenceType: 'workspace_owned_local_evm',
  generatedAt: new Date().toISOString(),
  chainId: '31337',
  hardfork: 'shanghai',
  compilerVersion: historyHarness.artifacts.compilerVersion,
  registryAddress: historyHarness.registryAddress,
  registryBytecodeKeccak256: registryBytecodeHash,
  domain: historyHarness.domain(),
  rawPayloadStored: false,
  conformance,
  validHistory: history,
  authorityHistory: authority,
  mutationCount: mutations.filter((entry) => entry.name !== 'branch_setup').length,
  mutationRejectedCount: mutations.filter((entry) => entry.name !== 'branch_setup' && entry.observed.status === 'REJECT').length,
  mutationMatrixPath: 'evidence/generated/mutation_matrix.json',
  independentVerifierPath: 'evidence/local/memory_lineage_evm_evidence.json',
  evidenceLimits: [
    'local EthereumJS VM, not public testnet evidence',
    'commitments prove continuity and authorization binding, not semantic truth of private memory',
    'the mock ERC-1271 authorizer is a fixture, not a wallet integration',
  ],
};

await fs.writeFile(path.join(artifactDir, 'contract_conformance.json'), asJson(conformance));
await fs.writeFile(path.join(artifactDir, 'mutation_matrix.json'), asJson({
  evidenceType: 'workspace_owned_local_evm_mutation_matrix',
  expectedAllReject: true,
  cases: mutations,
}));
await fs.writeFile(path.join(artifactDir, 'local_evm_summary.json'), asJson(summary));
await fs.writeFile(path.join(localEvidenceDir, 'memory_lineage_evm_evidence.json'), asJson({
  evidenceType: 'public_commitment_replay_bundle',
  chainId: '31337',
  registryAddress: historyHarness.registryAddress,
  registryBytecodeKeccak256: registryBytecodeHash,
  conformance,
  validHistory: history.transitions,
  authorityHistory: authority,
  mutationMatrix: mutations.map((entry) => ({
    name: entry.name,
    expected: entry.expected,
    observed: entry.observed,
    transition: entry.transition,
  })),
  rawPayloadStored: false,
}));

console.log(JSON.stringify({
  registryAddress: historyHarness.registryAddress,
  compilerVersion: historyHarness.artifacts.compilerVersion,
  conformance: conformance.allMatch,
  validTransitions: history.transitions.length,
  mutationCount: summary.mutationCount,
  mutationRejectedCount: summary.mutationRejectedCount,
  unexpected: unexpected.map((entry) => ({ name: entry.name, observed: entry.observed })),
  bytecodeKeccak256: registryBytecodeHash,
}, null, 2));
