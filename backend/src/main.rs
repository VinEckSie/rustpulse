<<<<<<< HEAD
// Copyright (c) 2025 Vincent Eckert Sierota
// Licensed under the MIT License

mod domain;
mod routes;

use sqlx::postgres::PgPoolOptions;
use crate::routes::create_router;
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::Level;
use tracing_appender::rolling;
use tracing_subscriber::{EnvFilter, fmt};

struct MySqlPool(&'static str);

impl MySqlPool {
    async fn connect(p0: &str) {
        todo!()
    }
}
=======
// #![deny(missing_docs)]
use backend::start;
>>>>>>> 0c4557a (feat(DDD): clean structure)

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    start().await
}
