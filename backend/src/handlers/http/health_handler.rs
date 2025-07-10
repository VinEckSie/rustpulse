use axum::response::IntoResponse;
use axum::{Router, routing::get};

pub fn routes() -> Router {
    Router::new().route("/health", get(health_check))
}

pub async fn health_check() -> impl IntoResponse {
    "OK"
}
