use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Telemetry {
    pub source_id: Uuid,          // node (aero) or device (bio)
    pub server_id: Uuid,          // collector id
    pub timestamp: DateTime<Utc>, // sample time
    // Generic metrics (cross-domain)
    pub cpu: Option<f64>,
    pub memory: Option<f64>,
    pub temperature: Option<f32>,
    // Optional domain-specific metrics go here as a loosed bag
    // so the shared pipeline stays stable.
    pub extras: serde_json::Value, // {} by default; attach domain fields if needed
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
