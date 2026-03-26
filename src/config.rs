//! Backend configuration loading and validation.
//!
//! # Examples
//!
//! Build a configuration from explicit values (useful for tests):
//!
//! ```rust
//! # fn main() -> anyhow::Result<()> {
//! use rustpulse::config::{Config, ConfigInput};
//!
//! let config = Config::from_input(ConfigInput {
//!     app_env: Some("test".to_string()),
//!     port: Some("3000".to_string()),
//!     ..Default::default()
//! })?;
//!
//! assert_eq!(config.host, "127.0.0.1");
//! # Ok(())
//! # }
//! ```

use crate::errors::ConfigError;
use dotenvy::dotenv;
use std::env;
use std::str::FromStr;
use tracing::instrument;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Application environment (dev/test/staging/prod).
pub enum AppEnv {
    /// Local development environment.
    Dev,
    /// Test environment.
    Test,
    /// Staging environment.
    Staging,
    /// Production environment.
    Prod,
}

impl AppEnv {
    /// Returns `true` when the environment is [`AppEnv::Dev`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustpulse::config::AppEnv;
    ///
    /// assert!(AppEnv::Dev.is_dev());
    /// assert!(!AppEnv::Prod.is_dev());
    /// ```
    pub fn is_dev(self) -> bool {
        matches!(self, Self::Dev)
    }

    /// Returns `true` when the environment is [`AppEnv::Test`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustpulse::config::AppEnv;
    ///
    /// assert!(AppEnv::Test.is_test());
    /// assert!(!AppEnv::Dev.is_test());
    /// ```
    pub fn is_test(self) -> bool {
        matches!(self, Self::Test)
    }

    /// Returns `true` when the environment is [`AppEnv::Staging`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustpulse::config::AppEnv;
    ///
    /// assert!(AppEnv::Staging.is_staging());
    /// assert!(!AppEnv::Prod.is_staging());
    /// ```
    pub fn is_staging(self) -> bool {
        matches!(self, Self::Staging)
    }

    /// Returns `true` when the environment is [`AppEnv::Prod`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustpulse::config::AppEnv;
    ///
    /// assert!(AppEnv::Prod.is_prod());
    /// assert!(!AppEnv::Dev.is_prod());
    /// ```
    pub fn is_prod(self) -> bool {
        matches!(self, Self::Prod)
    }
}

impl FromStr for AppEnv {
    type Err = ConfigError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_ascii_lowercase().as_str() {
            "dev" => Ok(Self::Dev),
            "test" => Ok(Self::Test),
            "staging" => Ok(Self::Staging),
            "prod" | "production" => Ok(Self::Prod),
            other => Err(ConfigError::InvalidAppEnv(other.to_string())),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Storage backend selection.
pub enum StorageMode {
    /// Persist telemetry in a JSONL file.
    Jsonl,
    /// Persist telemetry in Postgres.
    Postgres,
}

impl FromStr for StorageMode {
    type Err = ConfigError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_ascii_lowercase().as_str() {
            "" | "jsonl" => Ok(Self::Jsonl),
            "postgres" | "pg" => Ok(Self::Postgres),
            other => Err(ConfigError::InvalidStorageMode(other.to_string())),
        }
    }
}

#[derive(Debug, Clone, Default)]
/// Raw configuration values as read from the environment.
pub struct ConfigInput {
    /// Raw `APP_ENV` value.
    pub app_env: Option<String>,
    /// Raw `HOST` value.
    pub host: Option<String>,
    /// Raw `PORT` value.
    pub port: Option<String>,
    /// Raw `LOG_JSON` value.
    pub log_json: Option<String>,
    /// Raw `RUSTPULSE_STORAGE` value.
    pub storage_mode: Option<String>,
    /// Raw `DATABASE_URL` value.
    pub database_url: Option<String>,
    /// Raw `RUST_LOG` value.
    pub rust_log: Option<String>,
    /// Raw `JWT_SECRET` value.
    pub jwt_secret: Option<String>,
    /// Raw `RUSTPULSE_ALLOW_LOCAL_BIND` value.
    pub allow_local_bind: Option<String>,
    /// Raw `RUSTPULSE_ALLOW_LOCAL_DB` value.
    pub allow_local_db: Option<String>,
}

impl ConfigInput {
    /// Reads environment variables from the running process into a [`ConfigInput`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustpulse::config::ConfigInput;
    ///
    /// let _input = ConfigInput::from_process_env();
    /// ```
    pub fn from_process_env() -> Self {
        Self {
            app_env: env::var("APP_ENV").ok(),
            host: env::var("HOST").ok(),
            port: env::var("PORT").ok(),
            log_json: env::var("LOG_JSON").ok(),
            storage_mode: env::var("RUSTPULSE_STORAGE").ok(),
            database_url: env::var("DATABASE_URL").ok(),
            rust_log: env::var("RUST_LOG").ok(),
            jwt_secret: env::var("JWT_SECRET").ok(),
            allow_local_bind: env::var("RUSTPULSE_ALLOW_LOCAL_BIND").ok(),
            allow_local_db: env::var("RUSTPULSE_ALLOW_LOCAL_DB").ok(),
        }
    }
}

#[derive(Debug, Clone)]
/// Parsed and validated runtime configuration.
pub struct Config {
    /// Target application environment.
    pub app_env: AppEnv,
    /// Whether logs should be emitted as JSON.
    pub log_json: bool,
    /// Host interface to bind the HTTP server to.
    pub host: String,
    /// TCP port to bind the HTTP server to.
    pub port: u16,
    /// Storage backend selection.
    pub storage_mode: StorageMode,
    /// Database connection string (required for Postgres storage).
    pub database_url: Option<String>,
    /// Optional `tracing_subscriber` filter string or log level.
    pub rust_log: Option<String>,
    /// Optional JWT secret (required in production).
    pub jwt_secret: Option<String>,
}

impl Config {
    #[allow(clippy::manual_inspect)]
    #[instrument]
    /// Loads configuration from the environment (and `.env` when applicable).
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn main() -> anyhow::Result<()> {
    /// use rustpulse::config::Config;
    ///
    /// let _config = Config::from_env()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_env() -> Result<Self, ConfigError> {
        Self::load_dotenv_for_local_dev();
        Self::from_input(ConfigInput::from_process_env())
    }

