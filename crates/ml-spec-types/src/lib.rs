#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};

pub const SPEC_NAME: &str = "ERC-8350";
pub const SPEC_SNAPSHOT: &str = "v1-pinned-vector-2026-09-18";
pub const EVIDENCE_V1: &str = "memorylineage-evidence-v1";
pub const EVIDENCE_V2: &str = "memorylineage-evidence-v2";
pub const RECOVERY_RECEIPT_V1: &str = "memorylineage-recovery-receipt-v1";
pub const SNAPSHOT_PROFILE_V1: &str = "memorylineage/private-snapshot/v1";
pub const SNAPSHOT_PROFILE_V2: &str = "memorylineage/private-snapshot/v2";
pub const PORTABILITY_REHEARSAL_V1: &str = "memorylineage-portability-v1";
pub const POLKADOT_HUB_TESTNET_CHAIN_ID: &str = "420420417";
pub const RECOVERY_POLICY_STRICT_CURRENT_HEAD_V1: &str = "strict-current-head-only-v1";

pub const SOURCE_DEMO_SPACE_V2_LOCAL: &str = "DEMO_SPACE_V2_LOCAL";
pub const SOURCE_PROTOCOL_CORPUS_LOCAL: &str = "PROTOCOL_CORPUS_LOCAL";
pub const SOURCE_SEPOLIA_REFERENCE_OBSERVATION: &str = "SEPOLIA_REFERENCE_OBSERVATION";
pub const SOURCE_LEGACY_UNDECLARED: &str = "LEGACY_UNDECLARED";

fn default_source_class() -> String {
    SOURCE_LEGACY_UNDECLARED.to_owned()
}

pub const RECOVERY_CURRENT_HEAD: &str = "CURRENT_HEAD";
pub const RECOVERY_HISTORICAL_CHECKPOINT: &str = "KNOWN_HISTORICAL_CHECKPOINT";
pub const RECOVERY_UNKNOWN_OR_DIVERGED: &str = "UNKNOWN_OR_DIVERGED";
pub const RECOVERY_UNVERIFIED: &str = "UNVERIFIED";

