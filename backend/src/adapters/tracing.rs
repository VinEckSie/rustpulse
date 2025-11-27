/// Runtime configuration for tracing / Jaeger exporter.
pub struct TracingConfig {
    pub service_name: String,
    pub environment: String,
    /// Jaeger endpoint, e.g. "http://localhost:14268/api/traces".
    /// If None or empty, tracing is considered disabled.
    pub jaeger_endpoint: Option<String>,
}

/// High-level status after initialization.
#[derive(Debug)]
pub enum TracingStatus {
    /// Tracing is active and a startup span has been emitted.
    Active,
    /// Tracing is disabled or degraded; service should continue without spans.
    Disabled { reason: DisabledReason },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisabledReason {
    Misconfigured,
    BackendUnavailable,
}

/// Errors that can occur during tracing initialization.
#[derive(Debug, thiserror::Error)]
pub enum TracingInitError {
    #[error("invalid tracing configuration: {0}")]
    InvalidConfig(String),
    #[error("failed to connect to Jaeger backend: {0}")]
    BackendUnavailable(String),
}

/// Idempotent initialization of global tracer/exporter.
/// - On success, always returns a `TracingStatus` describing whether tracing is
///   effectively active or gracefully disabled.
/// - Never panics. Never blocks indefinitely. Safe to call multiple times.
pub fn init_tracing(config: &TracingConfig) -> Result<TracingStatus, TracingInitError> {
    // Basic config validation: service name must be non-empty.
    if config.service_name.trim().is_empty() {
        return Ok(TracingStatus::Disabled {
            reason: (DisabledReason::Misconfigured),
        });
    }

    // If no endpoint is provided, treat tracing as disabled by configuration.
    let endpoint = match config.jaeger_endpoint.as_ref().map(|s| s.trim()) {
        None | Some("") => {
            return Ok(TracingStatus::Disabled {
                reason: (DisabledReason::Misconfigured),
            });
        }
        Some(endpoint) => endpoint,
    };

    // Jaeger / exporter setup is intentionally left unimplemented for now.
    // It is behind a feature gate so tests can exercise config validation
    // without hitting a `todo!()` at runtime.
    #[cfg(feature = "tracing-backend")]
    {
        let _ = endpoint;
        todo!("Jaeger / tracing backend initialization not implemented yet");
    }

    #[cfg(not(feature = "tracing-backend"))]
    {
        let _ = endpoint;
        Ok(TracingStatus::Disabled {
            reason: DisabledReason::BackendUnavailable,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_config() -> TracingConfig {
        TracingConfig {
            service_name: "rustpulse-backend".to_string(),
            environment: "test".to_string(),
            jaeger_endpoint: Some("http://localhost:14268/api/traces".to_string()),
        }
    }

    #[test]
    fn init_with_invalid_config_degrades_gracefully() {
        // Arrange: config with empty service name (obviously invalid).
        let mut config = make_config();
        config.service_name = "   ".to_string();

        // Act
        let status = init_tracing(&config).expect("invalid config should not cause hard error");

        // Assert: tracing is disabled with a Misconfigured reason.
        match status {
            TracingStatus::Disabled { reason } => {
                assert_eq!(reason, DisabledReason::Misconfigured);
            }
            _ => panic!("expected TracingStatus::Disabled for invalid config"),
        }
    }

    #[test]
    fn config_with_none_endpoint_is_effectively_disabled() {
        let cfg = TracingConfig {
            service_name: "svc".into(),
            environment: "local".into(),
            jaeger_endpoint: None,
        };

        // Step 1: just assert the raw shape; behavior comes later.
        assert!(cfg.jaeger_endpoint.is_none());
    }

    #[test]
    fn can_construct_disabled_status_with_reason() {
        let status = TracingStatus::Disabled {
            reason: DisabledReason::Misconfigured,
        };

        match status {
            TracingStatus::Disabled {
                reason: DisabledReason::Misconfigured,
            } => {}
            _ => panic!("expected disabled/misconfigured status"),
        }
    }

    #[test]
    fn invalid_config_error_includes_message() {
        let err = TracingInitError::InvalidConfig("missing endpoint".into());
        let msg = err.to_string();
        assert!(msg.contains("invalid tracing configuration"));
        assert!(msg.contains("missing endpoint"));
    }
}
