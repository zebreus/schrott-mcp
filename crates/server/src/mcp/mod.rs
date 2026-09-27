//! Remote MCP server over Streamable HTTP (`POST /mcp`, JSON-RPC 2.0).
//! Requires a Bearer token (OAuth access token or personal token).
//!
//! Statelessness contract: this server issues no `Mcp-Session-Id`, requires
//! none, and ignores any the client sends. Every request stands alone —
//! no `initialize` handshake state, no SSE streams, no session teardown.
//! (`DELETE /mcp` therefore falls through to axum's automatic 405.)

use serde_json::{json, Value};

pub mod protocol;
pub mod tools;
mod worker;

pub use protocol::{mcp_get, mcp_post};

fn rpc_result(id: &Option<Value>, result: Value) -> Value {
    json!({"jsonrpc": "2.0", "id": id, "result": result})
}

fn rpc_error(id: &Option<Value>, code: i64, message: &str) -> Value {
    json!({"jsonrpc": "2.0", "id": id, "error": {"code": code, "message": message}})
}
