//! Application startup and infrastructure wiring.

use crate::adapters::input::http;
use crate::adapters::output::jsonl_repo::JsonlTelemetryRepo;
use crate::adapters::output::postgres_db;
use crate::adapters::output::postgres_telemetry_repo::PostgresTelemetryRepo;
use crate::config::{Config, StorageMode};
use crate::core::application::telemetry::{
    TelemetryIngestCase, TelemetryQueryCase, TelemetryService,
};
use crate::infra::mock_telemetry::MockDataGenerator;
use axum::Router;
use std::path::PathBuf;
use std::sync::Arc;
use tracing::instrument;

#[derive(thiserror::Error, Debug)]
/// Errors that can occur during infrastructure bootstrapping.
pub enum InfraBootError {
    /// Database connection failure.
    #[error(transparent)]
    Db(#[from] postgres_db::DbError),

    /// SQL execution error.
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),

    /// File or network I/O error.
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

/// Creates the `telemetry` table in Postgres if it does not exist.
///
/// # Examples
///
/// ```rust,no_run
/// # async fn demo() -> anyhow::Result<()> {
/// use rustpulse::adapters::output::postgres_db;
/// use rustpulse::infra::startup::init_postgres_schema;
///
/// let database_url = std::env::var("DATABASE_URL")?;
/// let pool = postgres_db::connect_pool(&database_url).await?;
/// init_postgres_schema(&pool).await?;
/// # Ok(())
/// # }
/// ```
pub async fn init_postgres_schema(pool: &sqlx::PgPool) -> Result<(), InfraBootError> {
    tracing::info!("db.schema_init.start");

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

/// Builds a concrete telemetry repository implementation from configuration.
///
/// # Examples
///
/// ```rust,no_run
/// # async fn demo() -> anyhow::Result<()> {
/// use rustpulse::config::{AppEnv, Config, StorageMode};
/// use rustpulse::infra::startup::build_telemetry_repository;
///
/// // JSONL mode does not require a database URL.
/// let cfg = Config {
///     app_env: AppEnv::Test,
///     log_json: false,
///     host: "127.0.0.1".to_string(),
///     port: 3000,
///     storage_mode: StorageMode::Jsonl,
///     database_url: None,
///     rust_log: None,
///     jwt_secret: None,
/// };
///
/// let _repo = build_telemetry_repository(&cfg).await?;
/// # Ok(())
/// # }
/// ```
pub async fn build_telemetry_repository(
    config: &Config,
) -> Result<
    Arc<dyn crate::core::application::telemetry::TelemetryRepository + Send + Sync>,
    InfraBootError,
> {
    match config.storage_mode {
        StorageMode::Jsonl => {
            let temp_file_path: PathBuf =
                PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("metrics_data.jsonl");
            Ok(Arc::new(JsonlTelemetryRepo::new(temp_file_path)))
        }
        StorageMode::Postgres => {
            let database_url = config
                .database_url
                .as_ref()
                .expect("DATABASE_URL validated by Config::from_env");
            let pool = postgres_db::connect_pool(database_url).await?;
            init_postgres_schema(&pool).await?;
            Ok(Arc::new(PostgresTelemetryRepo::new(pool)))
        }
    }
}

#[instrument(level = "info")]
/// Starts the HTTP server and runs until shutdown.
///
/// # Examples
///
/// ```rust,no_run
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use rustpulse::config::Config;
/// use rustpulse::infra::startup::start_server;
///
/// let cfg = Config::from_env()?;
/// start_server(&cfg).await?;
/// # Ok(())
/// # }
/// ```
pub async fn start_server(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let temp_file_path: PathBuf =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("metrics_data.jsonl");

    tracing::info!(?temp_file_path, "Using metrics data file");

    // Create mock data only for the JSONL storage mode.
    if config.storage_mode == StorageMode::Jsonl {
        MockDataGenerator::generate_mock_data(&temp_file_path, 20)?;
    }

    let repo = build_telemetry_repository(config)
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

    let addr = format!("{}:{}", config.host, config.port);
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
    use chrono::{DateTime, Utc};
    use serde_json::json;
    use uuid::Uuid;

    use super::*;
    use crate::core::domains::telemetry::Telemetry;

    fn fixed_time() -> DateTime<Utc> {
        DateTime::<Utc>::from_timestamp(1_700_000_000, 0).expect("valid timestamp")
    }

    #[tokio::test]
    async fn test_infra_storage_select_jsonl_by_default_without_database_url() {
        let config = Config {
            app_env: crate::config::AppEnv::Test,
            log_json: false,
            host: "127.0.0.1".to_string(),
            port: 0,
            storage_mode: StorageMode::Jsonl,
            database_url: None,
            rust_log: None,
            jwt_secret: None,
        };

        let repo = build_telemetry_repository(&config).await;
        assert!(repo.is_ok());
    }

    #[tokio::test]
    async fn test_infra_postgres_boot_runs_schema_init_idempotently() {
        let Some(database_url) = std::env::var("DATABASE_URL").ok() else {
            return;
        };

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
        let Some(database_url) = std::env::var("DATABASE_URL").ok() else {
            return;
        };

        // ensure isolation for this test
        let pool = postgres_db::connect_pool(&database_url).await.unwrap();
        init_postgres_schema(&pool).await.unwrap();
        sqlx::query("TRUNCATE TABLE telemetry")
            .execute(&pool)
            .await
            .unwrap();

        let config = Config {
            app_env: crate::config::AppEnv::Test,
            log_json: false,
            host: "127.0.0.1".to_string(),
            port: 0,
            storage_mode: StorageMode::Postgres,
            database_url: Some(database_url.clone()),
            rust_log: None,
            jwt_secret: None,
        };

        let repo = build_telemetry_repository(&config).await.unwrap();

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
