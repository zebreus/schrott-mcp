//! Client ID Metadata Documents (CIMD): stateless alternative to DCR.
//!
//! A CIMD client uses an `https://` URL as its `client_id`. That URL serves
//! a JSON metadata document (same shape as a DCR response). The server
//! fetches it on demand, validates it, and uses it for the OAuth flow —
//! no prior `POST /oauth/register` needed.
//!
//! Spec: IETF draft-ietf-oauth-client-id-metadata-document, adopted by MCP
//! via SEP-991 (`2025-11-25` makes CIMD the default client path).
//!
//! Trust policy here is deliberately the same as our open DCR policy: any
//! document that fetches, parses and validates is accepted. The user still
//! sees the consent screen with the verified `client_id` URL (plus
//! `client_name` when the document provides one) and must approve.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use serde::Deserialize;

use crate::state::AppState;

/// How long a fetched CIMD document is trusted without re-fetching.
const CACHE_TTL: Duration = Duration::from_secs(3600);
/// Upper bound for the in-memory CIMD cache (cleared when exceeded,
/// same pattern as the CSRF map in `state.rs`).
const CACHE_MAX_ENTRIES: usize = 1000;
/// Hard cap for a fetched document body (documents are tiny JSON).
const MAX_BODY_BYTES: usize = 32 * 1024;
/// Per-fetch timeout on top of the shared client's 30 s default.
const FETCH_TIMEOUT: Duration = Duration::from_secs(10);

/// Subset of OAuth client metadata we care about. Unknown fields are
/// ignored so future document extensions keep working.
#[derive(Debug, Clone, Deserialize)]
pub struct CimdDocument {
    /// Must exactly equal the `client_id` URL the client sent.
    pub client_id: String,
    pub redirect_uris: Vec<String>,
    pub client_name: Option<String>,
    #[allow(dead_code)]
    pub client_uri: Option<String>,
    #[allow(dead_code)]
    pub logo_uri: Option<String>,
    #[allow(dead_code)]
    pub scope: Option<String>,
    pub token_endpoint_auth_method: Option<String>,
}

/// Cached document plus fetch time for TTL expiry.
#[derive(Debug, Clone)]
pub struct CachedCimd {
    pub doc: CimdDocument,
    pub fetched_at: Instant,
}

/// Shared cache type stored on [`AppState`].
pub type CimdCache = Arc<Mutex<HashMap<String, CachedCimd>>>;

/// Create an empty CIMD cache (used at startup and in tests).
pub fn new_cache() -> CimdCache {
    Arc::new(Mutex::new(HashMap::new()))
}

/// True when `id` looks like a CIMD `client_id`: an `https://` URL without
/// fragment. `http://localhost|127.0.0.1|[::1]` is allowed for local dev
/// and tests (mirrors the redirect-URI policy in `register.rs`).
pub fn is_cimd_client_id(id: &str) -> bool {
    if id.is_empty() || id.len() > 2000 || id.contains('#') {
        return false;
    }
    if let Some(rest) = id.strip_prefix("https://") {
        // No userinfo (`user@host`) — avoids credential-bearing URLs.
        let authority = rest.split('/').next().unwrap_or_default();
        !authority.is_empty() && !authority.contains('@')
    } else if let Some(rest) = id
        .strip_prefix("http://localhost")
        .or_else(|| id.strip_prefix("http://127.0.0.1"))
        .or_else(|| id.strip_prefix("http://[::1]"))
    {
        // Localhost URLs must end at the host or continue with `:port`/`/`.
        rest.is_empty() || rest.starts_with(':') || rest.starts_with('/')
    } else {
        false
    }
}

/// Same redirect-URI policy as DCR registration: `https://` (or localhost
/// `http://`), no fragment.
pub fn valid_redirect_uri(uri: &str) -> bool {
    if uri.is_empty() || uri.contains('#') {
        return false;
    }
    uri.starts_with("https://")
        || uri.starts_with("http://localhost")
        || uri.starts_with("http://127.0.0.1")
}

/// Resolved client for the authorize flow, from DCR storage or CIMD.
#[derive(Debug, Clone)]
pub struct ResolvedClient {
    pub client_id: String,
    pub redirect_uris: Vec<String>,
    /// Human-readable label for the consent screen.
    pub display_name: String,
}

