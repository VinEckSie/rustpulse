// use crate::core::domains::node_telemetry::NodeTelemetry;
// use crate::core::port::;

use crate::core::domains::node_telemetry::NodeTelemetry;
use crate::core::port::telemetry_ingest_case::TelemetryIngestCase;
use crate::core::port::telemetry_query_case::TelemetryQueryCase;
use crate::core::port::telemetry_repo::TelemetryRepository;
use std::sync::Arc;

pub struct TelemetryService<TelemetryRepo: TelemetryRepository> {
    repo: Arc<TelemetryRepo>,
}

impl<TelemetryRepo: TelemetryRepository> TelemetryService<TelemetryRepo> {
    pub fn new(repo: Arc<TelemetryRepo>) -> Self {
        // Accept Arc instead of plain type
        Self { repo }
    }
}

#[async_trait::async_trait]
impl<TelemetryRepo: TelemetryRepository + Send + Sync> TelemetryQueryCase
    for TelemetryService<TelemetryRepo>
{
    async fn fetch_all(&self, node_id: Option<String>) -> anyhow::Result<Vec<NodeTelemetry>> {
        self.repo.query_all(node_id).await
    }
}
#[async_trait::async_trait]
impl<TelemetryRepo: TelemetryRepository + Send + Sync> TelemetryIngestCase
    for TelemetryService<TelemetryRepo>
{
    async fn ingest(&self, telemetry: NodeTelemetry) -> anyhow::Result<()> {
        self.repo.save(telemetry).await
    }
}
