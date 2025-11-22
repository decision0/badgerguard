//! BadgerGuard Auth Service
//!
//! Handles identity provider integration, MFA enrollment/authentication,
//! and session management.

use anyhow::Result;
use clap::Parser;
use tracing::{info, error};
use tracing_subscriber::EnvFilter;

mod config;
mod server;
mod idp;
mod mfa;
mod session;
mod db;

use config::Config;
use server::AuthServer;

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

    info!("Starting BadgerGuard Auth Service...");

    // Load configuration
    let config = Config::from_file(&args.config)?;
    info!("Configuration loaded from {}", args.config);

    // Initialize auth server
    let server = AuthServer::new(config).await?;

    // Start server
    info!("Auth service listening on {}", server.bind_addr());
    if let Err(e) = server.run().await {
        error!("Server error: {}", e);
        return Err(e.into());
    }

    Ok(())
}

