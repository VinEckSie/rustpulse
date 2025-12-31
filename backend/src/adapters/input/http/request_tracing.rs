use axum::{extract::{MatchedPath, Request}, middleware::Next, response::Response};
use opentelemetry::trace::TraceContextExt as _;
use tracing::field;
use tracing::Instrument as _;
use tracing_opentelemetry::OpenTelemetrySpanExt as _;

pub async fn trace_middleware(req: Request, next: Next) -> Response {
    let method = req.method().to_string();
    let route = req
        .extensions()
        .get::<MatchedPath>()
        .map(|matched| matched.as_str().to_string())
        .unwrap_or_else(|| req.uri().path().to_string());
    let otel_name = format!("{method} {route}");

    let span = tracing::info_span!(
        "http.request",
        "otel.name" = %otel_name,
        "http.method" = %method,
        "http.route" = %route,
        "http.status_code" = field::Empty,
        trace_id = field::Empty,
    );

    {
        let ctx = span.context();
        let otel_span = ctx.span();
        let span_ctx = otel_span.span_context();
        if span_ctx.is_valid() {
            let trace_id = span_ctx.trace_id().to_string();
            span.record("trace_id", trace_id.as_str());
        }
    }

    let response = next.run(req).instrument(span.clone()).await;
    span.record("http.status_code", response.status().as_u16());
    response
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{body::Body, http::Request as HttpRequest, middleware, routing::get, Router};
    use std::collections::{BTreeMap, HashMap};
    use std::fmt;
    use std::sync::{Arc, Mutex};
    use tower::ServiceExt;
    use tracing::Subscriber;
    use tracing::field::{Field, Visit};
    use tracing::span::{Attributes, Id, Record};
    use tracing_subscriber::{Layer, layer::Context, prelude::*};
    use tracing_subscriber::registry::LookupSpan;

    #[derive(Clone, Default)]
    struct Captured(Arc<Mutex<HashMap<u64, CapturedSpan>>>);

    #[derive(Debug, Clone, Default)]
    struct CapturedSpan {
        name: String,
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
            self.fields.insert(field.name().to_string(), value.to_string());
        }

        fn record_u64(&mut self, field: &Field, value: u64) {
            self.fields.insert(field.name().to_string(), value.to_string());
        }

        fn record_i64(&mut self, field: &Field, value: i64) {
            self.fields.insert(field.name().to_string(), value.to_string());
        }

        fn record_bool(&mut self, field: &Field, value: bool) {
            self.fields.insert(field.name().to_string(), value.to_string());
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
            attrs.record(&mut FieldVisitor { fields: &mut fields });

            let span = CapturedSpan {
                name: attrs.metadata().name().to_string(),
                fields,
            };

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

    fn build_app() -> Router {
        Router::new()
            .route("/health", get(|| async { "OK" }))
            .route_layer(middleware::from_fn(trace_middleware))
    }

    fn find_span(captured: &Captured, name: &str) -> Option<CapturedSpan> {
        let locked = captured.0.lock().ok()?;
        locked.values().find(|s| s.name == name).cloned()
    }

    #[tokio::test]
    async fn test_http_request_span_has_method_and_route() {
        let captured = Captured::default();
        let subscriber = tracing_subscriber::registry().with(CaptureLayer::new(captured.clone()));
        let _guard = tracing::subscriber::set_default(subscriber);

        let app = build_app();
        let req = HttpRequest::builder()
            .method("GET")
            .uri("/health")
            .body(Body::empty())
            .unwrap();

        let _ = app.oneshot(req).await.unwrap();

        let span = find_span(&captured, "http.request").expect("expected http.request span");
        assert_eq!(
            span.fields.get("http.method").map(String::as_str),
            Some("GET")
        );
        assert_eq!(
            span.fields.get("http.route").map(String::as_str),
            Some("/health")
        );
        assert_eq!(
            span.fields.get("otel.name").map(String::as_str),
            Some("GET /health")
        );
    }

    #[tokio::test]
    async fn test_http_request_span_records_status_code() {
        let captured = Captured::default();
        let subscriber = tracing_subscriber::registry().with(CaptureLayer::new(captured.clone()));
        let _guard = tracing::subscriber::set_default(subscriber);

        let app = build_app();
        let req = HttpRequest::builder()
            .method("GET")
            .uri("/health")
            .body(Body::empty())
            .unwrap();

        let _ = app.oneshot(req).await.unwrap();

        let span = find_span(&captured, "http.request").expect("expected http.request span");
        assert_eq!(
            span.fields.get("http.status_code").map(String::as_str),
            Some("200")
        );
    }

    #[tokio::test]
    async fn test_http_request_span_records_trace_id_when_otel_layer_enabled() {
        use opentelemetry::trace::TracerProvider as _;
        use opentelemetry_sdk::trace::SdkTracerProvider;

        let provider = SdkTracerProvider::builder().build();
        let tracer = provider.tracer("test");
        let otel_layer = tracing_opentelemetry::layer().with_tracer(tracer);

        let captured = Captured::default();
        let subscriber = tracing_subscriber::registry()
            .with(otel_layer)
            .with(CaptureLayer::new(captured.clone()));
        let _guard = tracing::subscriber::set_default(subscriber);

        let app = build_app();
        let req = HttpRequest::builder()
            .method("GET")
            .uri("/health")
            .body(Body::empty())
            .unwrap();

        let _ = app.oneshot(req).await.unwrap();

        let span = find_span(&captured, "http.request").expect("expected http.request span");
        let trace_id = span.fields.get("trace_id").cloned().unwrap_or_default();
        assert!(!trace_id.is_empty());
        assert_ne!(trace_id, "00000000000000000000000000000000");
    }
}
