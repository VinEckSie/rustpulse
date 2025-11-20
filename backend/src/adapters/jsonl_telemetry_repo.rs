// adapter/jsonl/telemetry_repo.rs
use crate::core::domains::telemetry::Telemetry;
use crate::core::port::telemetry_repo::TelemetryRepository;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use tokio::sync::Mutex;
use uuid::Uuid;

pub struct JsonlTelemetryRepo {
    pub path: PathBuf,
    pub lock: Mutex<()>,
}

impl JsonlTelemetryRepo {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            lock: Mutex::new(()),
        }
    }
}

#[async_trait::async_trait]
impl TelemetryRepository for JsonlTelemetryRepo {
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

        let mut result = Vec::new();

        for line in reader.lines() {
            let line = line?;
            let parsed: Telemetry = serde_json::from_str(&line)?;

            match &node_id {
                Some(id_str) => {
                    if let Ok(id) = Uuid::parse_str(id_str)
                        && parsed.source_id == id
                    {
                        result.push(parsed);
                    }
                }
                None => result.push(parsed),
            }
        }

        Ok(result)
    }
}

// Example: adapters/jsonl_telemetry_repo.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_metrics() {
        let _repo = JsonlTelemetryRepo::new("mock-path.jsonl".into());
        //let data = repo.load().unwrap();
        //assert_eq!(data.len(), 20); // Assuming 20 mock entries
    }
}
