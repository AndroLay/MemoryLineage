#![forbid(unsafe_code)]

use rusqlite::{Connection, params};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use thiserror::Error;

pub const SCHEMA_VERSION: i64 = 1;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MemorySnapshot {
    pub sequence: u64,
    pub values: BTreeMap<String, String>,
}

/// A public observation of a private snapshot. It deliberately contains only
/// the deterministic commitment, never the snapshot values themselves.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SnapshotObservation {
    pub sequence: u64,
    pub file: String,
    #[serde(rename = "visibleLabel", skip_serializing_if = "Option::is_none")]
    pub visible_label: Option<String>,
    #[serde(rename = "snapshotCommitment")]
    pub snapshot_commitment: String,
}

#[derive(Debug, Error)]
pub enum MemoryStoreError {
    #[error("SQLite error: {0}")]
    Sqlite(#[from] rusqlite::Error),
    #[error("filesystem error: {0}")]
    Io(#[from] std::io::Error),
    #[error("snapshot sequence is invalid: {0}")]
    InvalidSequence(i64),
    #[error("snapshot sequence does not fit SQLite INTEGER: {0}")]
    SequenceTooLarge(u64),
    #[error("snapshot has inconsistent sequence values")]
    InconsistentSequence,
    #[error("snapshot user_version is {actual}, expected {expected}")]
    SchemaVersion { actual: i64, expected: i64 },
    #[error("snapshot blinding secret must not be all zeroes")]
    ZeroBlindingSecret,
}

const SCHEMA: &str = r#"
CREATE TABLE memory_state (
  key TEXT PRIMARY KEY,
  value TEXT NOT NULL,
  state_sequence INTEGER NOT NULL
);
PRAGMA user_version = 1;
"#;

fn open_snapshot(path: &Path) -> Result<Connection, MemoryStoreError> {
    Ok(Connection::open(path)?)
}

fn create_schema(connection: &Connection) -> Result<(), MemoryStoreError> {
    connection.execute_batch(SCHEMA)?;
    Ok(())
}

pub fn write_snapshot(
    path: impl AsRef<Path>,
    sequence: u64,
    values: &BTreeMap<String, String>,
) -> Result<(), MemoryStoreError> {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    if path.exists() {
        std::fs::remove_file(path)?;
    }
    let connection = open_snapshot(path)?;
    create_schema(&connection)?;
    let transaction = connection.unchecked_transaction()?;
    let database_sequence =
        i64::try_from(sequence).map_err(|_| MemoryStoreError::SequenceTooLarge(sequence))?;
    for (key, value) in values {
        transaction.execute(
            "INSERT INTO memory_state(key, value, state_sequence) VALUES (?1, ?2, ?3)",
            params![key, value, database_sequence],
        )?;
    }
    transaction.commit()?;
    Ok(())
}

pub fn read_snapshot(path: impl AsRef<Path>) -> Result<MemorySnapshot, MemoryStoreError> {
    let connection = open_snapshot(path.as_ref())?;
    let actual_schema: i64 = connection.query_row("PRAGMA user_version", [], |row| row.get(0))?;
    if actual_schema != SCHEMA_VERSION {
        return Err(MemoryStoreError::SchemaVersion {
            actual: actual_schema,
            expected: SCHEMA_VERSION,
        });
    }

    let mut statement = connection
        .prepare("SELECT key, value, state_sequence FROM memory_state ORDER BY key ASC")?;
    let rows = statement.query_map([], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, i64>(2)?,
        ))
    })?;

    let mut values = BTreeMap::new();
    let mut sequence = None;
    for row in rows {
        let (key, value, raw_sequence) = row?;
        let row_sequence = u64::try_from(raw_sequence)
            .map_err(|_| MemoryStoreError::InvalidSequence(raw_sequence))?;
        if let Some(expected) = sequence
            && expected != row_sequence
        {
            return Err(MemoryStoreError::InconsistentSequence);
        }
        sequence = Some(row_sequence);
        values.insert(key, value);
    }

    Ok(MemorySnapshot {
        sequence: sequence.unwrap_or_default(),
        values,
    })
}

pub fn restore_snapshot(
    source: impl AsRef<Path>,
    destination: impl AsRef<Path>,
) -> Result<(), MemoryStoreError> {
    let snapshot = read_snapshot(source)?;
    write_snapshot(destination, snapshot.sequence, &snapshot.values)
}

