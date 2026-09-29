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

/// Schema documentation for agents (German). The identifiers stay English
/// so queries remain compact; the semantics are explained here.
const SCHEMA_DOC: &str = "Tabellen: traders(id, slug, name, trader_type, description, street, postcode, city, state, country, lat, lon, phone, email, website, website_status, website_checked_at, opening_hours, dropoff_json, pickup_json, min_quantity_kg, max_quantity_kg, certifications, status, notes, extra_json, first_seen_at, updated_at); materials(id, slug, name_de, category, unit, description, extra_json, updated_at); trader_materials(trader_id, material_id, accepts, conditions, valid_from, valid_to, observed_at, updated_at); prices(id, trader_id, material_id, variant, price, currency, unit, price_kind, price_min, price_max, confidence, source_type, published, source_url, observed_at, published_at, valid_from, valid_to, notes, extra_json, ingested_at); current_prices(trader_id, material_id, variant, price_id, updated_at); Sicht v_current_prices(trader_slug, trader, city, postcode, state, material_slug, material, variant, category, price, currency, unit, price_kind, price_min, price_max, confidence, source_type, published, source_url, observed_at, published_at, valid_from, valid_to). Variant = Händlersorte ('' = Standard); jede (Händler, Material, Variante) hat eigenen aktuellen Preis. Händlertypen: schrotthaendler, wertstoffhaendler, metallhaendler, autoverwertung, containerdienst, schrottplatz, mobil, sonstige. website_status: aktiv, tot, blockiert, unbekannt. dropoff_json/pickup_json sind Konditions-Objekte {allowed, customer_types:[privat,gewerbe], days:[Mo..So], time_windows, min_quantity_kg, max_quantity_kg, conditions} — fehlendes allowed = unbekannt; Beispiel: SELECT name, city, dropoff_json FROM traders WHERE json_extract(dropoff_json,'$.allowed') = 1. description ist die kuratierte deutsche Beschreibung (oft noch leer). price_kind: exact (Listenpreis), upto (Obergrenze, 'bis zu'), range, approx. PREISVERGLEICHE: nur price_kind='exact' direkt vergleichen; 'upto'-Werte sind Obergrenzen. Alle Preise stehen in der Katalogeinheit von materials.unit. published=1 heißt, der Händler hat den Preis selbst veröffentlicht; published_at fehlt, wenn die Seite kein Datum nennt — dann ist observed_at (Erstbeobachtung) der Altersmaßstab. observed_at ist immer der Abrufzeitpunkt, published_at das Seitendatum. valid_from/valid_to NULL = offen. confidence NULL = unbekannt. Händler-Volltextsuche über traders_fts (FTS5, z. B. JOIN traders_fts f ON t.id=f.rowid WHERE traders_fts MATCH 'Berlin*'). Neueste Beobachtung = größtes observed_at bzw. größte id.";

pub(super) const TOOL_QUERY: &str = "schrott_query_sql";
pub(super) const TOOL_FEEDBACK: &str = "schrott_feedback";

/// The single query tool every client sees: read-only SQL over the corpus.
/// The description carries the schema so agents can query without guessing.
pub(super) fn tool_catalog() -> Value {
    json!([
        {
            "name": TOOL_QUERY,
            "title": "Schrottdaten per SQL abfragen",
            "annotations": {"readOnlyHint": true},
            "description": format!("Führe genau ein lesendes SELECT (oder WITH … SELECT) über den geteilten Schrott-Datenbestand aus und erhalte {{download_url, columns, rows}}. {SCHEMA_DOC} columns listet {{name, type}} je Spalte; rows sind Objekte mit Spaltennamen als Schlüsseln. Regeln: genau eine Anweisung (ein Semikolon am Ende ist erlaubt), keine Schreib-/Pragma-Anweisungen (werden abgelehnt). Die Inline-Vorschau ist größenbegrenzt; download_url enthält immer das komplette Ergebnis als JSON und bleibt 7 Tage gültig. Beispiele: SELECT trader, city, price, unit FROM v_current_prices WHERE material_slug = 'kupfer-millberry' ORDER BY price DESC LIMIT 10 — SELECT slug, name, city, postcode FROM traders WHERE city = 'Berlin' LIMIT 20 — SELECT observed_at, price, source_type, published FROM prices WHERE trader_id = 1 AND material_id = 2 ORDER BY observed_at DESC LIMIT 50."),
            "inputSchema": {"type": "object",
                "properties": {
                    "sql": {"type": "string", "description": "Eine einzelne, eigenständige SELECT-Anweisung"},
                    "max_rows": {"type": "integer", "minimum": 1, "maximum": 200, "default": 50, "description": "Zeilenlimit; truncated=true, wenn mehr Zeilen existieren"},
                },
                "required": ["sql"], "additionalProperties": false},
        },
        {
            "name": TOOL_FEEDBACK,
            "title": "Datenproblem melden",
            "annotations": {"readOnlyHint": false},
            "description": "Melde ein Problem mit den Schrottdaten — falsche Werte, veraltete Preise, fehlende Händler. Meldungen werden für Menschen zur Prüfung gespeichert.",
            "inputSchema": {"type": "object",
                "properties": {
                    "severity": {"type": "string", "enum": ["low", "medium", "high", "critical"]},
                    "feedback": {"type": "string", "description": "Was ist falsch"},
                    "details": {"type": "string", "description": "Zusatzinfos: URLs, Händler-Slugs, Beispiele"},
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
            json!({"content": [{"type": "text", "text": "Danke für deine Rückmeldung"}]}),
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
    if name != TOOL_QUERY && name != TOOL_FEEDBACK {
        let msg = if name.is_empty() {
            format!("missing tool name; this server exposes two tools: \"{TOOL_QUERY}\", \"{TOOL_FEEDBACK}\"")
        } else {
            format!(
                "unknown tool: {name}; this server exposes two tools: \"{TOOL_QUERY}\", \"{TOOL_FEEDBACK}\""
            )
        };
        return rpc_result(id, tool_error(msg));
    }
    if name == TOOL_FEEDBACK {
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
    let secret = schrott_mcp_auth::new_base62_token(16);
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
