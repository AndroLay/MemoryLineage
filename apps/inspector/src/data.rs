#![forbid(unsafe_code)]

use ml_evidence::public_replay_to_v2;
use ml_spec_types::{
    EvidenceBundleV2, PortabilityRehearsalReport, PublicReplayBundle, RecoveryDecisionReceipt,
    SNAPSHOT_PROFILE_V2, TransitionRecord,
};
use ml_verifier_independent::{
    RecoveryVerificationReport, VerificationReport, build_recovery_receipt_with_snapshot_profile,
    verify_recovery_receipt, verify_v2_bundle,
};
use serde::Deserialize;

pub const LOCAL_EVIDENCE: &str =
    include_str!("../../../evidence/local/memory_lineage_evm_evidence.json");
pub const DEMO_EVIDENCE: &str = include_str!("../../../evidence/local/demo_space_v2_evidence.json");
pub const SILENT_ROLLBACK_MANIFEST: &str =
    include_str!("../../../fixtures/silent-rollback-v2/manifest.json");
pub const DEMO_RECOVERY_RECEIPT: &str =
    include_str!("../../../evidence/local/demo_space_v2_recovery_receipt.json");
const SEPOLIA_DEPLOYMENT: &str = include_str!("../../../evidence/sepolia/sepolia_deployment.json");
const SEPOLIA_REREAD: &str = include_str!("../../../evidence/sepolia/sepolia_reread.json");
const CONFORMANCE_REPORT: &str = include_str!("../../../evidence/local/rust_revm_conformance.json");
const REFERENCE_RUNTIME_REPORT: &str =
    include_str!("../../../evidence/local/reference_agent_runtime.json");
const SECURITY_ASSURANCE_REPORT: &str =
    include_str!("../../../evidence/local/security_assurance_report.json");
const POLKADOT_PORTABILITY_REPORT: &str =
    include_str!("../../../evidence/local/polkadot_hub_portability_rehearsal.json");

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Scenario {
    SilentRollback,
    SequenceGap,
    ParallelHistory,
    WrongEoa,
    LocatorBinding,
    WrongDomain,
    SemanticPoisoning,
}

impl Scenario {
    pub const ALL: [Self; 7] = [
        Self::SilentRollback,
        Self::SequenceGap,
        Self::ParallelHistory,
        Self::WrongEoa,
        Self::LocatorBinding,
        Self::WrongDomain,
        Self::SemanticPoisoning,
    ];

