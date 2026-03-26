//! RustPulse backend binary entrypoint.

#![deny(missing_docs)]

use rustpulse::start;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    start().await
}
