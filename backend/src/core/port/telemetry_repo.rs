// core/port/output/metrics_port.rs
use crate::core::domains::node_telemetry::NodeTelemetry;

#[async_trait::async_trait]
pub trait TelemetryRepository {
    async fn save(&self, telemetry: NodeTelemetry) -> anyhow::Result<()>;
    async fn query_all(&self, node_id: Option<String>) -> anyhow::Result<Vec<NodeTelemetry>>;
}
