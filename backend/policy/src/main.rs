//! BadgerGuard Policy Engine
//!
//! Integrates with OPA (Open Policy Agent) to evaluate access policies
//! based on user attributes, device posture, IP reputation, geo, and time.

use anyhow::Result;
use clap::Parser;
use tracing::{info, error};
use tracing_subscriber::EnvFilter;

mod config;
mod server;
mod opa;

use config::Config;
use server::PolicyServer;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Configuration file path
    #[arg(short, long, default_value = "config.toml")]
    config: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let args = Args::parse();

    info!("Starting BadgerGuard Policy Engine...");

    // Load configuration
    let config = Config::from_file(&args.config)?;
    info!("Configuration loaded from {}", args.config);

    // Initialize policy server
    let server = PolicyServer::new(config).await?;

    // Start server
    info!("Policy engine listening on {}", server.bind_addr());
    if let Err(e) = server.run().await {
        error!("Server error: {}", e);
        return Err(e.into());
    }

    Ok(())
}

