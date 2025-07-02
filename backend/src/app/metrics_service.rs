use crate::core::port::MetricsRepository;
use crate::core::domains::telemetry::NodeTelemetry;

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
}
