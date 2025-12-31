use axum::http::StatusCode;
use axum::{Router, middleware, routing::get};

use super::request_tracing;

pub fn routes() -> Router {
    Router::new()
        .route("/favicon.ico", get(favicon_handler))
        .route_layer(middleware::from_fn(request_tracing::trace_middleware))
}

pub async fn favicon_handler() -> StatusCode {
    StatusCode::NO_CONTENT // 204
}
