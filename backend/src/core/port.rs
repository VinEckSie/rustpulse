//! Traits for telemetry data input/output at the ports layer.
//!
//! This module defines abstract interfaces for fetching and polling telemetry data:
//! - `MetricsRepository`: used by services to fetch metrics from a repository.
//! - `TelemetrySource`: used by adapters to poll live metrics from external sources.
//!

use crate::core::domains::node::Node;
use crate::core::domains::telemetry::NodeTelemetry;

/// Repository interface to fetch telemetry data for a node.
pub trait MetricsRepository {
    /// Fetch metrics from the underlying store.
    fn fetch_metrics(&self) -> NodeTelemetry;
}

/// Abstraction for external telemetry polling mechanisms.
pub trait SimulatedTelemetryRepository {
    /// Poll telemetry data from a given node.
    fn poll(&mut self, node: &Node) -> NodeTelemetry;
}
