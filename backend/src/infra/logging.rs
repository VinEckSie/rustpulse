use tracing::instrument;
use tracing_appender::rolling;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, fmt};

#[instrument]
pub fn init(log_json: bool) {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        EnvFilter::new(
            "backend=info,tower_http=info,backend::handlers::http::telemetry_handler=info,tower_http=info",
        )
    });

    let tracing_config = crate::infra::tracing::TracingConfig::from_env();
    let tracing_init = crate::infra::tracing::init_tracing(&tracing_config).unwrap_or_else(|e| {
        eprintln!("Warning: tracing disabled due to initialization error: {e}");
        crate::infra::tracing::TracingInit {
            status: crate::infra::tracing::TracingStatus::Disabled {
                reason: crate::infra::tracing::DisabledReason::BackendUnavailable,
            },
            otel_layer: None,
            exporter_meta: None,
        }
    });

    let crate::infra::tracing::TracingInit {
        status,
        otel_layer,
        exporter_meta,
    } = tracing_init;

    let subscriber_installed = if log_json {
        match otel_layer {
            Some(otel_layer) => {
                let logfile = rolling::daily("./logs", "info");
                let fmt_layer = fmt::layer()
                    .json()
                    .with_writer(logfile)
                    .with_level(true)
                    .with_target(false)
                    .with_ansi(false);

                tracing_subscriber::registry()
                    .with(otel_layer)
                    .with(env_filter.clone())
                    .with(fmt_layer)
                    .try_init()
                    .is_ok()
            }
            None => {
                let logfile = rolling::daily("./logs", "info");
                let fmt_layer = fmt::layer()
                    .json()
                    .with_writer(logfile)
                    .with_level(true)
                    .with_target(false)
                    .with_ansi(false);

                tracing_subscriber::registry()
                    .with(env_filter.clone())
                    .with(fmt_layer)
                    .try_init()
                    .is_ok()
            }
        }
    } else {
        match otel_layer {
            Some(otel_layer) => {
                let logfile = rolling::daily("./logs", "info");
                let fmt_layer = fmt::layer()
                    .event_format(fmt::format().compact())
                    .with_writer(logfile)
                    .with_level(true)
                    .with_target(false)
                    .with_ansi(false);

                tracing_subscriber::registry()
                    .with(otel_layer)
                    .with(env_filter)
                    .with(fmt_layer)
                    .try_init()
                    .is_ok()
            }
            None => {
                let logfile = rolling::daily("./logs", "info");
                let fmt_layer = fmt::layer()
                    .event_format(fmt::format().compact())
                    .with_writer(logfile)
                    .with_level(true)
                    .with_target(false)
                    .with_ansi(false);

                tracing_subscriber::registry()
                    .with(env_filter)
                    .with(fmt_layer)
                    .try_init()
                    .is_ok()
            }
        }
    };

    tracing::info!("rustpulse boot: logger initialised");

    //emits exactly one “exporter initialized” log + a minimal startup span when OTEL_EXPORTER_OTLP_ENDPOINT is set.
    if subscriber_installed {
        if let (crate::infra::tracing::TracingStatus::Active, Some(meta)) = (status, exporter_meta)
        {
            crate::infra::tracing::emit_exporter_initialized_log(&meta);
            crate::infra::tracing::emit_startup_span();
        }
    }
}

pub fn check_logs() {
    std::env::vars().for_each(|(key, value)| println!("{key} = {value}"));
}