/// Derive the legacy V1 deterministic commitment for a private snapshot.
///
/// This delimiter-based encoding is retained so existing fixture and evidence
/// hashes remain reproducible. New portable artifacts should use
/// [`snapshot_commitment_v2`] instead.
pub fn snapshot_commitment(snapshot: &MemorySnapshot) -> String {
    let mut canonical = format!(
        "memorylineage/private-snapshot/v1|sequence={}|",
        snapshot.sequence
    );
    for (key, value) in &snapshot.values {
        canonical.push_str(key);
        canonical.push('=');
        canonical.push_str(value);
        canonical.push('|');
    }
    ml_core::keccak_text(&canonical)
}

/// Derive the V2 deterministic commitment for a private snapshot without
/// exposing its contents.
///
/// V2 uses a domain-separated, length-prefixed binary encoding. Keys are
/// traversed in `BTreeMap` order, so the result is deterministic while values
/// containing delimiters cannot collide with a different key/value layout.
/// This is a fixture commitment, not a claim that the raw SQLite state is a
/// semantic memory root.
pub fn snapshot_commitment_v2(snapshot: &MemorySnapshot) -> String {
    ml_core::format_hex(&ml_core::keccak256(&canonical_snapshot_v2_bytes(snapshot)))
}

/// Prepare a space-bound, secret-blinded commitment for a private snapshot.
///
/// The caller must generate a fresh, high-entropy 32-byte secret and retain it
/// privately for later recomputation. This helper does not manage secrets or
/// change the published, reproducible Demo Space V2 evidence profile.
pub fn snapshot_commitment_blinded_v1(
    snapshot: &MemorySnapshot,
    space_id: &[u8; 32],
    blinding_secret: &[u8; 32],
) -> Result<String, MemoryStoreError> {
    if blinding_secret.iter().all(|byte| *byte == 0) {
        return Err(MemoryStoreError::ZeroBlindingSecret);
    }
    let mut input = Vec::new();
    input.extend_from_slice(ml_spec_types::SNAPSHOT_PROFILE_BLINDED_V1.as_bytes());
    input.push(0);
    input.extend_from_slice(space_id);
    input.extend_from_slice(blinding_secret);
    input.extend_from_slice(&canonical_snapshot_v2_bytes(snapshot));
    Ok(ml_core::format_hex(&ml_core::keccak256(&input)))
}

fn canonical_snapshot_v2_bytes(snapshot: &MemorySnapshot) -> Vec<u8> {
    let mut canonical = Vec::new();
    canonical.extend_from_slice(ml_spec_types::SNAPSHOT_PROFILE_V2.as_bytes());
    canonical.extend_from_slice(&snapshot.sequence.to_be_bytes());
    canonical.extend_from_slice(
        &u64::try_from(snapshot.values.len())
            .expect("a BTreeMap length must fit in the canonical u64 count")
            .to_be_bytes(),
    );

    for (key, value) in &snapshot.values {
        append_length_prefixed(&mut canonical, key.as_bytes());
        append_length_prefixed(&mut canonical, value.as_bytes());
    }

    canonical
}

fn append_length_prefixed(output: &mut Vec<u8>, value: &[u8]) {
    output.extend_from_slice(
        &u64::try_from(value.len())
            .expect("a string length must fit in the canonical u64 length")
            .to_be_bytes(),
    );
    output.extend_from_slice(value);
}

pub fn snapshot_observations(
    root: impl AsRef<Path>,
) -> Result<Vec<SnapshotObservation>, MemoryStoreError> {
    let root = root.as_ref();
    inspect_silent_rollback_fixture(root)?
        .into_iter()
        .map(|snapshot| {
            let file = format!("snapshot-{}.db", snapshot.sequence);
            Ok(SnapshotObservation {
                sequence: snapshot.sequence,
                file,
                visible_label: None,
                snapshot_commitment: snapshot_commitment(&snapshot),
            })
        })
        .collect()
}

