export type TransitionRecord = {
  spaceId: string;
  sequence: number;
  prevStateRoot: string;
  deltaCommitment: string;
  provenanceCommitment: string;
  profileId: string;
  locatorCommitment: string;
  transitionId: string;
  nextStateRoot: string;
};

export type LocalEvidence = {
  evidenceType: string;
  chainId: string;
  registryAddress: string;
  rawPayloadStored: boolean;
  validHistory: {
    transitions: TransitionRecord[];
    finalHead: { transitionId: string; root: string; sequence: number };
  };
  authorityHistory?: Array<{
    label: string;
    controller: string;
    authorizer: string;
    configNonce: number;
  }>;
  registryBytecodeKeccak256?: string;
  conformance?: unknown;
  mutationMatrix?: Array<{
    name: string;
    expected: string;
    observed?: { status?: string; reason?: string };
  }>;
};

export type SepoliaEvidence = {
  evidenceType: string;
  chainId: string;
  rpcUrl: string;
  registryAddress: string;
  spaceId: string;
  onchainHead: { transitionId: string; stateRoot: string; sequence: number };
  deployedCodeKeccak256: string;
  bytecodeKeccak256: string;
};

export type SepoliaReread = {
  verdict: string;
  rpcUrl: string;
  registryAddress: string;
  blockNumber: number;
  head: { transitionId: string; stateRoot: string; sequence: number };
  checks: Record<string, boolean>;
};

export type FixtureSnapshot = {
  sequence: number;
  file: string;
  visibleLabel: string;
};

export type FixtureManifest = {
  fixtureId: string;
  synthetic: boolean;
  description: string;
  snapshots: FixtureSnapshot[];
  attack: {
    name: string;
    restoredSequence: number;
    attemptedSequence: number;
    expectedContractReason: string;
  };
};

export type InspectorData = {
  local: LocalEvidence;
  sepolia: SepoliaEvidence;
  reread: SepoliaReread;
  fixture: FixtureManifest;
};
