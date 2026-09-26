//! Website handlers: marketing page, signup/login, dashboard, token
//! management and manual ingestion triggers.
//!
//! Conventions: typed input echoes back on validation errors, one-time
//! secrets/notices travel in server-side flash state (never URLs), and every
//! cookie-authenticated POST carries a CSRF token.

use axum::{
    body::Body,
    extract::{Form, Path, Query, State},
    http::{HeaderMap, StatusCode, Uri},
    response::Response,
};
use offsite_data_core::{url_encode, validate_password, validate_username};
use serde::Deserialize;
use std::collections::HashMap;

use super::pages;
use super::respond;
use super::state::{
    clear_session_cookie, session_cookie, session_token, session_user, AppState, Flash,
};

/// Only allow relative redirects inside this site.
fn safe_next(next: &str) -> &str {
    if next.starts_with('/') && !next.starts_with("//") {
        next
    } else {
        "/dashboard"
    }
}

/// Look up the CSRF token for this browser session, if any.
fn csrf_for(state: &AppState, headers: &HeaderMap) -> Option<String> {
    let session = session_token(headers)?;
    state
        .csrf
        .lock()
        .ok()
        .and_then(|map| map.get(&session).cloned())
}

/// Reject cookie POSTs with a missing or wrong CSRF token.
fn check_csrf(state: &AppState, headers: &HeaderMap, provided: &str) -> bool {
    state.check_csrf(session_token(headers).as_deref(), provided)
}

