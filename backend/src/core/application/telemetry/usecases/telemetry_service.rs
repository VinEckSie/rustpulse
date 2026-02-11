use crate::core::application::telemetry::ports::input::telemetry_ingest_usecase::TelemetryIngestCase;
use crate::core::application::telemetry::ports::input::telemetry_query_usecase::TelemetryQueryCase;
use crate::core::application::telemetry::ports::output::telemetry_repository::TelemetryRepository;
use crate::core::domains::telemetry::Telemetry;
use std::sync::Arc;
use tracing::Instrument as _;
use tracing::field;

fn classify_anyhow_error(err: &anyhow::Error) -> (&'static str, &'static str) {
    if err.is::<uuid::Error>() {
        ("invalid_uuid", "uuid::Error")
    } else if err.is::<std::io::Error>() {
        ("io", "std::io::Error")
    } else if err.is::<serde_json::Error>() {
        ("serde_json", "serde_json::Error")
    } else {
        ("unknown", "unknown")
    }
}

fn truncate_for_span(value: String, max_len: usize) -> String {
    if value.len() <= max_len {
        return value;
    }

    let mut out = value;
    out.truncate(max_len);
    out.push('…');
    out
}

//use-case engine
//output port dependencies holding: what rustpulse needs from the outside
//to complete this use case, I need someone that can save/query telemetry.
//The service must call that port while running the use case, so it stores it as a field (a dependency).

//dyn TelemetryRepository means: I don’t know which repo implementation at compile time (JSONL, Postgres, mock)
//just anything that matches the port trait.
//output ports are “plugs” to infrastructure; the use case needs those plugs available when it runs.

//Output port (TelemetryRepository) is consumed by the use case → it must be a field the use case can call.
pub struct TelemetryService {
    repo: Arc<dyn TelemetryRepository + Send + Sync>,
}

//dependency injection
//the core defines the port, the edge provides the adapter, infra connect them
//example in infra/startup.rs:
// let repo = Arc::new(JsonlTelemetryRepo::new(temp_file_path));
// let service = Arc::new(TelemetryService::new(repo.clone()));
impl TelemetryService {
    pub fn new(repo: Arc<dyn TelemetryRepository + Send + Sync>) -> Self {
        // Accept Arc instead of plain type
        Self { repo }
    }
}

//input port APIs exposition: what rustpulse can do
//input ports: here are the operations the outside world is allowed to request
//The service is the implementation of those operations, so it “claims” the interface by implementing the trait.
//You don’t store input ports inside the service, because the service is the input port implementation.
//Storing them would mean “the service delegates to another thing to be the use case”, which is a different design.

