#![forbid(unsafe_code)]

use ml_spec_types::ExperienceDelta;
use thiserror::Error;
use tiny_keccak::{Hasher, Keccak};

pub const ZERO_ROOT: &str = "0x0000000000000000000000000000000000000000000000000000000000000000";
pub const EIP712_NAME: &str = "AgentMemoryState";
pub const EIP712_VERSION: &str = "1";
pub const EXPERIENCE_DELTA_TYPE: &str = "ExperienceDelta(bytes32 spaceId,uint64 sequence,bytes32 prevStateRoot,bytes32 deltaCommitment,bytes32 provenanceCommitment,bytes32 profileId,bytes32 locatorCommitment)";
pub const MEMORY_STATE_TYPE: &str = "MemoryState(bytes32 prevStateRoot,bytes32 transitionId)";
pub const MEMORY_SPACE_TYPE: &str = "MemorySpace(address initialController,bytes32 salt)";
pub const SPACE_REGISTRATION_TYPE: &str =
    "SpaceRegistration(bytes32 spaceId,address controller,address authorizer)";
pub const SPACE_AUTHORIZATION_TYPE: &str =
    "SpaceAuthorization(bytes32 spaceId,address newController,address newAuthorizer,uint64 nonce)";
pub const EIP712_DOMAIN_TYPE: &str =
    "EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)";

#[derive(Debug, Error, PartialEq, Eq)]
pub enum CoreError {
    #[error("hex value must have a 0x prefix")]
    MissingHexPrefix,
    #[error("hex value has invalid characters")]
    InvalidHex,
    #[error("hex value has length {actual}, expected {expected} bytes")]
    InvalidLength { actual: usize, expected: usize },
    #[error("uint64 value must fit in an ABI word")]
    InvalidUint,
}

pub type CoreResult<T> = Result<T, CoreError>;

pub fn keccak256(value: &[u8]) -> [u8; 32] {
    let mut output = [0u8; 32];
    let mut hasher = Keccak::v256();
    hasher.update(value);
    hasher.finalize(&mut output);
    output
}

pub fn keccak_text(value: &str) -> String {
    format_hex(&keccak256(value.as_bytes()))
}

pub fn format_hex(value: &[u8]) -> String {
    format!("0x{}", hex::encode(value))
}

pub fn parse_hex(value: &str, expected_len: usize) -> CoreResult<Vec<u8>> {
    let encoded = value
        .strip_prefix("0x")
        .ok_or(CoreError::MissingHexPrefix)?;
    let decoded = hex::decode(encoded).map_err(|_| CoreError::InvalidHex)?;
    if decoded.len() != expected_len {
        return Err(CoreError::InvalidLength {
            actual: decoded.len(),
            expected: expected_len,
        });
    }
    Ok(decoded)
}

pub fn bytes32(value: &str) -> CoreResult<[u8; 32]> {
    parse_hex(value, 32).map(|decoded| decoded.try_into().expect("length checked"))
}

pub fn address(value: &str) -> CoreResult<[u8; 20]> {
    parse_hex(value, 20).map(|decoded| decoded.try_into().expect("length checked"))
}

fn word_bytes32(value: &str) -> CoreResult<[u8; 32]> {
    bytes32(value)
}

fn word_address(value: &str) -> CoreResult<[u8; 32]> {
    let raw = address(value)?;
    let mut word = [0u8; 32];
    word[12..].copy_from_slice(&raw);
    Ok(word)
}

fn word_uint(value: u64) -> [u8; 32] {
    let mut word = [0u8; 32];
    word[24..].copy_from_slice(&value.to_be_bytes());
    word
}

fn abi_words(words: &[[u8; 32]]) -> Vec<u8> {
    words.iter().flat_map(|word| word.iter().copied()).collect()
}

pub fn type_hash(type_string: &str) -> String {
    keccak_text(type_string)
}

pub fn experience_delta_type_hash() -> String {
    type_hash(EXPERIENCE_DELTA_TYPE)
}

pub fn memory_state_type_hash() -> String {
    type_hash(MEMORY_STATE_TYPE)
}

pub fn memory_space_type_hash() -> String {
    type_hash(MEMORY_SPACE_TYPE)
}

pub fn space_registration_type_hash() -> String {
    type_hash(SPACE_REGISTRATION_TYPE)
}

pub fn space_authorization_type_hash() -> String {
    type_hash(SPACE_AUTHORIZATION_TYPE)
}

pub fn eip712_domain_type_hash() -> String {
    type_hash(EIP712_DOMAIN_TYPE)
}

pub fn commitment(value: &str, salt: &str) -> String {
    let mut input = String::with_capacity(salt.len() + value.len());
    input.push_str(salt);
    input.push_str(value);
    format_hex(&keccak256(input.as_bytes()))
}

pub fn space_id(initial_controller: &str, salt: &str) -> CoreResult<String> {
    let words = [
        word_bytes32(&memory_space_type_hash())?,
        word_address(initial_controller)?,
        word_bytes32(salt)?,
    ];
    Ok(format_hex(&keccak256(&abi_words(&words))))
}

