//! Telemetry domain model.
//!
//! # Examples
//!
//! ```rust
//! use rustpulse::core::domains::telemetry::Telemetry;
//! use chrono::Utc;
//! use uuid::Uuid;
//!
//! let telemetry = Telemetry {
//!     source_id: Uuid::new_v4(),
//!     server_id: Uuid::new_v4(),
//!     timestamp: Utc::now(),
//!     cpu: Some(0.5),
//!     memory: None,
//!     temperature: None,
//!     extras: serde_json::json!({"region":"eu"}),
//! };
//!
//! assert!(telemetry.cpu.is_some());
//! ```

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Telemetry datapoint captured from a source and reported by a server.
pub struct Telemetry {
    /// Unique identifier of the telemetry source (e.g., device or node).
    pub source_id: Uuid,
    /// Unique identifier of the server that observed/forwarded the telemetry.
    pub server_id: Uuid,
    /// Timestamp when the telemetry was recorded.
    pub timestamp: DateTime<Utc>,
    /// CPU usage (generic cross-domain metric).
    pub cpu: Option<f64>,
    /// Memory usage (generic cross-domain metric).
    pub memory: Option<f64>,
    /// Temperature reading (generic cross-domain metric).
    pub temperature: Option<f32>,
    /// Additional domain-specific metrics and attributes.
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
