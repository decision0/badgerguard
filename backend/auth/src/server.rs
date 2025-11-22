//! gRPC server implementation

use crate::config::Config;
use std::sync::Arc;
use tracing::info;

/// Auth service gRPC server
pub struct AuthServer {
    config: Arc<Config>,
}

impl AuthServer {
    /// Create a new auth server
    pub async fn new(config: Config) -> anyhow::Result<Self> {
        // TODO: Initialize database connection
        // TODO: Initialize Redis connection
        // TODO: Initialize IdP clients
        // TODO: Initialize MFA services
        
        Ok(Self {
            config: Arc::new(config),
        })
    }

    /// Get bind address
    pub fn bind_addr(&self) -> &str {
        &self.config.server.bind
    }

    /// Run the auth server
    pub async fn run(&self) -> anyhow::Result<()> {
        // TODO: Implement gRPC server with tonic
        // TODO: Register auth service handlers
        info!("Auth server ready (implementation in progress)");
        Ok(())
    }
}

