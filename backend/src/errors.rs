use std::io;
use std::path::PathBuf;
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

#[derive(Debug, Error)]
pub enum DataError {
    #[error("Failed to open file at {path}: {source}")]
    FileOpen {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("Failed to parse file")]
    Serde(serde_json::Error),
    #[error("IO Error")]
    Io(std::io::Error), //enable to wrap general IO errors
}
