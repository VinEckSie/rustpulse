use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Environment variable error: {0}")]
    MissingVar(#[from] std::env::VarError),
    #[error("Invalid port: {0}")]
    InvalidPort(#[from] std::num::ParseIntError),
    #[error("Invalid database URL format: {details}")]
    InvalidDatabaseUrl { details: String },
}
