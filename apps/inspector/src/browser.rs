#![forbid(unsafe_code)]

#[cfg(target_arch = "wasm32")]
pub const SEPOLIA_RPC: &str = "https://ethereum-sepolia-rpc.publicnode.com";
pub const SEPOLIA_PROVIDER_LABEL: &str = "PUBLICNODE RPC";
#[cfg(target_arch = "wasm32")]
pub const SEPOLIA_REGISTRY: &str = "0x36fE9FA585565615Adcfe8680a126F770931E160";
#[cfg(target_arch = "wasm32")]
pub const SEPOLIA_SPACE: &str =
    "0x910968e7e2ae2899858c72b71683d55d3b1b11a69aae9f38448ee4cbb896580c";

#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
#[derive(Clone, Debug, Eq, PartialEq)]
struct PinnedBlock {
    tag: String,
    number: u64,
    hash: String,
}

#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LiveRollbackOutcome {
    Rejected {
        sequence: u64,
        canonical_root: String,
        block_tag: String,
        block_number: u64,
        block_hash: String,
    },
    Unavailable {
        reason: String,
    },
    Unexpected {
        reason: String,
    },
}

#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
fn parse_rpc_quantity(value: &str) -> Result<u64, String> {
    let Some(encoded) = value.strip_prefix("0x") else {
        return Err("BLOCK_NUMBER_INVALID".to_owned());
    };
    if encoded.is_empty()
        || (encoded.len() > 1 && encoded.starts_with('0'))
        || !encoded.bytes().all(|byte| byte.is_ascii_hexdigit())
    {
        return Err("BLOCK_NUMBER_INVALID".to_owned());
    }
    u64::from_str_radix(encoded, 16).map_err(|_| "BLOCK_NUMBER_INVALID".to_owned())
}

#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
fn parse_block_identity(value: &serde_json::Value) -> Result<(u64, String), String> {
    let number = value
        .get("number")
        .and_then(serde_json::Value::as_str)
        .ok_or_else(|| "BLOCK_NUMBER_MISSING".to_owned())?;
    let number = parse_rpc_quantity(number)?;
    let hash = value
        .get("hash")
        .and_then(serde_json::Value::as_str)
        .ok_or_else(|| "BLOCK_HASH_MISSING".to_owned())?;
    let Some(encoded_hash) = hash.strip_prefix("0x") else {
        return Err("BLOCK_HASH_INVALID".to_owned());
    };
    if encoded_hash.len() != 64 || !encoded_hash.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err("BLOCK_HASH_INVALID".to_owned());
    }
    Ok((number, hash.to_owned()))
}

#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
fn parse_pinned_block(value: &serde_json::Value, tag: &str) -> Result<PinnedBlock, String> {
    if tag != "finalized" && tag != "safe" {
        return Err("BLOCK_TAG_NOT_FINALIZED_OR_SAFE".to_owned());
    }
    let (number, hash) = parse_block_identity(value)?;
    Ok(PinnedBlock {
        tag: tag.to_owned(),
        number,
        hash,
    })
}

#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
fn block_number_quantity(number: u64) -> String {
    format!("0x{number:x}")
}

#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
fn pinned_block_matches(pinned: &PinnedBlock, value: &serde_json::Value) -> bool {
    parse_block_identity(value)
        .map(|(number, hash)| number == pinned.number && hash.eq_ignore_ascii_case(&pinned.hash))
        .unwrap_or(false)
}

#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
fn decode_error_string(data: &str) -> Option<String> {
    let encoded = data.strip_prefix("0x")?;
    if encoded.len() % 2 != 0 {
        return None;
    }
    let bytes = hex::decode(encoded).ok()?;
    if bytes.get(..4)? != [0x08, 0xc3, 0x79, 0xa0] || bytes.len() < 68 {
        return None;
    }

    // Solidity's Error(string) ABI uses a single dynamic string at offset 32.
    if bytes[4..35].iter().any(|byte| *byte != 0) || bytes[35] != 32 {
        return None;
    }
    if bytes[36..60].iter().any(|byte| *byte != 0) {
        return None;
    }
    let length = u64::from_be_bytes(bytes[60..68].try_into().ok()?);
    let length = usize::try_from(length).ok()?;
    let end = 68usize.checked_add(length)?;
    let padded_length = length.checked_add(31)? / 32 * 32;
    let expected_end = 68usize.checked_add(padded_length)?;
    if bytes.len() != expected_end || bytes.get(end..expected_end)?.iter().any(|byte| *byte != 0) {
        return None;
    }
    String::from_utf8(bytes.get(68..end)?.to_vec()).ok()
}

