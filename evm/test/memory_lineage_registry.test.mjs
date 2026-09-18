import test from 'node:test';
import assert from 'node:assert/strict';
import fs from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import { keccak256, toUtf8Bytes } from 'ethers';
import {
  EXPERIENCE_DELTA_TYPES,
  buildTransition,
  computeNextStateRoot,
  computeSpaceId,
  computeTransitionId,
  createRegistryHarness,
  transitionTuple,
  ZERO_ROOT,
} from '../src/harness/run_tests.mjs';

const HERE = path.dirname(fileURLToPath(import.meta.url));

test('a registered space accepts a valid first transition and exposes its head', async () => {
  const harness = await createRegistryHarness();
  const result = await harness.commitValidFirstTransition();
  const head = await harness.readHead();

  assert.equal(result.verdict, 'PASS');
  assert.equal(head.sequence, 1);
  assert.equal(head.transitionId, result.transition.transitionId);
  assert.equal(head.root, result.transition.nextStateRoot);
  assert.equal(head.root, computeNextStateRoot(ZERO_ROOT, result.transition.transitionId));
});

test('a transition with a wrong predecessor is rejected by the registry', async () => {
  const harness = await createRegistryHarness();
  await harness.commitValidFirstTransition();
  await assert.rejects(harness.commitWithWrongPredecessor(), /BAD_PREVIOUS_STATE/);
});

test('the Solidity registry matches the published ERC-8350 vector and domain', async () => {
  const vector = JSON.parse(
    await fs.readFile(path.join(HERE, '..', '..', 'contracts', 'vectors', 'erc8350_conformance.json'), 'utf8'),
  );
  const harness = await createRegistryHarness();
  const spaceId = computeSpaceId(vector.initial_controller, vector.space_salt);
  assert.equal(spaceId, vector.expected_space_id);

  const delta = {
    spaceId,
    sequence: 1,
    prevStateRoot: ZERO_ROOT,
    deltaCommitment: vector.delta_commitment,
    provenanceCommitment: vector.provenance_commitment,
    profileId: vector.profile_id,
    locatorCommitment: vector.locator_commitment,
  };
  const transitionId = computeTransitionId(delta);
  const nextRoot = computeNextStateRoot(ZERO_ROOT, transitionId);
  assert.equal(transitionId, vector.expected_transition_id);
  assert.equal(nextRoot, vector.expected_next_state_root);

  const idRaw = await harness.call({
    data: harness.registryInterface.encodeFunctionData('hashExperienceDelta', [transitionTuple(delta)]),
  });
  const rootRaw = await harness.call({
    data: harness.registryInterface.encodeFunctionData('computeNextStateRoot', [ZERO_ROOT, transitionId]),
  });
  assert.equal(harness.registryInterface.decodeFunctionResult('hashExperienceDelta', idRaw)[0], transitionId);
  assert.equal(harness.registryInterface.decodeFunctionResult('computeNextStateRoot', rootRaw)[0], nextRoot);

  for (const [name, expected] of [
    ['EXPERIENCE_DELTA_TYPEHASH', vector.expected_experience_delta_typehash],
    ['MEMORY_STATE_TYPEHASH', vector.expected_memory_state_typehash],
    ['MEMORY_SPACE_TYPEHASH', vector.expected_memory_space_typehash],
  ]) {
    const raw = await harness.call({ data: harness.registryInterface.encodeFunctionData(name) });
    assert.equal(harness.registryInterface.decodeFunctionResult(name, raw)[0], expected, name);
  }
  const nameRaw = await harness.call({ data: harness.registryInterface.encodeFunctionData('EIP712_NAME') });
  assert.equal(harness.registryInterface.decodeFunctionResult('EIP712_NAME', nameRaw)[0], 'AgentMemoryState');
  const domainRaw = await harness.call({
    data: harness.registryInterface.encodeFunctionData('domainSeparator'),
  });
  const computedDomainRaw = await harness.call({
    data: harness.registryInterface.encodeFunctionData('computeDomainSeparator', [31337, harness.registryAddress]),
  });
  assert.equal(domainRaw, computedDomainRaw);
});

test('profile IDs remain part of each signed delta, as defined by v1', async () => {
  const harness = await createRegistryHarness();
  const transition = buildTransition({
    spaceId: harness.spaceId,
    sequence: 1,
    prevStateRoot: ZERO_ROOT,
    profileId: keccak256(toUtf8Bytes('another-nonzero-profile')),
  });
  await harness.commit(transition, { signature: await harness.signTransition(transition) });
  assert.equal((await harness.readHead()).sequence, 1);
});

