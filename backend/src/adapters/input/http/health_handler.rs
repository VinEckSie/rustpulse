use axum::response::IntoResponse;
use axum::{Router, middleware, routing::get};

use super::request_tracing;

pub fn routes() -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route_layer(middleware::from_fn(request_tracing::trace_middleware))
}

pub async fn health_check() -> impl IntoResponse {
    "OK"
}
