//! Tracing initialization helpers.

use opentelemetry::trace::TracerProvider as _;
use opentelemetry::{KeyValue, global};
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{Resource, trace as sdktrace};
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::Registry;

/// Runtime configuration for tracing export.
#[derive(Debug, Clone)]
pub struct TracingConfig {
    /// Service name reported to the tracing backend.
    pub service_name: String,
    /// Deployment environment tag (e.g. `local`, `staging`, `prod`).
    pub environment: String,
    /// OTLP endpoint for Jaeger/Collector, e.g. `http://localhost:4317`.
    ///
    /// If `None` or empty, tracing is considered disabled.
    pub otlp_endpoint: Option<String>,
}

impl TracingConfig {
    /// - `OTEL_SERVICE_NAME` (falls back to `rustpulse-backend`)
    /// - `RUSTPULSE_ENV` (falls back to `local`)
    /// - `OTEL_EXPORTER_OTLP_ENDPOINT` (required to enable exporting)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustpulse::infra::tracing::TracingConfig;
    ///
    /// let _cfg = TracingConfig::from_env();
    /// ```
    pub fn from_env() -> Self {
        let service_name =
            std::env::var("OTEL_SERVICE_NAME").unwrap_or_else(|_| "rustpulse-backend".to_string());
        let environment = std::env::var("RUSTPULSE_ENV").unwrap_or_else(|_| "local".to_string());
        let otlp_endpoint = std::env::var("OTEL_EXPORTER_OTLP_ENDPOINT").ok();

        Self {
            service_name,
            environment,
            otlp_endpoint,
        }
    }
}

