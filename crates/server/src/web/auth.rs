//! Account surface: signup, login, logout.

use axum::{
    extract::{Form, Query, State},
    http::HeaderMap,
    response::Response,
};
use offsite_data_core::{validate_password, validate_username};
use serde::Deserialize;
use std::collections::HashMap;

use super::pages;
use super::{check_csrf, forbidden, safe_next};
use crate::respond;
use crate::state::{clear_session_cookie, session_cookie, session_token, session_user, AppState};

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
