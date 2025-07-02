pub mod config;
pub mod infra;
pub mod handlers;
pub mod adapters;
pub mod app;
pub mod core;

use config::Config;
use infra::logging::init as init_logging;
use infra::startup::start_server;

pub async fn start() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let config = Config::from_env().expect("Invalid configuration");

    init_logging(config.log_json);
    // let pool = db::connect(&config.db_url).await?; // when ready

    start_server(config.port).await
}
