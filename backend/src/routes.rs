pub mod health;

use crate::routes::health::health_check;
use axum::{Router, response::Html, routing::get};

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(index_handler))
        .route("/health", get(health_check))
}

async fn index_handler() -> Html<&'static str> {
    tracing::info!("🚀 RustPulse started on http://localhost:3000");
    Html("<h1>Hello, RustPulse!</h1>")
}
