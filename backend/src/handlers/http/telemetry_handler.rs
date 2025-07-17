use crate::core::port::telemetry_query_case::TelemetryQueryCase;
// use reqwest::get;
use axum::extract::Query;
use axum::http::StatusCode;
use axum::routing::get;
use axum::{extract::State, response::IntoResponse, Json, Router};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(serde::Deserialize)]
pub struct TelemetryDto {
    pub node_id: String,
    pub cpu: f32,
    pub memory: f32,
    pub timestamp: i64,
}

// pub fn routes() -> Router {
//     Router::new().route("/metrics", get(fetch_telemetry_handler))
// }

pub fn routes(service: Arc<dyn TelemetryQueryCase>) -> Router {
    Router::new()
        .route("/metrics", get(fetch_telemetry_handler))
        .with_state(service)
}

pub async fn fetch_telemetry_handler(
    State(service): State<Arc<dyn TelemetryQueryCase>>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let node_id = params.get("node_id").cloned();

    match service.fetch_all(node_id).await {
        Ok(metrics) => Json(metrics).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

// pub async fn ingest_telemetry_handler(
//     State(service): State<Arc<dyn TelemetryIngestCase>>,
//     Json(payload): Json<TelemetryDto>,
// ) -> impl IntoResponse {
//     let telemetry = NodeTelemetry {
//         node_id: payload.node_id,
//         cpu: payload.cpu,
//         memory: payload.memory,
//         timestamp: payload.timestamp,
//     };
//
//     match service.ingest(telemetry).await {
//         Ok(_) => StatusCode::CREATED,
//         Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
//     }
// }
