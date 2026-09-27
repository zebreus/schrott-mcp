//! Minimal OAuth 2.0 authorization server (authorization-code + PKCE + refresh)
//! with public dynamic client registration — enough that an MCP host can be
//! pointed at our URL and complete login on its own.

use axum::{
    extract::{Form, Query, RawQuery, State},
    http::{HeaderMap, StatusCode},
    response::Response,
};
use offsite_data_core::url_encode;
use serde::{Deserialize, Serialize};
use serde_json::json;

use super::respond;
use super::state::{is_expired, session_token, session_user, AppState};

fn json_response(status: StatusCode, value: serde_json::Value) -> Response {
    respond::json(status, value)
}

fn redirect(to: &str) -> Response {
    respond::see_other(to, None)
}

/// RFC 8414 / draft-ietf-oauth-discovery authorization-server metadata.
pub async fn server_metadata(State(state): State<AppState>) -> Response {
    let b = &state.base_url;
    json_response(
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
    json_response(
        StatusCode::OK,
        json!({
            "resource": format!("{b}/mcp"),
            "authorization_servers": [b],
            "scopes_supported": ["read"],
            "bearer_methods_supported": ["header"],
        }),
    )
}

#[derive(Deserialize)]
pub struct RegisterRequest {
    redirect_uris: Vec<String>,
}

#[derive(Serialize)]
struct RegisterResponse {
    client_id: String,
    redirect_uris: Vec<String>,
    token_endpoint_auth_method: String,
    grant_types: Vec<String>,
    response_types: Vec<String>,
    scope: String,
}

/// Dynamic client registration (public, per RFC 7591).
pub async fn register(State(state): State<AppState>, body: String) -> Response {
    let req: RegisterRequest = match serde_json::from_str(&body) {
        Ok(r) => r,
        Err(_) => {
            return json_response(
                StatusCode::BAD_REQUEST,
                json!({"error": "invalid_client_metadata"}),
            );
        }
    };
    if req.redirect_uris.is_empty() || req.redirect_uris.len() > 10 {
        return json_response(
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
            return json_response(
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
        return json_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({"error": "server_error"}),
        );
    }
    json_response(
        StatusCode::CREATED,
        serde_json::to_value(RegisterResponse {
            client_id,
            redirect_uris: req.redirect_uris,
            token_endpoint_auth_method: "none".to_owned(),
            grant_types: vec!["authorization_code".to_owned(), "refresh_token".to_owned()],
            response_types: vec!["code".to_owned()],
            scope: "read".to_owned(),
        })
        .expect("register response serializes"),
    )
}

#[derive(Deserialize)]
pub struct AuthorizeQuery {
    response_type: Option<String>,
    client_id: Option<String>,
    redirect_uri: Option<String>,
    code_challenge: Option<String>,
    code_challenge_method: Option<String>,
    state: Option<String>,
    scope: Option<String>,
}

/// Authorization endpoint (GET): validate the request, require login,
/// then show the consent screen.
pub async fn authorize_get(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(q): Query<AuthorizeQuery>,
    RawQuery(raw): RawQuery,
) -> Response {
    let raw = raw.unwrap_or_default();
    let err_redirect = |client_uri: Option<&str>, error: &str, state_p: Option<&str>| {
        if let Some(uri) = client_uri {
            let sep = if uri.contains('?') { '&' } else { '?' };
            let mut loc = format!("{uri}{sep}error={error}");
            if let Some(s) = state_p {
                loc.push_str(&format!("&state={}", url_encode(s)));
            }
            redirect(&loc)
        } else {
            json_response(StatusCode::BAD_REQUEST, json!({"error": error}))
        }
    };

    if q.response_type.as_deref() != Some("code") {
        return err_redirect(
            q.redirect_uri.as_deref(),
            "unsupported_response_type",
            q.state.as_deref(),
        );
    }
    let (Some(client_id), Some(redirect_uri), Some(challenge)) = (
        q.client_id.clone(),
        q.redirect_uri.clone(),
        q.code_challenge.clone(),
    ) else {
        return err_redirect(
            q.redirect_uri.as_deref(),
            "invalid_request",
            q.state.as_deref(),
        );
    };
    if q.code_challenge_method.as_deref().unwrap_or("S256") != "S256" {
        return err_redirect(Some(&redirect_uri), "invalid_request", q.state.as_deref());
    }
    let Some(client) = state.internal.find_oauth_client(&client_id).ok().flatten() else {
        return err_redirect(None, "unauthorized_client", q.state.as_deref());
    };
    if !client.redirect_uris.iter().any(|u| u == &redirect_uri) {
        return json_response(
            StatusCode::BAD_REQUEST,
            json!({"error": "invalid_redirect_uri"}),
        );
    }
    let Some(user) = session_user(&state, &headers) else {
        let next = format!("/oauth/authorize?{raw}");
        return redirect(&format!("/login?next={}", url_encode(&next)));
    };
    let csrf = session_token(&headers)
        .map(|t| state.issue_csrf(&t))
        .unwrap_or_default();
    let scope = q.scope.clone().unwrap_or_else(|| "read".to_owned());
    respond::html(crate::web::pages::consent(
        crate::web::pages::ConsentData {
            client_id: &client_id,
            redirect_uri: &redirect_uri,
            scope: &scope,
            raw_query: &raw,
            username: &user.username,
            csrf: &csrf,
            code_challenge: &challenge,
            oauth_state: q.state.as_deref(),
        },
        &state.base_url,
    ))
}

#[derive(Deserialize)]
pub struct ConsentForm {
    decision: String,
    csrf: String,
    /// Carried hidden fields (query string is the fallback).
    client_id: Option<String>,
    redirect_uri: Option<String>,
    scope: Option<String>,
    state: Option<String>,
    code_challenge: Option<String>,
}

/// Authorization endpoint (POST): issue the code or deny.
///
/// The consent form carries the request fields as hidden inputs; the query
/// string is only a fallback. Missing fields are an `invalid_request`.
pub async fn authorize_post(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(q): Query<AuthorizeQuery>,
    Form(form): Form<ConsentForm>,
) -> Response {
    let session = session_token(&headers);
    if !state.check_csrf(session.as_deref(), &form.csrf) {
        return json_response(
            StatusCode::FORBIDDEN,
            json!({"error": "access_denied", "error_description": "bad CSRF token"}),
        );
    }
    let client_id = form.client_id.or(q.client_id).unwrap_or_default();
    let redirect_uri = form.redirect_uri.or(q.redirect_uri).unwrap_or_default();
    let scope = form.scope.or(q.scope).unwrap_or_else(|| "read".to_owned());
    let state_p = form.state.or(q.state);
    let challenge = form.code_challenge.or(q.code_challenge).unwrap_or_default();
    if client_id.is_empty() || redirect_uri.is_empty() || challenge.is_empty() {
        return json_response(StatusCode::BAD_REQUEST, json!({"error": "invalid_request"}));
    }
    if form.decision != "approve" {
        let sep = if redirect_uri.contains('?') { '&' } else { '?' };
        let mut loc = format!("{redirect_uri}{sep}error=access_denied");
        if let Some(s) = state_p {
            loc.push_str(&format!("&state={}", url_encode(&s)));
        }
        return redirect(&loc);
    }
    let Some(user) = session_user(&state, &headers) else {
        return redirect("/login?next=/dashboard");
    };
    let Some(client) = state.internal.find_oauth_client(&client_id).ok().flatten() else {
        return json_response(
            StatusCode::BAD_REQUEST,
            json!({"error": "unauthorized_client"}),
        );
    };
    if !client.redirect_uris.iter().any(|u| u == &redirect_uri) {
        return json_response(
            StatusCode::BAD_REQUEST,
            json!({"error": "invalid_redirect_uri"}),
        );
    }
    let code = format!("odc_{}", offsite_data_auth::new_token(32));
    let now = chrono::Utc::now();
    let exp = (now + chrono::Duration::minutes(10)).to_rfc3339();
    if state
        .internal
        .create_oauth_code(
            &code,
            &client_id,
            &redirect_uri,
            user.id,
            &challenge,
            "S256",
            &scope,
            &now.to_rfc3339(),
            &exp,
        )
        .is_err()
    {
        return json_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({"error": "server_error"}),
        );
    }
    let sep = if redirect_uri.contains('?') { '&' } else { '?' };
    let mut loc = format!("{redirect_uri}{sep}code={}", url_encode(&code));
    if let Some(s) = state_p {
        loc.push_str(&format!("&state={}", url_encode(&s)));
    }
    redirect(&loc)
}

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
        json_response(
            StatusCode::BAD_REQUEST,
            json!({"error": error, "error_description": desc}),
        )
    };
    let mint = |user_id: i64,
                client_id: &str,
                scope: &str,
                state: &AppState|
     -> Result<serde_json::Value, String> {
        let access = format!("oda_{}", offsite_data_auth::new_token(32));
        let refresh = format!("odr_{}", offsite_data_auth::new_token(32));
        let now = chrono::Utc::now();
        let access_exp = (now + chrono::Duration::hours(1)).to_rfc3339();
        let refresh_exp = (now + chrono::Duration::days(30)).to_rfc3339();
        state
            .internal
            .create_oauth_tokens(
                &access,
                &refresh,
                user_id,
                client_id,
                scope,
                &now.to_rfc3339(),
                &access_exp,
                &refresh_exp,
            )
            .map_err(|e| format!("storing tokens failed: {e}"))?;
        Ok(json!({
            "access_token": access,
            "token_type": "Bearer",
            "expires_in": 3600,
            "refresh_token": refresh,
            "scope": scope,
        }))
    };

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
                || !offsite_data_auth::verify_pkce_s256(&verifier, &stored.code_challenge)
            {
                return invalid("invalid_grant", "PKCE verification failed");
            }
            match mint(stored.user_id, &stored.client_id, &stored.scope, &state) {
                Ok(v) => json_response(StatusCode::OK, v),
                Err(e) => {
                    tracing::warn!("oauth mint failed: {e}");
                    json_response(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        json!({"error": "server_error"}),
                    )
                }
            }
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
            match mint(stored.user_id, &stored.client_id, &stored.scope, &state) {
                Ok(v) => json_response(StatusCode::OK, v),
                Err(e) => {
                    tracing::warn!("oauth mint failed: {e}");
                    json_response(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        json!({"error": "server_error"}),
                    )
                }
            }
        }
        _ => invalid(
            "unsupported_grant_type",
            "only authorization_code/refresh_token",
        ),
    }
}