//Input ports (TelemetryQueryCase, TelemetryIngestCase) are provided by the use case → they show up as trait impls
//on the service.
#[async_trait::async_trait]
impl TelemetryQueryCase for TelemetryService {
    async fn fetch_all(&self, node_id: Option<String>) -> anyhow::Result<Vec<Telemetry>> {
        let span = tracing::info_span!(
            "usecase.telemetry.fetch_all",
            outcome = field::Empty,
            "error.type" = field::Empty,
            "error.code" = field::Empty,
            "otel.status_code" = field::Empty,
            "exception.message" = field::Empty,
        );

        let result = self.repo.query_all(node_id).instrument(span.clone()).await;

        match &result {
            Ok(_) => {
                span.record("outcome", "ok");
            }
            Err(err) => {
                let (error_code, error_type) = classify_anyhow_error(err);
                let message = truncate_for_span(err.to_string(), 200);
                span.record("outcome", "error");
                span.record("otel.status_code", "ERROR");
                span.record("error.type", error_type);
                span.record("error.code", error_code);
                span.record("exception.message", message.as_str());
            }
        }

        result
    }
}
#[async_trait::async_trait]
impl TelemetryIngestCase for TelemetryService {
    async fn ingest(&self, telemetry: Telemetry) -> anyhow::Result<()> {
        let span = tracing::info_span!(
            "usecase.telemetry.ingest",
            outcome = field::Empty,
            "error.type" = field::Empty,
            "error.code" = field::Empty,
            "otel.status_code" = field::Empty,
            "exception.message" = field::Empty,
        );

        let result = self.repo.save(telemetry).instrument(span.clone()).await;

        match &result {
            Ok(_) => {
                span.record("outcome", "ok");
            }
            Err(err) => {
                let (error_code, error_type) = classify_anyhow_error(err);
                let message = truncate_for_span(err.to_string(), 200);
                span.record("outcome", "error");
                span.record("otel.status_code", "ERROR");
                span.record("error.type", error_type);
                span.record("error.code", error_code);
                span.record("exception.message", message.as_str());
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::anyhow;
    use chrono::Utc;
    use std::collections::{BTreeMap, HashMap};
    use std::fmt;
    use std::sync::{Arc, Mutex};
    use tracing::Subscriber;
    use tracing::field::{Field, Visit};
    use tracing::span::{Attributes, Id, Record};
    use tracing_subscriber::registry::LookupSpan;
    use tracing_subscriber::{Layer, layer::Context, prelude::*};
    use uuid::Uuid;

    #[derive(Clone, Default)]
    struct Captured(Arc<Mutex<HashMap<u64, CapturedSpan>>>);

    #[derive(Debug, Clone, Default)]
    struct CapturedSpan {
        name: String,
        parent_id: Option<u64>,
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
        fn on_new_span(&self, attrs: &Attributes<'_>, id: &Id, ctx: Context<'_, S>) {
            let mut fields = BTreeMap::new();
            attrs.record(&mut FieldVisitor {
                fields: &mut fields,
            });

            let parent_id = ctx
                .span(id)
                .and_then(|span| span.parent().map(|parent| parent.id().into_u64()));

            let span = CapturedSpan {
                name: attrs.metadata().name().to_string(),
                parent_id,
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

    fn find_span_by_name(captured: &Captured, name: &str) -> Option<(u64, CapturedSpan)> {
        let locked = captured.0.lock().ok()?;
        locked
            .iter()
            .find(|(_, s)| s.name == name)
            .map(|(id, span)| (*id, span.clone()))
    }

    struct OkRepo;
    struct ErrQueryRepo;

    #[async_trait::async_trait]
    impl TelemetryRepository for OkRepo {
        async fn save(&self, _telemetry: Telemetry) -> anyhow::Result<()> {
            Ok(())
        }

        async fn query_all(&self, _node_id: Option<String>) -> anyhow::Result<Vec<Telemetry>> {
            Ok(vec![])
        }
    }

    #[async_trait::async_trait]
    impl TelemetryRepository for ErrQueryRepo {
        async fn save(&self, _telemetry: Telemetry) -> anyhow::Result<()> {
            Ok(())
        }

        async fn query_all(&self, _node_id: Option<String>) -> anyhow::Result<Vec<Telemetry>> {
            Err(anyhow!("boom"))
        }
    }

    #[tokio::test(flavor = "current_thread")]
    async fn test_usecase_fetch_all_span_is_child_and_records_outcome_ok() {
        use tracing::Instrument as _;

        let captured = Captured::default();
        let subscriber = tracing_subscriber::registry().with(CaptureLayer::new(captured.clone()));
        let _guard = tracing::subscriber::set_default(subscriber);

        let repo = Arc::new(OkRepo);
        let service = TelemetryService::new(repo);

        let parent = tracing::info_span!("http.request");
        let result = service.fetch_all(None).instrument(parent.clone()).await;
        assert!(result.is_ok());

        let (parent_id, _) =
            find_span_by_name(&captured, "http.request").expect("expected http.request span");
        let (_, usecase_span) = find_span_by_name(&captured, "usecase.telemetry.fetch_all")
            .expect("expected usecase.telemetry.fetch_all span");

        assert_eq!(usecase_span.parent_id, Some(parent_id));
        assert_eq!(
            usecase_span.fields.get("outcome").map(String::as_str),
            Some("ok")
        );
    }

    #[tokio::test(flavor = "current_thread")]
    async fn test_usecase_fetch_all_span_records_error_outcome_and_status() {
        use tracing::Instrument as _;

        let captured = Captured::default();
        let subscriber = tracing_subscriber::registry().with(CaptureLayer::new(captured.clone()));
        let _guard = tracing::subscriber::set_default(subscriber);

        let repo = Arc::new(ErrQueryRepo);
        let service = TelemetryService::new(repo);

        let parent = tracing::info_span!("http.request");
        let result = service.fetch_all(None).instrument(parent.clone()).await;
        assert!(result.is_err());

        let (parent_id, _) =
            find_span_by_name(&captured, "http.request").expect("expected http.request span");
        let (_, usecase_span) = find_span_by_name(&captured, "usecase.telemetry.fetch_all")
            .expect("expected usecase.telemetry.fetch_all span");

        assert_eq!(usecase_span.parent_id, Some(parent_id));
        assert_eq!(
            usecase_span.fields.get("outcome").map(String::as_str),
            Some("error")
        );
        assert_eq!(
            usecase_span
                .fields
                .get("otel.status_code")
                .map(String::as_str),
            Some("ERROR")
        );

        let error_type = usecase_span
            .fields
            .get("error.type")
            .cloned()
            .unwrap_or_default();
        assert!(!error_type.is_empty());
    }

    #[tokio::test(flavor = "current_thread")]
    async fn test_usecase_ingest_span_is_child_and_records_outcome_ok() {
        use tracing::Instrument as _;

        let captured = Captured::default();
        let subscriber = tracing_subscriber::registry().with(CaptureLayer::new(captured.clone()));
        let _guard = tracing::subscriber::set_default(subscriber);

        let repo = Arc::new(OkRepo);
        let service = TelemetryService::new(repo);

        let telemetry = Telemetry {
            source_id: Uuid::nil(),
            server_id: Uuid::nil(),
            timestamp: Utc::now(),
            cpu: None,
            memory: None,
            temperature: None,
            extras: serde_json::json!({}),
        };

        let parent = tracing::info_span!("http.request");
        let result = service.ingest(telemetry).instrument(parent.clone()).await;
        assert!(result.is_ok());

        let (parent_id, _) =
            find_span_by_name(&captured, "http.request").expect("expected http.request span");
        let (_, usecase_span) = find_span_by_name(&captured, "usecase.telemetry.ingest")
            .expect("expected usecase.telemetry.ingest span");

        assert_eq!(usecase_span.parent_id, Some(parent_id));
        assert_eq!(
            usecase_span.fields.get("outcome").map(String::as_str),
            Some("ok")
        );
    }
}