#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
fn nested_revert_data(value: &serde_json::Value) -> Option<String> {
    match value {
        serde_json::Value::String(value) => decode_error_string(value),
        serde_json::Value::Array(values) => values.iter().find_map(nested_revert_data),
        serde_json::Value::Object(values) => values.values().find_map(nested_revert_data),
        _ => None,
    }
}

#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
fn rpc_revert_reason(error: &serde_json::Value) -> Option<String> {
    let object = error.as_object()?;
    object
        .get("data")
        .and_then(nested_revert_data)
        .or_else(|| object.get("originalError").and_then(rpc_revert_reason))
        .or_else(|| object.get("error").and_then(rpc_revert_reason))
        .or_else(|| object.get("cause").and_then(rpc_revert_reason))
}

#[cfg(target_arch = "wasm32")]
async fn rpc(method: &str, params: serde_json::Value) -> Result<serde_json::Value, String> {
    use wasm_bindgen::{JsCast, JsValue};
    use wasm_bindgen_futures::JsFuture;
    use web_sys::{Request, RequestInit, Response};

    let body = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": method,
        "params": params,
    });
    let options = RequestInit::new();
    options.set_method("POST");
    options.set_body(&JsValue::from_str(&body.to_string()));
    let request = Request::new_with_str_and_init(SEPOLIA_RPC, &options)
        .map_err(|_| "RPC_REQUEST_CREATE_FAILED".to_owned())?;
    request
        .headers()
        .set("content-type", "application/json")
        .map_err(|_| "RPC_HEADER_FAILED".to_owned())?;
    let window = web_sys::window().ok_or_else(|| "WINDOW_UNAVAILABLE".to_owned())?;
    let response = JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(|_| "RPC_NETWORK_FAILED".to_owned())?
        .dyn_into::<Response>()
        .map_err(|_| "RPC_RESPONSE_CAST_FAILED".to_owned())?;
    let value = JsFuture::from(response.json().map_err(|_| "RPC_JSON_FAILED".to_owned())?)
        .await
        .map_err(|_| "RPC_JSON_FAILED".to_owned())?;
    let serialized = js_sys::JSON::stringify(&value)
        .map_err(|_| "RPC_JSON_SERIALIZE_FAILED".to_owned())?
        .as_string()
        .ok_or_else(|| "RPC_JSON_STRING_FAILED".to_owned())?;
    let parsed: serde_json::Value =
        serde_json::from_str(&serialized).map_err(|_| "RPC_JSON_PARSE_FAILED".to_owned())?;
    if let Some(error) = parsed.get("error") {
        if let Some(reason) = rpc_revert_reason(error) {
            return Err(format!("RPC_REVERT:{reason}"));
        }
        let message = error
            .get("message")
            .and_then(serde_json::Value::as_str)
            .unwrap_or("RPC_ERROR");
        return Err(format!("RPC_ERROR:{message}"));
    }
    parsed
        .get("result")
        .cloned()
        .ok_or_else(|| "RPC_RESULT_MISSING".to_owned())
}

#[cfg(target_arch = "wasm32")]
async fn resolve_pinned_block() -> Result<PinnedBlock, String> {
    for tag in ["finalized", "safe"] {
        if let Ok(value) = rpc("eth_getBlockByNumber", serde_json::json!([tag, false])).await
            && let Ok(block) = parse_pinned_block(&value, tag)
        {
            return Ok(block);
        }
    }
    Err("FINALIZED_OR_SAFE_BLOCK_UNAVAILABLE".to_owned())
}

#[cfg(target_arch = "wasm32")]
fn decode_head(value: &str) -> Result<(u64, String), String> {
    let bytes = hex::decode(value.strip_prefix("0x").unwrap_or(value))
        .map_err(|_| "HEAD_RESULT_INVALID_HEX".to_owned())?;
    if bytes.len() < 96 {
        return Err("HEAD_RESULT_TOO_SHORT".to_owned());
    }
    let root = format!("0x{}", hex::encode(&bytes[32..64]));
    let sequence = u64::from_be_bytes(
        bytes[88..96]
            .try_into()
            .map_err(|_| "HEAD_SEQUENCE_INVALID".to_owned())?,
    );
    Ok((sequence, root))
}

