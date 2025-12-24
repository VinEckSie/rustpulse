use crate::core::domains::telemetry::Telemetry;
use crate::core::application::telemetry::ports::input::telemetry_ingest_usecase::TelemetryIngestCase;
use crate::core::application::telemetry::ports::input::telemetry_query_usecase::TelemetryQueryCase;
use crate::core::application::telemetry::ports::output::telemetry_repository::TelemetryRepository;
use std::sync::Arc;

pub struct TelemetryService {
    repo: Arc<dyn TelemetryRepository + Send + Sync>,
}

impl TelemetryService {
    pub fn new(repo: Arc<dyn TelemetryRepository + Send + Sync>) -> Self {
        // Accept Arc instead of plain type
        Self { repo }
    }
}

#[async_trait::async_trait]
impl TelemetryQueryCase for TelemetryService {
    async fn fetch_all(&self, node_id: Option<String>) -> anyhow::Result<Vec<Telemetry>> {
        self.repo.query_all(node_id).await
    }
}
#[async_trait::async_trait]
impl TelemetryIngestCase for TelemetryService {
    async fn ingest(&self, telemetry: Telemetry) -> anyhow::Result<()> {
        self.repo.save(telemetry).await
    }
}

// Example: core/application/telemetry/usecases/telemetry_service.rs
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::adapters::output::jsonl_repo::JsonlTelemetryRepo;
//
//     #[tokio::test]
//     async fn test_telemetry_service() {
//         let repo = JsonlTelemetryRepo::new("mock-path.jsonl".into());
//         let service = TelemetryService::new(repo);
//
//         let telemetry = service.fetch_all(Some("001".parse().unwrap())).await.unwrap();
//         // assert_eq!(telemetry, "mocked");
//     }
// }
