pub mod adapters;
pub mod app;
pub mod config;
pub mod core;
mod errors;
pub mod handlers;
pub mod infra;

use config::Config;
use infra::logging::init as init_logging;
use infra::startup::start_server;

pub async fn start() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::from_env().expect("Invalid configuration");

    init_logging(config.log_json);

    start_server(config.port).await
}