/// Validate an already-parsed document against the request values.
pub fn validate_document(
    doc: &CimdDocument,
    expected_client_id: &str,
    redirect_uri: &str,
) -> Result<(), String> {
    if doc.client_id != expected_client_id {
        return Err("client_id mismatch between request and document".to_owned());
    }
    if doc.redirect_uris.is_empty() || doc.redirect_uris.len() > 100 {
        return Err("redirect_uris must be non-empty (max 100)".to_owned());
    }
    for uri in &doc.redirect_uris {
        if !valid_redirect_uri(uri) {
            return Err(format!("invalid redirect_uri in document: {uri}"));
        }
    }
    if !doc.redirect_uris.iter().any(|u| u == redirect_uri) {
        return Err("redirect_uri not listed in client metadata".to_owned());
    }
    // We only serve public clients (`auth_method=none`, like DCR). A
    // document demanding a secret would fail later anyway — reject early
    // with a clear error instead of silently downgrading.
    if let Some(method) = doc.token_endpoint_auth_method.as_deref() {
        if method != "none" {
            return Err(format!(
                "unsupported token_endpoint_auth_method: {method} (only 'none')"
            ));
        }
    }
    Ok(())
}

/// Parse a fetched body and validate it. Split out for unit testing
/// without network access.
pub fn parse_and_validate(
    body: &[u8],
    expected_client_id: &str,
    redirect_uri: &str,
) -> Result<CimdDocument, String> {
    if body.len() > MAX_BODY_BYTES {
        return Err("client metadata document too large".to_owned());
    }
    let doc: CimdDocument =
        serde_json::from_slice(body).map_err(|e| format!("invalid client metadata JSON: {e}"))?;
    validate_document(&doc, expected_client_id, redirect_uri)?;
    Ok(doc)
}

/// Fetch the document at `client_id` (which *is* the URL) and validate it
/// against `redirect_uri`.
pub async fn fetch_cimd_document(
    http: &reqwest::Client,
    client_id: &str,
    redirect_uri: &str,
) -> Result<CimdDocument, String> {
    let res = http
        .get(client_id)
        .header("Accept", "application/json")
        .timeout(FETCH_TIMEOUT)
        .send()
        .await
        .map_err(|e| format!("cannot fetch client metadata: {e}"))?;
    if !res.status().is_success() {
        return Err(format!(
            "client metadata fetch failed with status {}",
            res.status()
        ));
    }
    let body = res
        .bytes()
        .await
        .map_err(|e| format!("cannot read client metadata: {e}"))?;
    parse_and_validate(&body, client_id, redirect_uri)
}

/// Resolve a client for the authorize flow: DCR storage first, then CIMD
/// (cache → network). Returns the resolved client or a human-readable
/// reason used for `unauthorized_client` responses.
pub async fn resolve_client(
    state: &AppState,
    client_id: &str,
    redirect_uri: &str,
) -> Result<ResolvedClient, String> {
    // 1. Dynamic Client Registration: `cli_*` IDs from `/oauth/register`.
    if let Ok(Some(row)) = state.internal.find_oauth_client(client_id) {
        if !row.redirect_uris.iter().any(|u| u == redirect_uri) {
            return Err("redirect_uri not registered for this client".to_owned());
        }
        return Ok(ResolvedClient {
            display_name: row.client_id.clone(),
            client_id: row.client_id,
            redirect_uris: row.redirect_uris,
        });
    }
    // 2. CIMD: URL-style `client_id` with a hosted metadata document.
    if !is_cimd_client_id(client_id) {
        return Err("unknown client".to_owned());
    }
    if !valid_redirect_uri(redirect_uri) {
        return Err("invalid redirect_uri".to_owned());
    }
    // Cache hit (fresh + already lists this redirect).
    if let Ok(map) = state.cimd_cache.lock() {
        if let Some(cached) = map.get(client_id) {
            if cached.fetched_at.elapsed() < CACHE_TTL
                && cached.doc.redirect_uris.iter().any(|u| u == redirect_uri)
                && cached.doc.client_id == client_id
            {
                let doc = cached.doc.clone();
                return Ok(ResolvedClient {
                    display_name: doc.client_name.clone().unwrap_or(doc.client_id.clone()),
                    client_id: doc.client_id.clone(),
                    redirect_uris: doc.redirect_uris.clone(),
                });
            }
        }
    }
    // Cache miss / stale: fetch, validate, store.
    let doc = fetch_cimd_document(&state.http, client_id, redirect_uri).await?;
    if let Ok(mut map) = state.cimd_cache.lock() {
        if map.len() > CACHE_MAX_ENTRIES {
            map.clear();
        }
        map.insert(
            client_id.to_owned(),
            CachedCimd {
                doc: doc.clone(),
                fetched_at: Instant::now(),
            },
        );
    }
    Ok(ResolvedClient {
        display_name: doc.client_name.clone().unwrap_or(doc.client_id.clone()),
        client_id: doc.client_id.clone(),
        redirect_uris: doc.redirect_uris.clone(),
    })
}

