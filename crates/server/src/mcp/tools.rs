//! MCP tools: catalog plus the query and feedback implementations.

use serde_json::{json, Value};

use super::rpc_result;
use super::worker;
use crate::state::AppState;

/// Inline previews stay under this many compact-JSON chars; the rest lives
/// behind `download_url`.
const PREVIEW_BUDGET: usize = 40_000;
/// Refuse to persist blobs past this size.
const MAX_BLOB_BYTES: usize = 10_000_000;

/// The single tool every client sees: read-only SQL over the shared corpus.
/// The description carries the schema so agents can query without guessing.
pub(super) fn tool_catalog() -> Value {
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

pub(super) async fn call_tool(
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
    let (columns, all_rows) = match worker::run_query(&state.data_dir, &sql).await {
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
