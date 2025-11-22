//! BadgerGuard Gateway
//!
//! A zero-trust MFA reverse proxy that enforces authentication and policy
//! decisions before forwarding requests to upstream applications.

use anyhow::Result;
use clap::Parser;
use tracing::{info, error};
use tracing_subscriber::EnvFilter;

mod config;
mod proxy;
mod middleware;
mod health;
mod auth;

use config::Config;
use proxy::ProxyServer;

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

    info!("Starting BadgerGuard Gateway...");

    // Load configuration
    let config = Config::from_file(&args.config)?;
    info!("Configuration loaded from {}", args.config);

    // Initialize proxy server
    let server = ProxyServer::new(config).await?;

    // Start server
    info!("Gateway listening on {}", server.bind_addr());
    if let Err(e) = server.run().await {
        error!("Server error: {}", e);
        return Err(e.into());
    }

    Ok(())
}

