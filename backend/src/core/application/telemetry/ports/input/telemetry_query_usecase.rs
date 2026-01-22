use crate::core::domains::telemetry::Telemetry;

#[async_trait::async_trait]
pub trait TelemetryQueryCase: Send + Sync {
    async fn fetch_all(&self, node_id: Option<String>) -> anyhow::Result<Vec<Telemetry>>;
}
