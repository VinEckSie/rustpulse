use axum::http::StatusCode;
use axum::{routing::get, Router};

pub fn routes() -> Router {
    Router::new().route("/favicon.ico", get(favicon_handler))
}

pub async fn favicon_handler() -> StatusCode {
    StatusCode::NO_CONTENT // 204
}
