mod routes;

use crate::routes::{create_router, health::health_check};
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::Level;
use tracing_appender::rolling;
use tracing_subscriber::{EnvFilter, fmt};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    // println!("LOG_JSON from env: {:?}", std::env::var("LOG_JSON"));
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

    //Listener handles network → Router handles logic
    axum::serve(listener, app).await.unwrap();
}
