use sqlx::{postgres::PgPoolOptions, PgPool};

pub async fn connect(db_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(db_url)
        .await
}