pub const RECOVERY_RESUME_ALLOWED: &str = "RESUME_ALLOWED";
pub const RECOVERY_REHEARSE_ONLY: &str = "REHEARSE_ONLY";
pub const RECOVERY_HOLD_FOR_REVIEW: &str = "HOLD_FOR_REVIEW";
pub const RECOVERY_BLOCK_UNVERIFIED: &str = "BLOCK_UNVERIFIED";

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExperienceDelta {
    #[serde(rename = "spaceId")]
    pub space_id: String,
    pub sequence: u64,
    #[serde(rename = "prevStateRoot")]
    pub prev_state_root: String,
    #[serde(rename = "deltaCommitment")]
    pub delta_commitment: String,
    #[serde(rename = "provenanceCommitment")]
    pub provenance_commitment: String,
    #[serde(rename = "profileId")]
    pub profile_id: String,
    #[serde(rename = "locatorCommitment")]
    pub locator_commitment: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TransitionRecord {
    #[serde(flatten)]
    pub delta: ExperienceDelta,
    #[serde(rename = "transitionId")]
    pub transition_id: String,
    #[serde(rename = "nextStateRoot")]
    pub next_state_root: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Head {
    #[serde(rename = "transitionId")]
    pub transition_id: String,
    #[serde(rename = "stateRoot")]
    pub state_root: String,
    pub sequence: u64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AuthorizationRecord {
    pub controller: String,
    pub authorizer: String,
    #[serde(rename = "configNonce")]
    pub config_nonce: u64,
    #[serde(
        rename = "effectiveFromSequence",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub effective_from_sequence: Option<u64>,
    #[serde(default)]
    pub label: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct EvidenceNetwork {
    pub name: String,
    #[serde(rename = "chainId")]
    pub chain_id: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RegistryObservation {
    pub address: String,
    #[serde(rename = "codeHash", default)]
    pub code_hash: Option<String>,
    #[serde(rename = "spaceId")]
    pub space_id: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PrivacyBoundary {
    #[serde(rename = "rawMemoryOnChain")]
    pub raw_memory_on_chain: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SpecSnapshot {
    pub name: String,
    pub snapshot: String,
    #[serde(rename = "vectorHash", default)]
    pub vector_hash: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ConformanceInputs {
    #[serde(rename = "initialController")]
    pub initial_controller: String,
    #[serde(rename = "spaceSalt")]
    pub space_salt: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ConformanceExpected {
    #[serde(rename = "spaceId")]
    pub space_id: String,
    #[serde(rename = "experienceDeltaTypehash")]
    pub experience_delta_type_hash: String,
    #[serde(rename = "memoryStateTypehash")]
    pub memory_state_type_hash: String,
    #[serde(rename = "memorySpaceTypehash")]
    pub memory_space_type_hash: String,
    #[serde(rename = "transitionId")]
    pub transition_id: String,
    #[serde(rename = "nextStateRoot")]
    pub next_state_root: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ConformanceIndependent {
    #[serde(rename = "spaceId")]
    pub space_id: String,
    #[serde(rename = "transitionId")]
    pub transition_id: String,
    #[serde(rename = "nextStateRoot")]
    pub next_state_root: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ConformanceTypeHashes {
    #[serde(rename = "EXPERIENCE_DELTA_TYPEHASH")]
    pub experience_delta: String,
    #[serde(rename = "MEMORY_STATE_TYPEHASH")]
    pub memory_state: String,
    #[serde(rename = "MEMORY_SPACE_TYPEHASH")]
    pub memory_space: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ConformanceContract {
    pub typehashes: ConformanceTypeHashes,
    #[serde(rename = "transitionId")]
    pub transition_id: String,
    #[serde(rename = "nextStateRoot")]
    pub next_state_root: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ConformanceArtifact {
    pub source: String,
    pub inputs: ConformanceInputs,
    pub expected: ConformanceExpected,
    #[serde(rename = "independentlyComputed")]
    pub independently_computed: ConformanceIndependent,
    pub contract: ConformanceContract,
    #[serde(rename = "allMatch")]
    pub all_match: bool,
    #[serde(rename = "fullTransition")]
    pub full_transition: TransitionRecord,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MutationObservation {
    pub status: String,
    pub reason: Option<String>,
    #[serde(rename = "gasUsed")]
    pub gas_used: Option<NumericOrString>,
    #[serde(rename = "receiptStatus")]
    pub receipt_status: u64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(untagged)]
pub enum NumericOrString {
    Number(u64),
    String(String),
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MutationRecord {
    pub name: String,
    pub expected: String,
    pub observed: MutationObservation,
    pub transition: TransitionRecord,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PublicReplayBundle {
    #[serde(rename = "evidenceType")]
    pub evidence_type: String,
    #[serde(rename = "chainId")]
    pub chain_id: String,
    #[serde(rename = "registryAddress")]
    pub registry_address: String,
    #[serde(rename = "registryBytecodeKeccak256")]
    pub registry_bytecode_keccak256: String,
    pub conformance: ConformanceArtifact,
    #[serde(rename = "validHistory")]
    pub valid_history: Vec<TransitionRecord>,
    #[serde(rename = "authorityHistory")]
    pub authority_history: Vec<AuthorizationRecord>,
    #[serde(rename = "mutationMatrix")]
    pub mutation_matrix: Vec<MutationRecord>,
    #[serde(rename = "rawPayloadStored")]
    pub raw_payload_stored: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AttackObservation {
    pub name: String,
    pub status: String,
    pub reason: Option<String>,
    #[serde(rename = "restoredSnapshotSequence", default)]
    pub restored_snapshot_sequence: Option<u64>,
    #[serde(rename = "attemptedSequence", default)]
    pub attempted_sequence: Option<u64>,
    /// The predecessor root taken from the restored private snapshot. This is
    /// optional because older V2 bundles only recorded the scenario result.
    #[serde(rename = "stalePredecessor", default)]
    pub stale_predecessor: Option<String>,
    /// The root that the registry considered canonical when the attack was
    /// simulated.
    #[serde(rename = "canonicalPredecessor", default)]
    pub canonical_predecessor: Option<String>,
    #[serde(rename = "executionSource", default)]
    pub execution_source: Option<String>,
    #[serde(rename = "transactionBroadcast", default)]
    pub transaction_broadcast: Option<bool>,
    #[serde(rename = "fixtureId", default)]
    pub fixture_id: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct VerificationMetadata {
    pub producer: String,
    #[serde(rename = "generatedAt", default)]
    pub generated_at: Option<String>,
    #[serde(rename = "legacySource", default)]
    pub legacy_source: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct EvidenceBundleV2 {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "evidenceType")]
    pub evidence_type: String,
    #[serde(rename = "sourceClass", default = "default_source_class")]
    pub source_class: String,
    pub network: EvidenceNetwork,
    pub registry: RegistryObservation,
    pub spec: SpecSnapshot,
    pub head: Head,
    pub transitions: Vec<TransitionRecord>,
    #[serde(
        rename = "authorizationProofs",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub authorization_proofs: Vec<AuthorizationProof>,
    #[serde(rename = "authorizationHistory")]
    pub authorization_history: Vec<AuthorizationRecord>,
    pub observations: Vec<NetworkObservation>,
    #[serde(default)]
    pub attack: Option<AttackObservation>,
    pub privacy: PrivacyBoundary,
    #[serde(rename = "verificationMetadata")]
    pub verification_metadata: VerificationMetadata,
}

/// A complete, replayable observation used by the portability rehearsal.
///
/// The report keeps the two bundles together so an independent verifier can
/// validate each bundle and then compare the resulting projections. It is
/// still evidence of local execution only; it does not imply a public network
/// observation.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PortabilityObservation {
    pub label: String,
    #[serde(rename = "executionBackend")]
    pub execution_backend: String,
    pub evidence: EvidenceBundleV2,
    #[serde(rename = "domainSeparator")]
    pub domain_separator: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PortabilityRehearsalReport {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "reportType")]
    pub report_type: String,
    pub status: String,
    pub target: PortabilityTarget,
    pub comparison: PortabilityComparison,
    pub claims: Vec<String>,
    pub observations: Vec<PortabilityObservation>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PortabilityTarget {
    pub network: String,
    #[serde(rename = "chainId")]
    pub chain_id: String,
    #[serde(rename = "executionBackend")]
    pub execution_backend: String,
    pub deployment: String,
    #[serde(rename = "publicRpcObservation")]
    pub public_rpc_observation: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PortabilityComparison {
    #[serde(rename = "canonicalTransitions")]
    pub canonical_transitions: String,
    #[serde(rename = "stateRoots")]
    pub state_roots: String,
    #[serde(rename = "stalePredecessorRejection")]
    pub stale_predecessor_rejection: String,
    #[serde(rename = "authorityHistory")]
    pub authority_history: String,
    #[serde(rename = "eip712Domain")]
    pub eip712_domain: String,
    #[serde(rename = "solidityArtifact")]
    pub solidity_artifact: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NetworkObservation {
    #[serde(rename = "providerLabel")]
    pub provider_label: String,
    #[serde(rename = "blockNumber")]
    pub block_number: u64,
    #[serde(rename = "observedHead")]
    pub observed_head: Head,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AuthorizationProof {
    pub sequence: u64,
    #[serde(rename = "configNonce", default)]
    pub config_nonce: Option<u64>,
    #[serde(rename = "transitionId")]
    pub transition_id: String,
    pub authorizer: String,
    #[serde(rename = "authorizationType")]
    pub authorization_type: String,
    #[serde(rename = "chainId")]
    pub chain_id: String,
    #[serde(rename = "verifyingContract")]
    pub verifying_contract: String,
    #[serde(rename = "structHash")]
    pub struct_hash: String,
    #[serde(rename = "domainSeparator")]
    pub domain_separator: String,
    #[serde(rename = "signingDigest")]
    pub signing_digest: String,
    pub signature: String,
}

/// A pinned chain context is optional for local evidence and required only
/// when a receipt claims that a public RPC observation was used.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RecoveryBlockContext {
    pub tag: String,
    pub number: u64,
    pub hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RecoveryCandidate {
    #[serde(rename = "snapshotProfile")]
    pub snapshot_profile: String,
    #[serde(rename = "candidateCommitment")]
    pub candidate_commitment: String,
    #[serde(rename = "snapshotSequence")]
    pub snapshot_sequence: u64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RecoveryEvidence {
    #[serde(rename = "sourceClass")]
    pub source_class: String,
    #[serde(rename = "bundleHash")]
    pub bundle_hash: String,
    #[serde(rename = "specSnapshot")]
    pub spec_snapshot: String,
    #[serde(rename = "registryAddress")]
    pub registry_address: String,
    #[serde(rename = "spaceId")]
    pub space_id: String,
    pub head: Head,
    #[serde(rename = "blockContext", default)]
    pub block_context: Option<RecoveryBlockContext>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RecoveryDecision {
    pub classification: String,
    #[serde(rename = "reasonCode")]
    pub reason_code: String,
    #[serde(rename = "recommendedAction")]
    pub recommended_action: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RecoveryAssurance {
    #[serde(rename = "lineageReplay")]
    pub lineage_replay: String,
    #[serde(rename = "authorityHistory")]
    pub authority_history: String,
    #[serde(rename = "transitionAuthorization")]
    pub transition_authorization: String,
    pub source: String,
}

/// A portable decision about whether a private snapshot may enter a protected
/// recovery path. It contains commitments and evidence references only; it
/// deliberately never contains raw memory or private locator contents.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RecoveryDecisionReceipt {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "decisionId")]
    pub decision_id: String,
    #[serde(rename = "policyId")]
    pub policy_id: String,
    pub candidate: RecoveryCandidate,
    pub evidence: RecoveryEvidence,
    pub decision: RecoveryDecision,
    pub assurance: RecoveryAssurance,
    pub limitations: Vec<String>,
}
