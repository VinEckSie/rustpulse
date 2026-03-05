use std::str::FromStr;
use std::time::{Duration, Instant};

use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::PgPool;

#[derive(thiserror::Error, Debug)]
pub enum DbError {
    #[error("invalid connection string")]
    InvalidConnectionString { message: String },

    #[error("db connect failed")]
    Connect { source: sqlx::Error },

    #[error("db connect timed out after {timeout_ms}ms")]
    ConnectTimeout { timeout_ms: u64 },

    #[error("db ping failed")]
    Ping { source: sqlx::Error },
}

fn sanitize_database_url(database_url: &str) -> String {
    let no_query = database_url.split('?').next().unwrap_or(database_url);

    let Some(scheme_idx) = no_query.find("://") else {
        return "<invalid>".to_string();
    };
    let after_scheme = scheme_idx + 3;

    let Some(at_idx_rel) = no_query[after_scheme..].find('@') else {
        return no_query.to_string();
    };
    let at_idx = after_scheme + at_idx_rel;

    let mut out = String::with_capacity(no_query.len());
    out.push_str(&no_query[..after_scheme]);
    out.push_str("<redacted>@");
    out.push_str(&no_query[at_idx + 1..]);
    out
}

pub async fn connect_pool(database_url: &str) -> Result<PgPool, DbError> {
    const CONNECT_TIMEOUT: Duration = Duration::from_millis(600);

    let endpoint = sanitize_database_url(database_url);

    let start = Instant::now();
    let opts = PgConnectOptions::from_str(database_url).map_err(|e| {
        DbError::InvalidConnectionString {
            message: e.to_string(),
        }
    })?;

    let connect_fut = PgPoolOptions::new()
        .acquire_timeout(Duration::from_millis(600))
        .max_connections(5)
        .connect_with(opts);

    let pool = match tokio::time::timeout(CONNECT_TIMEOUT, connect_fut).await {
        Ok(Ok(pool)) => pool,
        Ok(Err(e)) => {
            tracing::info!(
                db.endpoint = %endpoint,
                elapsed_ms = start.elapsed().as_millis(),
                error = %e,
                "db.connect failed"
            );
            return Err(DbError::Connect { source: e });
        }
        Err(_) => {
            tracing::info!(
                db.endpoint = %endpoint,
                elapsed_ms = start.elapsed().as_millis(),
                timeout_ms = CONNECT_TIMEOUT.as_millis(),
                "db.connect timed out"
            );
            return Err(DbError::ConnectTimeout {
                timeout_ms: CONNECT_TIMEOUT.as_millis() as u64,
            });
        }
    };

    tracing::info!(
        db.endpoint = %endpoint,
        elapsed_ms = start.elapsed().as_millis(),
        "db.connect ok"
    );
    Ok(pool)
}

pub async fn ping(pool: &PgPool) -> Result<(), DbError> {
    let start = Instant::now();
    let res = sqlx::query_scalar::<_, i32>("SELECT 1")
        .fetch_one(pool)
        .await;

    match res {
        Ok(_) => {
            tracing::info!(elapsed_ms = start.elapsed().as_millis(), "db.ping ok");
            Ok(())
        }
        Err(e) => {
            tracing::info!(
                elapsed_ms = start.elapsed().as_millis(),
                error = %e,
                "db.ping failed"
            );
            Err(DbError::Ping { source: e })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_db_connect_and_ping_ok_when_database_url_set() {
        let database_url = match std::env::var("DATABASE_URL") {
            Ok(v) => v,
            Err(_) => return, // skip: no real Postgres configured
        };

        let pool = connect_pool(&database_url).await.unwrap();
        ping(&pool).await.unwrap();
    }

    #[tokio::test]
    async fn test_db_connect_rejects_obviously_invalid_connection_string() {
        let err = connect_pool("not-a-url").await.unwrap_err();
        assert!(matches!(err, DbError::InvalidConnectionString { .. }));
    }

    #[tokio::test]
    async fn test_db_connect_reports_connect_failure_for_unreachable_host() {
        let url = "postgres://user:pass@127.0.0.1:1/db";
        let err = connect_pool(url).await.unwrap_err();
        assert!(matches!(err, DbError::Connect { .. } | DbError::ConnectTimeout { .. }));
    }
}
