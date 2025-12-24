// core/application/telemetry/ports/output/telemetry_repository.rs
use crate::core::domains::telemetry::Telemetry;

#[async_trait::async_trait]
pub trait TelemetryRepository {
    async fn save(&self, telemetry: Telemetry) -> anyhow::Result<()>;
    async fn query_all(&self, node_id: Option<String>) -> anyhow::Result<Vec<Telemetry>>;
}
