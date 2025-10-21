use crate::core::port::telemetry_query_case::TelemetryQueryCase;
use axum::extract::Query;
use axum::http::StatusCode;
use axum::routing::get;
use axum::{Router, extract::State, response::IntoResponse};
use std::collections::HashMap;
use std::sync::Arc;
// use tracing::{error, info, instrument};

#[derive(serde::Deserialize)]
pub struct TelemetryDto {
    pub node_id: String,
    pub cpu: f32,
    pub memory: f32,
    pub timestamp: i64,
}

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

    tracing::info!(node_id = ?node_id, "Fetching telemetry metrics for the given node");

    match service.fetch_all(node_id).await {
        Ok(metrics) => {
            tracing::info!(
                metrics_count = metrics.len(),
                "Fetched metrics successfully."
            );

            //only test purposes cause high payload for pretty Json
            let json = serde_json::to_string_pretty(&metrics).unwrap();
            (StatusCode::OK, json).into_response()
        }
        Err(_) => {
            tracing::error!("Failed to fetch metrics");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
