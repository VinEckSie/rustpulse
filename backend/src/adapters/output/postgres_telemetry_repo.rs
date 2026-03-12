use std::time::Instant;

use sqlx::PgPool;
use sqlx::Row;
use uuid::Uuid;

use crate::core::application::telemetry::TelemetryRepository;
use crate::core::domains::telemetry::Telemetry;

#[derive(thiserror::Error, Debug)]
pub enum PostgresRepoError {
    #[error("invalid source id filter")]
    InvalidSourceId { source: uuid::Error },

    #[error("database error")]
    Sqlx { source: sqlx::Error },
}

pub struct PostgresTelemetryRepo {
    pool: PgPool,
}

impl PostgresTelemetryRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl TelemetryRepository for PostgresTelemetryRepo {
    async fn save(&self, telemetry: Telemetry) -> anyhow::Result<()> {
        let start = Instant::now();
        let res = sqlx::query(
            r#"
INSERT INTO telemetry (source_id, server_id, timestamp, cpu, memory, temperature, extras)
VALUES ($1, $2, $3, $4, $5, $6, $7)
"#,
        )
        .bind(telemetry.source_id)
        .bind(telemetry.server_id)
        .bind(telemetry.timestamp)
        .bind(telemetry.cpu)
        .bind(telemetry.memory)
        .bind(telemetry.temperature)
        .bind(telemetry.extras)
        .execute(&self.pool)
        .await;

        match res {
            Ok(done) => {
                tracing::info!(
                    elapsed_ms = start.elapsed().as_millis(),
                    row_count = done.rows_affected(),
                    "repo.telemetry.save"
                );
                Ok(())
            }
            Err(e) => {
                tracing::info!(
                    elapsed_ms = start.elapsed().as_millis(),
                    error = %e,
                    "repo.telemetry.save"
                );
                Err(anyhow::Error::new(PostgresRepoError::Sqlx { source: e }))
            }
        }
    }

    async fn query_all(&self, node_id: Option<String>) -> anyhow::Result<Vec<Telemetry>> {
        let start = Instant::now();

        let filter: Option<Uuid> = match node_id {
            Some(s) => Some(Uuid::parse_str(&s).map_err(|e| {
                anyhow::Error::new(PostgresRepoError::InvalidSourceId { source: e })
            })?),
            None => None,
        };

        let rows = match filter {
            None => {
                sqlx::query(
                    r#"
SELECT source_id, server_id, timestamp, cpu, memory, temperature, extras
FROM telemetry
ORDER BY timestamp ASC
"#,
                )
                .fetch_all(&self.pool)
                .await
            }
            Some(source_id) => {
                sqlx::query(
                    r#"
SELECT source_id, server_id, timestamp, cpu, memory, temperature, extras
FROM telemetry
WHERE source_id = $1
ORDER BY timestamp ASC
"#,
                )
                .bind(source_id)
                .fetch_all(&self.pool)
                .await
            }
        };

        match rows {
            Ok(rows) => {
                let mut out = Vec::with_capacity(rows.len());
                for row in rows {
                    let telemetry = Telemetry {
                        source_id: row.try_get("source_id")?,
                        server_id: row.try_get("server_id")?,
                        timestamp: row.try_get("timestamp")?,
                        cpu: row.try_get("cpu")?,
                        memory: row.try_get("memory")?,
                        temperature: row.try_get("temperature")?,
                        extras: row.try_get("extras")?,
                    };
                    out.push(telemetry);
                }

                tracing::info!(
                    elapsed_ms = start.elapsed().as_millis(),
                    row_count = out.len(),
                    "repo.telemetry.query_all"
                );
                Ok(out)
            }
            Err(e) => {
                tracing::info!(
                    elapsed_ms = start.elapsed().as_millis(),
                    error = %e,
                    "repo.telemetry.query_all"
                );
                Err(anyhow::Error::new(PostgresRepoError::Sqlx { source: e }))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::OnceLock;

    use chrono::{DateTime, Utc};
    use serde_json::json;
    use tokio::sync::Mutex;

    use super::*;
    use crate::adapters::output::postgres_db;

    static TEST_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

    fn database_url() -> Option<String> {
        std::env::var("DATABASE_URL").ok()
    }

    async fn lock() -> tokio::sync::MutexGuard<'static, ()> {
        TEST_LOCK.get_or_init(|| Mutex::new(())).lock().await
    }

    async fn ensure_schema(pool: &PgPool) -> anyhow::Result<()> {
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

        sqlx::query("TRUNCATE TABLE telemetry")
            .execute(pool)
            .await?;
        Ok(())
    }

    fn fixed_time() -> DateTime<Utc> {
        DateTime::<Utc>::from_timestamp(1_700_000_000, 0).expect("valid timestamp")
    }

    #[tokio::test]
    async fn test_postgres_repo_save_then_query_all_returns_inserted() {
        let Some(database_url) = database_url() else {
            return;
        };

        let _guard = lock().await;
        let pool = postgres_db::connect_pool(&database_url).await.unwrap();
        ensure_schema(&pool).await.unwrap();

        let repo = PostgresTelemetryRepo::new(pool);
        let telemetry = Telemetry {
            source_id: Uuid::new_v4(),
            server_id: Uuid::new_v4(),
            timestamp: fixed_time(),
            cpu: Some(0.5),
            memory: Some(42.0),
            temperature: None,
            extras: json!({"k":"v"}),
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

    #[tokio::test]
    async fn test_postgres_repo_query_all_filters_by_source_id() {
        let Some(database_url) = database_url() else {
            return;
        };

        let _guard = lock().await;
        let pool = postgres_db::connect_pool(&database_url).await.unwrap();
        ensure_schema(&pool).await.unwrap();

        let repo = PostgresTelemetryRepo::new(pool);
        let source_a = Uuid::new_v4();
        let source_b = Uuid::new_v4();

        let t1 = Telemetry {
            source_id: source_a,
            server_id: Uuid::new_v4(),
            timestamp: fixed_time(),
            cpu: Some(1.0),
            memory: None,
            temperature: None,
            extras: json!({"a":1}),
        };
        let t2 = Telemetry {
            source_id: source_b,
            server_id: Uuid::new_v4(),
            timestamp: fixed_time(),
            cpu: Some(2.0),
            memory: None,
            temperature: None,
            extras: json!({"b":2}),
        };

        repo.save(t1.clone()).await.unwrap();
        repo.save(t2).await.unwrap();

        let got = repo.query_all(Some(source_a.to_string())).await.unwrap();
        assert_eq!(got.len(), 1);
        assert_eq!(got[0].source_id, t1.source_id);
        assert_eq!(got[0].extras, t1.extras);
    }

    #[tokio::test]
    async fn test_postgres_repo_query_all_rejects_invalid_uuid_filter() {
        let Some(database_url) = database_url() else {
            return;
        };

        let _guard = lock().await;
        let pool = postgres_db::connect_pool(&database_url).await.unwrap();
        ensure_schema(&pool).await.unwrap();

        let repo = PostgresTelemetryRepo::new(pool);
        let err = repo
            .query_all(Some("not-a-uuid".to_string()))
            .await
            .unwrap_err();
        assert!(
            err.downcast_ref::<PostgresRepoError>()
                .is_some_and(|e| matches!(e, PostgresRepoError::InvalidSourceId { .. }))
        );
    }
}
