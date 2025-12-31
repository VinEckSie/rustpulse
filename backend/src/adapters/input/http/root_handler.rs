use axum::{Router, middleware, response::Html, routing::get};

use super::request_tracing;

pub fn routes() -> Router {
    Router::new()
        .route("/", get(index_handler))
        .route_layer(middleware::from_fn(request_tracing::trace_middleware))
}

async fn index_handler() -> Html<&'static str> {
    tracing::info!("serving RustPulse home page at http://localhost:3000/");
    Html("<h1>Hello, RustPulse!</h1>")
}