    pub fn title(self) -> &'static str {
        match self {
            Self::SilentRollback => "Silent Rollback",
            Self::SequenceGap => "Sequence Gap",
            Self::ParallelHistory => "Parallel History",
            Self::WrongEoa => "Wrong EOA Signer",
            Self::LocatorBinding => "Locator Substitution",
            Self::WrongDomain => "Wrong Chain Domain",
            Self::SemanticPoisoning => "Semantic Poisoning",
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            Self::SilentRollback => "restore a stale private snapshot and try to continue",
            Self::SequenceGap => "skip the next canonical sequence number",
            Self::ParallelHistory => "present another successor from the same predecessor",
            Self::WrongEoa => "reuse the transition with an unrelated signer",
            Self::LocatorBinding => "change a locator commitment after signing",
            Self::WrongDomain => "reuse a signature from another chain or contract domain",
            Self::SemanticPoisoning => "write harmful content while keeping lineage valid",
        }
    }

    pub fn evidence_kind(self) -> &'static str {
        match self {
            Self::SilentRollback => "LOCAL EVIDENCE",
            Self::SemanticPoisoning => "SCOPE",
            _ => "CORPUS",
        }
    }

    pub fn corpus_name(self) -> Option<&'static str> {
        match self {
            Self::SilentRollback => Some("rollback_predecessor"),
            Self::SequenceGap => Some("sequence_gap"),
            Self::ParallelHistory => Some("parallel_history"),
            Self::WrongEoa => Some("wrong_eoa_signer"),
            Self::LocatorBinding => Some("locatorCommitment_signature_binding"),
            Self::WrongDomain => Some("wrong_chain_domain"),
            Self::SemanticPoisoning => None,
        }
    }

    pub fn from_id(value: &str) -> Self {
        match value {
            "silent-rollback" => Self::SilentRollback,
            "sequence-gap" => Self::SequenceGap,
            "parallel-history" => Self::ParallelHistory,
            "wrong-eoa-signer" => Self::WrongEoa,
            "locator-binding" => Self::LocatorBinding,
            "wrong-chain-domain" => Self::WrongDomain,
            "semantic-poisoning" => Self::SemanticPoisoning,
            _ => Self::SilentRollback,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Route {
    Home,
    Inspect,
    History,
    Transition(u64),
    Lab(Scenario),
    Verify,
    Evidence,
    Architecture,
    Security,
    Reproduce,
    PriorWork,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct DeploymentRecord {
    #[serde(default, rename = "evidenceType")]
    pub evidence_type: Option<String>,
    #[serde(rename = "chainId")]
    pub chain_id: String,
    #[serde(rename = "rpcUrl")]
    pub rpc_url: String,
    #[serde(default, rename = "secondRpcUrlConfigured")]
    pub second_rpc_url_configured: Option<bool>,
    #[serde(rename = "registryAddress")]
    pub registry_address: String,
    #[serde(default, rename = "deployerAddress")]
    pub deployer_address: Option<String>,
    #[serde(rename = "deploymentTxHash")]
    pub deployment_tx_hash: String,
    #[serde(rename = "registerTxHash")]
    pub register_tx_hash: String,
    #[serde(rename = "commitTxHash")]
    pub commit_tx_hash: String,
    #[serde(rename = "expectedRejectedTxHash")]
    pub expected_rejected_tx_hash: String,
    #[serde(default, rename = "expectedRejectedReceiptStatus")]
    pub expected_rejected_receipt_status: Option<u64>,
    #[serde(default, rename = "expectedRejectedReason")]
    pub expected_rejected_reason: Option<String>,
    #[serde(default, rename = "bytecodeKeccak256")]
    pub bytecode_hash: Option<String>,
    #[serde(rename = "deployedCodeKeccak256")]
    pub deployed_code_hash: String,
    #[serde(rename = "spaceId")]
    pub space_id: String,
    #[serde(default, rename = "transitionId")]
    pub transition_id: Option<String>,
    #[serde(default, rename = "expectedNextStateRoot")]
    pub expected_next_state_root: Option<String>,
    #[serde(default, rename = "onchainHead")]
    pub onchain_head: Option<SepoliaHead>,
    #[serde(rename = "rawPayloadStored")]
    pub raw_payload_stored: bool,
    #[serde(default)]
    pub note: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct SepoliaHead {
    #[serde(rename = "transitionId")]
    pub transition_id: String,
    #[serde(rename = "stateRoot")]
    pub state_root: String,
    pub sequence: u64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct SepoliaReread {
    #[serde(default, rename = "evidenceType")]
    pub evidence_type: Option<String>,
    #[serde(rename = "chainId")]
    pub chain_id: String,
    #[serde(rename = "rpcUrl")]
    pub rpc_url: String,
    #[serde(rename = "blockNumber")]
    pub block_number: u64,
    #[serde(rename = "registryAddress")]
    pub registry_address: String,
    #[serde(rename = "deployedCodeKeccak256")]
    pub deployed_code_hash: String,
    pub head: SepoliaHead,
    pub statuses: RereadStatuses,
    pub checks: RereadChecks,
    pub verdict: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct RereadStatuses {
    pub deployment: u64,
    pub registration: u64,
    #[serde(rename = "validCommit")]
    pub valid_commit: u64,
    #[serde(rename = "expectedRejectedCommit")]
    pub expected_rejected_commit: u64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct RereadChecks {
    #[serde(rename = "codeHashMatches")]
    pub code_hash_matches: bool,
    #[serde(rename = "deploymentMined")]
    pub deployment_mined: bool,
    #[serde(rename = "registrationMined")]
    pub registration_mined: bool,
    #[serde(rename = "validCommitMined")]
    pub valid_commit_mined: bool,
    #[serde(rename = "expectedRejectedCommitMined")]
    pub expected_rejected_commit_mined: bool,
    #[serde(rename = "headMatches")]
    pub head_matches: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ConformanceEvidence {
    pub pinned_vector: String,
    pub published_evidence: String,
    pub rust_reference: String,
    pub independent_verifier: String,
    pub revm_silent_rollback: String,
    pub revm_core_mutations: String,
    pub revm_erc1271: String,
    pub revm_authority_rotation: String,
    #[serde(default)]
    pub transition_id: Option<String>,
    #[serde(default)]
    pub next_state_root: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct ReferenceRuntimeCase {
    pub status: String,
    #[serde(rename = "loader_invoked", alias = "loaderInvoked")]
    pub loader_invoked: bool,
    #[serde(default, rename = "recommendedAction", alias = "recommended_action")]
    pub recommended_action: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct ReferenceRuntimeEvidence {
    #[serde(rename = "reportType")]
    pub report_type: String,
    pub runtime: String,
    #[serde(rename = "evidenceSource")]
    pub evidence_source: String,
    #[serde(rename = "rawMemoryExported")]
    pub raw_memory_exported: bool,
    #[serde(rename = "allExpected")]
    pub all_expected: bool,
    pub cases: ReferenceRuntimeCases,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct ReferenceRuntimeCases {
    #[serde(rename = "currentHead")]
    pub current_head: ReferenceRuntimeCase,
    #[serde(rename = "historicalCheckpoint")]
    pub historical_checkpoint: ReferenceRuntimeCase,
    #[serde(rename = "divergedSnapshot")]
    pub diverged_snapshot: ReferenceRuntimeCase,
    #[serde(rename = "invalidEvidence")]
    pub invalid_evidence: ReferenceRuntimeInvalidCase,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct ReferenceRuntimeInvalidCase {
    pub status: String,
    #[serde(rename = "loaderInvoked", alias = "loader_invoked")]
    pub loader_invoked: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct SecurityAssuranceCase {
    pub status: String,
    pub expected: String,
    pub observed: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct SecurityAssuranceMatrix {
    pub total: usize,
    pub rejected: usize,
    pub all_expected: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct SecurityAssuranceInvariant {
    pub name: String,
    pub status: String,
    pub expected: String,
    pub observed: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct SecurityAssuranceEvidence {
    pub report_type: String,
    pub status: String,
    pub formal_status: String,
    pub valid_transitions: usize,
    pub stale_predecessor_cases: Vec<SecurityAssuranceCase>,
    pub sequence_gap_cases: Vec<SecurityAssuranceCase>,
    pub mutation_matrix: SecurityAssuranceMatrix,
    pub invariants: Vec<SecurityAssuranceInvariant>,
    pub raw_memory_exported: bool,
    pub limitations: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct PrivateSnapshotRecord {
    pub sequence: u64,
    pub file: String,
    #[serde(rename = "visibleLabel")]
    pub visible_label: Option<String>,
    #[serde(rename = "snapshotCommitment")]
    pub snapshot_commitment: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct SilentRollbackAttack {
    pub name: String,
    #[serde(rename = "restoredSequence")]
    pub restored_sequence: u64,
    #[serde(rename = "attemptedSequence")]
    pub attempted_sequence: u64,
    #[serde(rename = "expectedContractReason")]
    pub expected_contract_reason: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct PrivateFixtureManifest {
    #[serde(rename = "fixtureId")]
    pub fixture_id: String,
    pub synthetic: bool,
    pub description: String,
    #[serde(rename = "commitmentDomain")]
    pub commitment_domain: String,
    #[serde(default, rename = "generatedBy")]
    pub generated_by: Option<String>,
    pub snapshots: Vec<PrivateSnapshotRecord>,
    pub attack: SilentRollbackAttack,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiData {
    pub bundle: PublicReplayBundle,
    pub protocol_v2: EvidenceBundleV2,
    pub v2: EvidenceBundleV2,
    pub fixture: PrivateFixtureManifest,
    pub report: Option<VerificationReport>,
    pub recovery_receipt: RecoveryDecisionReceipt,
    pub deployment: DeploymentRecord,
    pub reread: SepoliaReread,
    pub conformance: ConformanceEvidence,
    pub reference_runtime: ReferenceRuntimeEvidence,
    pub security_assurance: SecurityAssuranceEvidence,
    pub portability: PortabilityRehearsalReport,
    pub mutation_count: usize,
    pub mutation_rejected: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HistoryLedgerEvent {
    pub order: usize,
    pub event: String,
    pub sequence: Option<u64>,
    pub reference: String,
    pub source: String,
    pub status: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RestoreAssessment {
    EvidenceHeadMatch { sequence: u64 },
    KnownHistoricalCheckpoint { sequence: u64, head_sequence: u64 },
    UnknownOrDiverged,
    Unverified { reason: String },
}

/// Change one nibble in a bytes32 commitment for the Inspector's local
/// preflight demonstration. This does not modify the SQLite fixture or chain.
pub fn tamper_commitment(commitment: &str) -> Option<String> {
    let encoded = commitment.strip_prefix("0x")?;
    if encoded.len() != 64 || !encoded.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return None;
    }
    let mut altered = encoded.as_bytes().to_vec();
    altered[0] = if altered[0].eq_ignore_ascii_case(&b'0') {
        b'1'
    } else {
        b'0'
    };
    String::from_utf8(altered)
        .ok()
        .map(|encoded| format!("0x{encoded}"))
}

/// Classify a candidate snapshot against a replay-verified evidence bundle.
/// This is an evidence-workspace assessment, not a live-chain observation or
/// an agent-runtime resume gate.
pub fn classify_restore_candidate(
    candidate_commitment: &str,
    bundle: &EvidenceBundleV2,
) -> RestoreAssessment {
    let Some(encoded) = candidate_commitment.strip_prefix("0x") else {
        return RestoreAssessment::Unverified {
            reason: "SNAPSHOT_COMMITMENT_INVALID".to_owned(),
        };
    };
    if encoded.len() != 64 || !encoded.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return RestoreAssessment::Unverified {
            reason: "SNAPSHOT_COMMITMENT_INVALID".to_owned(),
        };
    }
    if let Err(error) = verify_v2_bundle(bundle) {
        return RestoreAssessment::Unverified {
            reason: error.to_string(),
        };
    }

    let Some(last) = bundle.transitions.last() else {
        return RestoreAssessment::Unverified {
            reason: "EMPTY_TRANSITION_HISTORY".to_owned(),
        };
    };
    let matching_sequence = bundle
        .transitions
        .iter()
        .filter(|transition| {
            transition
                .delta
                .delta_commitment
                .eq_ignore_ascii_case(candidate_commitment)
        })
        .map(|transition| transition.delta.sequence)
        .max();

    match matching_sequence {
        Some(sequence) if sequence == last.delta.sequence => {
            RestoreAssessment::EvidenceHeadMatch { sequence }
        }
        Some(sequence) if sequence < last.delta.sequence => {
            RestoreAssessment::KnownHistoricalCheckpoint {
                sequence,
                head_sequence: last.delta.sequence,
            }
        }
        Some(_) => RestoreAssessment::Unverified {
            reason: "SNAPSHOT_SEQUENCE_AHEAD_OF_EVIDENCE_HEAD".to_owned(),
        },
        None => RestoreAssessment::UnknownOrDiverged,
    }
}

impl UiData {
    pub fn load() -> Self {
        let bundle: PublicReplayBundle =
            serde_json::from_str(LOCAL_EVIDENCE).expect("published evidence must remain valid");
        let protocol_v2 = public_replay_to_v2(&bundle);
        let v2: EvidenceBundleV2 =
            serde_json::from_str(DEMO_EVIDENCE).expect("Demo Space V2 evidence must remain valid");
        let fixture: PrivateFixtureManifest = serde_json::from_str(SILENT_ROLLBACK_MANIFEST)
            .expect("private fixture manifest must remain valid");
        let report = verify_v2_bundle(&v2).ok();
        let recovery_receipt: RecoveryDecisionReceipt = serde_json::from_str(DEMO_RECOVERY_RECEIPT)
            .expect("published recovery receipt must remain valid");
        verify_recovery_receipt(&recovery_receipt, &v2)
            .expect("published recovery receipt must verify against Demo Space V2");
        let deployment =
            serde_json::from_str(SEPOLIA_DEPLOYMENT).expect("Sepolia deployment evidence valid");
        let reread = serde_json::from_str(SEPOLIA_REREAD).expect("Sepolia reread evidence valid");
        let conformance =
            serde_json::from_str(CONFORMANCE_REPORT).expect("conformance evidence valid");
        let reference_runtime = serde_json::from_str(REFERENCE_RUNTIME_REPORT)
            .expect("reference agent runtime evidence valid");
        let security_assurance = serde_json::from_str(SECURITY_ASSURANCE_REPORT)
            .expect("bounded security assurance evidence valid");
        let portability = serde_json::from_str(POLKADOT_PORTABILITY_REPORT)
            .expect("Polkadot portability rehearsal evidence valid");
        let mutation_count = bundle
            .mutation_matrix
            .iter()
            .filter(|mutation| mutation.expected == "REJECT")
            .count();
        let mutation_rejected = bundle
            .mutation_matrix
            .iter()
            .filter(|mutation| {
                mutation.expected == "REJECT" && mutation.observed.status == "REJECT"
            })
            .count();
        Self {
            bundle,
            protocol_v2,
            v2,
            fixture,
            report,
            recovery_receipt,
            deployment,
            reread,
            conformance,
            reference_runtime,
            security_assurance,
            portability,
            mutation_count,
            mutation_rejected,
        }
    }

    pub fn evidence_json(&self) -> String {
        serde_json::to_string_pretty(&self.v2).expect("evidence v2 must serialize")
    }

    pub fn recovery_receipt_for_candidate(
        &self,
        sequence: u64,
        candidate_commitment: &str,
    ) -> Result<RecoveryDecisionReceipt, String> {
        build_recovery_receipt_with_snapshot_profile(
            &self.v2,
            candidate_commitment,
            sequence,
            "DEMO_SPACE_V2_LOCAL",
            None,
            SNAPSHOT_PROFILE_V2,
        )
        .map_err(|error| error.to_string())
    }

    pub fn recovery_receipt_json(&self) -> String {
        serde_json::to_string_pretty(&self.recovery_receipt)
            .expect("recovery receipt must serialize")
    }

    pub fn tampered_recovery_receipt_json(&self) -> String {
        let mut receipt = self.recovery_receipt.clone();
        receipt.decision.reason_code = "TAMPERED_REASON_CODE".to_owned();
        serde_json::to_string_pretty(&receipt).expect("tampered recovery receipt must serialize")
    }

    pub fn verify_recovery_receipt(&self) -> Result<RecoveryVerificationReport, String> {
        verify_recovery_receipt(&self.recovery_receipt, &self.v2).map_err(|error| error.to_string())
    }

    pub fn tampered_json(&self) -> String {
        let mut evidence = self.v2.clone();
        if let Some(first) = evidence.transitions.first_mut() {
            first.delta.locator_commitment = format!("0x{}", "ff".repeat(32));
        }
        serde_json::to_string_pretty(&evidence).expect("tampered evidence must serialize")
    }

    pub fn head(&self) -> &TransitionRecord {
        self.v2
            .transitions
            .last()
            .expect("Demo Space V2 history is non-empty")
    }

    pub fn transition(&self, sequence: u64) -> &TransitionRecord {
        self.v2
            .transitions
            .iter()
            .find(|transition| transition.delta.sequence == sequence)
            .unwrap_or_else(|| self.head())
    }

    pub fn snapshot_label(&self, sequence: u64) -> Option<&str> {
        self.fixture
            .snapshots
            .iter()
            .find(|snapshot| snapshot.sequence == sequence)
            .and_then(|snapshot| snapshot.visible_label.as_deref())
    }

    pub fn restored_snapshot(&self) -> &PrivateSnapshotRecord {
        self.fixture
            .snapshots
            .iter()
            .find(|snapshot| snapshot.sequence == self.fixture.attack.restored_sequence)
            .unwrap_or_else(|| {
                self.fixture
                    .snapshots
                    .first()
                    .expect("fixture is non-empty")
            })
    }

    pub fn canonical_snapshot(&self) -> &PrivateSnapshotRecord {
        self.fixture.snapshots.last().expect("fixture is non-empty")
    }

    pub fn assess_snapshot(&self, sequence: u64) -> RestoreAssessment {
        let Some(snapshot) = self
            .fixture
            .snapshots
            .iter()
            .find(|snapshot| snapshot.sequence == sequence)
        else {
            return RestoreAssessment::Unverified {
                reason: "SNAPSHOT_NOT_AVAILABLE_IN_FIXTURE".to_owned(),
            };
        };
        classify_restore_candidate(&snapshot.snapshot_commitment, &self.v2)
    }

    pub fn history_ledger(&self) -> Vec<HistoryLedgerEvent> {
        let mut events = self
            .v2
            .transitions
            .iter()
            .map(|transition| HistoryLedgerEvent {
                order: 0,
                event: "Transition committed".to_owned(),
                sequence: Some(transition.delta.sequence),
                reference: transition.next_state_root.clone(),
                source: "Demo Space V2 evidence".to_owned(),
                status: "COMMITTED".to_owned(),
            })
            .collect::<Vec<_>>();

        if let Some(attack) = self.v2.attack.as_ref()
            && let (Some(sequence), Some(stale_predecessor), Some(reason)) = (
                attack.attempted_sequence,
                attack.stale_predecessor.as_ref(),
                attack.reason.as_ref(),
            )
        {
            events.push(HistoryLedgerEvent {
                order: 0,
                event: "Rollback attempt".to_owned(),
                sequence: Some(sequence),
                reference: stale_predecessor.clone(),
                source: attack
                    .execution_source
                    .clone()
                    .unwrap_or_else(|| "Demo Space V2 attack record".to_owned()),
                status: format!("{} / {}", attack.status, reason),
            });
        }

        for (index, event) in events.iter_mut().enumerate() {
            event.order = index + 1;
        }
        events
    }
}

pub fn verify_evidence_json(json: &str) -> Result<VerificationReport, String> {
    match serde_json::from_str::<EvidenceBundleV2>(json) {
        Ok(bundle) => verify_v2_bundle(&bundle).map_err(|error| error.to_string()),
        Err(_) => match serde_json::from_str::<PublicReplayBundle>(json) {
            Ok(bundle) => {
                ml_verifier_independent::verify_bundle(&bundle).map_err(|error| error.to_string())
            }
            Err(_) => Err("SCHEMA_INVALID".to_owned()),
        },
    }
}

pub fn verify_recovery_receipt_json(
    receipt_json: &str,
    evidence_json: &str,
) -> Result<RecoveryVerificationReport, String> {
    let receipt: RecoveryDecisionReceipt = serde_json::from_str(receipt_json)
        .map_err(|_| "RECOVERY_RECEIPT_SCHEMA_INVALID".to_owned())?;
    let evidence: EvidenceBundleV2 =
        serde_json::from_str(evidence_json).map_err(|_| "EVIDENCE_V2_SCHEMA_INVALID".to_owned())?;
    verify_recovery_receipt(&receipt, &evidence).map_err(|error| error.to_string())
}

pub fn short_hash(value: &str, head: usize, tail: usize) -> String {
    if value.len() <= head + tail + 1 {
        return value.to_owned();
    }
    format!("{}…{}", &value[..head], &value[value.len() - tail..])
}

#[cfg(test)]
mod tests {
    use super::{
        RestoreAssessment, Scenario, UiData, classify_restore_candidate,
        verify_recovery_receipt_json,
    };

    #[test]
    fn scenario_routes_resolve_to_known_tampering_cases() {
        let cases = [
            ("silent-rollback", Scenario::SilentRollback),
            ("sequence-gap", Scenario::SequenceGap),
            ("parallel-history", Scenario::ParallelHistory),
            ("wrong-eoa-signer", Scenario::WrongEoa),
            ("locator-binding", Scenario::LocatorBinding),
            ("wrong-chain-domain", Scenario::WrongDomain),
            ("semantic-poisoning", Scenario::SemanticPoisoning),
        ];
        for (slug, expected) in cases {
            assert_eq!(Scenario::from_id(slug), expected, "{slug}");
        }
        assert_eq!(
            Scenario::from_id("unknown"),
            Scenario::SilentRollback,
            "unknown scenario slugs fail back to the hero case"
        );
    }

    #[test]
    fn demo_snapshot_labels_are_available_for_product_explanations() {
        let data = UiData::load();
        assert_eq!(data.snapshot_label(1), Some("Language preference recorded"));
        assert_eq!(
            data.snapshot_label(2),
            Some("Evidence-backed claim policy recorded")
        );
        assert_eq!(
            data.snapshot_label(3),
            Some("Raw-memory privacy boundary recorded")
        );
        assert_eq!(data.snapshot_label(4), None);
    }

    #[test]
    fn restore_preflight_distinguishes_head_from_historical_checkpoint() {
        let data = UiData::load();

        assert_eq!(
            data.assess_snapshot(3),
            RestoreAssessment::EvidenceHeadMatch { sequence: 3 }
        );
        assert_eq!(
            data.assess_snapshot(1),
            RestoreAssessment::KnownHistoricalCheckpoint {
                sequence: 1,
                head_sequence: 3,
            }
        );
    }

    #[test]
    fn restore_preflight_marks_unknown_commitment_as_diverged() {
        let data = UiData::load();
        let unknown = format!("0x{}", "ab".repeat(32));

        assert_eq!(
            classify_restore_candidate(&unknown, &data.v2),
            RestoreAssessment::UnknownOrDiverged
        );
    }

    #[test]
    fn tampered_snapshot_commitment_is_classified_as_diverged() {
        let data = UiData::load();
        let original = data.fixture.snapshots[2].snapshot_commitment.as_str();
        let tampered = super::tamper_commitment(original).expect("fixture commitment is bytes32");

        assert_ne!(tampered, original);
        assert_eq!(
            classify_restore_candidate(&tampered, &data.v2),
            RestoreAssessment::UnknownOrDiverged
        );
    }

    #[test]
    fn restore_preflight_fails_closed_when_history_does_not_replay() {
        let data = UiData::load();
        let candidate = data.fixture.snapshots[2].snapshot_commitment.clone();
        let mut invalid = data.v2.clone();
        invalid.transitions[0].transition_id = "0x00".to_owned();

        assert!(matches!(
            classify_restore_candidate(&candidate, &invalid),
            RestoreAssessment::Unverified { .. }
        ));
    }

    #[test]
    fn browser_receipt_path_verifies_current_head_and_rejects_tampering() {
        let data = UiData::load();
        let current = data
            .verify_recovery_receipt()
            .expect("published receipt should verify");
        assert_eq!(current.verdict, "VERIFIED");
        assert_eq!(current.classification, "CURRENT_HEAD");
        assert_eq!(current.recommended_action, "RESUME_ALLOWED");

        let historical = data
            .recovery_receipt_for_candidate(1, &data.fixture.snapshots[0].snapshot_commitment)
            .expect("historical candidate should produce a receipt");
        assert_eq!(
            historical.decision.classification,
            "KNOWN_HISTORICAL_CHECKPOINT"
        );
        assert_eq!(historical.decision.recommended_action, "REHEARSE_ONLY");

        let error = verify_recovery_receipt_json(
            &data.tampered_recovery_receipt_json(),
            &data.evidence_json(),
        )
        .expect_err("tampered decision must fail closed");
        assert_eq!(error, "RECOVERY_DECISION_MISMATCH");
    }

    #[test]
    fn bundled_workspace_data_loads_and_replays() {
        let data = UiData::load();
        assert_eq!(data.bundle.valid_history.len(), 4);
        assert_eq!(data.v2.transitions.len(), 3);
        assert_eq!(data.v2.head.sequence, 3);
        assert_eq!(data.bundle.mutation_matrix.len(), 21);
        assert_eq!(
            data.report
                .as_ref()
                .map(|report| report.attack_evidence.as_str()),
            Some("PASS")
        );
        assert_eq!(data.deployment.chain_id, "11155111");
        assert_eq!(data.fixture.snapshots.len(), 3);
        assert_eq!(data.fixture.snapshots[0].sequence, 1);
        assert!(data.reference_runtime.all_expected);
        assert_eq!(data.reference_runtime.cases.current_head.status, "RESUMED");
        assert!(data.reference_runtime.cases.current_head.loader_invoked);
        assert_eq!(
            data.reference_runtime
                .cases
                .historical_checkpoint
                .recommended_action
                .as_deref(),
            Some("REHEARSE_ONLY")
        );
        assert_eq!(data.security_assurance.status, "BOUNDED_ASSURANCE_PASS");
        assert_eq!(
            data.security_assurance.formal_status,
            "NOT_FORMALLY_VERIFIED"
        );
        assert_eq!(data.security_assurance.mutation_matrix.total, 20);
        assert!(!data.security_assurance.raw_memory_exported);
        assert_eq!(data.portability.status, "LOCAL_REHEARSAL_PASS");
        assert_eq!(data.portability.target.chain_id, "420420417");
        assert_eq!(data.portability.target.deployment, "NOT_PERFORMED");
        assert_eq!(
            data.portability.target.public_rpc_observation,
            "NOT_PERFORMED"
        );
        assert_eq!(
            data.portability.comparison.stale_predecessor_rejection,
            "BAD_PREVIOUS_STATE"
        );
        assert_eq!(
            data.fixture.attack.expected_contract_reason,
            "BAD_PREVIOUS_STATE"
        );
        assert!(
            data.restored_snapshot()
                .snapshot_commitment
                .starts_with("0x")
        );
        assert_eq!(
            super::verify_evidence_json(&data.evidence_json())
                .expect("bundled evidence should replay")
                .verdict,
            "VERIFIED"
        );
        assert_eq!(
            super::verify_evidence_json(&data.tampered_json())
                .expect_err("tampering the locator commitment must fail")
                .to_string(),
            "TRANSITION_ID_MISMATCH"
        );
    }

    #[test]
    fn history_ledger_uses_demo_bundle_and_separates_protocol_mutations() {
        let data = UiData::load();
        let events = data.history_ledger();

        assert_eq!(events.len(), 4);
        assert_eq!(events[0].event, "Transition committed");
        assert_eq!(events[0].sequence, Some(1));
        assert_eq!(events[1].sequence, Some(2));
        assert_eq!(events[0].status, "COMMITTED");
        assert_eq!(events[2].event, "Transition committed");
        assert_eq!(events[2].sequence, Some(3));
        assert_eq!(events[3].event, "Rollback attempt");
        assert_eq!(events[3].sequence, Some(4));
        assert_eq!(events[3].status, "REJECTED / BAD_PREVIOUS_STATE");
        assert_eq!(events[3].reference, data.v2.transitions[0].next_state_root);
        assert_eq!(
            events[3].source,
            "Rust/revm against published Solidity bytecode"
        );
        assert_eq!(data.mutation_rejected, 20);
        assert_eq!(data.mutation_count, 20);
    }
}
