//! Streamable-HTTP transport: auth, guards, JSON-RPC dispatch.

use axum::{
    body::Bytes,
    extract::State,
    http::{HeaderMap, StatusCode},
    response::Response,
};
use serde::Deserialize;
use serde_json::{json, Value};

use super::tools::{call_tool, tool_catalog};
use super::{rpc_error, rpc_result};
use crate::respond;
use crate::state::{bearer_user, AppState};

/// Protocol versions this server speaks, newest first.
const SUPPORTED_VERSIONS: &[&str] = &["2025-06-18", "2025-03-26", "2024-11-05"];
const PROTOCOL_VERSION: &str = SUPPORTED_VERSIONS[0];

#[derive(Deserialize)]
struct RpcRequest {
    #[allow(dead_code)]
    jsonrpc: Option<String>,
    id: Option<Value>,
    method: String,
    params: Option<Value>,
}

fn json_body(value: Value) -> Response {
    respond::json(StatusCode::OK, value)
}

/// `WWW-Authenticate` challenge pointing at our discovery document.
fn unauthorized(state: &AppState) -> Response {
    let challenge = format!(
        "Bearer resource_metadata=\"{}/.well-known/oauth-protected-resource\"",
        state.base_url
    );
    let mut res = respond::json(
        StatusCode::UNAUTHORIZED,
        json!({"error": "unauthorized", "error_description": "valid Bearer token required"}),
    );
    res.headers_mut().insert(
        "WWW-Authenticate",
        challenge.parse().expect("challenge parses"),
    );
    res
}

/// MCP endpoint (POST only). The blocking store work runs off the async
/// executor; plain GET explains itself with 405.
pub async fn mcp_post(State(state): State<AppState>, headers: HeaderMap, body: Bytes) -> Response {
    let started = std::time::Instant::now();
    let Some(user_id) = bearer_user(&state, &headers) else {
        return unauthorized(&state);
    };
    // DNS-rebinding protection applies to browser contexts (Origin set).
    if !origin_allowed(
        headers.get("origin").and_then(|v| v.to_str().ok()),
        &state.base_url,
    ) {
        return respond::json(
            StatusCode::FORBIDDEN,
            json!({"error": "forbidden", "error_description": "origin not allowed"}),
        );
    }
    if !accepts_json(headers.get("accept").and_then(|v| v.to_str().ok())) {
        return respond::json(
            StatusCode::NOT_ACCEPTABLE,
            json!({"error": "not_acceptable", "error_description": "send Accept: application/json, text/event-stream"}),
        );
    }
    if is_batch(&body) {
        return json_body(rpc_error(
            &None,
            -32600,
            "invalid request: batches are not supported",
        ));
    }
    let req: RpcRequest = match serde_json::from_slice(&body) {
        Ok(r) => r,
        Err(_) => return json_body(rpc_error(&None, -32700, "parse error")),
    };
    if req.jsonrpc.as_deref() != Some("2.0") {
        return json_body(rpc_error(
            &req.id,
            -32600,
            "invalid request: jsonrpc must be \"2.0\"",
        ));
    }
    // Only real `notifications/*` may omit `id`; anything else without an
    // id is malformed, not a notification.
    if req.id.is_none() && !req.method.starts_with("notifications/") {
        return json_body(rpc_error(&None, -32600, "invalid request: missing id"));
    }
    let is_notification = req.id.is_none();
    // Ad-hoc SQL runs in a confined worker process, so there is no
    // blocking store work left here — awaiting it never stalls the executor.
    let method = req.method.clone();
    let response = dispatch(&state, user_id, req).await;
    // `isError` lives beside `content` on the result object (per MCP);
    // a missing result (RPC error) counts as failure too.
    let failed = match response.get("result") {
        None => true,
        Some(r) => r
            .get("isError")
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false),
    };
    tracing::info!(
        "mcp {} -> {} in {}ms",
        method,
        if failed { "err" } else { "ok" },
        started.elapsed().as_millis()
    );
    if is_notification {
        respond::empty(StatusCode::ACCEPTED)
    } else {
        json_body(response)
    }
}

/// Direct GETs at the endpoint get a hint instead of a stream.
pub async fn mcp_get() -> Response {
    respond::json(
        StatusCode::METHOD_NOT_ALLOWED,
        json!({"error": "use POST with a JSON-RPC body and a Bearer token"}),
    )
}

