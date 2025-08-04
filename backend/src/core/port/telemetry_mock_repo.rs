use crate::core::domains::node_telemetry::NodeTelemetry;
use crate::errors::DataError;
use chrono::{Duration, Utc};
use rand::Rng;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use uuid::Uuid;

pub struct MockDataGenerator;

impl MockDataGenerator {
    pub fn generate_mock_data(path: &str, count: usize) -> Result<(), DataError> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            //.expect("Failed to open path for writing mock data");
            .map_err(|e| DataError::FileOpen {
                path: PathBuf::from(path),
                source: e,
            })?;

        let mut rng = rand::thread_rng();

        for _ in 0..count {
            let mock_telemetry = NodeTelemetry {
                node_id: Uuid::new_v4(),
                server_id: Uuid::new_v4(),
                cpu: rng.gen_range(0.0..100.0),
                memory: rng.gen_range(0.0..16000.0),
                timestamp: Utc::now(),
                connected_users: rng.gen_range(0..100),
                network_usage: rng.gen_range(0.0..1.0),
                disk_usage: rng.gen_range(0.0..100.0),
                uptime: Duration::seconds(rng.gen_range(0..3600)),
                errors_detected: Some(vec!["error1".to_string(), "error2".to_string()]),
                anomaly: rng.gen_bool(0.1), // 10% chance of anomaly
                battery_level: Some(rng.gen_range(0.0..100.0)),
                temperature: Some(rng.gen_range(-10.0..50.0)),
                signal_strength: Some(rng.gen_range(0.0..1.0)),
                orientation: Some((
                    rng.gen_range(-180.0..180.0),
                    rng.gen_range(-180.0..180.0),
                    rng.gen_range(-180.0..180.0),
                )),
            };

            let telemetry_json =
                serde_json::to_string(&mock_telemetry).map_err(DataError::Serde)?;

            writeln!(file, "{telemetry_json}").map_err(DataError::Io)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapters::jsonl_telemetry_repo::JsonlTelemetryRepo;
    use crate::core::port::telemetry_repo::TelemetryRepository;
    use std::path::PathBuf;
    use tokio::runtime::Runtime;

    #[test]
    fn test_fetch_all_with_mock_data() -> Result<(), DataError> {
        let rt = Runtime::new().unwrap();

        // Use a temporary file for testing
        let temp_file_path = "test_data.jsonl";
        std::fs::remove_file(temp_file_path).ok();

        // Generate mock data
        MockDataGenerator::generate_mock_data(temp_file_path, 10)?;

        // Test the repository
        rt.block_on(async {
            let repo = JsonlTelemetryRepo::new(PathBuf::from(temp_file_path));
            let result = repo.query_all(None).await;

            assert!(result.is_ok());
            let data = result.unwrap();
            assert_eq!(data.len(), 10); // Verify we retrieved the right amount
        });

        // Clean up a test file
        std::fs::remove_file(temp_file_path).ok();
        Ok(())
    }
}
