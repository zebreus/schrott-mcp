//! Token endpoint: authorization-code (with PKCE) and refresh-token grants.

use axum::{
    extract::{Form, State},
    http::StatusCode,
    response::Response,
};
use serde::Deserialize;
use serde_json::json;

use crate::respond;
use crate::state::{is_expired, AppState};

#[derive(Deserialize)]
pub struct TokenForm {
    grant_type: String,
    code: Option<String>,
    redirect_uri: Option<String>,
    client_id: Option<String>,
    code_verifier: Option<String>,
    refresh_token: Option<String>,
}

/// Token endpoint: authorization-code (with PKCE) and refresh-token grants.
pub async fn token(State(state): State<AppState>, Form(form): Form<TokenForm>) -> Response {
    let invalid = |error: &str, desc: &str| {
        respond::json(
            StatusCode::BAD_REQUEST,
            json!({"error": error, "error_description": desc}),
        )
    };
    /// Mint and persist an access/refresh pair, already wrapped as a response.
    fn minted(state: &AppState, user_id: i64, client_id: &str, scope: &str) -> Response {
        let access = format!("sma_{}", schrott_mcp_auth::new_token(32));
        let refresh = format!("smr_{}", schrott_mcp_auth::new_token(32));
        let now = chrono::Utc::now();
        let access_exp = (now + chrono::Duration::hours(1)).to_rfc3339();
        let refresh_exp = (now + chrono::Duration::days(30)).to_rfc3339();
        if let Err(e) = state.internal.create_oauth_tokens(
            &access,
            &refresh,
            user_id,
            client_id,
            scope,
            &now.to_rfc3339(),
            &access_exp,
            &refresh_exp,
        ) {
            tracing::warn!("oauth mint failed: {e}");
            return super::server_error();
        }
        respond::json(
            StatusCode::OK,
            json!({
                "access_token": access,
                "token_type": "Bearer",
                "expires_in": 3600,
                "refresh_token": refresh,
                "scope": scope,
            }),
        )
    }

    match form.grant_type.as_str() {
        "authorization_code" => {
            let (Some(code), Some(redirect_uri), Some(client_id), Some(verifier)) = (
                form.code,
                form.redirect_uri,
                form.client_id,
                form.code_verifier,
            ) else {
                return invalid("invalid_request", "missing code exchange fields");
            };
            let Some(stored) = state.internal.take_oauth_code(&code).ok().flatten() else {
                return invalid("invalid_grant", "unknown or reused code");
            };
            if is_expired(&stored.expires_at) {
                return invalid("invalid_grant", "code expired");
            }
            if stored.client_id != client_id || stored.redirect_uri != redirect_uri {
                return invalid("invalid_grant", "client or redirect mismatch");
            }
            if stored.code_challenge_method != "S256"
                || !schrott_mcp_auth::verify_pkce_s256(&verifier, &stored.code_challenge)
            {
                return invalid("invalid_grant", "PKCE verification failed");
            }
            minted(&state, stored.user_id, &stored.client_id, &stored.scope)
        }
        "refresh_token" => {
            let Some(refresh) = form.refresh_token else {
                return invalid("invalid_request", "missing refresh token");
            };
            let Some(stored) = state.internal.find_refresh_token(&refresh).ok().flatten() else {
                return invalid("invalid_grant", "unknown refresh token");
            };
            if is_expired(&stored.expires_at) {
                let _ = state.internal.delete_refresh_token(&refresh);
                return invalid("invalid_grant", "refresh token expired");
            }
            let _ = state.internal.delete_refresh_token(&refresh);
            minted(&state, stored.user_id, &stored.client_id, &stored.scope)
        }
        _ => invalid(
            "unsupported_grant_type",
            "only authorization_code/refresh_token",
        ),
    }
}
