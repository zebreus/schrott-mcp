//! Dynamic client registration (public, per RFC 7591).

use axum::{extract::State, http::StatusCode, response::Response};
use serde::Deserialize;
use serde_json::json;

use crate::respond;
use crate::state::AppState;

#[derive(Deserialize)]
pub struct RegisterRequest {
    redirect_uris: Vec<String>,
}

/// Dynamic client registration (public, per RFC 7591).
pub async fn register(State(state): State<AppState>, body: String) -> Response {
    let req: RegisterRequest = match serde_json::from_str(&body) {
        Ok(r) => r,
        Err(_) => {
            return respond::json(
                StatusCode::BAD_REQUEST,
                json!({"error": "invalid_client_metadata"}),
            );
        }
    };
    if req.redirect_uris.is_empty() || req.redirect_uris.len() > 10 {
        return respond::json(
            StatusCode::BAD_REQUEST,
            json!({"error": "invalid_redirect_uri"}),
        );
    }
    for uri in &req.redirect_uris {
        let http = uri.starts_with("http://localhost")
            || uri.starts_with("http://127.0.0.1")
            || uri.starts_with("https://");
        let no_fragment = !uri.contains('#');
        if !(http && no_fragment) {
            return respond::json(
                StatusCode::BAD_REQUEST,
                json!({"error": "invalid_redirect_uri"}),
            );
        }
    }
    let client_id = format!("cli_{}", offsite_data_auth::new_token(16));
    if state
        .internal
        .upsert_oauth_client(&client_id, &req.redirect_uris, &AppState::now())
        .is_err()
    {
        return super::server_error();
    }
    respond::json(
        StatusCode::CREATED,
        json!({
            "client_id": client_id,
            "redirect_uris": req.redirect_uris,
            "token_endpoint_auth_method": "none",
            "grant_types": ["authorization_code", "refresh_token"],
            "response_types": ["code"],
            "scope": "read",
        }),
    )
}
