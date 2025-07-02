use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "rustpulse", version, author, about)]
pub struct CliArgs {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Get telemetry metrics from a specific node type
    Metrics {
        #[arg(short, long)]
        target: String,
        #[arg(short, long, default_value = "json")]
        output: String,
    },
    /// Check system health
    Health,
}
