//! Output ports used by authentication use cases.
//!
//! These are implemented by adapters (DB, crypto, JWT, etc.) and called by the
//! application authentication use cases.

use std::time::SystemTime;

use crate::core::domains::auth::{
    Email, JwtToken, Password, PasswordHash, PasswordHasherError, TokenIssuerError, UserId,
    UserRepoError,
};

/// Port for persisting and retrieving users and their credential material.
pub trait UserRepo {
    /// Finds a user id by email address.
    fn find_user_id_by_email(&self, email: &Email) -> Result<Option<UserId>, UserRepoError>;
    /// Inserts a new user record and returns the assigned id.
    fn insert_user(
        &self,
        email: &Email,
        password_hash: &PasswordHash,
    ) -> Result<UserId, UserRepoError>;
    /// Retrieves the stored password hash for a user.
    fn get_password_hash(&self, user_id: &UserId) -> Result<PasswordHash, UserRepoError>;
}

/// Port for hashing and verifying passwords.
pub trait PasswordHasher {
    /// Hashes the given password.
    fn hash(&self, password: &Password) -> Result<PasswordHash, PasswordHasherError>;
    /// Verifies a password against a stored hash.
    fn verify(&self, password: &Password, hash: &PasswordHash)
    -> Result<bool, PasswordHasherError>;
}

/// Port for issuing access tokens.
pub trait TokenIssuer {
    /// Issues an access token for `user_id` expiring at `expires_at`.
    fn issue_access_token(
        &self,
        user_id: &UserId,
        expires_at: SystemTime,
    ) -> Result<JwtToken, TokenIssuerError>;
}
