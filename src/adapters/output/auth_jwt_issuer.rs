//! JWT issuance adapter for authentication.

use crate::core::application::ports::output::auth_repository::TokenIssuer;
use crate::core::domains::auth::{JwtToken, TokenIssuerError, UserId};
use jsonwebtoken::{Algorithm, EncodingKey, Header};
use serde::Serialize;
use std::time::{Instant, SystemTime, UNIX_EPOCH};
use tracing::field;

#[derive(Debug, Serialize)]
struct AccessTokenClaims {
    sub: String,
    exp: u64,
}

fn unix_seconds(time: SystemTime) -> Result<u64, TokenIssuerError> {
    let duration_since_epoch = time
        .duration_since(UNIX_EPOCH)
        .map_err(|_| TokenIssuerError::Failed)?;
    Ok(duration_since_epoch.as_secs())
}

/// HS256 JWT issuer adapter.
#[derive(Debug, Clone)]
pub struct Hs256JwtIssuer {
    secret: Vec<u8>,
}

impl Hs256JwtIssuer {
    /// Creates a new HS256 JWT issuer from a shared secret.
    pub fn new(secret: impl AsRef<[u8]>) -> Self {
        Self {
            secret: secret.as_ref().to_vec(),
        }
    }
}

impl TokenIssuer for Hs256JwtIssuer {
    fn issue_access_token(
        &self,
        user_id: &UserId,
        expires_at: SystemTime,
    ) -> Result<JwtToken, TokenIssuerError> {
        let span = tracing::info_span!(
            "auth.jwt.issue",
            outcome = field::Empty,
            elapsed_ms = field::Empty
        );
        let _guard = span.enter();
        let started = Instant::now();

        let exp = unix_seconds(expires_at)?;

        let claims = AccessTokenClaims {
            sub: user_id.0.clone(),
            exp,
        };

        let mut header = Header::new(Algorithm::HS256);
        header.typ = Some("JWT".to_string());

        let result =
            jsonwebtoken::encode(&header, &claims, &EncodingKey::from_secret(&self.secret))
                .map(JwtToken)
                .map_err(|_| TokenIssuerError::Failed);

        span.record("elapsed_ms", started.elapsed().as_millis() as u64);
        match &result {
            Ok(_) => span.record("outcome", "ok"),
            Err(_) => span.record("outcome", "error"),
        };

        result
    }
}