pub async fn live_silent_rollback(stale_predecessor: &str) -> LiveRollbackOutcome {
    #[cfg(target_arch = "wasm32")]
    {
        let pinned = match resolve_pinned_block().await {
            Ok(block) => block,
            Err(reason) => return LiveRollbackOutcome::Unavailable { reason },
        };
        let block_selector = block_number_quantity(pinned.number);
        let head_data = match ml_ethereum::head_call_data(SEPOLIA_SPACE) {
            Ok(data) => data,
            Err(error) => {
                return LiveRollbackOutcome::Unexpected {
                    reason: format!("HEAD_CALL_DATA_INVALID:{error}"),
                };
            }
        };
        let head = match rpc(
            "eth_call",
            serde_json::json!([{"to": SEPOLIA_REGISTRY, "data": head_data}, block_selector]),
        )
        .await
        {
            Ok(value) => value,
            Err(reason) => return LiveRollbackOutcome::Unavailable { reason },
        };
        let Some(head_hex) = head.as_str() else {
            return LiveRollbackOutcome::Unexpected {
                reason: "HEAD_RESULT_NOT_STRING".to_owned(),
            };
        };
        let (sequence, canonical_root) = match decode_head(head_hex) {
            Ok(head) => head,
            Err(reason) => return LiveRollbackOutcome::Unexpected { reason },
        };
        let rollback_data = match ml_ethereum::silent_rollback_call_data(
            SEPOLIA_SPACE,
            sequence,
            stale_predecessor,
        ) {
            Ok(data) => data,
            Err(error) => {
                return LiveRollbackOutcome::Unexpected {
                    reason: format!("ROLLBACK_CALL_DATA_INVALID:{error}"),
                };
            }
        };
        let attempt = rpc(
            "eth_call",
            serde_json::json!([{"to": SEPOLIA_REGISTRY, "data": rollback_data}, block_selector]),
        )
        .await;
        let block_after = match rpc(
            "eth_getBlockByNumber",
            serde_json::json!([block_number_quantity(pinned.number), false]),
        )
        .await
        {
            Ok(value) => value,
            Err(reason) => return LiveRollbackOutcome::Unavailable { reason },
        };
        if !pinned_block_matches(&pinned, &block_after) {
            return LiveRollbackOutcome::Unavailable {
                reason: "PINNED_BLOCK_IDENTITY_CHANGED".to_owned(),
            };
        }
        return match attempt {
            Err(message) if message == "RPC_REVERT:BAD_PREVIOUS_STATE" => {
                LiveRollbackOutcome::Rejected {
                    sequence: sequence.saturating_add(1),
                    canonical_root,
                    block_tag: pinned.tag,
                    block_number: pinned.number,
                    block_hash: pinned.hash,
                }
            }
            Ok(_) => LiveRollbackOutcome::Unexpected {
                reason: "UNEXPECTED_SUCCESS".to_owned(),
            },
            Err(message) => LiveRollbackOutcome::Unexpected {
                reason: format!("LIVE_RPC_REVERT_UNEXPECTED:{message}"),
            },
        };
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = stale_predecessor;
        LiveRollbackOutcome::Unavailable {
            reason: "BROWSER_RPC_UNAVAILABLE".to_owned(),
        }
    }
}

