#![forbid(unsafe_code)]

use ml_evidence::public_replay_to_v2;
use ml_spec_types::{EvidenceBundleV2, PublicReplayBundle, TransitionRecord};
use ml_verifier_independent::{VerificationReport, verify_v2_bundle};
use serde::Deserialize;

pub const LOCAL_EVIDENCE: &str =
    include_str!("../../../evidence/local/memory_lineage_evm_evidence.json");
const SEPOLIA_DEPLOYMENT: &str = include_str!("../../../evidence/sepolia/sepolia_deployment.json");
const SEPOLIA_REREAD: &str = include_str!("../../../evidence/sepolia/sepolia_reread.json");
const CONFORMANCE_REPORT: &str = include_str!("../../../evidence/local/rust_revm_conformance.json");

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

    pub fn id(self) -> &'static str {
        match self {
            Self::SilentRollback => "silent-rollback",
            Self::SequenceGap => "sequence-gap",
            Self::ParallelHistory => "parallel-history",
            Self::WrongEoa => "wrong-eoa-signer",
            Self::LocatorBinding => "locator-binding",
            Self::WrongDomain => "wrong-chain-domain",
            Self::SemanticPoisoning => "semantic-poisoning",
        }
    }

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
            Self::SilentRollback => "LIVE",
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
        Self::ALL
            .into_iter()
            .find(|scenario| scenario.id() == value)
            .unwrap_or(Self::SilentRollback)
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

impl Route {
    pub fn from_path(path: &str) -> Self {
        let clean = path.split('?').next().unwrap_or(path).trim_end_matches('/');
        match clean {
            "" => Self::Home,
            "/inspect" => Self::Inspect,
            "/history" => Self::History,
            "/lab" => Self::Lab(Scenario::SilentRollback),
            "/verify" => Self::Verify,
            "/evidence" => Self::Evidence,
            "/architecture" => Self::Architecture,
            "/security" => Self::Security,
            "/reproduce" => Self::Reproduce,
            "/prior-work" => Self::PriorWork,
            path if path.starts_with("/history/") => path
                .strip_prefix("/history/")
                .and_then(|sequence| sequence.parse().ok())
                .map(Self::Transition)
                .unwrap_or(Self::History),
            path if path.starts_with("/lab/") => Self::Lab(Scenario::from_id(
                path.strip_prefix("/lab/").unwrap_or("silent-rollback"),
            )),
            _ => Self::Home,
        }
    }
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiData {
    pub bundle: PublicReplayBundle,
    pub v2: EvidenceBundleV2,
    pub report: Option<VerificationReport>,
    pub deployment: DeploymentRecord,
    pub reread: SepoliaReread,
    pub conformance: ConformanceEvidence,
    pub mutation_count: usize,
    pub mutation_rejected: usize,
}

impl UiData {
    pub fn load() -> Self {
        let bundle: PublicReplayBundle =
            serde_json::from_str(LOCAL_EVIDENCE).expect("published evidence must remain valid");
        let v2 = public_replay_to_v2(&bundle);
        let report = verify_v2_bundle(&v2).ok();
        let deployment =
            serde_json::from_str(SEPOLIA_DEPLOYMENT).expect("Sepolia deployment evidence valid");
        let reread = serde_json::from_str(SEPOLIA_REREAD).expect("Sepolia reread evidence valid");
        let conformance =
            serde_json::from_str(CONFORMANCE_REPORT).expect("conformance evidence valid");
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
            v2,
            report,
            deployment,
            reread,
            conformance,
            mutation_count,
            mutation_rejected,
        }
    }

    pub fn evidence_json(&self) -> String {
        serde_json::to_string_pretty(&self.v2).expect("evidence v2 must serialize")
    }

    pub fn tampered_json(&self) -> String {
        let mut evidence = self.v2.clone();
        if let Some(first) = evidence.transitions.first_mut() {
            first.delta.locator_commitment = format!("0x{}", "ff".repeat(32));
        }
        serde_json::to_string_pretty(&evidence).expect("tampered evidence must serialize")
    }

    pub fn head(&self) -> &TransitionRecord {
        self.bundle
            .valid_history
            .last()
            .expect("published history is non-empty")
    }

    pub fn transition(&self, sequence: u64) -> &TransitionRecord {
        self.bundle
            .valid_history
            .iter()
            .find(|transition| transition.delta.sequence == sequence)
            .unwrap_or_else(|| self.head())
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

pub fn short_hash(value: &str, head: usize, tail: usize) -> String {
    if value.len() <= head + tail + 1 {
        return value.to_owned();
    }
    format!("{}…{}", &value[..head], &value[value.len() - tail..])
}

#[cfg(test)]
mod tests {
    use super::UiData;

    #[test]
    fn bundled_workspace_data_loads_and_replays() {
        let data = UiData::load();
        assert_eq!(data.bundle.valid_history.len(), 4);
        assert_eq!(data.bundle.mutation_matrix.len(), 21);
        assert!(data.report.is_some());
        assert_eq!(data.deployment.chain_id, "11155111");
    }
}