pub fn transition_id(delta: &ExperienceDelta) -> CoreResult<String> {
    let words = [
        word_bytes32(&experience_delta_type_hash())?,
        word_bytes32(&delta.space_id)?,
        word_uint(delta.sequence),
        word_bytes32(&delta.prev_state_root)?,
        word_bytes32(&delta.delta_commitment)?,
        word_bytes32(&delta.provenance_commitment)?,
        word_bytes32(&delta.profile_id)?,
        word_bytes32(&delta.locator_commitment)?,
    ];
    Ok(format_hex(&keccak256(&abi_words(&words))))
}

pub fn next_state_root(previous_root: &str, transition: &str) -> CoreResult<String> {
    let words = [
        word_bytes32(&memory_state_type_hash())?,
        word_bytes32(previous_root)?,
        word_bytes32(transition)?,
    ];
    Ok(format_hex(&keccak256(&abi_words(&words))))
}

pub fn registration_id(space: &str, controller: &str, authorizer: &str) -> CoreResult<String> {
    let words = [
        word_bytes32(&space_registration_type_hash())?,
        word_bytes32(space)?,
        word_address(controller)?,
        word_address(authorizer)?,
    ];
    Ok(format_hex(&keccak256(&abi_words(&words))))
}

pub fn authorization_id(
    space: &str,
    new_controller: &str,
    new_authorizer: &str,
    nonce: u64,
) -> CoreResult<String> {
    let words = [
        word_bytes32(&space_authorization_type_hash())?,
        word_bytes32(space)?,
        word_address(new_controller)?,
        word_address(new_authorizer)?,
        word_uint(nonce),
    ];
    Ok(format_hex(&keccak256(&abi_words(&words))))
}

pub fn domain_separator(chain_id: u64, verifying_contract: &str) -> CoreResult<String> {
    let words = [
        word_bytes32(&eip712_domain_type_hash())?,
        word_bytes32(&keccak_text(EIP712_NAME))?,
        word_bytes32(&keccak_text(EIP712_VERSION))?,
        word_uint(chain_id),
        word_address(verifying_contract)?,
    ];
    Ok(format_hex(&keccak256(&abi_words(&words))))
}

pub fn signing_digest(
    struct_hash: &str,
    chain_id: u64,
    verifying_contract: &str,
) -> CoreResult<String> {
    let domain = bytes32(&domain_separator(chain_id, verifying_contract)?)?;
    let structure = bytes32(struct_hash)?;
    let mut encoded = Vec::with_capacity(66);
    encoded.extend_from_slice(&[0x19, 0x01]);
    encoded.extend_from_slice(&domain);
    encoded.extend_from_slice(&structure);
    Ok(format_hex(&keccak256(&encoded)))
}

pub fn transition_and_root(delta: &ExperienceDelta) -> CoreResult<(String, String)> {
    let transition = transition_id(delta)?;
    let root = next_state_root(&delta.prev_state_root, &transition)?;
    Ok((transition, root))
}

#[cfg(test)]
mod tests {
    use super::*;
    use ml_spec_types::ExperienceDelta;
    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    struct Vector {
        initial_controller: String,
        space_salt: String,
        profile_id: String,
        delta_commitment: String,
        provenance_commitment: String,
        locator_commitment: String,
        expected_space_id: String,
        expected_experience_delta_typehash: String,
        expected_memory_state_typehash: String,
        expected_memory_space_typehash: String,
        expected_transition_id: String,
        expected_next_state_root: String,
    }

    fn vector() -> Vector {
        serde_json::from_str(include_str!(
            "../../../contracts/vectors/erc8350_conformance.json"
        ))
        .expect("valid conformance vector")
    }

    #[test]
    fn pinned_type_hashes_match_vector() {
        let vector = vector();
        assert_eq!(
            experience_delta_type_hash(),
            vector.expected_experience_delta_typehash
        );
        assert_eq!(
            memory_state_type_hash(),
            vector.expected_memory_state_typehash
        );
        assert_eq!(
            memory_space_type_hash(),
            vector.expected_memory_space_typehash
        );
    }

    #[test]
    fn space_id_matches_vector() {
        let vector = vector();
        assert_eq!(
            space_id(&vector.initial_controller, &vector.space_salt).unwrap(),
            vector.expected_space_id
        );
    }

    #[test]
    fn transition_and_state_root_match_vector() {
        let vector = vector();
        let delta = ExperienceDelta {
            space_id: vector.expected_space_id.clone(),
            sequence: 1,
            prev_state_root: ZERO_ROOT.to_owned(),
            delta_commitment: vector.delta_commitment,
            provenance_commitment: vector.provenance_commitment,
            profile_id: vector.profile_id,
            locator_commitment: vector.locator_commitment,
        };
        let (transition, root) = transition_and_root(&delta).unwrap();
        assert_eq!(transition, vector.expected_transition_id);
        assert_eq!(root, vector.expected_next_state_root);
    }

    #[test]
    fn eip712_digest_is_domain_separated() {
        let vector = vector();
        let first = signing_digest(
            &vector.expected_transition_id,
            31337,
            "0x1111111111111111111111111111111111111111",
        )
        .unwrap();
        let second = signing_digest(
            &vector.expected_transition_id,
            1,
            "0x1111111111111111111111111111111111111111",
        )
        .unwrap();
        assert_ne!(first, second);
    }

    #[test]
    fn invalid_width_fails_closed() {
        assert_eq!(
            bytes32("0x12"),
            Err(CoreError::InvalidLength {
                actual: 1,
                expected: 32
            })
        );
        assert_eq!(address("0xzz"), Err(CoreError::InvalidHex));
    }
}
