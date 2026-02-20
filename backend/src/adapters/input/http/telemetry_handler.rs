use crate::core::application::telemetry::{TelemetryIngestCase, TelemetryQueryCase};
use crate::core::domains::telemetry::Telemetry;
use axum::Json;
use axum::extract::{Query, Request, State};
use axum::http::{HeaderMap, StatusCode};
use axum::routing::{get, post};
use axum::{Router, middleware, response::IntoResponse};
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{field, instrument};

use super::request_tracing;

#[instrument(level = "info", skip(service))]
pub fn routes(service: Arc<dyn TelemetryQueryCase>) -> Router {
    Router::new()
        .route("/metrics", get(fetch_telemetry_handler))
        .with_state(service)
        .route_layer(middleware::from_fn(request_tracing::trace_middleware))
}

const MAX_TELEMETRY_BODY_BYTES: usize = 1024 * 1024;

#[instrument(level = "info", skip(service))]
pub fn ingest_routes(service: Arc<dyn TelemetryIngestCase + Send + Sync>) -> Router {
    Router::new()
        .route("/telemetry", post(ingest_telemetry_handler))
        .with_state(service)
        .route_layer(middleware::from_fn(request_tracing::trace_middleware))
}

#[derive(Debug)]
pub enum TelemetryIngestHttpError {
    InvalidCrc,
    CrcMismatch,
    InvalidJson,
    IngestFailed,
}

#[derive(serde::Serialize)]
struct ErrorResponse {
    code: &'static str,
    message: String,
}

impl IntoResponse for TelemetryIngestHttpError {
    fn into_response(self) -> axum::response::Response {
        let (status, code, message) = match self {
            Self::InvalidCrc => (
                StatusCode::BAD_REQUEST,
                "invalid_crc",
                "X-CRC32 must be 8 hex digits (CRC-32/IEEE)".to_string(),
            ),
            Self::CrcMismatch => (
                StatusCode::BAD_REQUEST,
                "crc_mismatch",
                "CRC does not match request body".to_string(),
            ),
            Self::InvalidJson => (
                StatusCode::BAD_REQUEST,
                "invalid_json",
                "Request body must be valid telemetry JSON".to_string(),
            ),
            Self::IngestFailed => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal",
                "Failed to ingest telemetry".to_string(),
            ),
        };

        (status, Json(ErrorResponse { code, message })).into_response()
    }
}

#[instrument(
    name = "ingest telemetry",
    skip(service, req),
    fields(crc_check = field::Empty)
)]
pub async fn ingest_telemetry_handler(
    State(service): State<Arc<dyn TelemetryIngestCase + Send + Sync>>,
    req: Request,
) -> Result<StatusCode, TelemetryIngestHttpError> {
    let provided_crc = parse_crc32_header(req.headers())?;

    let body = axum::body::to_bytes(req.into_body(), MAX_TELEMETRY_BODY_BYTES)
        .await
        .map_err(|_| TelemetryIngestHttpError::InvalidJson)?;

    if let Some(expected) = provided_crc {
        let actual = crc32_ieee(&body);
        if actual != expected {
            tracing::Span::current().record("crc_check", "fail");
            return Err(TelemetryIngestHttpError::CrcMismatch);
        }
    }

    let telemetry: Telemetry =
        serde_json::from_slice(&body).map_err(|_| TelemetryIngestHttpError::InvalidJson)?;

    service
        .ingest(telemetry)
        .await
        .map_err(|_| TelemetryIngestHttpError::IngestFailed)?;

    Ok(StatusCode::ACCEPTED)
}

fn parse_crc32_header(headers: &HeaderMap) -> Result<Option<u32>, TelemetryIngestHttpError> {
    let Some(value) = headers.get("x-crc32") else {
        return Ok(None);
    };

    let s = value
        .to_str()
        .map_err(|_| TelemetryIngestHttpError::InvalidCrc)?
        .trim();

    let s = s.strip_prefix("0x").unwrap_or(s);
    if s.len() != 8 {
        return Err(TelemetryIngestHttpError::InvalidCrc);
    }

    u32::from_str_radix(s, 16)
        .map(Some)
        .map_err(|_| TelemetryIngestHttpError::InvalidCrc)
}

