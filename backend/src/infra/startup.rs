use crate::adapters::input::http;
use crate::adapters::output::jsonl_repo::JsonlTelemetryRepo;
use crate::adapters::output::postgres_db;
use crate::adapters::output::postgres_telemetry_repo::PostgresTelemetryRepo;
use crate::core::application::telemetry::{
    TelemetryIngestCase, TelemetryQueryCase, TelemetryService,
};
use crate::infra::mock_telemetry::MockDataGenerator;
use axum::Router;
use std::path::PathBuf;
use std::sync::Arc;
use tracing::instrument;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StorageMode {
    Jsonl,
    Postgres,
}

#[derive(thiserror::Error, Debug)]
pub enum InfraBootError {
    #[error("missing DATABASE_URL for postgres storage")]
    MissingDatabaseUrl,

    #[error("invalid RUSTPULSE_STORAGE value: {value}")]
    InvalidStorageMode { value: String },

    #[error(transparent)]
    Db(#[from] postgres_db::DbError),

    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),

    #[error(transparent)]
    Io(#[from] std::io::Error),
}

pub fn storage_mode_from_env() -> Result<StorageMode, InfraBootError> {
    match std::env::var("RUSTPULSE_STORAGE") {
        Ok(v) if v.eq_ignore_ascii_case("postgres") || v.eq_ignore_ascii_case("pg") => {
            Ok(StorageMode::Postgres)
        }
        Ok(v) if v.eq_ignore_ascii_case("jsonl") || v.is_empty() => Ok(StorageMode::Jsonl),
        Err(_) => Ok(StorageMode::Jsonl),
        Ok(value) => Err(InfraBootError::InvalidStorageMode { value }),
    }
}

pub async fn init_postgres_schema(pool: &sqlx::PgPool) -> Result<(), InfraBootError> {
    tracing::info!("db.migrate");

    sqlx::query(
        r#"
CREATE TABLE IF NOT EXISTS telemetry (
    source_id UUID NOT NULL,
    server_id UUID NOT NULL,
    timestamp TIMESTAMPTZ NOT NULL,
    cpu DOUBLE PRECISION NULL,
    memory DOUBLE PRECISION NULL,
    temperature REAL NULL,
    extras JSONB NOT NULL
)
"#,
    )
    .execute(pool)
    .await?;

    tracing::info!("db.ready");
    Ok(())
}

pub async fn build_telemetry_repository_from_env() -> Result<
    Arc<dyn crate::core::application::telemetry::TelemetryRepository + Send + Sync>,
    InfraBootError,
> {
    match storage_mode_from_env()? {
        StorageMode::Jsonl => {
            let temp_file_path: PathBuf =
                PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("metrics_data.jsonl");
            Ok(Arc::new(JsonlTelemetryRepo::new(temp_file_path)))
        }
        StorageMode::Postgres => {
            let database_url =
                std::env::var("DATABASE_URL").map_err(|_| InfraBootError::MissingDatabaseUrl)?;
            let pool = postgres_db::connect_pool(&database_url).await?;
            init_postgres_schema(&pool).await?;
            Ok(Arc::new(PostgresTelemetryRepo::new(pool)))
        }
    }
}

#[instrument(level = "info")]
pub async fn start_server(port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let temp_file_path: PathBuf =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("metrics_data.jsonl");

    tracing::info!(?temp_file_path, "Using metrics data file");

    // Create mock data only for the JSONL storage mode.
    let mode = storage_mode_from_env().map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
    if mode == StorageMode::Jsonl {
        MockDataGenerator::generate_mock_data(&temp_file_path, 20)?;
    }

    let repo = build_telemetry_repository_from_env()
        .await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
    let service = Arc::new(TelemetryService::new(repo.clone()));
    let query_service: Arc<dyn TelemetryQueryCase> = service.clone();
    let ingest_service: Arc<dyn TelemetryIngestCase + Send + Sync> = service.clone();

    //Build Router
    let app = Router::new()
        .merge(http::root_handler::routes())
        .merge(http::health_handler::routes())
        .merge(http::telemetry_handler::routes(query_service)) // now injecting state
        .merge(http::telemetry_handler::ingest_routes(ingest_service))
        .merge(http::favicon_handler::routes());

    let addr = format!("127.0.0.1:{port}");
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    let local_addr = listener.local_addr()?;
    tracing::info!(%local_addr, "listening");

    //Start Server
    //Listener handles network → Router handles logic
    axum::serve(listener, app).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::sync::OnceLock;

    use chrono::{DateTime, Utc};
    use serde_json::json;
    use tokio::sync::Mutex;
    use uuid::Uuid;

    use super::*;
    use crate::core::domains::telemetry::Telemetry;

    static ENV_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

    async fn lock_env() -> tokio::sync::MutexGuard<'static, ()> {
        ENV_LOCK.get_or_init(|| Mutex::new(())).lock().await
    }

