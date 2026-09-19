#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};

pub const SPEC_NAME: &str = "ERC-8350";
pub const SPEC_SNAPSHOT: &str = "v1-pinned-vector-2026-09-18";
pub const EVIDENCE_V1: &str = "memorylineage-evidence-v1";
pub const EVIDENCE_V2: &str = "memorylineage-evidence-v2";

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
    pub network: EvidenceNetwork,
    pub registry: RegistryObservation,
    pub spec: SpecSnapshot,
    pub head: Head,
    pub transitions: Vec<TransitionRecord>,
    #[serde(rename = "authorizationHistory")]
    pub authorization_history: Vec<AuthorizationRecord>,
    pub observations: Vec<NetworkObservation>,
    #[serde(default)]
    pub attack: Option<AttackObservation>,
    pub privacy: PrivacyBoundary,
    #[serde(rename = "verificationMetadata")]
    pub verification_metadata: VerificationMetadata,
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
