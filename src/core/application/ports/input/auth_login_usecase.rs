//! Input port for user login.

use crate::core::domains::auth::{AuthError, LoginRequest, LoginResult};

#[async_trait::async_trait]
/// Use case for authenticating users.
pub trait AuthLoginUseCase: Send + Sync {
    /// Authenticates a user and returns an access token.
    async fn login(
        &self,
        req: LoginRequest,
        now: std::time::SystemTime,
        token_ttl: std::time::Duration,
    ) -> Result<LoginResult, AuthError>;
}
