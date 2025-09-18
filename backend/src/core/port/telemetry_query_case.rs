// core/port/input/telemetry_query_case.rs
use crate::core::domains::node_telemetry::NodeTelemetry;

#[async_trait::async_trait]
pub trait TelemetryQueryCase: Send + Sync {
    async fn fetch_all(&self, node_id: Option<String>) -> anyhow::Result<Vec<NodeTelemetry>>;
}
