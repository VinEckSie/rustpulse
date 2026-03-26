//! JSONL-backed telemetry repository.
//!
//! # Examples
//!
//! ```rust,no_run
//! # async fn demo() -> anyhow::Result<()> {
//! use rustpulse::adapters::output::jsonl_repo::JsonlTelemetryRepo;
//! use rustpulse::core::application::telemetry::TelemetryRepository as _;
//! use rustpulse::core::domains::telemetry::Telemetry;
//! use chrono::Utc;
//! use uuid::Uuid;
//!
//! let path = std::env::temp_dir().join("telemetry.jsonl");
//! let repo = JsonlTelemetryRepo::new(path);
//!
//! repo.save(Telemetry {
//!     source_id: Uuid::new_v4(),
//!     server_id: Uuid::new_v4(),
//!     timestamp: Utc::now(),
//!     cpu: Some(1.0),
//!     memory: None,
//!     temperature: None,
//!     extras: serde_json::json!({}),
//! })
//! .await?;
//! # Ok(())
//! # }
//! ```

// adapter/jsonl/telemetry_repo.rs
use crate::core::application::telemetry::TelemetryRepository;
use crate::core::domains::telemetry::Telemetry;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Write};
use tokio::sync::Mutex;
use uuid::Uuid;

/// Stores and retrieves telemetry from a newline-delimited JSON file.
pub struct JsonlTelemetryRepo<P: AsRef<std::path::Path>> {
    /// Path to the JSONL file.
    pub path: P,
    /// In-process lock used to serialize file access.
    pub lock: Mutex<()>,
}

impl<P: AsRef<std::path::Path>> JsonlTelemetryRepo<P> {
    /// Creates a repository backed by the provided file path.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustpulse::adapters::output::jsonl_repo::JsonlTelemetryRepo;
    ///
    /// let repo = JsonlTelemetryRepo::new("metrics.jsonl");
    /// let _ = repo.path;
    /// ```
    pub fn new(path: P) -> Self {
        Self {
            path,
            lock: Mutex::new(()),
        }
    }
}

#[async_trait::async_trait]
impl<P> TelemetryRepository for JsonlTelemetryRepo<P>
where
    P: AsRef<std::path::Path> + Send + Sync,
{
    async fn save(&self, telemetry: Telemetry) -> anyhow::Result<()> {
        let _guard = self.lock.lock().await;
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)?;
        let line = serde_json::to_string(&telemetry)?;
        writeln!(file, "{line}")?;
        Ok(())
    }

    async fn query_all(&self, node_id: Option<String>) -> anyhow::Result<Vec<Telemetry>> {
        let file = OpenOptions::new().read(true).open(&self.path)?;
        let reader = BufReader::new(file);
        let iter = reader.lines().map(|line| -> anyhow::Result<Telemetry> {
            let line = line?;
            Ok(serde_json::from_str(&line)?)
        });

        let result: anyhow::Result<Vec<Telemetry>> = match node_id {
            Some(id_str) => {
                let id = Uuid::parse_str(&id_str)?;
                iter.filter_map(|res| match res {
                    Ok(t) if t.source_id == id => Some(Ok(t)),
                    Ok(_) => None,
                    Err(e) => Some(Err(e)),
                })
                .collect()
            }
            None => iter.collect(),
        };

        result
    }
}

// Example: adapters/jsonl_telemetry_repo.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_metrics() {
        let repo: JsonlTelemetryRepo<String> = JsonlTelemetryRepo::new("mock-path.jsonl".into());
        let _data = repo.query_all(None);
    }
}
