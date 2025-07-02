use axum::response::IntoResponse;
use axum::{Router, response::Html, routing::get};

pub fn router() -> Router {
    Router::new()
        .route("/health", get(health_check))
}

pub async fn health_check() -> impl IntoResponse {
    "OK"
}