#[cfg(test)]
mod tests {
    use super::{
        is_cimd_client_id, parse_and_validate, valid_redirect_uri, validate_document, CimdDocument,
    };

    fn doc() -> CimdDocument {
        CimdDocument {
            client_id: "https://client.example.com/meta.json".to_owned(),
            redirect_uris: vec!["https://client.example.com/cb".to_owned()],
            client_name: Some("Example".to_owned()),
            client_uri: None,
            logo_uri: None,
            scope: None,
            token_endpoint_auth_method: Some("none".to_owned()),
        }
    }

    #[test]
    fn client_id_shape() {
        assert!(is_cimd_client_id("https://client.example.com/meta.json"));
        assert!(is_cimd_client_id("https://example.com/cimd"));
        assert!(is_cimd_client_id("http://localhost:3000/cimd"));
        assert!(is_cimd_client_id("http://127.0.0.1:8080/meta"));
        // Rejected: DCR IDs, fragments, plain http, userinfo, empty, too long.
        assert!(!is_cimd_client_id("cli_abc123"));
        assert!(!is_cimd_client_id("https://example.com/meta#frag"));
        assert!(!is_cimd_client_id("http://example.com/meta"));
        assert!(!is_cimd_client_id("https://user@example.com/meta"));
        assert!(!is_cimd_client_id(""));
        assert!(!is_cimd_client_id(&format!(
            "https://example.com/{}",
            "a".repeat(2000)
        )));
    }

    #[test]
    fn redirect_policy_matches_dcr() {
        assert!(valid_redirect_uri("https://app.example.com/cb"));
        assert!(valid_redirect_uri("http://localhost:3000/cb"));
        assert!(!valid_redirect_uri("http://example.com/cb"));
        assert!(!valid_redirect_uri("https://example.com/cb#frag"));
        assert!(!valid_redirect_uri(""));
    }

    #[test]
    fn document_validation() {
        let d = doc();
        assert!(validate_document(&d, &d.client_id, &d.redirect_uris[0]).is_ok());
        assert!(validate_document(&d, "https://other.example.com/x", &d.redirect_uris[0]).is_err());
        assert!(validate_document(&d, &d.client_id, "https://evil.example.com/cb").is_err());
        let mut bad_auth = d.clone();
        bad_auth.token_endpoint_auth_method = Some("client_secret_basic".to_owned());
        assert!(
            validate_document(&bad_auth, &bad_auth.client_id, &bad_auth.redirect_uris[0]).is_err()
        );
        let mut missing_auth = d.clone();
        missing_auth.token_endpoint_auth_method = None;
        assert!(validate_document(
            &missing_auth,
            &missing_auth.client_id,
            &missing_auth.redirect_uris[0]
        )
        .is_ok());
    }

    #[test]
    fn parse_happy_path_and_mismatch() {
        let body = serde_json::to_vec(&serde_json::json!({
            "client_id": "https://client.example.com/meta.json",
            "redirect_uris": ["https://client.example.com/cb"],
            "client_name": "Example",
            "token_endpoint_auth_method": "none",
        }))
        .unwrap();
        let got = parse_and_validate(
            &body,
            "https://client.example.com/meta.json",
            "https://client.example.com/cb",
        )
        .unwrap();
        assert_eq!(got.client_name.as_deref(), Some("Example"));
        assert!(parse_and_validate(
            &body,
            "https://other.example.com/x",
            "https://client.example.com/cb"
        )
        .is_err());
        assert!(parse_and_validate(
            &b"not json"[..],
            "https://client.example.com/meta.json",
            "https://client.example.com/cb"
        )
        .is_err());
        assert!(parse_and_validate(
            &vec![b'x'; 40 * 1024],
            "https://client.example.com/meta.json",
            "https://client.example.com/cb"
        )
        .is_err());
    }
}