    fn fixed_time() -> DateTime<Utc> {
        DateTime::<Utc>::from_timestamp(1_700_000_000, 0).expect("valid timestamp")
    }

    #[tokio::test]
    async fn test_infra_storage_select_jsonl_by_default_without_database_url() {
        let _guard = lock_env().await;
        unsafe {
            std::env::remove_var("RUSTPULSE_STORAGE");
            std::env::remove_var("DATABASE_URL");
        }

        let repo = build_telemetry_repository_from_env().await;
        assert!(repo.is_ok());
    }

    #[tokio::test]
    async fn test_infra_postgres_boot_runs_schema_init_idempotently() {
        let _guard = lock_env().await;
        let Some(database_url) = std::env::var("DATABASE_URL").ok() else {
            return;
        };
        unsafe {
            std::env::set_var("RUSTPULSE_STORAGE", "postgres");
            std::env::set_var("DATABASE_URL", &database_url);
        }

        let pool = postgres_db::connect_pool(&database_url).await.unwrap();
        init_postgres_schema(&pool).await.unwrap();
        init_postgres_schema(&pool).await.unwrap();

        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM telemetry")
            .fetch_one(&pool)
            .await
            .unwrap();
        let _ = count;
    }

    #[tokio::test]
    async fn test_infra_postgres_boot_wires_repo_and_roundtrips() {
        let _guard = lock_env().await;
        let Some(database_url) = std::env::var("DATABASE_URL").ok() else {
            return;
        };
        unsafe {
            std::env::set_var("RUSTPULSE_STORAGE", "postgres");
            std::env::set_var("DATABASE_URL", &database_url);
        }

        // ensure isolation for this test
        let pool = postgres_db::connect_pool(&database_url).await.unwrap();
        init_postgres_schema(&pool).await.unwrap();
        sqlx::query("TRUNCATE TABLE telemetry")
            .execute(&pool)
            .await
            .unwrap();

        let repo = build_telemetry_repository_from_env().await.unwrap();

        let telemetry = Telemetry {
            source_id: Uuid::new_v4(),
            server_id: Uuid::new_v4(),
            timestamp: fixed_time(),
            cpu: Some(0.1),
            memory: Some(1.2),
            temperature: Some(3.4),
            extras: json!({"hello":"world"}),
        };

        repo.save(telemetry.clone()).await.unwrap();
        let got = repo.query_all(None).await.unwrap();
        assert_eq!(got.len(), 1);
        assert_eq!(got[0].source_id, telemetry.source_id);
        assert_eq!(got[0].server_id, telemetry.server_id);
        assert_eq!(got[0].timestamp, telemetry.timestamp);
        assert_eq!(got[0].cpu, telemetry.cpu);
        assert_eq!(got[0].memory, telemetry.memory);
        assert_eq!(got[0].temperature, telemetry.temperature);
        assert_eq!(got[0].extras, telemetry.extras);
    }
}