fn crc32_ieee(bytes: &[u8]) -> u32 {
    let mut crc: u32 = 0xFFFF_FFFF;
    for &b in bytes {
        crc ^= b as u32;
        for _ in 0..8 {
            let mask = 0u32.wrapping_sub(crc & 1);
            crc = (crc >> 1) ^ (0xEDB8_8320 & mask);
        }
    }
    !crc
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

            //only test purposes for now. Pretty Json causes high payload
            match serde_json::to_string_pretty(&metrics) {
                Ok(json) => (StatusCode::OK, json).into_response(),
                Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            }
        }
        Err(_) => {
            tracing::error!("Failed to fetch metrics");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

#[cfg(test)]
mod ingest_crc_tests {
    use crate::core::application::telemetry::TelemetryIngestCase;
    use crate::core::domains::telemetry::Telemetry;

    use async_trait::async_trait;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use serde_json::Value;
    use std::collections::{BTreeMap, HashMap};
    use std::fmt;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::{Arc, Mutex};
    use tower::ServiceExt;
    use tracing::Subscriber;
    use tracing::field::{Field, Visit};
    use tracing::span::{Attributes, Id, Record};
    use tracing_subscriber::Layer;
    use tracing_subscriber::layer::Context;
    use tracing_subscriber::prelude::*;
    use tracing_subscriber::registry::LookupSpan;

    struct FakeIngest {
        calls: Arc<AtomicUsize>,
    }

    #[async_trait]
    impl TelemetryIngestCase for FakeIngest {
        async fn ingest(&self, _telemetry: Telemetry) -> anyhow::Result<()> {
            self.calls.fetch_add(1, Ordering::SeqCst);
            Ok(())
        }
    }

    fn crc32_ieee(bytes: &[u8]) -> u32 {
        let mut crc: u32 = 0xFFFF_FFFF;
        for &b in bytes {
            crc ^= b as u32;
            for _ in 0..8 {
                let mask = 0u32.wrapping_sub(crc & 1);
                crc = (crc >> 1) ^ (0xEDB8_8320 & mask);
            }
        }
        !crc
    }

    fn telemetry_body() -> &'static str {
        r#"{"source_id":"00000000-0000-0000-0000-000000000001","server_id":"00000000-0000-0000-0000-000000000002","timestamp":"2026-02-18T00:00:00Z","cpu":1.0,"memory":null,"temperature":null,"extras":{}}"#
    }

    #[derive(Clone, Default)]
    struct Captured(Arc<Mutex<HashMap<u64, CapturedSpan>>>);

    #[derive(Debug, Clone, Default)]
    struct CapturedSpan {
        fields: BTreeMap<String, String>,
    }

    #[derive(Clone, Default)]
    struct CaptureLayer {
        captured: Captured,
    }

    impl CaptureLayer {
        fn new(captured: Captured) -> Self {
            Self { captured }
        }
    }

    struct FieldVisitor<'a> {
        fields: &'a mut BTreeMap<String, String>,
    }

    impl<'a> Visit for FieldVisitor<'a> {
        fn record_str(&mut self, field: &Field, value: &str) {
            self.fields
                .insert(field.name().to_string(), value.to_string());
        }

        fn record_u64(&mut self, field: &Field, value: u64) {
            self.fields
                .insert(field.name().to_string(), value.to_string());
        }

        fn record_i64(&mut self, field: &Field, value: i64) {
            self.fields
                .insert(field.name().to_string(), value.to_string());
        }

        fn record_bool(&mut self, field: &Field, value: bool) {
            self.fields
                .insert(field.name().to_string(), value.to_string());
        }

        fn record_debug(&mut self, field: &Field, value: &dyn fmt::Debug) {
            self.fields
                .insert(field.name().to_string(), format!("{value:?}"));
        }
    }

    impl<S> Layer<S> for CaptureLayer
    where
        S: Subscriber + for<'a> LookupSpan<'a>,
    {
        fn on_new_span(&self, attrs: &Attributes<'_>, id: &Id, _ctx: Context<'_, S>) {
            let mut fields = BTreeMap::new();
            attrs.record(&mut FieldVisitor {
                fields: &mut fields,
            });

            let span = CapturedSpan { fields };

            let mut locked = self.captured.0.lock().expect("capture lock poisoned");
            locked.insert(id.into_u64(), span);
        }

        fn on_record(&self, id: &Id, values: &Record<'_>, _ctx: Context<'_, S>) {
            let mut locked = self.captured.0.lock().expect("capture lock poisoned");
            if let Some(span) = locked.get_mut(&id.into_u64()) {
                values.record(&mut FieldVisitor {
                    fields: &mut span.fields,
                });
            }
        }
    }

    fn find_span_with_field(
        captured: &Captured,
        field_name: &str,
        field_value: &str,
    ) -> Option<CapturedSpan> {
        let locked = captured.0.lock().ok()?;
        locked
            .values()
            .find(|s| {
                s.fields
                    .get(field_name)
                    .map(String::as_str)
                    .is_some_and(|v| v == field_value)
            })
            .cloned()
    }

    #[tokio::test]
    async fn test_ingest_telemetry_without_crc_header_returns_202_and_calls_ingest_once() {
        let calls = Arc::new(AtomicUsize::new(0));
        let service: Arc<dyn TelemetryIngestCase + Send + Sync> = Arc::new(FakeIngest {
            calls: calls.clone(),
        });

        let app = super::ingest_routes(service);

        let req = Request::builder()
            .method("POST")
            .uri("/telemetry")
            .header("content-type", "application/json")
            .body(Body::from(telemetry_body()))
            .unwrap();

        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::ACCEPTED);
        assert_eq!(calls.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn test_ingest_telemetry_with_valid_crc_header_returns_202_and_calls_ingest_once() {
        let calls = Arc::new(AtomicUsize::new(0));
        let service: Arc<dyn TelemetryIngestCase + Send + Sync> = Arc::new(FakeIngest {
            calls: calls.clone(),
        });

        let app = super::ingest_routes(service);

        let body = telemetry_body().as_bytes();
        let crc = crc32_ieee(body);
        let crc_hex = format!("{crc:08x}");

        let req = Request::builder()
            .method("POST")
            .uri("/telemetry")
            .header("content-type", "application/json")
            .header("x-crc32", crc_hex)
            .body(Body::from(telemetry_body()))
            .unwrap();

        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::ACCEPTED);
        assert_eq!(calls.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn test_ingest_telemetry_with_mismatched_crc_returns_400_does_not_call_ingest_and_records_crc_check_fail_span_field()
     {
        let captured = Captured::default();
        let subscriber = tracing_subscriber::registry().with(CaptureLayer::new(captured.clone()));
        let _guard = tracing::subscriber::set_default(subscriber);

        let calls = Arc::new(AtomicUsize::new(0));
        let service: Arc<dyn TelemetryIngestCase + Send + Sync> = Arc::new(FakeIngest {
            calls: calls.clone(),
        });

        let app = super::ingest_routes(service);

        let req = Request::builder()
            .method("POST")
            .uri("/telemetry")
            .header("content-type", "application/json")
            .header("x-crc32", "00000000")
            .body(Body::from(telemetry_body()))
            .unwrap();

        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
        assert_eq!(calls.load(Ordering::SeqCst), 0);

        let bytes = axum::body::to_bytes(resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let v: Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(v.get("code").and_then(|x| x.as_str()), Some("crc_mismatch"));

        let span = find_span_with_field(&captured, "crc_check", "fail")
            .expect("expected a span with crc_check=fail");
        assert_eq!(
            span.fields.get("crc_check").map(String::as_str),
            Some("fail")
        );
    }
}
