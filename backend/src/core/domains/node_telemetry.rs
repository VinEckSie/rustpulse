use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeTelemetry {
    pub node_id: Uuid,
    pub server_id: Uuid,
    pub cpu: f64,
    pub memory: f64,
    pub timestamp: DateTime<Utc>,
    pub connected_users: u16,
    pub network_usage: f64,
    pub disk_usage: f64,
    pub uptime: Duration,
    pub errors_detected: Option<Vec<String>>,
    pub anomaly: bool,
    pub battery_level: Option<f32>,   // for UAV or satellite nodes
    pub temperature: Option<f32>,     // thermal status
    pub signal_strength: Option<f32>, // link health
    pub orientation: Option<(f64, f64, f64)>, // pitch, roll, yaw
}

//todo tests
