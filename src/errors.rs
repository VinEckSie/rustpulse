//! Error types used across the backend.

use std::io;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
/// Configuration parsing and validation errors.
pub enum ConfigError {
    /// A required environment variable was not set.
    #[error("missing environment variable: {0}")]
    MissingVar(&'static str),

    /// `APP_ENV` was present but not a known value.
    #[error("invalid APP_ENV value: {0}")]
    InvalidAppEnv(String),

    /// `PORT` was present but could not be parsed.
    #[error("invalid PORT value: {0}")]
    InvalidPort(String),

    /// `RUSTPULSE_STORAGE` was present but not a known value.
    #[error("invalid RUSTPULSE_STORAGE value: {0}")]
    InvalidStorageMode(String),

    /// `DATABASE_URL` was present but failed validation.
    #[error("invalid DATABASE_URL value: {0}")]
    InvalidDatabaseUrl(String),

    /// A higher-level validation rule failed.
    #[error("config validation error: {0}")]
    Validation(String),
}

#[derive(Debug, Error)]
/// Errors produced when generating or reading mock telemetry data.
pub enum DataError {
    /// Failed to open the requested file.
    #[error("Failed to open file at {path}: {source}")]
    FileOpen {
        /// Path that was attempted.
        path: PathBuf,
        #[source]
        /// Underlying OS error.
        source: io::Error,
    },
    /// Failed to parse JSON.
    #[error("Failed to parse file")]
    Serde(serde_json::Error),
    /// An I/O operation failed.
    #[error("IO Error")]
    Io(std::io::Error), //enable to wrap general IO errors
}
