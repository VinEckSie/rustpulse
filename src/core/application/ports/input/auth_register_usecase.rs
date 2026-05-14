//! Input port for user registration.

use crate::core::domains::auth::{AuthError, RegisterRequest, RegisterResult};

#[async_trait::async_trait]
/// Use case for registering users.
pub trait AuthRegisterUseCase: Send + Sync {
    /// Registers a new user.
    async fn register(&self, req: RegisterRequest) -> Result<RegisterResult, AuthError>;
}

