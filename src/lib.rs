//! Backend crate.
//!
//! This crate contains the server entrypoints, configuration, and infrastructure wiring.
//!
//! # Examples
//!
//! Start the HTTP server (typically from the `rustpulse` binary):
//!
//! ```rust,no_run
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     rustpulse::start().await
//! }
//! ```

#![deny(missing_docs)]

/// Input/output adapters (HTTP handlers, presenters, etc.).
pub mod adapters;
/// Environment and runtime configuration.
pub mod config;
/// Core domain and application logic.
pub mod core;

mod errors;
// pub mod features;
/// Infrastructure concerns (startup, logging, tracing, telemetry).
pub mod infra;

use config::Config;
use infra::logging::init as init_logging;
use infra::startup::start_server;

/// Starts the HTTP server with configuration loaded from the environment.
///
/// # Examples
///
/// ```rust,no_run
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// rustpulse::start().await
/// # }
/// ```
pub async fn start() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::from_env()?;

    init_logging(config.log_json);

    start_server(&config).await
}
