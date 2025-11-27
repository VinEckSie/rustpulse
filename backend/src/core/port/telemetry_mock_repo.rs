use crate::core::domains::telemetry::Telemetry;
use crate::errors::DataError;
use chrono::Utc;
use rand::Rng;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use uuid::Uuid;

pub struct MockDataGenerator;

impl MockDataGenerator {
    pub fn generate_mock_data(path: &PathBuf, count: usize) -> Result<(), DataError> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            //.expect("Failed to open path for writing mock data");
            .map_err(|e| DataError::FileOpen {
                path: PathBuf::from(path),
                source: e,
            })?;

        let mut rng = rand::rng();

        for _ in 0..count {
            let mock_telemetry = Telemetry {
                source_id: Uuid::new_v4(),
                server_id: Uuid::new_v4(),
                cpu: Option::from(rng.random_range(0.0..100.0)),
                memory: Option::from(rng.random_range(0.0..16000.0)),
                timestamp: Utc::now(),
                temperature: Some(rng.random_range(-10.0..50.0)),
                extras: Default::default(),
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
        let temp_file_path: PathBuf =
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test_data.jsonl");

        // Generate mock data
        MockDataGenerator::generate_mock_data(&temp_file_path, 10)?;

        // Test the repository
        rt.block_on(async {
            let repo = JsonlTelemetryRepo::new(PathBuf::from(&temp_file_path));
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
