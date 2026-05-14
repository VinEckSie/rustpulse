//! Input port for telemetry ingestion.

use crate::core::domains::telemetry::Telemetry;

#[async_trait::async_trait]
/// Use case that accepts telemetry and persists it.
pub trait TelemetryIngestCase {
    /// Ingests a telemetry datapoint.
    async fn ingest(&self, telemetry: Telemetry) -> anyhow::Result<()>;
}