    /// Parses and validates configuration from a provided [`ConfigInput`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn main() -> anyhow::Result<()> {
    /// use rustpulse::config::{Config, ConfigInput};
    ///
    /// let config = Config::from_input(ConfigInput {
    ///     app_env: Some("dev".to_string()),
    ///     port: Some("3000".to_string()),
    ///     ..Default::default()
    /// })?;
    ///
    /// assert_eq!(config.port, 3000);
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_input(input: ConfigInput) -> Result<Self, ConfigError> {
        let app_env = input
            .app_env
            .unwrap_or_else(|| "dev".to_string())
            .parse::<AppEnv>()?;

        let host = match input.host {
            Some(v) => v,
            None if app_env.is_prod() || app_env.is_staging() => "0.0.0.0".to_string(),
            None => "127.0.0.1".to_string(),
        };

        let port_raw = match input.port {
            Some(v) => v,
            None if app_env.is_dev() || app_env.is_test() => "3000".to_string(),
            None => return Err(ConfigError::MissingVar("PORT")),
        };
        let port = port_raw
            .parse::<u16>()
            .map_err(|_| ConfigError::InvalidPort(port_raw.clone()))?;

        let storage_mode = input
            .storage_mode
            .unwrap_or_default()
            .parse::<StorageMode>()?;

        let database_url = match storage_mode {
            StorageMode::Jsonl => input.database_url,
            StorageMode::Postgres => Some(
                input
                    .database_url
                    .ok_or(ConfigError::MissingVar("DATABASE_URL"))?,
            ),
        };

        let log_json = input
            .log_json
            .map(|v| matches!(v.as_str(), "1" | "true" | "TRUE"))
            .unwrap_or(false);

        let rust_log = match input.rust_log {
            None => None,
            Some(v) => {
                let level = v.to_ascii_lowercase();
                match level.as_str() {
                    "debug" | "info" | "warn" => Some(level),
                    _ => {
                        return Err(ConfigError::Validation(
                            "RUST_LOG must be one of: debug, info, warn".to_string(),
                        ));
                    }
                }
            }
        };

        let jwt_secret = input.jwt_secret;

        let allow_local_bind = input.allow_local_bind.as_deref() == Some("1");
        let allow_local_db = input.allow_local_db.as_deref() == Some("1");

        let config = Self {
            app_env,
            log_json,
            host,
            port,
            storage_mode,
            database_url,
            rust_log,
            jwt_secret,
        };

        Self::validate(&config, allow_local_bind, allow_local_db)?;
        Ok(config)
    }

    fn load_dotenv_for_local_dev() {
        let app_env_was_set = env::var("APP_ENV").ok();

        if app_env_was_set.is_none() {
            let _ = dotenv();
        }

        // Re-read after attempting dotenv to allow `.env` to define APP_ENV.
        let app_env = env::var("APP_ENV").unwrap_or_else(|_| "dev".to_string());
        if app_env_was_set.is_some() && app_env.eq_ignore_ascii_case("dev") {
            let _ = dotenv();
        }
    }

    fn validate(
        config: &Self,
        allow_local_bind: bool,
        allow_local_db: bool,
    ) -> Result<(), ConfigError> {
        if config.port == 0 {
            return Err(ConfigError::InvalidPort("0".to_string()));
        }

        if config.app_env.is_prod() {
            if config.database_url.as_ref().is_none() {
                return Err(ConfigError::MissingVar("DATABASE_URL"));
            }

            let jwt_secret = config
                .jwt_secret
                .as_ref()
                .ok_or(ConfigError::MissingVar("JWT_SECRET"))?;
            if jwt_secret.len() < 32 {
                return Err(ConfigError::Validation(
                    "JWT_SECRET must be at least 32 characters in prod".to_string(),
                ));
            }

            if matches!(config.host.as_str(), "127.0.0.1" | "localhost") && !allow_local_bind {
                return Err(ConfigError::Validation(
                    "HOST must not be localhost in prod (set RUSTPULSE_ALLOW_LOCAL_BIND=1 to override)"
                        .to_string(),
                ));
            }

            if let Some(rust_log) = &config.rust_log
                && rust_log == "debug"
            {
                return Err(ConfigError::Validation(
                    "RUST_LOG must not use debug in prod".to_string(),
                ));
            }

            if let Some(database_url) = &config.database_url {
                if !(database_url.starts_with("postgres://")
                    || database_url.starts_with("postgresql://"))
                {
                    return Err(ConfigError::InvalidDatabaseUrl(database_url.clone()));
                }

                let looks_localhost =
                    database_url.contains("localhost") || database_url.contains("127.0.0.1");
                if looks_localhost && !allow_local_db {
                    return Err(ConfigError::Validation(
                        "DATABASE_URL must not point to localhost in prod (set RUSTPULSE_ALLOW_LOCAL_DB=1 to override)"
                            .to_string(),
                    ));
                }
            }
        }

        Ok(())
    }
}
