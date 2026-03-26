//! Adapter implementations for outbound dependencies (storage, databases, etc.).

pub mod fault_injecting_repo;
pub mod jsonl_repo;
pub mod postgres_db;
pub mod postgres_telemetry_repo;