/// High-level status after initialization.
#[derive(Debug)]
pub enum TracingStatus {
    /// Exporting is configured and an OpenTelemetry layer was built.
    Active,
    /// Exporting is disabled, along with the reason.
    Disabled {
        /// Reason why export is disabled.
        reason: DisabledReason,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Why tracing export is disabled.
pub enum DisabledReason {
    /// Tracing is disabled because configuration is missing or invalid.
    Misconfigured,
    /// Tracing is disabled because the exporter could not be initialized.
    BackendUnavailable,
}

/// Errors that can occur during tracing initialization.
#[derive(Debug, thiserror::Error)]
pub enum TracingInitError {
    /// The configuration is syntactically valid but rejected by validation.
    #[error("invalid tracing configuration: {0}")]
    InvalidConfig(String),
    /// The exporter could not be initialized (e.g., OTLP client error).
    #[error("failed to connect to Jaeger backend: {0}")]
    BackendUnavailable(String),
}

/// Type alias for the OpenTelemetry tracing layer used by this crate.
pub type OtelLayer = OpenTelemetryLayer<Registry, sdktrace::SdkTracer>;

#[derive(Debug, Clone)]
/// Metadata about the configured exporter.
pub struct ExporterMeta {
    /// Service name reported to the backend.
    pub service_name: String,
    /// OTLP endpoint used by the exporter.
    pub otlp_endpoint: String,
}

/// Result of tracing initialization.
pub struct TracingInit {
    /// Overall status.
    pub status: TracingStatus,
    /// The OpenTelemetry layer to attach to a subscriber when active.
    pub otel_layer: Option<OtelLayer>,
    /// Exporter metadata when active.
    pub exporter_meta: Option<ExporterMeta>,
}

/// Builds (but does not install) the OpenTelemetry layer.
/// - Returns a status describing whether tracing is effectively active or disabled.
/// - Never panics.
/// - Does not attempt any network I/O when disabled by configuration.
///
/// # Examples
///
/// ```rust
/// use rustpulse::infra::tracing::{init_tracing, TracingConfig, TracingStatus};
///
/// // No endpoint => disabled without attempting any exporter setup.
/// let cfg = TracingConfig {
///     service_name: "svc".to_string(),
///     environment: "test".to_string(),
///     otlp_endpoint: None,
/// };
///
/// let init = init_tracing(&cfg).unwrap();
/// assert!(matches!(init.status, TracingStatus::Disabled { .. }));
/// ```
pub fn init_tracing(config: &TracingConfig) -> Result<TracingInit, TracingInitError> {
    if config.service_name.trim().is_empty() {
        return Ok(TracingInit {
            status: TracingStatus::Disabled {
                reason: DisabledReason::Misconfigured,
            },
            otel_layer: None,
            exporter_meta: None,
        });
    }

    let endpoint = match config.otlp_endpoint.as_ref().map(|s| s.trim()) {
        None | Some("") => {
            return Ok(TracingInit {
                status: TracingStatus::Disabled {
                    reason: DisabledReason::Misconfigured,
                },
                otel_layer: None,
                exporter_meta: None,
            });
        }
        Some(endpoint) => endpoint.to_string(),
    };

    let resource = Resource::builder_empty()
        .with_service_name(config.service_name.clone())
        .with_attribute(KeyValue::new(
            "deployment.environment",
            config.environment.clone(),
        ))
        .build();

    //builds an OTLP SpanExporter with with_tonic() (that’s the gRPC path),
    let exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_tonic()
        .with_endpoint(endpoint.clone());

    let exporter = exporter
        .build()
        .map_err(|e| TracingInitError::BackendUnavailable(e.to_string()))?;

    let provider = sdktrace::SdkTracerProvider::builder()
        .with_resource(resource)
        .with_batch_exporter(exporter)
        .build();

    let tracer = provider.tracer("rustpulse-backend");
    global::set_tracer_provider(provider);

    Ok(TracingInit {
        status: TracingStatus::Active,
        otel_layer: Some(tracing_opentelemetry::layer().with_tracer(tracer)),
        exporter_meta: Some(ExporterMeta {
            service_name: config.service_name.clone(),
            otlp_endpoint: endpoint,
        }),
    })
}

/// Emits a one-time "exporter initialized" log line.
///
/// # Examples
///
/// ```rust,no_run
/// use rustpulse::infra::tracing::{emit_exporter_initialized_log, ExporterMeta};
///
/// emit_exporter_initialized_log(&ExporterMeta {
///     service_name: "svc".to_string(),
///     otlp_endpoint: "http://127.0.0.1:4317".to_string(),
/// });
/// ```
pub fn emit_exporter_initialized_log(meta: &ExporterMeta) {
    tracing::info!(
        service_name = %meta.service_name,
        endpoint = %meta.otlp_endpoint,
        "rustpulse boot: tracing exporter initialized"
    );
}

/// Emits a minimal `startup` span to validate span export configuration.
///
/// # Examples
///
/// ```rust,no_run
/// use rustpulse::infra::tracing::emit_startup_span;
///
/// emit_startup_span();
/// ```
pub fn emit_startup_span() {
    let span = tracing::info_span!("startup");
    let _enter = span.enter();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_tracing_disabled_for_missing_service_or_endpoint() {
        let missing_service = TracingConfig {
            service_name: "   ".to_string(),
            environment: "test".to_string(),
            otlp_endpoint: Some("http://127.0.0.1:4317".to_string()),
        };
        let init = init_tracing(&missing_service).expect("disabled path should not error");
        assert!(matches!(
            init.status,
            TracingStatus::Disabled {
                reason: DisabledReason::Misconfigured
            }
        ));
        assert!(init.otel_layer.is_none());
        assert!(init.exporter_meta.is_none());

        let missing_endpoint = TracingConfig {
            service_name: "svc".to_string(),
            environment: "test".to_string(),
            otlp_endpoint: None,
        };
        let init = init_tracing(&missing_endpoint).expect("disabled path should not error");
        assert!(matches!(
            init.status,
            TracingStatus::Disabled {
                reason: DisabledReason::Misconfigured
            }
        ));
        assert!(init.otel_layer.is_none());
        assert!(init.exporter_meta.is_none());
    }

    #[tokio::test]
    async fn test_init_tracing_enabled_builds_otel_layer() {
        let cfg = TracingConfig {
            service_name: "svc".to_string(),
            environment: "test".to_string(),
            otlp_endpoint: Some("http://127.0.0.1:4317".to_string()),
        };

        let init = init_tracing(&cfg).expect("should build OTLP layer without contacting backend");
        assert!(matches!(init.status, TracingStatus::Active));
        assert!(init.otel_layer.is_some());
        let meta = init
            .exporter_meta
            .expect("active path includes exporter metadata");
        assert_eq!(meta.service_name, "svc");
        assert_eq!(meta.otlp_endpoint, "http://127.0.0.1:4317");
    }

    #[test]
    fn test_exporter_initialized_log_includes_service_and_endpoint() {
        use std::io::{Result as IoResult, Write};
        use std::sync::{Arc, Mutex};
        use tracing_subscriber::fmt::MakeWriter;

        #[derive(Clone, Default)]
        struct SharedBuf(Arc<Mutex<Vec<u8>>>);

        struct BufWriter(Arc<Mutex<Vec<u8>>>);

        impl Write for BufWriter {
            fn write(&mut self, buf: &[u8]) -> IoResult<usize> {
                let mut locked = self
                    .0
                    .lock()
                    .map_err(|_| std::io::Error::other("lock error"))?;
                locked.extend_from_slice(buf);
                Ok(buf.len())
            }

            fn flush(&mut self) -> IoResult<()> {
                Ok(())
            }
        }

        impl<'a> MakeWriter<'a> for SharedBuf {
            type Writer = BufWriter;

            fn make_writer(&'a self) -> Self::Writer {
                BufWriter(self.0.clone())
            }
        }

        let buf = SharedBuf::default();
        let subscriber = tracing_subscriber::fmt()
            .without_time()
            .with_writer(buf.clone())
            .finish();

        let meta = ExporterMeta {
            service_name: "svc".to_string(),
            otlp_endpoint: "http://127.0.0.1:4317".to_string(),
        };

        tracing::subscriber::with_default(subscriber, || {
            emit_exporter_initialized_log(&meta);
        });

        let locked = buf.0.lock().expect("mutex poisoned");
        let out = String::from_utf8_lossy(&locked);
        assert!(out.contains("tracing exporter initialized"));
        assert!(out.contains("svc"));
        assert!(out.contains("http://127.0.0.1:4317"));
    }
}
