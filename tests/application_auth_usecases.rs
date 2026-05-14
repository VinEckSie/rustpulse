use rustpulse::core::application::ports::input::auth_login_usecase::AuthLoginUseCase as _;
use rustpulse::core::application::ports::input::auth_register_usecase::AuthRegisterUseCase as _;
use rustpulse::core::application::ports::output::auth_repository::{PasswordHasher, TokenIssuer, UserRepo};
use rustpulse::core::application::usecases::auth_service::AuthService;
use rustpulse::core::domains::auth::{
    AuthError, Email, JwtToken, LoginRequest, Password, PasswordHash, PasswordHasherError,
    RegisterRequest, TokenIssuerError, UserId, UserRepoError,
};
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};
use tracing::Subscriber;
use tracing::field::{Field, Visit};
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::{Layer, layer::Context, prelude::*};

#[derive(Clone, Default)]
struct CapturedEvents(Arc<Mutex<Vec<CapturedEvent>>>);

#[derive(Debug, Clone)]
struct CapturedEvent {
    fields: std::collections::BTreeMap<String, String>,
}

#[derive(Clone, Default)]
struct CaptureLayer {
    events: CapturedEvents,
}

struct EventVisitor<'a> {
    fields: &'a mut std::collections::BTreeMap<String, String>,
}

impl<'a> Visit for EventVisitor<'a> {
    fn record_str(&mut self, field: &Field, value: &str) {
        self.fields
            .insert(field.name().to_string(), value.to_string());
    }

    fn record_debug(&mut self, field: &Field, value: &dyn std::fmt::Debug) {
        self.fields
            .insert(field.name().to_string(), format!("{value:?}"));
    }
}

impl<S> Layer<S> for CaptureLayer
where
    S: Subscriber + for<'a> LookupSpan<'a>,
{
    fn on_event(&self, event: &tracing::Event<'_>, _ctx: Context<'_, S>) {
        let mut fields = std::collections::BTreeMap::new();
        event.record(&mut EventVisitor { fields: &mut fields });
        self.events
            .0
            .lock()
            .expect("poisoned mutex")
            .push(CapturedEvent {
                fields,
            });
    }
}

struct FakeRepo {
    find_by_email: Mutex<Result<Option<UserId>, UserRepoError>>,
    insert_result: Mutex<Result<UserId, UserRepoError>>,
    stored_hash: Mutex<Result<PasswordHash, UserRepoError>>,
}

impl UserRepo for FakeRepo {
    fn find_user_id_by_email(&self, _email: &Email) -> Result<Option<UserId>, UserRepoError> {
        self.find_by_email.lock().expect("poisoned mutex").clone()
    }

    fn insert_user(&self, _email: &Email, _password_hash: &PasswordHash) -> Result<UserId, UserRepoError> {
        self.insert_result.lock().expect("poisoned mutex").clone()
    }

    fn get_password_hash(&self, _user_id: &UserId) -> Result<PasswordHash, UserRepoError> {
        self.stored_hash.lock().expect("poisoned mutex").clone()
    }
}

struct FakeHasher {
    hash_result: Mutex<Result<PasswordHash, PasswordHasherError>>,
    verify_result: Mutex<Result<bool, PasswordHasherError>>,
}

impl PasswordHasher for FakeHasher {
    fn hash(&self, _password: &Password) -> Result<PasswordHash, PasswordHasherError> {
        self.hash_result.lock().expect("poisoned mutex").clone()
    }

    fn verify(&self, _password: &Password, _hash: &PasswordHash) -> Result<bool, PasswordHasherError> {
        self.verify_result.lock().expect("poisoned mutex").clone()
    }
}

struct FakeIssuer {
    issued: Mutex<Vec<(String, SystemTime)>>,
    token_result: Mutex<Result<JwtToken, TokenIssuerError>>,
}

impl TokenIssuer for FakeIssuer {
    fn issue_access_token(&self, user_id: &UserId, expires_at: SystemTime) -> Result<JwtToken, TokenIssuerError> {
        self.issued
            .lock()
            .expect("poisoned mutex")
            .push((user_id.0.clone(), expires_at));
        self.token_result.lock().expect("poisoned mutex").clone()
    }
}

