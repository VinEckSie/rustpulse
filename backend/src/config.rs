use crate::errors::ConfigError;
use dotenvy::dotenv;

pub struct Config {
    pub log_json: bool,
    pub port: u16,
}

impl Config {
    #[allow(clippy::manual_inspect)]
    pub fn from_env() -> Result<Self, ConfigError> {
        // Initialize dotenv within the method
        dotenv().ok();

        let port = match std::env::var("PORT") {
            Ok(port_str) => port_str.parse::<u16>().map_err(ConfigError::InvalidPort)?,
            Err(e) => {
                eprintln!(
                    "Warning: {}. Defaulting to 3000.",
                    ConfigError::MissingVar(e)
                );
                3000
            }
        };

        Ok(Self {
            log_json: std::env::var("LOG_JSON").map(|v| v == "1").unwrap_or(false),
            port,
        })
    }
}
