//! Output port for telemetry persistence.

use crate::core::domains::telemetry::Telemetry;

#[async_trait::async_trait]
/// Repository abstraction for storing and retrieving telemetry.
pub trait TelemetryRepository {
    /// Persists a telemetry datapoint.
    async fn save(&self, telemetry: Telemetry) -> anyhow::Result<()>;
    /// Retrieves all telemetry, optionally filtered by a node/source identifier.
    async fn query_all(&self, node_id: Option<String>) -> anyhow::Result<Vec<Telemetry>>;
}