async fn dispatch(state: &AppState, user_id: i64, req: RpcRequest) -> Value {
    let id = &req.id;
    match req.method.as_str() {
        "initialize" => {
            // Speak the client's version when we support it, else our newest.
            let asked = req
                .params
                .as_ref()
                .and_then(|p| p.get("protocolVersion"))
                .and_then(Value::as_str)
                .unwrap_or("");
            let negotiated = if SUPPORTED_VERSIONS.contains(&asked) {
                asked
            } else {
                PROTOCOL_VERSION
            };
            rpc_result(
                id,
                json!({
                    "protocolVersion": negotiated,
                    "capabilities": {"tools": {"listChanged": false}},
                    "serverInfo": {"name": "schrott-mcp", "version": "0.1.0"},
                }),
            )
        }
        "notifications/initialized" | "notifications/cancelled" => rpc_result(id, json!({})),
        "ping" => rpc_result(id, json!({})),
        "tools/list" => rpc_result(id, json!({"tools": tool_catalog()})),
        "tools/call" => call_tool(state, user_id, id, req.params).await,
        _ => rpc_error(id, -32601, "method not found"),
    }
}

/// True when an `Accept` header (if any) allows a JSON response.
/// Missing headers are accepted for leniency; anything else must include
/// `application/json` or `text/event-stream` per Streamable HTTP.
fn accepts_json(accept: Option<&str>) -> bool {
    let Some(header) = accept else {
        return true;
    };
    header.split(',').any(|part| {
        let mime = part.split(';').next().unwrap_or("").trim().to_lowercase();
        mime == "application/json" || mime == "text/event-stream" || mime == "*/*"
    })
}

/// `scheme://host[:port]` lowercased, with default ports stripped.
fn origin_of(url: &str) -> Option<String> {
    let (scheme, rest) = url.split_once("://")?;
    let host = rest.split('/').next().unwrap_or_default().to_lowercase();
    if host.is_empty() {
        return None;
    }
    let host = match scheme.to_lowercase().as_str() {
        "https" => host.strip_suffix(":443").unwrap_or(&host).to_owned(),
        "http" => host.strip_suffix(":80").unwrap_or(&host).to_owned(),
        _ => return None,
    };
    Some(format!("{}://{host}", scheme.to_lowercase()))
}

/// DNS-rebinding protection: a present `Origin` must match our own origin.
/// Absent headers (curl, server-side MCP hosts) pass through.
fn origin_allowed(origin: Option<&str>, base_url: &str) -> bool {
    let Some(origin) = origin else {
        return true;
    };
    match (origin_of(origin), origin_of(base_url)) {
        (Some(a), Some(b)) => a == b,
        _ => false,
    }
}

/// True when the body is a JSON array. MCP forbids batches: one request,
/// one response — arrays are `Invalid Request`, not a parse error.
fn is_batch(body: &[u8]) -> bool {
    body.iter()
        .find(|b| !b.is_ascii_whitespace())
        .is_some_and(|b| *b == b'[')
}

#[cfg(test)]
mod tests {
    use super::{accepts_json, is_batch, origin_allowed, origin_of};

    #[test]
    fn accept_negotiation() {
        assert!(accepts_json(None));
        assert!(accepts_json(Some("application/json, text/event-stream")));
        assert!(accepts_json(Some("text/event-stream")));
        assert!(accepts_json(Some("application/json;q=0.9")));
        assert!(accepts_json(Some("*/*")));
        assert!(!accepts_json(Some("text/html")));
        assert!(!accepts_json(Some("text/html, image/png")));
    }

    #[test]
    fn origin_parsing_and_matching() {
        assert_eq!(
            origin_of("https://schrott.offsite.lol/mcp").as_deref(),
            Some("https://schrott.offsite.lol")
        );
        assert_eq!(
            origin_of("http://localhost:4000/").as_deref(),
            Some("http://localhost:4000")
        );
        // Default ports normalize away.
        assert!(origin_allowed(
            Some("https://schrott.offsite.lol:443"),
            "https://schrott.offsite.lol"
        ));
        assert!(origin_allowed(
            Some("https://schrott.offsite.lol"),
            "https://schrott.offsite.lol"
        ));
        // Absent Origin (server-side clients) passes; anything else fails.
        assert!(origin_allowed(None, "https://schrott.offsite.lol"));
        assert!(!origin_allowed(
            Some("https://evil.com"),
            "https://schrott.offsite.lol"
        ));
        assert!(!origin_allowed(
            Some("https://schrott.offsite.lol.evil.com"),
            "https://schrott.offsite.lol"
        ));
        assert!(!origin_allowed(Some("null"), "https://schrott.offsite.lol"));
        assert!(!origin_allowed(
            Some("not a url"),
            "https://schrott.offsite.lol"
        ));
    }

    #[test]
    fn batch_detection() {
        assert!(is_batch(b"  \n [{\"jsonrpc\": \"2.0\"}]"));
        assert!(!is_batch(br#"{"jsonrpc": "2.0"}"#));
        assert!(!is_batch(b"   "));
    }
}
