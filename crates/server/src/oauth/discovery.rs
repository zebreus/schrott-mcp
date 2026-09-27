//! RFC 8414 / RFC 9728 discovery documents: tell MCP clients where to log in.

use axum::{extract::State, http::StatusCode, response::Response};
use serde_json::json;

use crate::respond;
use crate::state::AppState;

/// RFC 8414 / draft-ietf-oauth-discovery authorization-server metadata.
pub async fn server_metadata(State(state): State<AppState>) -> Response {
    let b = &state.base_url;
    respond::json(
        StatusCode::OK,
        json!({
            "issuer": b,
            "authorization_endpoint": format!("{b}/oauth/authorize"),
            "token_endpoint": format!("{b}/oauth/token"),
            "registration_endpoint": format!("{b}/oauth/register"),
            "response_types_supported": ["code"],
            "grant_types_supported": ["authorization_code", "refresh_token"],
            "code_challenge_methods_supported": ["S256"],
            "token_endpoint_auth_methods_supported": ["none"],
            "scopes_supported": ["read"],
        }),
    )
}

/// RFC 9728 protected-resource metadata: tells MCP clients where to log in.
pub async fn protected_resource(State(state): State<AppState>) -> Response {
    let b = &state.base_url;
    respond::json(
        StatusCode::OK,
        json!({
            "resource": format!("{b}/mcp"),
            "authorization_servers": [b],
            "scopes_supported": ["read"],
            "bearer_methods_supported": ["header"],
        }),
    )
}
