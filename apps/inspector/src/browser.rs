#![forbid(unsafe_code)]

#[cfg(target_arch = "wasm32")]
pub const SEPOLIA_RPC: &str = "https://ethereum-sepolia-rpc.publicnode.com";
#[cfg(target_arch = "wasm32")]
pub const SEPOLIA_REGISTRY: &str = "0x36fE9FA585565615Adcfe8680a126F770931E160";
#[cfg(target_arch = "wasm32")]
pub const SEPOLIA_SPACE: &str =
    "0x910968e7e2ae2899858c72b71683d55d3b1b11a69aae9f38448ee4cbb896580c";

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
        let message = error
            .get("message")
            .and_then(serde_json::Value::as_str)
            .unwrap_or("RPC_ERROR");
        return Err(message.to_owned());
    }
    parsed
        .get("result")
        .cloned()
        .ok_or_else(|| "RPC_RESULT_MISSING".to_owned())
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

pub async fn live_silent_rollback() -> Result<String, String> {
    #[cfg(target_arch = "wasm32")]
    {
        let head_data =
            ml_ethereum::head_call_data(SEPOLIA_SPACE).map_err(|error| error.to_string())?;
        let head = rpc(
            "eth_call",
            serde_json::json!([{"to": SEPOLIA_REGISTRY, "data": head_data}, "latest"]),
        )
        .await?;
        let head_hex = head
            .as_str()
            .ok_or_else(|| "HEAD_RESULT_NOT_STRING".to_owned())?;
        let (sequence, canonical_root) = decode_head(head_hex)?;
        let rollback_data = ml_ethereum::silent_rollback_call_data(SEPOLIA_SPACE, sequence)
            .map_err(|error| error.to_string())?;
        let attempt = rpc(
            "eth_call",
            serde_json::json!([{"to": SEPOLIA_REGISTRY, "data": rollback_data}, "latest"]),
        )
        .await;
        return match attempt {
            Err(message) if message.contains("BAD_PREVIOUS_STATE") => Ok(format!(
                "LIVE RPC / BAD_PREVIOUS_STATE / sequence {} / canonical root {}",
                sequence + 1,
                crate::data::short_hash(&canonical_root, 10, 8)
            )),
            Ok(_) => Err("UNEXPECTED_SUCCESS".to_owned()),
            Err(message) => Err(format!("LIVE_RPC_REVERT_UNEXPECTED:{message}")),
        };
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        Err("BROWSER_RPC_UNAVAILABLE".to_owned())
    }
}

pub fn browser_path() -> String {
    #[cfg(target_arch = "wasm32")]
    {
        return web_sys::window()
            .map(|window| {
                window
                    .location()
                    .pathname()
                    .unwrap_or_else(|_| "/".to_owned())
            })
            .unwrap_or_else(|| "/".to_owned());
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        "/".to_owned()
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
