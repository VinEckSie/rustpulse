//! Traits for telemetry data input/output at the ports layer.
//!
//! This module defines abstract interfaces for fetching and polling telemetry data:
//! - `MetricsRepository`: used by services to fetch metrics from a repository.
//! - `TelemetrySource`: used by adapters to poll live metrics from external sources.
//!

pub mod telemetry_ingest_case;
mod telemetry_mock_repo;
pub mod telemetry_query_case;
pub mod telemetry_repo;
//
// use crate::core::domains::node::Node;
// use crate::core::domains::node_telemetry::NodeTelemetry;
//
// /// Input Port: Defines the interface for fetching telemetry data.
// pub trait TelemetryCase {
//     fn fetch_metrics(&self) -> NodeTelemetry;
// }
//
// /// Repository interface to fetch telemetry data for a node.
// pub trait MetricsRepository {
//     /// Fetch metrics from the underlying store.
//     fn fetch_metrics(&self) -> NodeTelemetry;
// }
//
// /// Abstraction for external telemetry polling mechanisms.
// pub trait SimulatedTelemetryRepository {
//     /// Poll telemetry data from a given node.
//     fn poll(&mut self, node: &Node) -> NodeTelemetry;
// }
