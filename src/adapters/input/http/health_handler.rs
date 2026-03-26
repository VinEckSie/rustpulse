//! Health-check endpoint.

use axum::response::IntoResponse;
use axum::{Router, middleware, routing::get};

use super::request_tracing;

/// Router for the `GET /health` endpoint.
///
/// # Examples
///
/// ```rust
/// use rustpulse::adapters::input::http::health_handler;
///
/// let _router = health_handler::routes();
/// ```
pub fn routes() -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route_layer(middleware::from_fn(request_tracing::trace_middleware))
}

/// Handles `GET /health`.
///
/// # Examples
///
/// ```rust,no_run
/// # async fn demo() {
/// use rustpulse::adapters::input::http::health_handler;
///
/// let _router = health_handler::routes();
/// # }
/// ```
pub async fn health_check() -> impl IntoResponse {
    "OK"
}
