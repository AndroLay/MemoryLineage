//! Submission packaging is separate from the protocol's V2 replay format.
//! Hashes bind checked-in bytes to a named source class; neither a hash nor a
//! self-declared class authenticates a public network observation.

use ml_spec_types::{
    SOURCE_DEMO_SPACE_V2_LOCAL, SOURCE_PROTOCOL_CORPUS_LOCAL, SOURCE_SEPOLIA_REFERENCE_OBSERVATION,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashSet;
use std::path::{Component, Path};
use thiserror::Error;

pub const SUBMISSION_SCHEMA: &str = "memorylineage-submission-v1";
pub const INCIDENT_SCHEMA: &str = "memorylineage-submission-incident-v1";
pub const ROLLBACK_SCHEMA: &str = "memorylineage-rollback-rehearsal-v1";

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SubmissionManifest {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "submissionCommit")]
    pub submission_commit: Option<String>,
    #[serde(rename = "incidentId")]
    pub incident_id: String,
    #[serde(rename = "sourceClasses")]
    pub source_classes: Vec<String>,
    pub artifacts: Vec<SubmissionArtifact>,
    pub claims: Vec<SubmissionClaim>,
    #[serde(rename = "verificationCommands")]
    pub verification_commands: Vec<String>,
    pub limitations: Vec<String>,
    #[serde(rename = "demoIncident")]
    pub demo_incident: SubmissionSection,
    #[serde(rename = "protocolCorpus")]
    pub protocol_corpus: SubmissionSection,
    #[serde(rename = "runtimeIntegration")]
    pub runtime_integration: SubmissionSection,
    #[serde(rename = "boundedAssurance")]
    pub bounded_assurance: SubmissionSection,
    #[serde(rename = "externalReproduction")]
    pub external_reproduction: SubmissionSection,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SubmissionArtifact {
    pub path: String,
    pub sha256: String,
    #[serde(rename = "sourceClass")]
    pub source_class: String,
    #[serde(rename = "generatedBy")]
    pub generated_by: String,
    pub status: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SubmissionClaim {
    pub id: String,
    pub status: String,
    #[serde(rename = "artifactPaths")]
    pub artifact_paths: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SubmissionSection {
    #[serde(rename = "sourceClass")]
    pub source_class: String,
    pub status: String,
    #[serde(rename = "artifactPath")]
    pub artifact_path: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct IncidentManifest {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "incidentId")]
    pub incident_id: String,
    #[serde(rename = "sourceClass")]
    pub source_class: String,
    #[serde(rename = "fixtureId")]
    pub fixture_id: String,
    pub canonical: IncidentCanonical,
    #[serde(rename = "restoredSnapshot")]
    pub restored_snapshot: IncidentSnapshot,
    pub attempt: IncidentAttempt,
    pub artifacts: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct IncidentCanonical {
    #[serde(rename = "headSequence")]
    pub head_sequence: u64,
    #[serde(rename = "headStateRoot")]
    pub head_state_root: String,
    pub evidence: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct IncidentSnapshot {
    pub sequence: u64,
    pub snapshot: String,
    #[serde(rename = "snapshotCommitment")]
    pub snapshot_commitment: String,
    #[serde(rename = "staleRoot")]
    pub stale_root: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct IncidentAttempt {
    pub sequence: u64,
    #[serde(rename = "revertReason")]
    pub revert_reason: String,
    #[serde(rename = "transactionBroadcast")]
    pub transaction_broadcast: bool,
    pub execution: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RollbackRehearsal {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "incidentId")]
    pub incident_id: String,
    #[serde(rename = "sourceClass")]
    pub source_class: String,
    #[serde(rename = "snapshotCommitment")]
    pub snapshot_commitment: String,
    #[serde(rename = "stalePredecessor")]
    pub stale_predecessor: String,
    #[serde(rename = "canonicalHead")]
    pub canonical_head: String,
    #[serde(rename = "attemptedSequence")]
    pub attempted_sequence: u64,
    #[serde(rename = "revertReason")]
    pub revert_reason: String,
    #[serde(rename = "transactionBroadcast")]
    pub transaction_broadcast: bool,
    #[serde(rename = "executionSource")]
    pub execution_source: String,
}

#[derive(Debug, Error)]
pub enum SubmissionError {
    #[error("SUBMISSION_SCHEMA_MISMATCH")]
    Schema,
    #[error("SUBMISSION_MANIFEST_INVALID: {0}")]
    Invalid(String),
    #[error("SUBMISSION_ARTIFACT_PATH_INVALID: {0}")]
    Path(String),
    #[error("SUBMISSION_ARTIFACT_HASH_MISMATCH: {0}")]
    Hash(String),
    #[error("SUBMISSION_IO: {0}")]
    Io(#[from] std::io::Error),
    #[error("SUBMISSION_JSON: {0}")]
    Json(#[from] serde_json::Error),
}

pub fn read_manifest(path: impl AsRef<Path>) -> Result<SubmissionManifest, SubmissionError> {
    let manifest: SubmissionManifest = serde_json::from_slice(&std::fs::read(path)?)?;
    if manifest.schema_version != SUBMISSION_SCHEMA {
        return Err(SubmissionError::Schema);
    }
    Ok(manifest)
}

pub fn artifact_sha256(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

pub fn artifact_path(root: &Path, relative: &str) -> Result<std::path::PathBuf, SubmissionError> {
    let candidate = Path::new(relative);
    if candidate.as_os_str().is_empty()
        || !candidate
            .components()
            .all(|component| matches!(component, Component::Normal(_)))
    {
        return Err(SubmissionError::Path(relative.to_owned()));
    }
    let canonical_root = root.canonicalize()?;
    let full = root
        .join(candidate)
        .canonicalize()
        .map_err(|_| SubmissionError::Path(relative.to_owned()))?;
    if !full.starts_with(canonical_root) || !full.is_file() {
        return Err(SubmissionError::Path(relative.to_owned()));
    }
    Ok(full)
}

pub fn verify_artifacts(manifest: &SubmissionManifest, root: &Path) -> Result<(), SubmissionError> {
    if manifest.artifacts.is_empty()
        || manifest.verification_commands.is_empty()
        || manifest.claims.is_empty()
        || manifest.source_classes.is_empty()
        || manifest.source_classes.iter().any(|source| {
            !matches!(
                source.as_str(),
                SOURCE_DEMO_SPACE_V2_LOCAL
                    | SOURCE_PROTOCOL_CORPUS_LOCAL
                    | SOURCE_SEPOLIA_REFERENCE_OBSERVATION
                    | "EXTERNAL_REPRODUCTION"
            )
        })
    {
        return Err(SubmissionError::Invalid(
            "missing or unsupported package fields".to_owned(),
        ));
    }
    let mut seen = HashSet::new();
    for artifact in &manifest.artifacts {
        if !seen.insert(artifact.path.as_str()) {
            return Err(SubmissionError::Invalid(format!(
                "duplicate artifact: {}",
                artifact.path
            )));
        }
        if !manifest.source_classes.contains(&artifact.source_class)
            || artifact.generated_by.is_empty()
            || !matches!(
                artifact.status.as_str(),
                "VERIFIED_LOCAL" | "BOUNDED_LOCAL" | "OBSERVED"
            )
        {
            return Err(SubmissionError::Invalid(format!(
                "incomplete artifact: {}",
                artifact.path
            )));
        }
        let bytes = std::fs::read(artifact_path(root, &artifact.path)?)?;
        if artifact_sha256(&bytes) != artifact.sha256 {
            return Err(SubmissionError::Hash(artifact.path.clone()));
        }
    }
    let mut claim_ids = HashSet::new();
    for claim in &manifest.claims {
        if claim.id.is_empty()
            || !claim_ids.insert(claim.id.as_str())
            || !matches!(
                claim.status.as_str(),
                "VERIFIED_LOCAL" | "BOUNDED_LOCAL" | "OBSERVED" | "NOT_YET_DEMONSTRATED"
            )
            || (claim.status != "NOT_YET_DEMONSTRATED" && claim.artifact_paths.is_empty())
            || (claim.status == "NOT_YET_DEMONSTRATED" && !claim.artifact_paths.is_empty())
            || claim
                .artifact_paths
                .iter()
                .any(|path| !seen.contains(path.as_str()))
        {
            return Err(SubmissionError::Invalid(format!(
                "incomplete claim: {}",
                claim.id
            )));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn current_manifest() -> SubmissionManifest {
        read_manifest(
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../evidence/submission/manifest.json"),
        )
        .expect("published submission envelope is typed")
    }

    #[test]
    fn artifact_hash_and_path_checks_reject_tampering_and_escape() {
        let root = std::env::temp_dir().join(format!(
            "memorylineage-submission-test-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("clock should be after epoch")
                .as_nanos()
        ));
        std::fs::create_dir(&root).expect("temporary directory");
        std::fs::write(root.join("artifact.json"), b"hello").expect("fixture artifact");
        let mut manifest = current_manifest();
        manifest.artifacts = vec![SubmissionArtifact {
            path: "artifact.json".to_owned(),
            sha256: "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824".to_owned(),
            source_class: SOURCE_DEMO_SPACE_V2_LOCAL.to_owned(),
            generated_by: "test fixture".to_owned(),
            status: "VERIFIED_LOCAL".to_owned(),
        }];
        manifest.claims = vec![SubmissionClaim {
            id: "hash-check".to_owned(),
            status: "VERIFIED_LOCAL".to_owned(),
            artifact_paths: vec!["artifact.json".to_owned()],
        }];
        assert!(verify_artifacts(&manifest, &root).is_ok());
        std::fs::write(root.join("artifact.json"), b"hellO").expect("tampered fixture");
        assert!(
            matches!(verify_artifacts(&manifest, &root), Err(SubmissionError::Hash(path)) if path == "artifact.json")
        );
        manifest.artifacts[0].path = "../artifact.json".to_owned();
        assert!(
            matches!(verify_artifacts(&manifest, &root), Err(SubmissionError::Path(path)) if path == "../artifact.json")
        );
        std::fs::remove_dir_all(root).expect("temporary directory cleanup");
    }

    #[test]
    fn security_critical_unknown_manifest_fields_are_rejected() {
        let mut manifest: serde_json::Value = serde_json::from_slice(
            &std::fs::read(
                Path::new(env!("CARGO_MANIFEST_DIR"))
                    .join("../../evidence/submission/manifest.json"),
            )
            .expect("published manifest exists"),
        )
        .expect("valid JSON");
        manifest["artifacts"][0]["privateLocator"] = serde_json::json!("secret");
        assert!(serde_json::from_value::<SubmissionManifest>(manifest).is_err());
    }
}
