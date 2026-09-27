//! Isolated ad-hoc SQL worker: one query, one read-only database, stdout.
//!
//! The server spawns this per query inside OS limits (address space, CPU,
//! fds) as an unprivileged user. If a query blows up, this process dies —
//! the main server never feels it. No network, no threads, no secrets:
//! stdin carries `{"sql": "..."}`, stdout carries the result.

use std::io::Read as _;

use serde::Deserialize;

#[derive(Deserialize)]
struct Request {
    sql: String,
}

fn fail(message: String) -> ! {
    println!("{}", serde_json::json!({"error": message}));
    std::process::exit(1);
}

fn run() -> Result<(), String> {
    let data_dir = std::env::args()
        .nth(1)
        .filter(|s| !s.is_empty())
        .ok_or_else(|| "usage: schrott-mcp-query-worker <data-dir> < request.json".to_owned())?;
    // Cap the request: SQL text, not a data upload.
    let mut input = String::new();
    std::io::stdin()
        .take(64 * 1024)
        .read_to_string(&mut input)
        .map_err(|e| format!("reading request: {e}"))?;
    let req: Request =
        serde_json::from_str(&input).map_err(|e| format!("bad request JSON: {e}"))?;
    if req.sql.len() > 64 * 1024 {
        return Err("query text too long".to_owned());
    }
    let db = schrott_mcp_store::PublicDb::open_read_only(std::path::Path::new(&data_dir))
        .map_err(|e| format!("opening database: {e}"))?;
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