/// The final judge-facing fixture uses registry-aligned sequence numbers. The
/// original 17/18/19 fixture remains available as historical provenance and is
/// intentionally not rewritten by this helper.
pub fn demo_space_v2_states() -> [(u64, &'static [(&'static str, &'static str)]); 3] {
    [
        (1, &[("language", "Indonesian")]),
        (
            2,
            &[
                ("language", "Indonesian"),
                ("claim_policy", "evidence-backed"),
            ],
        ),
        (
            3,
            &[
                ("language", "Indonesian"),
                ("claim_policy", "evidence-backed"),
                ("privacy", "raw-memory-private"),
            ],
        ),
    ]
}

pub fn demo_space_v2_paths(root: impl AsRef<Path>) -> [PathBuf; 3] {
    let root = root.as_ref();
    [
        root.join("snapshot-1.db"),
        root.join("snapshot-2.db"),
        root.join("snapshot-3.db"),
    ]
}

pub fn create_demo_space_v2_fixture(root: impl AsRef<Path>) -> Result<(), MemoryStoreError> {
    let root = root.as_ref();
    for ((sequence, entries), path) in demo_space_v2_states()
        .into_iter()
        .zip(demo_space_v2_paths(root))
    {
        let values = entries
            .iter()
            .map(|&(key, value)| (key.to_owned(), value.to_owned()))
            .collect();
        write_snapshot(path, sequence, &values)?;
    }
    Ok(())
}

pub fn inspect_demo_space_v2_fixture(
    root: impl AsRef<Path>,
) -> Result<[MemorySnapshot; 3], MemoryStoreError> {
    let snapshots = demo_space_v2_paths(root)
        .into_iter()
        .map(read_snapshot)
        .collect::<Result<Vec<_>, _>>()?;
    snapshots
        .try_into()
        .map_err(|_| MemoryStoreError::InconsistentSequence)
}

pub fn demo_space_v2_observations(
    root: impl AsRef<Path>,
) -> Result<Vec<SnapshotObservation>, MemoryStoreError> {
    let observations = inspect_demo_space_v2_fixture(root)?
        .into_iter()
        .map(|snapshot| SnapshotObservation {
            sequence: snapshot.sequence,
            file: format!("snapshot-{}.db", snapshot.sequence),
            visible_label: None,
            snapshot_commitment: snapshot_commitment_v2(&snapshot),
        })
        .collect::<Vec<_>>();
    Ok(observations)
}

pub fn silent_rollback_states() -> [(u64, &'static [(&'static str, &'static str)]); 3] {
    [
        (17, &[("language", "Indonesian")]),
        (
            18,
            &[
                ("language", "Indonesian"),
                ("claim_policy", "evidence-backed"),
            ],
        ),
        (
            19,
            &[
                ("language", "Indonesian"),
                ("claim_policy", "evidence-backed"),
                ("privacy", "raw-memory-private"),
            ],
        ),
    ]
}

pub fn fixture_paths(root: impl AsRef<Path>) -> [PathBuf; 3] {
    let root = root.as_ref();
    [
        root.join("snapshot-17.db"),
        root.join("snapshot-18.db"),
        root.join("snapshot-19.db"),
    ]
}

pub fn create_silent_rollback_fixture(root: impl AsRef<Path>) -> Result<(), MemoryStoreError> {
    let root = root.as_ref();
    for ((sequence, entries), path) in silent_rollback_states()
        .into_iter()
        .zip(fixture_paths(root))
    {
        let values = entries
            .iter()
            .map(|&(key, value)| (key.to_owned(), value.to_owned()))
            .collect();
        write_snapshot(path, sequence, &values)?;
    }
    Ok(())
}

pub fn inspect_silent_rollback_fixture(
    root: impl AsRef<Path>,
) -> Result<[MemorySnapshot; 3], MemoryStoreError> {
    let snapshots = fixture_paths(root)
        .into_iter()
        .map(read_snapshot)
        .collect::<Result<Vec<_>, _>>()?;
    snapshots
        .try_into()
        .map_err(|_| MemoryStoreError::InconsistentSequence)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn existing_fixture_root() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/silent-rollback")
    }

    #[test]
    fn existing_python_fixture_has_expected_logical_states() {
        let snapshots = inspect_silent_rollback_fixture(existing_fixture_root())
            .expect("existing fixture should be readable by Rust");
        assert_eq!(snapshots[0].sequence, 17);
        assert_eq!(snapshots[0].values["language"], "Indonesian");
        assert_eq!(snapshots[1].sequence, 18);
        assert_eq!(snapshots[1].values["claim_policy"], "evidence-backed");
        assert_eq!(snapshots[2].sequence, 19);
        assert_eq!(snapshots[2].values["privacy"], "raw-memory-private");
    }

    #[test]
    fn rust_fixture_generation_is_deterministic() {
        let root =
            std::env::temp_dir().join(format!("memorylineage-rust-fixture-{}", std::process::id()));
        create_silent_rollback_fixture(&root).expect("Rust fixture should be generated");
        let first = inspect_silent_rollback_fixture(&root).expect("generated fixture should read");
        create_silent_rollback_fixture(&root).expect("fixture should be repeatable");
        let second = inspect_silent_rollback_fixture(&root).expect("fixture should read twice");
        assert_eq!(first, second);
        let _ = std::fs::remove_dir_all(root);
    }

    #[test]
    fn restore_keeps_private_state_local() {
        let root =
            std::env::temp_dir().join(format!("memorylineage-rust-restore-{}", std::process::id()));
        create_silent_rollback_fixture(&root).expect("fixture should be generated");
        let restored = root.join("restored.db");
        restore_snapshot(root.join("snapshot-17.db"), &restored).expect("restore should work");
        let snapshot = read_snapshot(restored).expect("restored snapshot should read");
        assert_eq!(snapshot.sequence, 17);
        assert_eq!(snapshot.values["language"], "Indonesian");
        assert!(!snapshot.values.contains_key("privacy"));
        let _ = std::fs::remove_dir_all(root);
    }

    #[test]
    fn snapshot_commitment_is_deterministic_and_private() {
        let snapshots = inspect_silent_rollback_fixture(existing_fixture_root())
            .expect("existing fixture should be readable by Rust");
        let first = snapshot_commitment(&snapshots[0]);
        let second = snapshot_commitment(&snapshots[0]);
        assert_eq!(first, second);
        assert!(first.starts_with("0x"));
        assert_eq!(first.len(), 66);
        assert_ne!(first, snapshot_commitment(&snapshots[1]));
        assert!(!first.contains("Indonesian"));
    }

    #[test]
    fn v2_snapshot_commitment_separates_delimiter_ambiguous_values() {
        let first = MemorySnapshot {
            sequence: 1,
            values: [("a".to_owned(), "b=c|d".to_owned())].into_iter().collect(),
        };
        let second = MemorySnapshot {
            sequence: 1,
            values: [("a=b".to_owned(), "c|d".to_owned())].into_iter().collect(),
        };

        assert_eq!(
            snapshot_commitment(&first),
            snapshot_commitment(&second),
            "the legacy V1 encoding documents the ambiguity this profile fixes"
        );
        assert_ne!(
            snapshot_commitment_v2(&first),
            snapshot_commitment_v2(&second)
        );
    }

    #[test]
    fn blinded_commitment_binds_secret_and_space_and_rejects_zero_secret() {
        let snapshot = MemorySnapshot {
            sequence: 1,
            values: [("language".to_owned(), "Indonesian".to_owned())]
                .into_iter()
                .collect(),
        };
        let first = snapshot_commitment_blinded_v1(&snapshot, &[1; 32], &[7; 32])
            .expect("nonzero blinding secret");
        assert_eq!(
            first,
            snapshot_commitment_blinded_v1(&snapshot, &[1; 32], &[7; 32]).unwrap()
        );
        assert_ne!(
            first,
            snapshot_commitment_blinded_v1(&snapshot, &[1; 32], &[8; 32]).unwrap()
        );
        assert_ne!(
            first,
            snapshot_commitment_blinded_v1(&snapshot, &[2; 32], &[7; 32]).unwrap()
        );
        assert_ne!(first, snapshot_commitment_v2(&snapshot));
        assert!(matches!(
            snapshot_commitment_blinded_v1(&snapshot, &[1; 32], &[0; 32]),
            Err(MemoryStoreError::ZeroBlindingSecret)
        ));
    }

    #[test]
    fn demo_space_v2_uses_registry_aligned_sequences_without_rewriting_v1() {
        let root = std::env::temp_dir().join(format!(
            "memorylineage-demo-space-v2-{}",
            std::process::id()
        ));
        create_demo_space_v2_fixture(&root).expect("V2 fixture should be generated");
        let snapshots = inspect_demo_space_v2_fixture(&root).expect("V2 fixture should read");
        assert_eq!(
            snapshots
                .iter()
                .map(|snapshot| snapshot.sequence)
                .collect::<Vec<_>>(),
            [1, 2, 3]
        );
        assert_eq!(snapshots[2].values["privacy"], "raw-memory-private");
        let observations = demo_space_v2_observations(&root).expect("observations should derive");
        assert_eq!(observations.len(), 3);
        assert_eq!(
            observations[0].snapshot_commitment,
            snapshot_commitment_v2(&snapshots[0])
        );
        assert!(!observations[0].snapshot_commitment.contains("Indonesian"));
        let _ = std::fs::remove_dir_all(root);
    }
}
