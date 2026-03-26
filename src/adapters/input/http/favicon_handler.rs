//! Favicon endpoint.

use axum::http::StatusCode;
use axum::{Router, middleware, routing::get};

use super::request_tracing;

/// Router for the `GET /favicon.ico` endpoint.
///
/// # Examples
///
/// ```rust
/// use rustpulse::adapters::input::http::favicon_handler;
///
/// let _router = favicon_handler::routes();
/// ```
pub fn routes() -> Router {
    Router::new()
        .route("/favicon.ico", get(favicon_handler))
        .route_layer(middleware::from_fn(request_tracing::trace_middleware))
}

/// Handles `GET /favicon.ico`.
///
/// The service does not currently serve a favicon, so this responds with 204.
///
/// # Examples
///
/// ```rust,no_run
/// # async fn demo() {
/// use rustpulse::adapters::input::http::favicon_handler;
///
/// let _router = favicon_handler::routes();
/// # }
/// ```
pub async fn favicon_handler() -> StatusCode {
    StatusCode::NO_CONTENT // 204
}
