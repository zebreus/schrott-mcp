//! Website handlers live here, split by surface.
//! Conventions: typed input echoes back on validation errors, one-time
//! secrets/notices travel in server-side flash state (never URLs), and every
//! cookie-authenticated POST carries a CSRF token.

use axum::{http::HeaderMap, response::Response};

use crate::respond;
use crate::state::{session_token, AppState};

pub mod auth;
pub mod dashboard;
pub mod pages;
pub mod site;
pub(super) mod style;

/// Only allow relative redirects inside this site.
pub(super) fn safe_next(next: &str) -> &str {
    if next.starts_with('/') && !next.starts_with("//") {
        next
    } else {
        "/dashboard"
    }
}

/// Look up the CSRF token for this browser session, if any.
pub(super) fn csrf_for(state: &AppState, headers: &HeaderMap) -> Option<String> {
    let session = session_token(headers)?;
    state
        .csrf
        .lock()
        .ok()
        .and_then(|map| map.get(&session).cloned())
}

/// Reject cookie POSTs with a missing or wrong CSRF token.
pub(super) fn check_csrf(state: &AppState, headers: &HeaderMap, provided: &str) -> bool {
    state.check_csrf(session_token(headers).as_deref(), provided)
}

pub(super) fn forbidden(state: &AppState) -> Response {
    respond::html(self::pages::error_page(
        &state.base_url,
        "/forbidden",
        "Verboten",
        "Ungültiges oder fehlendes CSRF-Token. Bitte lade die Seite neu und versuche es erneut.",
        None,
        None,
    ))
}
