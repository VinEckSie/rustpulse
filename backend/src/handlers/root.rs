

use axum::{Router, response::Html, routing::get};

pub fn router() -> Router {
    Router::new()
        .route("/", get(index_handler))
}

async fn index_handler() -> Html<&'static str> {
    tracing::info!("🚀 RustPulse started on http://localhost:3000");
    Html("<h1>Hello, RustPulse!</h1>")
}