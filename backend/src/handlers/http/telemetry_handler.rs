use crate::core::port::telemetry_query_case::TelemetryQueryCase;
use axum::extract::Query;
use axum::http::StatusCode;
use axum::routing::get;
use axum::{Router, extract::State, response::IntoResponse};
use std::collections::HashMap;
use std::sync::Arc;
use tracing::instrument;

#[instrument(level = "info", skip(service))]
pub fn routes(service: Arc<dyn TelemetryQueryCase>) -> Router {
    Router::new()
        .route("/metrics", get(fetch_telemetry_handler))
        .with_state(service)
}

#[instrument(name = "fetch telemetry", skip(service), fields(
    source_id = tracing::field::Empty
))]
pub async fn fetch_telemetry_handler(
    State(service): State<Arc<dyn TelemetryQueryCase>>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let span = tracing::Span::current();

    if let Some(source_id) = params.get("source_id") {
        span.record("source_id", source_id.as_str());
    }

    let source_id = params.get("source_id").cloned();

    match service.fetch_all(source_id).await {
        Ok(metrics) => {
            tracing::info!(
                metrics_count = metrics.len(),
                "fetched metrics successfully."
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
