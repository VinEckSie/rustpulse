use crate::core::domains::telemetry::Telemetry;

#[async_trait::async_trait]
pub trait TelemetryIngestCase {
    async fn ingest(&self, telemetry: Telemetry) -> anyhow::Result<()>;
}
