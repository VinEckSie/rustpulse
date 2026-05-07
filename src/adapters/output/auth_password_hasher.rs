//! Password hashing adapter for authentication.

use crate::core::application::ports::output::auth_repository::PasswordHasher;
use crate::core::domains::auth::{Password, PasswordHash, PasswordHasherError};
use argon2::password_hash::SaltString;
use argon2::password_hash::rand_core::OsRng;
use argon2::{Argon2, PasswordHash as ParsedHash, PasswordHasher as _, PasswordVerifier as _};
use std::time::Instant;
use tracing::field;

/// Argon2-based password hasher adapter.
#[derive(Debug, Default, Clone, Copy)]
pub struct Argon2PasswordHasher;

impl Argon2PasswordHasher {
    /// Creates a new hasher adapter.
    pub fn new() -> Self {
        Self
    }
}

impl PasswordHasher for Argon2PasswordHasher {
    fn hash(&self, password: &Password) -> Result<PasswordHash, PasswordHasherError> {
        let span = tracing::info_span!(
            "auth.crypto.hash",
            outcome = field::Empty,
            elapsed_ms = field::Empty
        );
        let _guard = span.enter();
        let started = Instant::now();

        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        let result = argon2
            .hash_password(password.as_str().as_bytes(), &salt)
            .map(|hash| PasswordHash(hash.to_string()))
            .map_err(|_| PasswordHasherError::Failed);

        span.record("elapsed_ms", started.elapsed().as_millis() as u64);
        match &result {
            Ok(_) => span.record("outcome", "ok"),
            Err(_) => span.record("outcome", "error"),
        };

        result
    }

    fn verify(
        &self,
        password: &Password,
        hash: &PasswordHash,
    ) -> Result<bool, PasswordHasherError> {
        // No extra observability beyond the single improvement introduced by `auth.crypto.hash` span.
        let parsed = ParsedHash::new(hash.0.as_str()).map_err(|_| PasswordHasherError::Failed)?;
        let argon2 = Argon2::default();
        Ok(argon2
            .verify_password(password.as_str().as_bytes(), &parsed)
            .is_ok())
    }
}
