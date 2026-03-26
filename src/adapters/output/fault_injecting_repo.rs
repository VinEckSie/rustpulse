//! Telemetry repository wrapper that can inject faults for testing and demos.

use crate::core::application::telemetry::TelemetryRepository;
use crate::core::domains::telemetry::Telemetry;
use std::sync::Mutex;

#[derive(Debug, Clone)]
/// Configuration for [`FaultInjectingTelemetryRepo`].
pub struct FaultInjectionConfig {
    /// Enables or disables fault injection.
    pub enabled: bool,
    /// Probability in `[0.0, 1.0]` that a `save()` call is dropped (returns `Ok(())`).
    pub drop_rate: f64,
    /// Probability in `[0.0, 1.0]` that a `save()` call is corrupted (mutates `Telemetry.extras`).
    pub corrupt_rate: f64,
    /// Seed used for deterministic pseudo-random decisions.
    pub seed: u64,
}

impl FaultInjectionConfig {
    /// Validates and constructs a new configuration.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustpulse::adapters::output::fault_injecting_repo::FaultInjectionConfig;
    ///
    /// let cfg = FaultInjectionConfig::try_new(true, 0.1, 0.0, 42).unwrap();
    /// assert!(cfg.enabled);
    /// ```
    pub fn try_new(
        enabled: bool,
        drop_rate: f64,
        corrupt_rate: f64,
        seed: u64,
    ) -> Result<Self, FaultInjectionConfigError> {
        if !drop_rate.is_finite() || !(0.0..=1.0).contains(&drop_rate) {
            return Err(FaultInjectionConfigError::InvalidRate {
                field: "drop_rate",
                value: drop_rate,
            });
        }
        if !corrupt_rate.is_finite() || !(0.0..=1.0).contains(&corrupt_rate) {
            return Err(FaultInjectionConfigError::InvalidRate {
                field: "corrupt_rate",
                value: corrupt_rate,
            });
        }
        if drop_rate + corrupt_rate > 1.0 {
            return Err(FaultInjectionConfigError::RateSumTooHigh {
                drop_rate,
                corrupt_rate,
            });
        }

        Ok(Self {
            enabled,
            drop_rate,
            corrupt_rate,
            seed,
        })
    }
}

#[derive(Debug, thiserror::Error)]
/// Errors returned when building [`FaultInjectionConfig`].
pub enum FaultInjectionConfigError {
    /// A rate field is outside `[0.0, 1.0]` or not finite.
    #[error("invalid {field}: {value}")]
    InvalidRate {
        /// Name of the invalid field.
        field: &'static str,
        /// The rejected value.
        value: f64,
    },

    /// The sum of `drop_rate` and `corrupt_rate` must not exceed `1.0`.
    #[error(
        "drop_rate + corrupt_rate must be <= 1.0 (got drop={drop_rate}, corrupt={corrupt_rate})"
    )]
    RateSumTooHigh {
        /// Requested drop rate.
        drop_rate: f64,
        /// Requested corrupt rate.
        corrupt_rate: f64,
    },
}

/// A [`TelemetryRepository`] decorator that probabilistically drops or corrupts writes.
pub struct FaultInjectingTelemetryRepo<R> {
    inner: R,
    config: FaultInjectionConfig,
    rng_state: Mutex<u64>,
}

impl<R> FaultInjectingTelemetryRepo<R> {
    /// Wraps an existing repository with fault injection.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustpulse::adapters::output::fault_injecting_repo::{
    ///     FaultInjectionConfig, FaultInjectingTelemetryRepo,
    /// };
    ///
    /// let cfg = FaultInjectionConfig::try_new(true, 0.0, 0.0, 1).unwrap();
    /// let inner = ();
    /// let _repo = FaultInjectingTelemetryRepo::new(inner, cfg);
    /// ```
    pub fn new(inner: R, config: FaultInjectionConfig) -> Self {
        Self {
            rng_state: Mutex::new(config.seed),
            inner,
            config,
        }
    }

