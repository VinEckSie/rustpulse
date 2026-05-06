use chrono::Utc;
use reqwest::Client;
use serde::Serialize;
use std::time::Duration;
use uuid::Uuid;

#[derive(Debug, Serialize)]
struct TelemetryPayload {
    source_id: Uuid,
    server_id: Uuid,
    timestamp: String,
    cpu: Option<f64>,
    memory: Option<f64>,
    temperature: Option<f32>,
    extras: serde_json::Value,
}

fn env_u64(key: &str, default: u64) -> u64 {
    std::env::var(key)
        .ok()
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(default)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let telemetry_url = std::env::var("TELEMETRY_URL")
        .unwrap_or_else(|_| "http://127.0.0.1:3000/telemetry".to_string());
    let host_id = std::env::var("HOST_ID").unwrap_or_else(|_| "local-dev-machine".to_string());

    // Defaults are chosen so `cargo run --bin agent` "just works" as a demo:
    // send 10 events, one per second.
    // Set COUNT=0 to run forever.
    let count = env_u64("COUNT", 10);
    let interval_ms = env_u64("INTERVAL_MS", 1000);
    let startup_wait_s = env_u64("STARTUP_WAIT_S", 30);

    let client = Client::new();

    let mut i: u64 = 0;
    let mut consecutive_errors: u64 = 0;
    let max_consecutive_errors = startup_wait_s.max(1);

    loop {
        if count != 0 && i >= count {
            break;
        }

        let payload = TelemetryPayload {
            source_id: Uuid::new_v4(),
            server_id: Uuid::new_v4(),
            timestamp: Utc::now().to_rfc3339(),
            cpu: Some(42.5),
            memory: Some(68.2),
            temperature: None,
            extras: serde_json::json!({ "host_id": host_id, "seq": i }),
        };

        let response = match client.post(&telemetry_url).json(&payload).send().await {
            Ok(r) => {
                consecutive_errors = 0;
                r
            }
            Err(e) => {
                consecutive_errors += 1;
                eprintln!(
                    "agent: failed to send telemetry (attempt {}): {}",
                    consecutive_errors, e
                );
                if consecutive_errors >= max_consecutive_errors {
                    anyhow::bail!(
                        "backend not reachable at {} (is `just backend` running?)",
                        telemetry_url
                    );
                }
                tokio::time::sleep(Duration::from_secs(1)).await;
                continue;
            }
        };

        println!("Status: {}", response.status());

        i += 1;
        tokio::time::sleep(Duration::from_millis(interval_ms)).await;
    }
    Ok(())
}
