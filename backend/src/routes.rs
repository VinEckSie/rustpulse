pub mod health;

use axum::{Router, routing::get, response::Html};
use crate::routes::health::health_check;

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(index_handler))
        .route("/health", get(health_check))
}

async fn index_handler() -> Html<&'static str> {
    tracing::info!("🚀 RustPulse started on http://localhost:3000");
    Html("<h1>Hello, RustPulse!</h1>")
}