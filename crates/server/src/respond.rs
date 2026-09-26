//! One place for HTTP responses so handlers stay thin and consistent.

use axum::{body::Body, http::StatusCode, response::Response};

fn body(status: StatusCode, content_type: &str, text: String) -> Response {
    Response::builder()
        .status(status)
        .header("content-type", content_type)
        .body(Body::from(text))
        .expect("response builds")
}

/// Rendered HTML page.
pub fn html(page: String) -> Response {
    body(StatusCode::OK, "text/html; charset=utf-8", page)
}

/// JSON API response.
pub fn json(status: StatusCode, value: serde_json::Value) -> Response {
    let mut res = body(status, "application/json", value.to_string());
    res.headers_mut().insert(
        "cache-control",
        "no-store".parse().expect("static header parses"),
    );
    res
}

/// 303 redirect, optionally setting a cookie.
pub fn see_other(to: &str, cookie: Option<String>) -> Response {
    let mut builder = Response::builder().status(StatusCode::SEE_OTHER);
    builder = builder.header("location", to);
    if let Some(c) = cookie {
        builder = builder.header("set-cookie", c);
    }
    builder.body(Body::empty()).expect("redirect builds")
}

/// Empty response with just a status (e.g. 202 for notifications, 405 hints
/// are JSON instead — see `json`).
pub fn empty(status: StatusCode) -> Response {
    Response::builder()
        .status(status)
        .body(Body::empty())
        .expect("empty response builds")
}