#[tokio::test]
async fn test_register_success_persists_user_and_returns_user_id() {
    let events = CapturedEvents::default();
    let subscriber = tracing_subscriber::registry().with(CaptureLayer {
        events: events.clone(),
    });
    let _guard = tracing::subscriber::set_default(subscriber);

    let repo = Arc::new(FakeRepo {
        find_by_email: Mutex::new(Ok(None)),
        insert_result: Mutex::new(Ok(UserId("u1".to_string()))),
        stored_hash: Mutex::new(Err(UserRepoError::NotFound)),
    });
    let hasher = Arc::new(FakeHasher {
        hash_result: Mutex::new(Ok(PasswordHash("hash".to_string()))),
        verify_result: Mutex::new(Ok(true)),
    });
    let issuer = Arc::new(FakeIssuer {
        issued: Mutex::new(Vec::new()),
        token_result: Mutex::new(Ok(JwtToken("t".to_string()))),
    });

    let service = AuthService::new(repo, hasher, issuer);
    let req = RegisterRequest::try_new("a@b.com", "correcthorsebattery").unwrap();

    let out = service.register(req).await.unwrap();
    assert_eq!(out.user_id.0, "u1");

    let captured = events.0.lock().expect("poisoned mutex");
    assert!(captured.iter().any(|e| e.fields.get("event").map(String::as_str) == Some("auth.register.success")));
}

#[tokio::test]
async fn test_login_success_returns_token_and_expires_at() {
    let events = CapturedEvents::default();
    let subscriber = tracing_subscriber::registry().with(CaptureLayer {
        events: events.clone(),
    });
    let _guard = tracing::subscriber::set_default(subscriber);

    let user_id = UserId("user-123".to_string());
    let repo = Arc::new(FakeRepo {
        find_by_email: Mutex::new(Ok(Some(user_id.clone()))),
        insert_result: Mutex::new(Err(UserRepoError::Conflict)),
        stored_hash: Mutex::new(Ok(PasswordHash("hash".to_string()))),
    });
    let hasher = Arc::new(FakeHasher {
        hash_result: Mutex::new(Err(PasswordHasherError::Failed)),
        verify_result: Mutex::new(Ok(true)),
    });
    let issuer = Arc::new(FakeIssuer {
        issued: Mutex::new(Vec::new()),
        token_result: Mutex::new(Ok(JwtToken("token".to_string()))),
    });

    let service = AuthService::new(repo, hasher, issuer.clone());
    let req = LoginRequest::try_new("a@b.com", "correcthorsebattery").unwrap();
    let now = SystemTime::UNIX_EPOCH + Duration::from_secs(10);
    let ttl = Duration::from_secs(3600);

    let out = service.login(req, now, ttl).await.unwrap();
    assert_eq!(out.user_id.0, "user-123");
    assert_eq!(out.token.0, "token");
    assert_eq!(out.expires_at, now + ttl);

    let issued = issuer.issued.lock().expect("poisoned mutex");
    assert_eq!(issued.len(), 1);
    assert_eq!(issued[0].0, "user-123");
    assert_eq!(issued[0].1, now + ttl);

    let captured = events.0.lock().expect("poisoned mutex");
    assert!(captured.iter().any(|e| e.fields.get("event").map(String::as_str) == Some("auth.login.success")));
}

#[tokio::test]
async fn test_login_invalid_credentials_returns_domain_error_and_emits_failure() {
    let events = CapturedEvents::default();
    let subscriber = tracing_subscriber::registry().with(CaptureLayer {
        events: events.clone(),
    });
    let _guard = tracing::subscriber::set_default(subscriber);

    let user_id = UserId("user-123".to_string());
    let repo = Arc::new(FakeRepo {
        find_by_email: Mutex::new(Ok(Some(user_id.clone()))),
        insert_result: Mutex::new(Err(UserRepoError::Conflict)),
        stored_hash: Mutex::new(Ok(PasswordHash("hash".to_string()))),
    });
    let hasher = Arc::new(FakeHasher {
        hash_result: Mutex::new(Err(PasswordHasherError::Failed)),
        verify_result: Mutex::new(Ok(false)),
    });
    let issuer = Arc::new(FakeIssuer {
        issued: Mutex::new(Vec::new()),
        token_result: Mutex::new(Ok(JwtToken("token".to_string()))),
    });

    let service = AuthService::new(repo, hasher, issuer);
    let req = LoginRequest::try_new("a@b.com", "wrongpassword").unwrap();
    let now = SystemTime::UNIX_EPOCH + Duration::from_secs(10);
    let ttl = Duration::from_secs(3600);

    let err = service.login(req, now, ttl).await.unwrap_err();
    assert_eq!(err, AuthError::InvalidCredentials);

    let captured = events.0.lock().expect("poisoned mutex");
    assert!(captured.iter().any(|e| {
        e.fields.get("event").map(String::as_str) == Some("auth.login.failure")
            && e.fields.get("reason").map(String::as_str) == Some("invalid_credentials")
    }));
}
