// core/port/input/telemetry_ingest_case.rs
use crate::core::domains::node_telemetry::NodeTelemetry;

#[async_trait::async_trait]
pub trait TelemetryIngestCase {
    async fn ingest(&self, telemetry: NodeTelemetry) -> anyhow::Result<()>;
}
