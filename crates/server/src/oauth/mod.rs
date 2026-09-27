//! OAuth 2.0 authorization server (authorization-code + PKCE + refresh)
//! with public dynamic client registration — enough that an MCP host can be
//! pointed at our URL and complete login on its own.
//! Split by flow; each submodule owns one endpoint family.

use axum::{http::StatusCode, response::Response};

use crate::respond;

pub mod authorize;
pub mod discovery;
pub mod register;
pub mod token;

/// The one generic 500 this server can return.
pub(super) fn server_error() -> Response {
    respond::json(
        StatusCode::INTERNAL_SERVER_ERROR,
        serde_json::json!({"error": "server_error"}),
    )
}
