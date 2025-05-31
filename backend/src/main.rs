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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // struct Config {
    //     database_url: String,
    //     port: u16,
    // }
    // 
    // impl Config {
    //     fn from_env() -> Self {
    //         // read from env vars here
    //     }
    // }

    //config test example
    // #[test]
    // fn missing_database_url_returns_error() {
    //     std::env::remove_var("DATABASE_URL");
    //     assert!(Config::from_env().is_err());
    // }

    //DB tests example
    // #[tokio::test]
    // async fn store_event_returns_201() {
    //     let app = spawn_app();
    //     let payload = json!({"title": "RustConf", "date": "2025-09-10"});
    // 
    //     let response = app.post("/events").json(&payload).send().await.unwrap();
    // 
    //     assert_eq!(response.status(), 201);
    // }


    //let config = Config::from_env().expect("Failed to load config");
    // pass it to Axum or DB setup
    
    
    //Load envs
    dotenvy::dotenv().ok();
    // 
    // for (key, value) in std::env::vars() {
    //     println!("{key} = {value}");
    // }

    //Init Logs
    let logfile = rolling::daily("./logs", "info");
    let use_json = std::env::var("LOG_JSON")
        .map(|v| v == "1")
        .unwrap_or_else(|_| false);
    
    let subscriber = fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("backend=info,tower_http=info")),
        )
        .with_target(false)
        .with_writer(logfile)
        .with_max_level(tracing::Level::TRACE)
        .with_ansi(false);
    
    if use_json {
        subscriber.json().init();
    } else {
        subscriber.compact().init();
    }

    //Connect DB
    //println!("LOG_JSON from env: {:?}", std::env::var("DATABASE_URL").expect("Missing DB Connection"));
    let db_connection = std::env::var("DATABASE_URL").expect("Missing DB Connection");

    println!("✅ Connected to PostgreSQL at {}", db_connection);

    // Create a connection pool
    //  for MySQL/MariaDB, use MySqlPoolOptions::new()
    //  for SQLite, use SqlitePoolOptions::new()
    //  etc.
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(db_connection.as_str()).await?;
    
    //Build Router
    let app = create_router().layer(
        TraceLayer::new_for_http()
            .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
            .on_response(DefaultOnResponse::new().level(Level::INFO))
            .on_request(DefaultOnRequest::new().level(Level::INFO)),
    );
    
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    
    println!("listening on {}", listener.local_addr().unwrap());
    
    //Start Server
    //Listener handles network → Router handles logic
    axum::serve(listener, app).await.unwrap();
    
    Ok(())
}
