//! Remote MCP server over Streamable HTTP (`POST /mcp`, JSON-RPC 2.0).
//!
//! Requires a Bearer token (OAuth access token or personal token) and
//! answers the MCP lifecycle plus five data tools over the shared corpus.

use axum::{
    body::Bytes,
    extract::State,
    http::{HeaderMap, StatusCode},
    response::Response,
};
use serde::Deserialize;
use serde_json::{json, Value};

use super::respond;
use super::state::{bearer_user, AppState};

/// Protocol versions this server speaks, newest first.
const SUPPORTED_VERSIONS: &[&str] = &["2025-06-18", "2025-03-26", "2024-11-05"];
const PROTOCOL_VERSION: &str = SUPPORTED_VERSIONS[0];

/// Inline previews stay under this many compact-JSON chars; the rest lives
/// behind `download_url`.
const PREVIEW_BUDGET: usize = 40_000;
/// Refuse to persist blobs past this size.
const MAX_BLOB_BYTES: usize = 10_000_000;

/// Statelessness contract: this server issues no `Mcp-Session-Id`, requires
/// none, and ignores any the client sends. Every request stands alone —
/// no `initialize` handshake state, no SSE streams, no session teardown.
/// (`DELETE /mcp` therefore falls through to axum's automatic 405.)

#[derive(Deserialize)]
struct RpcRequest {
    #[allow(dead_code)]
    jsonrpc: Option<String>,
    id: Option<Value>,
    method: String,
    params: Option<Value>,
}

fn rpc_result(id: &Option<Value>, result: Value) -> Value {
    json!({"jsonrpc": "2.0", "id": id, "result": result})
}

