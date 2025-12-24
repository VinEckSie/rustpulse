use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Telemetry {
    pub source_id: Uuid,
    pub server_id: Uuid,
    pub timestamp: DateTime<Utc>,
    // Generic metrics (cross-domain)
    pub cpu: Option<f64>,
    pub memory: Option<f64>,
    pub temperature: Option<f32>,
    // Optional domain-specific metrics go here as a loosed bag
    pub extras: serde_json::Value,
}

impl Telemetry {
    fn _empty(source_id: Uuid, server_id: Uuid) -> Self {
        Self {
            source_id,
            server_id,
            timestamp: Utc::now(),
            cpu: None,
            memory: None,
            temperature: None,
            extras: serde_json::json!({}),
        }
    }
}
