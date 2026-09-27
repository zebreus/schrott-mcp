//! Public site surface: marketing page, health, downloads, 404.

use axum::{
    body::Body,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::Response,
};

use super::pages;
use crate::respond;
use crate::state::{session_user, AppState};

/// Download a full query result blob: `/d/{secret}/result.json`.
/// The 128-bit base62 secret *is* the authorization — unguessable links,
/// valid for 7 days, served without login so agents can fetch them.
pub async fn download_result(
    State(state): State<AppState>,
    Path(secret): Path<String>,
) -> Response {
    let not_found = || {
        respond::json(
            StatusCode::NOT_FOUND,
            serde_json::json!({"error": "not_found"}),
        )
    };
    if secret.is_empty() || secret.len() > 64 || !secret.chars().all(|c| c.is_ascii_alphanumeric())
    {
        return not_found();
    }
    match state.internal.find_result_blob(&secret) {
        Ok(Some((payload, expires_at))) => {
            if crate::state::is_expired(&expires_at) {
                let _ = state.internal.delete_result_blob(&secret);
                return not_found();
            }
            Response::builder()
                .status(StatusCode::OK)
                .header("content-type", "application/json")
                .header("cache-control", "public, max-age=604800")
                .body(Body::from(payload))
                .expect("blob response builds")
        }
        _ => not_found(),
    }
}

// -- public pages ---------------------------------------------------------

/// Site health for nginx/systemd checks.
pub async fn health() -> Response {
    respond::json(
        StatusCode::OK,
        serde_json::json!({"ok": true, "service": "offsite-data"}),
    )
}

/// Marketing front page.
pub async fn index(State(state): State<AppState>, headers: HeaderMap) -> Response {
    let user = session_user(&state, &headers).map(|u| u.username);
    respond::html(pages::marketing(&state.base_url, user.as_deref()))
}

/// Styled 404 for everything unmapped.
pub async fn fallback_404(State(state): State<AppState>) -> Response {
    let mut res = respond::html(pages::error_page(
        &state.base_url,
        "/not-found",
        "Not found",
        "There is nothing at this address.",
        None,
        None,
    ));
    *res.status_mut() = StatusCode::NOT_FOUND;
    res
}