fn rpc_error(id: &Option<Value>, code: i64, message: &str) -> Value {
    json!({"jsonrpc": "2.0", "id": id, "error": {"code": code, "message": message}})
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
                    "serverInfo": {"name": "offsite-data", "version": "0.1.0"},
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

/// The single tool every client sees: read-only SQL over the shared corpus.
/// The description carries the schema so agents can query without guessing.
fn tool_catalog() -> Value {
    json!([
        {
            "name": "data_query_sql",
            "title": "Run read-only SQL",
            "annotations": {"readOnlyHint": true},
            "description": "Run one read-only SELECT (or WITH … SELECT) against the shared corpus and get {download_url, columns, rows}. Schema: sources(slug, name, url, description); datasets(slug, source_slug, name, description); items(id, dataset_slug, source_slug, external_id, title, url, published_at, summary, content_hash, data_json, fetched_at, updated_at). data_json holds scraper-specific fields as JSON text — use json_extract() on it. columns lists {name, type} per column; rows are objects keyed by column name. Rules: one statement, no semicolons, no writes/pragmas (rejected); newest items have the largest id. The inline rows preview is size-capped; download_url always carries the complete result as JSON and stays valid for 7 days. Example: SELECT id, title, url FROM items WHERE dataset_slug = 'rust-releases' ORDER BY id DESC LIMIT 10.",
            "inputSchema": {"type": "object",
                "properties": {
                    "sql": {"type": "string", "description": "A single self-contained SELECT statement"},
                    "max_rows": {"type": "integer", "minimum": 1, "maximum": 200, "default": 50, "description": "Row cap; truncated=true when more rows exist"},
                },
                "required": ["sql"], "additionalProperties": false},
        },
        {
            "name": "data_feedback",
            "title": "Report a data issue",
            "annotations": {"readOnlyHint": false},
            "description": "Report a problem with the corpus data — wrong values, stale records, missing coverage. Reports are stored for human review.",
            "inputSchema": {"type": "object",
                "properties": {
                    "severity": {"type": "string", "enum": ["low", "medium", "high", "critical"]},
                    "feedback": {"type": "string", "description": "What is wrong"},
                    "details": {"type": "string", "description": "Extra context: URLs, item ids, examples"},
                },
                "required": ["severity", "feedback"], "additionalProperties": false},
        },
    ])
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

fn text_result(value: Value) -> Value {
    json!({"content": [{"type": "text", "text": value.to_string()}]})
}

fn tool_error(message: String) -> Value {
    json!({"content": [{"type": "text", "text": message}], "isError": true})
}

fn str_arg(params: &Value, key: &str) -> Option<String> {
    params.get(key).and_then(Value::as_str).map(str::to_owned)
}

/// Handle the feedback tool: validate, store, acknowledge.
/// The response is exactly one fixed string — nothing else leaks out.
fn feedback_tool(state: &AppState, user_id: i64, id: &Option<Value>, args: &Value) -> Value {
    const SEVERITIES: &[&str] = &["low", "medium", "high", "critical"];
    let severity = str_arg(args, "severity")
        .map(|s| s.trim().to_lowercase())
        .filter(|s| SEVERITIES.contains(&s.as_str()));
    let Some(severity) = severity else {
        return rpc_result(
            id,
            tool_error("severity must be one of: low, medium, high, critical".to_owned()),
        );
    };
    let Some(feedback) = str_arg(args, "feedback").filter(|s| !s.trim().is_empty()) else {
        return rpc_result(
            id,
            tool_error("missing required argument: feedback".to_owned()),
        );
    };
    let details = str_arg(args, "details").unwrap_or_default();
    match state.internal.create_feedback(
        Some(user_id),
        &severity,
        feedback.trim(),
        details.trim(),
        &AppState::now(),
    ) {
        Ok(_) => rpc_result(
            id,
            json!({"content": [{"type": "text", "text": "Thank you for your feedback"}]}),
        ),
        Err(e) => rpc_result(id, tool_error(e.to_string())),
    }
}

/// Integer argument that also accepts numeric strings (`"5"` → `5`),
/// because LLM clients routinely emit ids as strings.
fn int_arg(params: &Value, key: &str) -> Option<i64> {
    match params.get(key) {
        Some(Value::Number(n)) => n.as_i64(),
        Some(Value::String(s)) => s.trim().parse().ok(),
        _ => None,
    }
}

async fn call_tool(
    state: &AppState,
    user_id: i64,
    id: &Option<Value>,
    params: Option<Value>,
) -> Value {
    let params = params.unwrap_or(Value::Null);
    let name = str_arg(&params, "name").unwrap_or_default();
    let args = params.get("arguments").cloned().unwrap_or(Value::Null);
    if name != "data_query_sql" && name != "data_feedback" {
        let msg = if name.is_empty() {
            "missing tool name; this server exposes two tools: \"data_query_sql\", \"data_feedback\""
                .to_owned()
        } else {
            format!(
                "unknown tool: {name}; this server exposes two tools: \"data_query_sql\", \"data_feedback\""
            )
        };
        return rpc_result(id, tool_error(msg));
    }
    if name == "data_feedback" {
        return feedback_tool(state, user_id, id, &args);
    }
    let Some(sql) = str_arg(&args, "sql").filter(|s| !s.trim().is_empty()) else {
        return rpc_result(id, tool_error("missing required argument: sql".to_owned()));
    };
    let max_rows = int_arg(&args, "max_rows").unwrap_or(50).clamp(1, 200) as usize;
    // Full result first, from the isolated worker: the download blob
    // always carries everything the query produced.
    let (columns, all_rows) = match super::worker::run_query(&state.data_dir, &sql).await {
        Ok(r) => (r.columns, r.rows),
        Err(e) => return rpc_result(id, tool_error(e.to_string())),
    };
    let names: Vec<&str> = columns.iter().map(|c| c.name.as_str()).collect();
    let full_rows: Vec<Value> = all_rows
        .iter()
        .map(|row| {
            names
                .iter()
                .zip(row.iter())
                .map(|(k, v)| ((*k).to_owned(), v.clone()))
                .collect::<serde_json::Map<String, Value>>()
        })
        .map(Value::Object)
        .collect();
    let full_text = json!({"columns": columns, "rows": full_rows}).to_string();
    if full_text.len() > MAX_BLOB_BYTES {
        return rpc_result(
            id,
            tool_error("result too large to share; narrow it with WHERE / LIMIT".to_owned()),
        );
    }
    // Persist the full blob behind a 128-bit base62 secret, valid 7 days.
    let secret = offsite_data_auth::new_base62_token(16);
    let now = chrono::Utc::now();
    let expires = now + chrono::Duration::days(7);
    if state
        .internal
        .create_result_blob(
            &secret,
            &full_text,
            &now.to_rfc3339(),
            &expires.to_rfc3339(),
        )
        .is_err()
    {
        return rpc_result(id, tool_error("could not store the result blob".to_owned()));
    }
    let download_url = format!("{}/d/{secret}/result.json", state.base_url);
    // Inline preview: as many leading rows as fit under the byte budget
    // (within max_rows). `truncated_after` appears only when rows were cut —
    // and is intentionally undocumented: agents discover it in responses.
    let mut shown = full_rows.len().min(max_rows);
    let preview = loop {
        let mut obj = serde_json::Map::with_capacity(4);
        obj.insert(
            "download_url".to_owned(),
            Value::String(download_url.clone()),
        );
        if shown < full_rows.len() {
            obj.insert("truncated_after".to_owned(), Value::Number(shown.into()));
        }
        obj.insert("columns".to_owned(), json!(columns));
        obj.insert("rows".to_owned(), Value::Array(full_rows[..shown].to_vec()));
        if Value::Object(obj.clone()).to_string().len() < PREVIEW_BUDGET || shown == 0 {
            break Value::Object(obj);
        }
        shown -= 1;
    };
    rpc_result(id, text_result(preview))
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
            origin_of("https://data.offsite.lol/mcp").as_deref(),
            Some("https://data.offsite.lol")
        );
        assert_eq!(
            origin_of("http://localhost:4000/").as_deref(),
            Some("http://localhost:4000")
        );
        // Default ports normalize away.
        assert!(origin_allowed(
            Some("https://data.offsite.lol:443"),
            "https://data.offsite.lol"
        ));
        assert!(origin_allowed(
            Some("https://data.offsite.lol"),
            "https://data.offsite.lol"
        ));
        // Absent Origin (server-side clients) passes; anything else fails.
        assert!(origin_allowed(None, "https://data.offsite.lol"));
        assert!(!origin_allowed(
            Some("https://evil.com"),
            "https://data.offsite.lol"
        ));
        assert!(!origin_allowed(
            Some("https://data.offsite.lol.evil.com"),
            "https://data.offsite.lol"
        ));
        assert!(!origin_allowed(Some("null"), "https://data.offsite.lol"));
        assert!(!origin_allowed(
            Some("not a url"),
            "https://data.offsite.lol"
        ));
    }

    #[test]
    fn batch_detection() {
        assert!(is_batch(b"  \n [{\"jsonrpc\": \"2.0\"}]"));
        assert!(!is_batch(br#"{"jsonrpc": "2.0"}"#));
        assert!(!is_batch(b"   "));
    }
}
