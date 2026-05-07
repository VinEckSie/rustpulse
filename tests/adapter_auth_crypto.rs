use rustpulse::adapters::output::auth_jwt_issuer::Hs256JwtIssuer;
use rustpulse::adapters::output::auth_password_hasher::Argon2PasswordHasher;
use rustpulse::core::application::ports::output::auth_repository::{
    PasswordHasher as _, TokenIssuer as _,
};
use rustpulse::core::domains::auth::{JwtToken, Password, UserId};

#[test]
fn test_password_hash_verify_roundtrip_ok() {
    let hasher = Argon2PasswordHasher::new();
    let password = Password::parse("correcthorsebattery").unwrap();

    let hash = hasher.hash(&password).unwrap();
    let ok = hasher.verify(&password, &hash).unwrap();

    assert!(ok);
}

#[test]
fn test_password_hash_verify_wrong_password_false() {
    let hasher = Argon2PasswordHasher::new();
    let password = Password::parse("correcthorsebattery").unwrap();
    let wrong_password = Password::parse("correcthorsebattery-but-wrong").unwrap();

    let hash = hasher.hash(&password).unwrap();
    let ok = hasher.verify(&wrong_password, &hash).unwrap();

    assert!(!ok);
}

#[test]
fn test_jwt_issue_contains_sub_and_exp_and_verifies() {
    let secret = b"test-secret";
    let issuer = Hs256JwtIssuer::new(secret);
    let user_id = UserId("user-123".to_string());
    let expires_at =
        std::time::SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(1_700_000_000);

    let JwtToken(token) = issuer.issue_access_token(&user_id, expires_at).unwrap();

    let mut validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256);
    validation.validate_exp = false;
    let data = jsonwebtoken::decode::<serde_json::Value>(
        &token,
        &jsonwebtoken::DecodingKey::from_secret(secret),
        &validation,
    )
    .unwrap();

    let sub = data.claims.get("sub").and_then(|v| v.as_str()).unwrap();
    let exp = data.claims.get("exp").and_then(|v| v.as_u64()).unwrap();
    assert_eq!(sub, "user-123");
    assert_eq!(exp, 1_700_000_000u64);
}