pub fn copy_text(value: String) {
    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen_futures::{JsFuture, spawn_local};
        if let Some(window) = web_sys::window() {
            let promise = window.navigator().clipboard().write_text(&value);
            spawn_local(async move {
                let _ = JsFuture::from(promise).await;
            });
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    let _ = value;
}

pub fn download_json(filename: &str, contents: &str) -> Result<(), String> {
    #[cfg(target_arch = "wasm32")]
    {
        use js_sys::Array;
        use wasm_bindgen::{JsCast, JsValue};
        use web_sys::{Blob, HtmlAnchorElement, Url};

        let parts = Array::new();
        parts.push(&JsValue::from_str(contents));
        let blob =
            Blob::new_with_str_sequence(&parts).map_err(|_| "BLOB_CREATE_FAILED".to_owned())?;
        let url = Url::create_object_url_with_blob(&blob)
            .map_err(|_| "OBJECT_URL_CREATE_FAILED".to_owned())?;
        let window = web_sys::window().ok_or_else(|| "WINDOW_UNAVAILABLE".to_owned())?;
        let document = window
            .document()
            .ok_or_else(|| "DOCUMENT_UNAVAILABLE".to_owned())?;
        let anchor = document
            .create_element("a")
            .map_err(|_| "DOWNLOAD_ELEMENT_CREATE_FAILED".to_owned())?
            .dyn_into::<HtmlAnchorElement>()
            .map_err(|_| "DOWNLOAD_ELEMENT_CAST_FAILED".to_owned())?;
        anchor.set_href(&url);
        anchor.set_download(filename);
        let body = document
            .body()
            .ok_or_else(|| "DOCUMENT_BODY_UNAVAILABLE".to_owned())?;
        body.append_child(&anchor)
            .map_err(|_| "DOWNLOAD_ELEMENT_ATTACH_FAILED".to_owned())?;
        anchor.click();
        let _ = body.remove_child(&anchor);
        let _ = Url::revoke_object_url(&url);
        Ok(())
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (filename, contents);
        Err("BROWSER_DOWNLOAD_UNAVAILABLE".to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::{block_number_quantity, parse_pinned_block, pinned_block_matches};

    #[test]
    fn parses_a_finalized_block_context() {
        let value = serde_json::json!({
            "number": "0x2a",
            "hash": format!("0x{}", "12".repeat(32)),
        });

        let pinned = parse_pinned_block(&value, "finalized").expect("block context is valid");
        assert_eq!(pinned.tag, "finalized");
        assert_eq!(pinned.number, 42);
        assert_eq!(pinned.hash, format!("0x{}", "12".repeat(32)));
    }

    #[test]
    fn refuses_latest_or_malformed_block_contexts() {
        let valid = serde_json::json!({
            "number": "0x2a",
            "hash": format!("0x{}", "12".repeat(32)),
        });
        let missing_hash = serde_json::json!({ "number": "0x2a" });
        let noncanonical_number = serde_json::json!({
            "number": "0x02a",
            "hash": format!("0x{}", "12".repeat(32)),
        });

        assert!(parse_pinned_block(&valid, "latest").is_err());
        assert!(parse_pinned_block(&missing_hash, "safe").is_err());
        assert!(parse_pinned_block(&noncanonical_number, "safe").is_err());
    }

    #[test]
    fn detects_a_changed_block_hash_and_formats_pinned_quantities() {
        let original = serde_json::json!({
            "number": "0x2a",
            "hash": format!("0x{}", "12".repeat(32)),
        });
        let changed = serde_json::json!({
            "number": "0x2a",
            "hash": format!("0x{}", "34".repeat(32)),
        });
        let pinned = parse_pinned_block(&original, "safe").expect("block context is valid");

        assert!(pinned_block_matches(&pinned, &original));
        assert!(!pinned_block_matches(&pinned, &changed));
        assert_eq!(block_number_quantity(42), "0x2a");
        assert_eq!(block_number_quantity(0), "0x0");
    }

    #[test]
    fn decodes_the_exact_contract_revert_reason_from_rpc_data() {
        let encoded = encode_error_string("BAD_PREVIOUS_STATE");
        let error = serde_json::json!({
            "code": 3,
            "message": "execution reverted",
            "data": encoded,
        });

        assert_eq!(
            super::rpc_revert_reason(&error).as_deref(),
            Some("BAD_PREVIOUS_STATE")
        );
    }

    #[test]
    fn does_not_infer_a_contract_revert_reason_from_free_form_rpc_text() {
        let error = serde_json::json!({
            "code": -32000,
            "message": "provider diagnostic mentions BAD_PREVIOUS_STATE but contains no revert data",
        });

        assert_eq!(super::rpc_revert_reason(&error), None);
    }

    #[test]
    fn only_reads_encoded_revert_data_from_rpc_data_fields() {
        let error = serde_json::json!({
            "code": 3,
            "message": encode_error_string("BAD_PREVIOUS_STATE"),
        });

        assert_eq!(super::rpc_revert_reason(&error), None);
    }

    #[test]
    fn rejects_malformed_or_noncanonical_error_string_data() {
        let malformed = serde_json::json!({
            "data": "0x08c379a0"
        });
        let trailing_bytes = serde_json::json!({
            "data": format!("{}00", encode_error_string("BAD_PREVIOUS_STATE")),
        });

        assert_eq!(super::rpc_revert_reason(&malformed), None);
        assert_eq!(super::rpc_revert_reason(&trailing_bytes), None);
    }

    fn encode_error_string(reason: &str) -> String {
        let bytes = reason.as_bytes();
        let mut encoded = Vec::from([0x08, 0xc3, 0x79, 0xa0]);
        encoded.extend([0u8; 31]);
        encoded.push(32);
        encoded.extend([0u8; 31]);
        encoded.push(bytes.len() as u8);
        encoded.extend_from_slice(bytes);
        let padding = (32 - bytes.len() % 32) % 32;
        encoded.extend(std::iter::repeat_n(0, padding));
        format!("0x{}", hex::encode(encoded))
    }
}
