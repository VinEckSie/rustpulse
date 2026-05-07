use rustpulse::core::{
    application::ports::output::auth_repository::{PasswordHasher, TokenIssuer, UserRepo},
    domains::auth::{
        AuthError, Email, JwtToken, LoginRequest, Password, PasswordHash, PasswordHasherError,
        RegisterRequest, TokenIssuerError, UserId, UserRepoError,
    },
};

#[test]
fn test_register_rejects_invalid_email() {
    let err = RegisterRequest::try_new("not-an-email", "correcthorsebattery").unwrap_err();
    assert_eq!(err, AuthError::InvalidEmail);
}

#[test]
fn test_login_rejects_too_short_password() {
    let err = LoginRequest::try_new("a@b.com", "short").unwrap_err();
    assert!(matches!(err, AuthError::PasswordTooShort { min: 8 }));
}

#[test]
fn test_ports_are_implementable() {
    struct DummyRepo;
    impl UserRepo for DummyRepo {
        fn find_user_id_by_email(&self, _email: &Email) -> Result<Option<UserId>, UserRepoError> {
            Ok(Some(UserId("u1".to_string())))
        }

        fn insert_user(
            &self,
            _email: &Email,
            _password_hash: &PasswordHash,
        ) -> Result<UserId, UserRepoError> {
            Ok(UserId("u2".to_string()))
        }

        fn get_password_hash(&self, _user_id: &UserId) -> Result<PasswordHash, UserRepoError> {
            Ok(PasswordHash("hash".to_string()))
        }
    }

    struct DummyHasher;
    impl PasswordHasher for DummyHasher {
        fn hash(&self, _password: &Password) -> Result<PasswordHash, PasswordHasherError> {
            Ok(PasswordHash("hash".to_string()))
        }

        fn verify(
            &self,
            _password: &Password,
            _hash: &PasswordHash,
        ) -> Result<bool, PasswordHasherError> {
            Ok(true)
        }
    }

    struct DummyIssuer;
    impl TokenIssuer for DummyIssuer {
        fn issue_access_token(
            &self,
            _user_id: &UserId,
            _expires_at: std::time::SystemTime,
        ) -> Result<JwtToken, TokenIssuerError> {
            Ok(JwtToken("token".to_string()))
        }
    }

    let email = Email::parse("x@y.com").unwrap();
    let password = Password::parse("correcthorsebattery").unwrap();
    let repo = DummyRepo;
    let hasher = DummyHasher;
    let issuer = DummyIssuer;

    let user_id = repo.find_user_id_by_email(&email).unwrap().unwrap();
    let hash = hasher.hash(&password).unwrap();
    let inserted = repo.insert_user(&email, &hash).unwrap();
    let stored = repo.get_password_hash(&inserted).unwrap();
    let ok = hasher.verify(&password, &stored).unwrap();
    let token = issuer
        .issue_access_token(&user_id, std::time::SystemTime::now())
        .unwrap();

    assert!(ok);
    assert_eq!(token.0, "token");
}
