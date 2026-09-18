import {
  buildTransition,
  createRegistryHarness,
  ZERO_ROOT,
} from '../src/harness/run_tests.mjs';

const harness = await createRegistryHarness();
const canonical = [];

const first = await harness.commitValidFirstTransition();
canonical.push(first.transition);

for (const label of ['claim-policy', 'privacy-boundary']) {
  const transition = await harness.nextTransition({ payload: `private-snapshot-${label}` });
  await harness.commit(transition, { signature: await harness.signTransition(transition) });
  canonical.push(transition);
}

const head = await harness.readHead();
const staleAttempt = buildTransition({
  spaceId: harness.spaceId,
  sequence: 4,
  prevStateRoot: canonical[0].nextStateRoot,
  profileId: harness.profileId,
  payload: 'private-snapshot-restored-17',
  provenance: 'private provenance omitted',
  locator: 'private locator omitted',
});

let result;
try {
  await harness.commit(staleAttempt, { signature: await harness.signTransition(staleAttempt) });
  result = { status: 'UNEXPECTED_PASS', reason: null };
} catch (error) {
  result = { status: 'REJECTED', reason: error instanceof Error ? error.message : String(error) };
}

const publicTransition = (transition) => ({
  spaceId: transition.spaceId,
  sequence: Number(transition.sequence),
  prevStateRoot: transition.prevStateRoot,
  deltaCommitment: transition.deltaCommitment,
  provenanceCommitment: transition.provenanceCommitment,
  profileId: transition.profileId,
  locatorCommitment: transition.locatorCommitment,
  transitionId: transition.transitionId,
  nextStateRoot: transition.nextStateRoot,
});

console.log(JSON.stringify({
  evidenceType: 'silent_rollback_local_ethereumjs_simulation',
  execution: 'local-ethereumjs',
  status: result.status,
  reason: result.reason,
  registryAddress: harness.registryAddress,
  canonicalHead: head,
  canonicalTransitions: canonical.map(publicTransition),
  restoredSnapshotSequence: 17,
  attemptedSequence: 4,
  attemptedPrevStateRoot: staleAttempt.prevStateRoot,
  expectedReason: 'BAD_PREVIOUS_STATE',
  rawMemoryOnChain: false,
}, null, 2));

if (result.status !== 'REJECTED' || result.reason !== 'BAD_PREVIOUS_STATE') process.exitCode = 1;
