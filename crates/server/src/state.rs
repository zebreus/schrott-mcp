//! Shared server state plus cookie/bearer authentication helpers.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use axum::http::HeaderMap;
use chrono::{DateTime, Utc};
use schrott_mcp_store::{InternalDb, PublicDb};

/// One-time dashboard payloads. Secrets and notices travel here — never in
/// URLs (which leak into history, logs and Referer headers).
#[derive(Debug, Default)]
pub struct Flash {
    /// A freshly created personal token, shown exactly once.
    pub token_secret: Option<String>,
    /// A one-line confirmation ("Token revoked", "Ingestion started", …).
    pub notice: Option<String>,
}

/// Everything handlers need. Cheap to clone.
#[derive(Clone)]
pub struct AppState {
    pub internal: Arc<InternalDb>,
    pub public: Arc<PublicDb>,
    pub http: reqwest::Client,
    pub base_url: String,
    /// Data directory, handed to the isolated query worker per query.
    pub data_dir: std::path::PathBuf,
    /// Pending one-time payloads per user id.
    pub flash: Arc<Mutex<HashMap<i64, Flash>>>,
    /// CSRF tokens per browser session token (double-submit, no schema).
    pub csrf: Arc<Mutex<HashMap<String, String>>>,
}

impl AppState {
    /// Current time as RFC 3339.
    pub fn now() -> String {
        Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
    }

    /// Cookies should be `Secure` when serving public HTTPS.
    pub fn secure_cookies(&self) -> bool {
        self.base_url.starts_with("https://")
    }

    /// Store a one-time payload for a user.
    pub fn set_flash(&self, user_id: i64, flash: Flash) {
        if let Ok(mut map) = self.flash.lock() {
            map.insert(user_id, flash);
        }
    }

    /// Take (and clear) a user's pending payload, if any.
    pub fn take_flash(&self, user_id: i64) -> Flash {
        self.flash
            .lock()
            .ok()
            .and_then(|mut map| map.remove(&user_id))
            .unwrap_or_default()
    }

    /// Mint a CSRF token bound to a browser session.
    pub fn issue_csrf(&self, session_token: &str) -> String {
        let csrf = schrott_mcp_auth::new_token(16);
        if let Ok(mut map) = self.csrf.lock() {
            if map.len() > 10_000 {
                map.clear();
            }
            map.insert(session_token.to_owned(), csrf.clone());
        }
        csrf
    }

    /// Check a submitted CSRF token against the session's token.
    /// Entries whose session vanished are dropped on sight.
    pub fn check_csrf(&self, session_token: Option<&str>, provided: &str) -> bool {
        let Some(session) = session_token else {
            return false;
        };
        let Ok(map) = self.csrf.lock() else {
            return false;
        };
        match map.get(session) {
            Some(expected) if expected == provided => true,
            Some(_) => false,
            None => false,
        }
    }

    /// Forget a session's CSRF token (on logout).
    pub fn drop_csrf(&self, session_token: &str) {
        if let Ok(mut map) = self.csrf.lock() {
            map.remove(session_token);
        }
    }
}

/// Parse the `Cookie` header into pairs.
fn cookies(headers: &HeaderMap) -> Vec<(String, String)> {
    let mut out = Vec::new();
    for value in headers.get_all("cookie").iter() {
        let Ok(raw) = value.to_str() else { continue };
        for part in raw.split(';') {
            let part = part.trim();
            if let Some((k, v)) = part.split_once('=') {
                out.push((k.trim().to_owned(), v.trim().to_owned()));
            }
        }
    }
    out
}

/// Session token from the browser cookie, if present.
pub fn session_token(headers: &HeaderMap) -> Option<String> {
    cookies(headers)
        .into_iter()
        .find(|(k, _)| k == "od_session")
        .map(|(_, v)| v)
}

/// Bearer token from `Authorization: Bearer ...`, if present.
pub fn bearer_token(headers: &HeaderMap) -> Option<String> {
    headers
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .map(str::to_owned)
}

/// Logged-in website user for this request, if the session is valid.
pub fn session_user(
    state: &AppState,
    headers: &HeaderMap,
) -> Option<schrott_mcp_store::internal::UserRow> {
    let token = session_token(headers)?;
    let session = state.internal.find_session(&token).ok()??;
    if is_expired(&session.expires_at) {
        return None;
    }
    state.internal.find_user_by_id(session.user_id).ok()?
}

/// MCP caller identity: a live OAuth access token or a personal token.
/// Touches `last_used_at` for personal tokens.
pub fn bearer_user(state: &AppState, headers: &HeaderMap) -> Option<i64> {
    let token = bearer_token(headers)?;
    if let Ok(Some(row)) = state.internal.find_access_token(&token) {
        if !is_expired(&row.expires_at) {
            return Some(row.user_id);
        }
    }
    let hash = schrott_mcp_auth::sha256_hex(&token);
    if let Ok(Some(secret)) = state.internal.find_api_token_by_hash(&hash) {
        let _ = state.internal.touch_api_token(secret.id, &AppState::now());
        return Some(secret.user_id);
    }
    None
}

/// True when an RFC 3339 timestamp lies in the past (or is unparsable).
pub fn is_expired(ts: &str) -> bool {
    DateTime::parse_from_rfc3339(ts)
        .map(|t| t.with_timezone(&Utc) <= Utc::now())
        .unwrap_or(true)
}

/// Value for the `Set-Cookie` header carrying a browser session.
pub fn session_cookie(token: &str, secure: bool) -> String {
    let mut c = format!("od_session={token}; Path=/; HttpOnly; SameSite=Lax; Max-Age=2592000");
    if secure {
        c.push_str("; Secure");
    }
    c
}

/// Value clearing the browser session.
pub fn clear_session_cookie(secure: bool) -> String {
    let mut c = "od_session=; Path=/; HttpOnly; SameSite=Lax; Max-Age=0".to_owned();
    if secure {
        c.push_str("; Secure");
    }
    c
}
