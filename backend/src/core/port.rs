//traits / interfaces
use crate::core::domains::telemetry::NodeTelemetry;

pub trait MetricsRepository {
    fn fetch_metrics(&self) -> NodeTelemetry;
}
