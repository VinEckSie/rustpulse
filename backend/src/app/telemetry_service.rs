use crate::core::domains::node_telemetry::NodeTelemetry;
use crate::core::port::MetricsRepository;

pub struct MetricsService<'a, MetricsRepo: MetricsRepository> {
    repo: &'a MetricsRepo,
}

impl<'a, MetricsRepo: MetricsRepository> MetricsService<'a, MetricsRepo> {
    pub fn new(repo: &'a MetricsRepo) -> Self {
        Self { repo }
    }

    pub fn get_status(&self) -> NodeTelemetry {
        self.repo.fetch_metrics()
    }

    // pub fn list_all(&self, node_id: Uuid, limit: usize) -> Vec<NodeTelemetry> {
    //     self.repo.fetch_recent(node_id, limit)
    // }
}
