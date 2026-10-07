//! Isolated ad-hoc SQL worker: one query, one read-only database, stdout.
//!
//! The server spawns this per query inside OS limits (address space, CPU,
//! fds) as an unprivileged user. If a query blows up, this process dies —
//! the main server never feels it. No network, no threads, no secrets:
//! stdin carries `{"sql": "..."}`, stdout carries the result.

use std::io::Read as _;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Request {
    sql: String,
}

/// Largest request we will parse: SQL text, not a data upload.
const REQUEST_CAP: usize = 64 * 1024;

fn fail(message: String) -> ! {
    println!("{}", serde_json::json!({"error": message}));
    std::process::exit(1);
}

/// Strip the data directory from an error string so internal paths never
/// reach the client (they stay in the server log via our stderr).
fn sanitize(mut message: String, data_dir: &str) -> String {
    if !data_dir.is_empty() {
        message = message.replace(data_dir, "<data-dir>");
    }
    message
}

/// Parse one stdin request. Oversized input reports the cap — never a
/// confusing truncated-JSON error (stdin is cut one byte past the cap, so
/// anything that does not fit names the limit, not a serde EOF).
fn parse_request(input: &str) -> Result<Request, String> {
    if input.len() > REQUEST_CAP {
        return Err("query text too long".to_owned());
    }
    let req: Request = serde_json::from_str(input).map_err(|e| format!("bad request JSON: {e}"))?;
    if req.sql.len() > REQUEST_CAP {
        return Err("query text too long".to_owned());
    }
    Ok(req)
}

fn run() -> Result<(), String> {
    let data_dir = std::env::args()
        .nth(1)
        .filter(|s| !s.is_empty())
        .ok_or_else(|| "usage: schrott-mcp-query-worker <data-dir> < request.json".to_owned())?;
    // One byte past the cap: `parse_request` turns overflow into
    // "query text too long" instead of a truncated-JSON parse error.
    let mut input = String::new();
    std::io::stdin()
        .take(REQUEST_CAP as u64 + 1)
        .read_to_string(&mut input)
        .map_err(|e| format!("reading request: {e}"))?;
    let req = parse_request(&input)?;
    let db = schrott_mcp_store::PublicDb::open_read_only(std::path::Path::new(&data_dir))
        .map_err(|e| format!("opening database: {}", sanitize(e.to_string(), &data_dir)))?;
    let result = db
        .query_sql(&req.sql)
        .map_err(|e| format!("query failed: {e}"))?;
    println!(
        "{}",
        serde_json::json!({"ok": {"columns": result.columns, "rows": result.rows}})
    );
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        fail(e);
    }
}

#[cfg(test)]
mod tests {
    use super::{parse_request, sanitize, REQUEST_CAP};

    #[test]
    fn oversized_request_reports_too_long_not_truncated_json() {
        // Simulate stdin cut one byte past the cap: must name the limit.
        let big = "x".repeat(REQUEST_CAP + 1);
        assert_eq!(
            parse_request(&big).expect_err("oversized input"),
            "query text too long"
        );
        // A huge SQL inside valid JSON is the same actionable error.
        let req = format!(r#"{{"sql":"SELECT '{}'"}}"#, "y".repeat(REQUEST_CAP + 1));
        assert_eq!(
            parse_request(&req).expect_err("huge sql"),
            "query text too long"
        );
    }

    #[test]
    fn bad_json_stays_specific() {
        let err = parse_request("{nope").expect_err("bad json");
        assert!(err.starts_with("bad request JSON:"), "{err}");
    }

    #[test]
    fn valid_request_parses() {
        let req = parse_request(r#"{"sql":"SELECT 1"}"#).expect("valid");
        assert_eq!(req.sql, "SELECT 1");
    }

    #[test]
    fn sanitize_strips_data_dir() {
        let msg = sanitize(
            "database error: unable to open database file: /srv/data/public.db".to_owned(),
            "/srv/data",
        );
        assert!(!msg.contains("/srv/data"), "{msg}");
        assert!(msg.contains("<data-dir>"), "{msg}");
    }
}
