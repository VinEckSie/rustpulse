// core/port/input/telemetry_query_case.rs
use crate::core::domains::node_telemetry::NodeTelemetry;

#[async_trait::async_trait]
pub trait TelemetryQueryCase: Send + Sync {
    async fn fetch_all(&self, node_id: Option<String>) -> anyhow::Result<Vec<NodeTelemetry>>;
}

#[cfg(test)]
mod tests {
    struct MockRepo;

    // impl TelemetryRepository for MockRepo {
    //
    //     // fn save(&self, telemetry: NodeTelemetry) -> NodeTelemetry {
    //     //     // Mock behavior
    //     //     oneNode: Node
    //     // }
    //     //
    //     // fn query_all(&self) -> NodeTelemetry {
    //     //     // Mock behavior
    //     //     NodeTelemetry { data: "test".into() }
    //     // }
    // }

    // #[test]
    // fn test_fetch_metrics_default() {
    //     let repo = MockRepo;
    //     let metrics = repo.fetch_metrics();
    //     assert_eq!(metrics.data, "test");
    // }
}
