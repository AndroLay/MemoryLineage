#![forbid(unsafe_code)]

use rusqlite::{Connection, params};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use thiserror::Error;

pub const SCHEMA_VERSION: i64 = 1;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MemorySnapshot {
    pub sequence: u64,
    pub values: BTreeMap<String, String>,
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
}
