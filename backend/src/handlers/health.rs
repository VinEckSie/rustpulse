use axum::response::IntoResponse;
use axum::{routing::get, Router};

pub fn router() -> Router {
    Router::new().route("/health", get(health_check))
}

pub async fn health_check() -> impl IntoResponse {
    "OK"
}
