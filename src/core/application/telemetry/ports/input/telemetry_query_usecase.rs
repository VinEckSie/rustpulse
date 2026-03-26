//! Input port for telemetry queries.

use crate::core::domains::telemetry::Telemetry;

#[async_trait::async_trait]
/// Use case that queries stored telemetry.
pub trait TelemetryQueryCase: Send + Sync {
    /// Fetches all telemetry, optionally filtered by a node/source identifier.
    async fn fetch_all(&self, node_id: Option<String>) -> anyhow::Result<Vec<Telemetry>>;
}
