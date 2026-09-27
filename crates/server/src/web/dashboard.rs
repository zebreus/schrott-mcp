//! Dashboard surface: account home, tokens, ingestion triggers.

use axum::{
    extract::{Form, State},
    http::{HeaderMap, Uri},
    response::Response,
};
use offsite_data_core::{url_encode, Stats};
use serde::Deserialize;

use super::pages;
use super::{check_csrf, csrf_for, forbidden, safe_next};
use crate::respond;
use crate::state::{session_user, AppState, Flash};

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
    let stats = state.public.counts().unwrap_or(Stats {
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