    fn next_f64(&self) -> anyhow::Result<f64> {
        let mut state = self
            .rng_state
            .lock()
            .map_err(|_| anyhow::anyhow!("rng_state lock poisoned"))?;

        // LCG: deterministic, cheap, good enough for fault injection.
        *state = state.wrapping_mul(6364136223846793005u64).wrapping_add(1);

        // Map to [0, 1). Avoid 1.0 exactly.
        let x = (*state >> 11) as f64; // 53-ish bits
        Ok(x / ((1u64 << 53) as f64))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum InjectionDecision {
    Drop,
    Corrupt,
    Pass,
}

#[async_trait::async_trait]
impl<R> TelemetryRepository for FaultInjectingTelemetryRepo<R>
where
    R: TelemetryRepository + Send + Sync,
{
    async fn save(&self, mut telemetry: Telemetry) -> anyhow::Result<()> {
        if !self.config.enabled {
            return self.inner.save(telemetry).await;
        }

        let roll = self.next_f64()?;
        let decision = if roll < self.config.drop_rate {
            InjectionDecision::Drop
        } else if roll < (self.config.drop_rate + self.config.corrupt_rate) {
            InjectionDecision::Corrupt
        } else {
            InjectionDecision::Pass
        };

        match decision {
            InjectionDecision::Drop => {
                tracing::event!(
                    tracing::Level::INFO,
                    decision = "drop",
                    drop_rate = self.config.drop_rate,
                    corrupt_rate = self.config.corrupt_rate,
                    "fault injection decision"
                );
                Ok(())
            }
            InjectionDecision::Corrupt => {
                tracing::event!(
                    tracing::Level::INFO,
                    decision = "corrupt",
                    drop_rate = self.config.drop_rate,
                    corrupt_rate = self.config.corrupt_rate,
                    "fault injection decision"
                );

                telemetry.extras = serde_json::json!({
                    "fault_injected": "corrupt",
                    "original": telemetry.extras
                });

                self.inner.save(telemetry).await
            }
            InjectionDecision::Pass => self.inner.save(telemetry).await,
        }
    }

    async fn query_all(&self, node_id: Option<String>) -> anyhow::Result<Vec<Telemetry>> {
        self.inner.query_all(node_id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone as _, Utc};
    use serde_json::json;
    use std::collections::BTreeMap;
    use std::fmt;
    use std::sync::{Arc, Mutex as StdMutex};
    use tracing::Subscriber;
    use tracing::field::{Field, Visit};
    use tracing_subscriber::layer::Context;
    use tracing_subscriber::prelude::*;
    use tracing_subscriber::{Layer, registry::LookupSpan};
    use uuid::Uuid;

    #[derive(Default)]
    struct CapturingRepo {
        saved: StdMutex<Vec<Telemetry>>,
    }

    #[async_trait::async_trait]
    impl TelemetryRepository for CapturingRepo {
        async fn save(&self, telemetry: Telemetry) -> anyhow::Result<()> {
            let mut locked = self
                .saved
                .lock()
                .map_err(|_| anyhow::anyhow!("lock poisoned"))?;
            locked.push(telemetry);
            Ok(())
        }

        async fn query_all(&self, _node_id: Option<String>) -> anyhow::Result<Vec<Telemetry>> {
            let locked = self
                .saved
                .lock()
                .map_err(|_| anyhow::anyhow!("lock poisoned"))?;
            Ok(locked.clone())
        }
    }

    #[async_trait::async_trait]
    impl TelemetryRepository for Arc<CapturingRepo> {
        async fn save(&self, telemetry: Telemetry) -> anyhow::Result<()> {
            (**self).save(telemetry).await
        }

        async fn query_all(&self, node_id: Option<String>) -> anyhow::Result<Vec<Telemetry>> {
            (**self).query_all(node_id).await
        }
    }

    fn sample_telemetry() -> Telemetry {
        Telemetry {
            source_id: Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap(),
            server_id: Uuid::parse_str("00000000-0000-0000-0000-000000000002").unwrap(),
            timestamp: Utc.with_ymd_and_hms(2026, 2, 18, 0, 0, 0).unwrap(),
            cpu: Some(1.0),
            memory: None,
            temperature: None,
            extras: json!({"k":"v"}),
        }
    }

    #[derive(Clone, Default)]
    struct CapturedEvents(Arc<StdMutex<Vec<BTreeMap<String, String>>>>);

    #[derive(Clone, Default)]
    struct CaptureEventLayer {
        captured: CapturedEvents,
    }

    struct EventFieldVisitor<'a> {
        fields: &'a mut BTreeMap<String, String>,
    }

    impl<'a> Visit for EventFieldVisitor<'a> {
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

    impl<S> Layer<S> for CaptureEventLayer
    where
        S: Subscriber + for<'a> LookupSpan<'a>,
    {
        fn on_event(&self, event: &tracing::Event<'_>, _ctx: Context<'_, S>) {
            let mut fields = BTreeMap::new();
            event.record(&mut EventFieldVisitor {
                fields: &mut fields,
            });

            if let Ok(mut locked) = self.captured.0.lock() {
                locked.push(fields);
            }
        }
    }

    fn any_event_has_field(captured: &CapturedEvents, key: &str, value: &str) -> bool {
        let Ok(locked) = captured.0.lock() else {
            return false;
        };
        locked
            .iter()
            .any(|m| m.get(key).map(String::as_str) == Some(value))
    }

    #[tokio::test]
    async fn test_fault_injection_disabled_delegates_unchanged() {
        let inner = Arc::new(CapturingRepo::default());
        let cfg = FaultInjectionConfig::try_new(false, 1.0, 0.0, 123).unwrap();
        let repo = FaultInjectingTelemetryRepo::new(inner.clone(), cfg);

        let t = sample_telemetry();
        repo.save(t.clone()).await.unwrap();

        let saved = inner.saved.lock().unwrap();
        assert_eq!(saved.len(), 1);
        assert_eq!(saved[0].source_id, t.source_id);
        assert_eq!(saved[0].server_id, t.server_id);
        assert_eq!(saved[0].timestamp, t.timestamp);
        assert_eq!(saved[0].extras, t.extras);
    }

    #[tokio::test]
    async fn test_fault_injection_drop_never_calls_inner_and_emits_event() {
        let captured = CapturedEvents::default();
        let subscriber = tracing_subscriber::registry().with(CaptureEventLayer {
            captured: captured.clone(),
        });
        let _guard = tracing::subscriber::set_default(subscriber);

        let inner = Arc::new(CapturingRepo::default());
        let cfg = FaultInjectionConfig::try_new(true, 1.0, 0.0, 123).unwrap();
        let repo = FaultInjectingTelemetryRepo::new(inner.clone(), cfg);

        repo.save(sample_telemetry()).await.unwrap();

        let saved = inner.saved.lock().unwrap();
        assert_eq!(saved.len(), 0);
        assert!(any_event_has_field(&captured, "decision", "drop"));
    }

    #[tokio::test]
    async fn test_fault_injection_corrupt_calls_inner_with_modified_payload_and_emits_event() {
        let captured = CapturedEvents::default();
        let subscriber = tracing_subscriber::registry().with(CaptureEventLayer {
            captured: captured.clone(),
        });
        let _guard = tracing::subscriber::set_default(subscriber);

        let inner = Arc::new(CapturingRepo::default());
        let cfg = FaultInjectionConfig::try_new(true, 0.0, 1.0, 123).unwrap();
        let repo = FaultInjectingTelemetryRepo::new(inner.clone(), cfg);

        let t = sample_telemetry();
        repo.save(t.clone()).await.unwrap();

        let saved = inner.saved.lock().unwrap();
        assert_eq!(saved.len(), 1);
        assert_ne!(saved[0].extras, t.extras);
        assert!(any_event_has_field(&captured, "decision", "corrupt"));
    }
}