fn forbidden(state: &AppState) -> Response {
    respond::html(pages::error_page(
        &state.base_url,
        "/forbidden",
        "Forbidden",
        "Invalid or missing CSRF token. Please reload the page and try again.",
        None,
        None,
    ))
}

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
            if super::state::is_expired(&expires_at) {
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

/// Signup form. Already logged in? Go to the dashboard.
/// `next` carries the post-signup destination (e.g. an OAuth authorize URL).
pub async fn signup_form(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(q): Query<HashMap<String, String>>,
) -> Response {
    if session_user(&state, &headers).is_some() {
        return respond::see_other("/dashboard", None);
    }
    let next = q.get("next").map(String::as_str).unwrap_or("/dashboard");
    respond::html(pages::signup(&state.base_url, next, "", false, None))
}

#[derive(Deserialize)]
pub struct SignupForm {
    username: String,
    password: String,
    confirm: String,
    /// Present (any value) only when the checkbox was ticked.
    professional: Option<String>,
    next: Option<String>,
}

/// Handle signup: username + password + professional checkbox, nothing else.
/// Typed values echo back on errors; passwords never do.
pub async fn signup_submit(
    State(state): State<AppState>,
    Form(form): Form<SignupForm>,
) -> Response {
    let username = form.username.trim().to_owned();
    let professional = form.professional.is_some();
    let next = form.next.unwrap_or_else(|| "/dashboard".to_owned());
    let next = safe_next(&next).to_owned();
    let err_base = state.base_url.clone();
    let err_next = next.clone();
    let err = |msg: &str| {
        respond::html(pages::signup(
            &err_base,
            &err_next,
            &username,
            professional,
            Some(msg),
        ))
    };
    if let Err(e) = validate_username(&username) {
        return err(&e.to_string());
    }
    if let Err(e) = validate_password(&form.password) {
        return err(&e.to_string());
    }
    if form.password != form.confirm {
        return err("Passwords do not match.");
    }
    if !professional {
        return err("Please confirm you are a professional data-user.");
    }
    let hash = match offsite_data_auth::hash_password(&form.password) {
        Ok(h) => h,
        Err(e) => return err(&e.to_string()),
    };
    let user_id = match state
        .internal
        .create_user(&username, &hash, true, &AppState::now())
    {
        Ok(id) => id,
        Err(offsite_data_store::StoreError::Exists(_)) => {
            return err("That username is taken.");
        }
        Err(e) => {
            return respond::html(pages::error_page(
                &state.base_url,
                "/signup",
                "Signup failed",
                &format!("Could not create your account: {e}"),
                None,
                None,
            ));
        }
    };
    let token = offsite_data_auth::new_token(32);
    let exp = (chrono::Utc::now() + chrono::Duration::days(30)).to_rfc3339();
    if state
        .internal
        .create_session(&token, user_id, &AppState::now(), &exp)
        .is_err()
    {
        return respond::html(pages::error_page(
            &state.base_url,
            "/signup",
            "Signup failed",
            "Account created, but the login session could not be stored. Please log in.",
            None,
            None,
        ));
    }
    let _ = state.issue_csrf(&token);
    respond::see_other(&next, Some(session_cookie(&token, state.secure_cookies())))
}

/// Login form. `next` is where to go afterwards.
pub async fn login_form(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(q): Query<HashMap<String, String>>,
) -> Response {
    if session_user(&state, &headers).is_some() {
        return respond::see_other("/dashboard", None);
    }
    let next = q.get("next").map(String::as_str).unwrap_or("/dashboard");
    respond::html(pages::login(&state.base_url, next, "", None))
}

#[derive(Deserialize)]
pub struct LoginForm {
    username: String,
    password: String,
    next: Option<String>,
}

/// Handle login. The username echoes back on errors.
pub async fn login_submit(State(state): State<AppState>, Form(form): Form<LoginForm>) -> Response {
    let username = form.username.trim().to_owned();
    let next = form.next.unwrap_or_else(|| "/dashboard".to_owned());
    let next = safe_next(&next).to_owned();
    let err_base = state.base_url.clone();
    let err = |msg: &str| respond::html(pages::login(&err_base, &next, &username, Some(msg)));
    let user = match state.internal.find_user_by_username(&username) {
        Ok(Some(u)) => u,
        Ok(None) => return err("Unknown username or wrong password."),
        Err(e) => {
            return respond::html(pages::error_page(
                &state.base_url,
                "/login",
                "Login failed",
                &format!("Could not look up your account: {e}"),
                None,
                None,
            ));
        }
    };
    if !offsite_data_auth::verify_password(&user.password_hash, &form.password) {
        return err("Unknown username or wrong password.");
    }
    let token = offsite_data_auth::new_token(32);
    let exp = (chrono::Utc::now() + chrono::Duration::days(30)).to_rfc3339();
    if state
        .internal
        .create_session(&token, user.id, &AppState::now(), &exp)
        .is_err()
    {
        return respond::html(pages::error_page(
            &state.base_url,
            "/login",
            "Login failed",
            "Password correct, but the login session could not be stored. Please try again.",
            None,
            None,
        ));
    }
    let _ = state.issue_csrf(&token);
    respond::see_other(&next, Some(session_cookie(&token, state.secure_cookies())))
}

#[derive(Deserialize)]
pub struct LogoutForm {
    csrf: String,
}

/// Log out (drop the session cookie server-side too).
pub async fn logout(
    State(state): State<AppState>,
    headers: HeaderMap,
    Form(form): Form<LogoutForm>,
) -> Response {
    if !check_csrf(&state, &headers, &form.csrf) {
        return forbidden(&state);
    }
    if let Some(token) = session_token(&headers) {
        let _ = state.internal.delete_session(&token);
        state.drop_csrf(&token);
    }
    respond::see_other("/", Some(clear_session_cookie(state.secure_cookies())))
}

// -- dashboard ------------------------------------------------------------

/// Account dashboard. Requires a session; consumes flash state.
pub async fn dashboard(State(state): State<AppState>, headers: HeaderMap, uri: Uri) -> Response {
    let Some(user) = session_user(&state, &headers) else {
        // Keep the full destination (path + query) for after login.
        let dest = uri
            .path_and_query()
            .map(|pq| pq.as_str().to_owned())
            .unwrap_or_else(|| "/dashboard".to_owned());
        let next = url_encode(safe_next(&dest));
        return respond::see_other(&format!("/login?next={next}"), None);
    };
    let stats = state.public.counts().unwrap_or(offsite_data_core::Stats {
        sources: 0,
        datasets: 0,
        items: 0,
    });
    let tokens = state.internal.list_api_tokens(user.id).unwrap_or_default();
    let runs = state.internal.last_runs(8).unwrap_or_default();
    let flash = state.take_flash(user.id);
    let csrf = csrf_for(&state, &headers).unwrap_or_default();
    let data = pages::DashboardData {
        username: &user.username,
        csrf: &csrf,
        stats: &stats,
        tokens: &tokens,
        runs: &runs,
        flash_secret: flash.token_secret.as_deref(),
        flash_notice: flash.notice.as_deref(),
    };
    respond::html(pages::dashboard(&state.base_url, data))
}

#[derive(Deserialize)]
pub struct TokenForm {
    name: String,
    csrf: String,
}

/// Create a personal access token; the secret renders once via flash state.
pub async fn create_token(
    State(state): State<AppState>,
    headers: HeaderMap,
    Form(form): Form<TokenForm>,
) -> Response {
    let Some(user) = session_user(&state, &headers) else {
        return respond::see_other("/login?next=/dashboard", None);
    };
    if !check_csrf(&state, &headers, &form.csrf) {
        return forbidden(&state);
    }
    let name = form.name.trim();
    let name = if name.is_empty() {
        "mcp".to_owned()
    } else {
        name.to_owned()
    };
    let secret = format!("odp_{}", offsite_data_auth::new_token(30));
    let prefix: String = secret.chars().take(12).collect();
    let hash = offsite_data_auth::sha256_hex(&secret);
    match state
        .internal
        .create_api_token(user.id, &name, &prefix, &hash, &AppState::now())
    {
        Ok(_) => {
            state.set_flash(
                user.id,
                Flash {
                    token_secret: Some(secret),
                    notice: None,
                },
            );
            respond::see_other("/dashboard", None)
        }
        Err(_) => {
            state.set_flash(
                user.id,
                Flash {
                    token_secret: None,
                    notice: Some("Could not create the token. Please try again.".to_owned()),
                },
            );
            respond::see_other("/dashboard", None)
        }
    }
}

#[derive(Deserialize)]
pub struct DeleteTokenForm {
    id: String,
    csrf: String,
}

/// Revoke a personal access token, with a confirmation banner.
pub async fn delete_token(
    State(state): State<AppState>,
    headers: HeaderMap,
    Form(form): Form<DeleteTokenForm>,
) -> Response {
    let Some(user) = session_user(&state, &headers) else {
        return respond::see_other("/login?next=/dashboard", None);
    };
    if !check_csrf(&state, &headers, &form.csrf) {
        return forbidden(&state);
    }
    let notice = match form.id.parse::<i64>() {
        Ok(id)
            if state
                .internal
                .delete_api_token(id, user.id)
                .unwrap_or(false) =>
        {
            "Token revoked."
        }
        Ok(_) => "That token does not exist (or is already gone).",
        Err(_) => "Invalid token id.",
    };
    state.set_flash(
        user.id,
        Flash {
            token_secret: None,
            notice: Some(notice.to_owned()),
        },
    );
    respond::see_other("/dashboard", None)
}

#[derive(Deserialize)]
pub struct IngestForm {
    csrf: String,
}

/// Trigger an ingestion run in the background. Requires a session.
/// The dashboard shows completion via the runs table; the banner is flash-only
/// so it appears solely after a real POST, never on deep links.
pub async fn ingest_run(
    State(state): State<AppState>,
    headers: HeaderMap,
    Form(form): Form<IngestForm>,
) -> Response {
    let Some(user) = session_user(&state, &headers) else {
        return respond::see_other("/login?next=/dashboard", None);
    };
    if !check_csrf(&state, &headers, &form.csrf) {
        return forbidden(&state);
    }
    let task_state = state.clone();
    tokio::spawn(async move {
        offsite_data_ingestion::run_once(
            &task_state.internal,
            &task_state.public,
            &task_state.http,
        )
        .await;
    });
    state.set_flash(
        user.id,
        Flash {
            token_secret: None,
            notice: Some(
                "Ingestion run started in the background — watch the runs table below.".to_owned(),
            ),
        },
    );
    respond::see_other("/dashboard", None)
}
