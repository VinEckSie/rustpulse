//! Telemetry use cases and ports.

pub mod ports;
pub mod usecases;

/// Use case for ingesting telemetry.
pub use ports::input::telemetry_ingest_usecase::TelemetryIngestCase;
/// Use case for querying telemetry.
pub use ports::input::telemetry_query_usecase::TelemetryQueryCase;
/// Output port for telemetry persistence.
pub use ports::output::telemetry_repository::TelemetryRepository;
/// Default telemetry use case implementation.
pub use usecases::telemetry_service::TelemetryService;
