//! Authentication use case implementation.

use crate::core::application::ports::input::auth_login_usecase::AuthLoginUseCase;
use crate::core::application::ports::input::auth_register_usecase::AuthRegisterUseCase;
use crate::core::application::ports::output::auth_repository::{PasswordHasher, TokenIssuer, UserRepo};
use crate::core::domains::auth::{
    AuthError, LoginRequest, LoginResult, RegisterRequest, RegisterResult,
    TokenIssuerError,
};
use std::sync::Arc;
use std::time::{Duration, SystemTime};

fn auth_event(event: &'static str, reason: Option<&'static str>) {
    // Single observability improvement: structured auth events from the Application layer.
    // Do not log PII/secrets/tokens.
    tracing::info!(event, reason, "auth");
}

/// Authentication service implementing the register/login use cases.
pub struct AuthService {
    repo: Arc<dyn UserRepo + Send + Sync>,
    hasher: Arc<dyn PasswordHasher + Send + Sync>,
    issuer: Arc<dyn TokenIssuer + Send + Sync>,
}

impl AuthService {
    /// Creates a new authentication service.
    pub fn new(
        repo: Arc<dyn UserRepo + Send + Sync>,
        hasher: Arc<dyn PasswordHasher + Send + Sync>,
        issuer: Arc<dyn TokenIssuer + Send + Sync>,
    ) -> Self {
        Self { repo, hasher, issuer }
    }
}

#[async_trait::async_trait]
impl AuthRegisterUseCase for AuthService {
    async fn register(&self, req: RegisterRequest) -> Result<RegisterResult, AuthError> {
        let password_hash = self
            .hasher
            .hash(&req.password)
            .map_err(AuthError::PasswordHasher)?;

        let user_id = self
            .repo
            .insert_user(&req.email, &password_hash)
            .map_err(AuthError::UserRepo)?;

        auth_event("auth.register.success", None);
        Ok(RegisterResult { user_id })
    }
}

#[async_trait::async_trait]
impl AuthLoginUseCase for AuthService {
    async fn login(
        &self,
        req: LoginRequest,
        now: SystemTime,
        token_ttl: Duration,
    ) -> Result<LoginResult, AuthError> {
        let user_id = match self
            .repo
            .find_user_id_by_email(&req.email)
            .map_err(AuthError::UserRepo)?
        {
            Some(id) => id,
            None => {
                auth_event("auth.login.failure", Some("invalid_credentials"));
                return Err(AuthError::InvalidCredentials);
            }
        };

        let stored_hash = self
            .repo
            .get_password_hash(&user_id)
            .map_err(AuthError::UserRepo)?;

        let ok = self
            .hasher
            .verify(&req.password, &stored_hash)
            .map_err(AuthError::PasswordHasher)?;

        if !ok {
            auth_event("auth.login.failure", Some("invalid_credentials"));
            return Err(AuthError::InvalidCredentials);
        }

        let expires_at = now
            .checked_add(token_ttl)
            .ok_or(AuthError::TokenIssuer(TokenIssuerError::Failed))?;

        let token = self
            .issuer
            .issue_access_token(&user_id, expires_at)
            .map_err(AuthError::TokenIssuer)?;

        auth_event("auth.login.success", None);
        Ok(LoginResult {
            user_id,
            token,
            expires_at,
        })
    }
}
