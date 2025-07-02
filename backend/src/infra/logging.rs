
use tracing_subscriber::{fmt, EnvFilter};
use tracing_appender::rolling;

pub fn init(log_json: bool) {
    //Init Logs
    let logfile = rolling::daily("./logs", "info");
    
    //Subscriber
    let subscriber = fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("backend=info,tower_http=info")),
        )
        .with_target(false)
        .with_writer(logfile)
        .with_max_level(tracing::Level::TRACE)
        .with_ansi(false);

    if log_json {
        subscriber.json().init();
    } else {
        subscriber.compact().init();
    }
}

pub fn check_logs() {
    for (key, value) in std::env::vars() {
        println!("{key} = {value}");
    }
}


