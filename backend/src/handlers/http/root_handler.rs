use axum::{response::Html, routing::get, Router};

pub fn routes() -> Router {
    Router::new().route("/", get(index_handler))
}

async fn index_handler() -> Html<&'static str> {
    tracing::info!("serving RustPulse home page at http://localhost:3000/");
    Html("<h1>Hello, RustPulse!</h1>")
}
