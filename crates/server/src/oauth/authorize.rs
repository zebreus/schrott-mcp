//! Authorization endpoint: login check, consent screen, code issuance.

use axum::{
    extract::{Form, Query, RawQuery, State},
    http::{HeaderMap, StatusCode},
    response::Response,
};
use schrott_mcp_core::url_encode;
use serde::Deserialize;
use serde_json::json;

use crate::respond;
use crate::state::{session_token, session_user, AppState};

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
            respond::see_other(&loc, None)
        } else {
            respond::json(StatusCode::BAD_REQUEST, json!({"error": error}))
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
    // DCR storage first, then CIMD (cache → HTTPS fetch). Redirect-URI
    // mismatches never redirect (open-redirect protection); unknown
    // clients report `unauthorized_client` without a redirect target.
    let client = match super::cimd::resolve_client(&state, &client_id, &redirect_uri).await {
        Ok(c) => c,
        Err(e) if e.contains("redirect") => {
            return respond::json(
                StatusCode::BAD_REQUEST,
                json!({"error": "invalid_redirect_uri"}),
            );
        }
        Err(_) => {
            return err_redirect(None, "unauthorized_client", q.state.as_deref());
        }
    };
    let Some(user) = session_user(&state, &headers) else {
        let next = format!("/oauth/authorize?{raw}");
        return respond::see_other(&format!("/login?next={}", url_encode(&next)), None);
    };
    let csrf = session_token(&headers)
        .map(|t| state.issue_csrf(&t))
        .unwrap_or_default();
    let scope = q.scope.clone().unwrap_or_else(|| "read".to_owned());
    respond::html(crate::web::pages::consent(
        crate::web::pages::ConsentData {
            client_id: &client.client_id,
            client_display: &client.display_name,
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
        return respond::json(
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
        return respond::json(StatusCode::BAD_REQUEST, json!({"error": "invalid_request"}));
    }
    if form.decision != "approve" {
        let sep = if redirect_uri.contains('?') { '&' } else { '?' };
        let mut loc = format!("{redirect_uri}{sep}error=access_denied");
        if let Some(s) = state_p {
            loc.push_str(&format!("&state={}", url_encode(&s)));
        }
        return respond::see_other(&loc, None);
    }
    let Some(user) = session_user(&state, &headers) else {
        return respond::see_other("/login?next=/dashboard", None);
    };
    // Re-resolve on POST (hidden fields are user-controlled): DCR or CIMD.
    let client = match super::cimd::resolve_client(&state, &client_id, &redirect_uri).await {
        Ok(c) => c,
        Err(e) if e.contains("redirect") => {
            return respond::json(
                StatusCode::BAD_REQUEST,
                json!({"error": "invalid_redirect_uri"}),
            );
        }
        Err(_) => {
            return respond::json(
                StatusCode::BAD_REQUEST,
                json!({"error": "unauthorized_client"}),
            );
        }
    };
    let client_id = client.client_id;
    if !client.redirect_uris.iter().any(|u| u == &redirect_uri) {
        return respond::json(
            StatusCode::BAD_REQUEST,
            json!({"error": "invalid_redirect_uri"}),
        );
    }
    let code = format!("smc_{}", schrott_mcp_auth::new_token(32));
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
        return respond::json(
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({"error": "server_error"}),
        );
    }
    let sep = if redirect_uri.contains('?') { '&' } else { '?' };
    let mut loc = format!("{redirect_uri}{sep}code={}", url_encode(&code));
    if let Some(s) = state_p {
        loc.push_str(&format!("&state={}", url_encode(&s)));
    }
    respond::see_other(&loc, None)
}
