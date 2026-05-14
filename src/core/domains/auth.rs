//! Authentication domain types and ports.
//!
//! This module is IO-free: it defines request/response types, validation rules,
//! and error types shared by port traits.

use std::time::SystemTime;

/// Domain-level authentication errors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthError {
    /// The provided email is invalid.
    InvalidEmail,
    /// The provided password is shorter than the minimum length.
    PasswordTooShort {
        /// Minimum password length.
        min: usize,
    },
    /// The provided credentials are invalid.
    ///
    /// This error is intentionally ambiguous (it does not reveal whether the email exists).
    InvalidCredentials,
    /// User repository port error.
    UserRepo(UserRepoError),
    /// Password hashing port error.
    PasswordHasher(PasswordHasherError),
    /// Token issuance port error.
    TokenIssuer(TokenIssuerError),
}

/// Errors returned by the `UserRepo` port.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UserRepoError {
    /// A unique constraint or conflicting record prevented the operation.
    Conflict,
    /// The requested entity was not found.
    NotFound,
    /// The underlying storage is unavailable.
    Unavailable,
    /// A domain-agnostic error message.
    Other(String),
}

/// Errors returned by the `PasswordHasher` port.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PasswordHasherError {
    /// The hashing or verification operation failed.
    Failed,
    /// A domain-agnostic error message.
    Other(String),
}

/// Errors returned by the `TokenIssuer` port.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenIssuerError {
    /// The token could not be issued.
    Failed,
    /// A domain-agnostic error message.
    Other(String),
}

/// Opaque user identifier for authentication flows.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserId(pub String);

/// Normalized email address.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Email(String);

impl Email {
    /// Parses and normalizes an email address.
    pub fn parse(raw: impl AsRef<str>) -> Result<Self, AuthError> {
        let candidate = raw.as_ref().trim().to_ascii_lowercase();
        if !is_valid_email(&candidate) {
            return Err(AuthError::InvalidEmail);
        }

        // Observability improvement (exactly one): emit a structured debug event with the domain.
        // Avoid logging the full email to reduce PII exposure.
        if let Some(domain) = candidate.split('@').nth(1) {
            tracing::debug!(email_domain = domain, "auth.email.parsed");
        }

        Ok(Self(candidate))
    }

    /// Returns the email as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Password with basic validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Password(String);

impl Password {
    /// Minimum allowed password length.
    pub const MIN_LEN: usize = 8;

    /// Parses a password applying minimum length validation.
    pub fn parse(raw: impl AsRef<str>) -> Result<Self, AuthError> {
        let candidate = raw.as_ref().to_string();
        if candidate.chars().count() < Self::MIN_LEN {
            return Err(AuthError::PasswordTooShort { min: Self::MIN_LEN });
        }
        Ok(Self(candidate))
    }

    /// Returns the password as a string slice.
    ///
    /// Callers should avoid logging this value.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Opaque password hash.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PasswordHash(pub String);

/// Opaque JWT access token.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JwtToken(pub String);

/// Registration request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegisterRequest {
    /// User email.
    pub email: Email,
    /// User password.
    pub password: Password,
}

impl RegisterRequest {
    /// Builds a registration request applying domain validation.
    pub fn try_new(email: impl AsRef<str>, password: impl AsRef<str>) -> Result<Self, AuthError> {
        Ok(Self {
            email: Email::parse(email)?,
            password: Password::parse(password)?,
        })
    }
}

/// Login request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoginRequest {
    /// User email.
    pub email: Email,
    /// User password.
    pub password: Password,
}

impl LoginRequest {
    /// Builds a login request applying domain validation.
    pub fn try_new(email: impl AsRef<str>, password: impl AsRef<str>) -> Result<Self, AuthError> {
        Ok(Self {
            email: Email::parse(email)?,
            password: Password::parse(password)?,
        })
    }
}

/// Result of a registration request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegisterResult {
    /// Identifier of the created user.
    pub user_id: UserId,
}

/// Result of a login request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoginResult {
    /// Authenticated user identifier.
    pub user_id: UserId,
    /// Issued access token.
    pub token: JwtToken,
    /// Token expiration timestamp.
    pub expires_at: SystemTime,
}

/// Port for persisting and retrieving users and their credential material.
fn is_valid_email(candidate: &str) -> bool {
    // Intentionally minimal: enough to prevent obviously bad values in the Core layer.
    // More strict rules can be added later without changing ports.
    let (local, domain) = match candidate.split_once('@') {
        Some(parts) => parts,
        None => return false,
    };

    if local.is_empty() || domain.is_empty() {
        return false;
    }

    // Domain should contain at least one dot and no spaces.
    if domain.contains(' ') || !domain.contains('.') {
        return false;
    }

    // Local part should not contain spaces.
    if local.contains(' ') {
        return false;
    }

    true
}
