
use axum::Router;
use tokio::net::TcpListener;
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::Level;

use crate::handlers::{self, health, metrics, root};

pub async fn start_server(port: u16) -> Result<(), Box<dyn std::error::Error>> {
    //Build Router
    let app = Router::new()
        .merge(root::router())
        .merge(health::router())
        //.merge(metrics::router())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO))
                .on_request(DefaultOnRequest::new().level(Level::INFO)),
        );

    let addr = format!("127.0.0.1:{port}");
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());

    //Start Server
    //Listener handles network → Router handles logic
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
