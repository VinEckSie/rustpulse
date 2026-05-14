//! Application layer (use cases and ports).

/// Port traits (inputs/outputs) for use cases.
pub mod ports;
/// Use case implementations.
pub mod usecases;

// Re-exports for stable external paths.
pub use ports::input::telemetry_ingest_usecase::TelemetryIngestCase;
pub use ports::input::telemetry_query_usecase::TelemetryQueryCase;
pub use ports::input::auth_login_usecase::AuthLoginUseCase;
pub use ports::input::auth_register_usecase::AuthRegisterUseCase;
pub use ports::output::auth_repository::{PasswordHasher, TokenIssuer, UserRepo};
pub use ports::output::telemetry_repository::TelemetryRepository;
pub use usecases::telemetry_service::TelemetryService;
pub use usecases::auth_service::AuthService;
