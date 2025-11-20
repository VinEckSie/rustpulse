use tracing::instrument;
use tracing_appender::rolling;
use tracing_subscriber::{EnvFilter, fmt};

#[instrument]
pub fn init(log_json: bool) {
    //Init Logs
    let logfile = rolling::daily("./logs", "info");

    //Subscriber
    let layer = fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("backend=info,tower_http=info,backend::handlers::http::telemetry_handler=info,tower_http=info")),
        )
        .with_writer(logfile)
        .with_level(true)
        .with_target(false)
        .with_max_level(tracing::Level::TRACE)
        .with_ansi(false);

    if log_json {
        layer.json().init();
    } else {
        layer.compact().init();
    }

    tracing::info!("rustpulse boot: logger initialised");
}

pub fn check_logs() {
    for (key, value) in std::env::vars() {
        println!("{key} = {value}");
    }
}
