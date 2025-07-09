use crate::adapters::jsonl_telemetry_repo::JsonlTelemetryRepo;
use crate::app::telemetry_service::TelemetryService;
use crate::handlers::http;
use axum::Router;
use std::path::PathBuf;
use std::sync::Arc;
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::Level;

pub async fn start_server(port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let telemetry_path = PathBuf::from("data/telemetry.jsonl"); // adapte ce chemin
    let repo = Arc::new(JsonlTelemetryRepo::new(telemetry_path)); // implémente TelemetryRepository
    let service = Arc::new(TelemetryService::new(repo.clone())); // implémente TelemetryQueryCase

    //Build Router
    let app = Router::new()
        .merge(http::root_handler::routes())
        .merge(http::health__handler::routes())
        .merge(http::telemetry_handler::routes(service)) // now injecting state
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO))
                .on_request(DefaultOnRequest::new().level(Level::INFO)),
        );

    let addr = format!("127.0.0.1:{port}");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("listening on {}", listener.local_addr().unwrap());

    //Start Server
    //Listener handles network → Router handles logic
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
