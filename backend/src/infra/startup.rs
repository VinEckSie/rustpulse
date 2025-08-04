use crate::adapters::jsonl_telemetry_repo::JsonlTelemetryRepo;
use crate::app::telemetry_service::TelemetryService;
use crate::core::port::telemetry_mock_repo::MockDataGenerator;
use crate::handlers::http;
use axum::Router;
use std::path::PathBuf;
use std::sync::Arc;
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::Level;

pub async fn start_server(port: u16) -> Result<(), Box<dyn std::error::Error>> {
    // Create mock data for testing
    let temp_file_path = "metrics_data.jsonl";
    MockDataGenerator::generate_mock_data(temp_file_path, 20)?;

    // Setup the JSONL repository and service
    let repo = Arc::new(JsonlTelemetryRepo::new(PathBuf::from(temp_file_path)));
    let service = Arc::new(TelemetryService::new(repo.clone()));

    //Build Router
    let app = Router::new()
        .merge(http::root_handler::routes())
        .merge(http::health_handler::routes())
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
