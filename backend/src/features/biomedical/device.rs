// Rust
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BioDevice {
    pub id: Uuid,
    pub device_kind: String, // "ECG", "EEG", "SpO2", ...
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub firmware: Option<String>,
    pub patient_ref: Option<String>, // external id / FHIR ref
    pub labels: Vec<String>,
}