test('EOA authorization binds every ExperienceDelta field and the domain', async () => {
  const harness = await createRegistryHarness();
  await harness.commitValidFirstTransition();
  const transition = await harness.nextTransition();
  const validSignature = await harness.signTransition(transition);

  await assert.rejects(
    harness.commit(transition, { signature: await harness.signTransition(transition, harness.relayerWallet) }),
    /INVALID_AUTHORIZATION/,
  );
  await assert.rejects(
    harness.commit(transition, {
      signature: await harness.signerWallet.signTypedData(
        { ...harness.domain(), chainId: 1 },
        EXPERIENCE_DELTA_TYPES,
        transition,
      ),
    }),
    /INVALID_AUTHORIZATION/,
  );
  await assert.rejects(
    harness.commit(transition, {
      signature: await harness.signerWallet.signTypedData(
        { ...harness.domain(), verifyingContract: `0x${'99'.repeat(20)}` },
        EXPERIENCE_DELTA_TYPES,
        transition,
      ),
    }),
    /INVALID_AUTHORIZATION/,
  );
  await assert.rejects(harness.commit(transition, { signature: '0x' }), /INVALID_AUTHORIZATION/);
  await assert.rejects(harness.commit(transition, { signature: validSignature.slice(0, -2) }), /INVALID_AUTHORIZATION/);

  for (const field of ['deltaCommitment', 'provenanceCommitment', 'locatorCommitment']) {
    const mutated = { ...transition, [field]: keccak256(toUtf8Bytes(`mutated-${field}`)) };
    await assert.rejects(
      harness.commit(mutated, { signature: validSignature }),
      /INVALID_AUTHORIZATION/,
      `signature must bind ${field}`,
    );
  }
});

test('ERC-1271 authorization accepts the owner signature and fails closed', async () => {
  const harness = await createRegistryHarness();
  const authorizer = await harness.deploy1271Authorizer();
  const spaceSalt = keccak256(toUtf8Bytes('erc1271-space-salt'));
  const spaceId = computeSpaceId(harness.signerWallet.address, spaceSalt);
  const profileId = keccak256(toUtf8Bytes('erc1271-profile'));
  await harness.createSpace({ spaceId, authorizer, salt: spaceSalt });

  const first = buildTransition({
    spaceId,
    sequence: 1,
    prevStateRoot: ZERO_ROOT,
    profileId,
    payload: '1271 payload',
    provenance: 'agent://1271',
    locator: 'cid://1271',
  });
  const signature = await harness.signTransition(first);
  const result = await harness.commit(first, { signature });
  assert.equal(result.receipt.status, 1);

  await harness.set1271Accept(authorizer, false);
  const second = await harness.nextTransition({ spaceId, profileId });
  await assert.rejects(
    harness.commit(second, { signature: await harness.signTransition(second) }),
    /INVALID_AUTHORIZATION/,
  );
});

test('controller authorization rotates with a checked config nonce', async () => {
  const harness = await createRegistryHarness();
  await harness.updateAuthorization({
    newController: harness.signerWallet.address,
    newAuthorizer: harness.relayerWallet.address,
    nonce: 1,
  });
  const authorizationRaw = await harness.call({
    data: harness.registryInterface.encodeFunctionData('spaceAuthorization', [harness.spaceId]),
  });
  const authorization = harness.registryInterface.decodeFunctionResult('spaceAuthorization', authorizationRaw);
  assert.equal(authorization[1], harness.relayerWallet.address);
  assert.equal(Number(authorization[2]), 1);

  const first = buildTransition({
    spaceId: harness.spaceId,
    sequence: 1,
    prevStateRoot: ZERO_ROOT,
    profileId: harness.profileId,
  });
  await assert.rejects(
    harness.commit(first, { signature: await harness.signTransition(first, harness.signerWallet) }),
    /INVALID_AUTHORIZATION/,
  );
  await harness.commit(first, { signature: await harness.signTransition(first, harness.relayerWallet) });
});

