use crate::core::domains::telemetry::NodeTelemetry;
use crate::core::port::MetricsRepository;
use chrono::{DateTime, Duration, Utc};
use uuid::Uuid;

pub struct MockMetricsRepository;

impl MetricsRepository for MockMetricsRepository {
    fn fetch_metrics(&self) -> NodeTelemetry {
        let timestamp = DateTime::<Utc>::from_timestamp_millis(61)
            .expect("Hardcoded timestamp should be valid");

        NodeTelemetry {
            id: Uuid::from_u128(989898),
            server_id: Uuid::from_u128(989866),
            cpu: 23.3,
            ram: 54.0,
            timestamp,
            connected_users: 4,
            network_usage: 34.0,
            disk_usage: 66.0,
            uptime: Duration::days(1),
            errors_detected: None,
            anomaly: false,
            battery_level: None,
            temperature: None,
            signal_strength: None,
            orientation: None,
        }
    }
}