test('the mutation corpus rejects twenty named attacks', async () => {
  const harness = await createRegistryHarness();
  const firstResult = await harness.commitValidFirstTransition();
  const first = firstResult.transition;
  const validNext = await harness.nextTransition();
  const validSignature = await harness.signTransition(validNext);
  const rejected = [];
  const expectReject = async (name, transition, signature, reason) => {
    await assert.rejects(harness.commit(transition, { signature }), new RegExp(reason), name);
    rejected.push(name);
  };

  const sequenceGap = buildTransition({ spaceId: harness.spaceId, sequence: 3, prevStateRoot: firstResult.root, profileId: harness.profileId });
  await expectReject('sequence_gap', sequenceGap, await harness.signTransition(sequenceGap), 'BAD_SEQUENCE');
  const sequenceZero = buildTransition({ spaceId: harness.spaceId, sequence: 0, prevStateRoot: firstResult.root, profileId: harness.profileId });
  await expectReject('sequence_zero', sequenceZero, await harness.signTransition(sequenceZero), 'BAD_SEQUENCE');
  const rollback = buildTransition({ spaceId: harness.spaceId, sequence: 2, prevStateRoot: ZERO_ROOT, profileId: harness.profileId });
  await expectReject('rollback_predecessor', rollback, await harness.signTransition(rollback), 'BAD_PREVIOUS_STATE');
  const randomPredecessor = buildTransition({ spaceId: harness.spaceId, sequence: 2, prevStateRoot: `0x${'12'.repeat(32)}`, profileId: harness.profileId });
  await expectReject('wrong_predecessor_random', randomPredecessor, await harness.signTransition(randomPredecessor), 'BAD_PREVIOUS_STATE');
  await expectReject('zero_delta_commitment', { ...validNext, deltaCommitment: ZERO_ROOT }, validSignature, 'ZERO_DELTA_COMMITMENT');
  await expectReject('zero_profile_id', { ...validNext, profileId: ZERO_ROOT }, validSignature, 'ZERO_PROFILE_ID');

  for (const [name, id] of [
    ['unknown_space_a', 'unknown-space-a'],
    ['unknown_space_b', 'unknown-space-b'],
  ]) {
    const unknown = buildTransition({ spaceId: keccak256(toUtf8Bytes(id)), sequence: 1, prevStateRoot: ZERO_ROOT, profileId: harness.profileId });
    await expectReject(name, unknown, await harness.signTransition(unknown), 'UNKNOWN_SPACE');
  }
  await expectReject('wrong_eoa_signer', validNext, await harness.signTransition(validNext, harness.relayerWallet), 'INVALID_AUTHORIZATION');
  await expectReject('missing_signature', validNext, '0x', 'INVALID_AUTHORIZATION');
  await expectReject('truncated_signature', validNext, validSignature.slice(0, -4), 'INVALID_AUTHORIZATION');
  const flipped = `${validSignature.slice(0, -4)}${validSignature.slice(-4, -2) === '00' ? '01' : '00'}${validSignature.slice(-2)}`;
  await expectReject('signature_byte_tamper', validNext, flipped, 'INVALID_AUTHORIZATION');
  const invalidV = `${validSignature.slice(0, -2)}1d`;
  await expectReject('signature_invalid_v', validNext, invalidV, 'INVALID_AUTHORIZATION');
  await expectReject('wrong_chain_domain', validNext, await harness.signerWallet.signTypedData({ ...harness.domain(), chainId: 1 }, EXPERIENCE_DELTA_TYPES, validNext), 'INVALID_AUTHORIZATION');
  await expectReject('wrong_contract_domain', validNext, await harness.signerWallet.signTypedData({ ...harness.domain(), verifyingContract: `0x${'98'.repeat(20)}` }, EXPERIENCE_DELTA_TYPES, validNext), 'INVALID_AUTHORIZATION');

  for (const field of ['deltaCommitment', 'provenanceCommitment', 'locatorCommitment']) {
    const mutated = { ...validNext, [field]: keccak256(toUtf8Bytes(`bound-${field}`)) };
    await expectReject(`${field}_signature_binding`, mutated, validSignature, 'INVALID_AUTHORIZATION');
  }
  await expectReject('duplicate_replay', first, await harness.signTransition(first), 'BAD_SEQUENCE');

  const branchA = await harness.nextTransition({ payload: 'branch-a' });
  const branchB = await harness.nextTransition({ payload: 'branch-b' });
  await harness.commit(branchA, { signature: await harness.signTransition(branchA) });
  await expectReject('parallel_history', branchB, await harness.signTransition(branchB), 'BAD_SEQUENCE');

  assert.equal(rejected.length, 20);
});
