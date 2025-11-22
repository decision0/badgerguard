//! Policy server implementation

use crate::config::Config;
use std::sync::Arc;
use tracing::info;

/// Policy service gRPC server
pub struct PolicyServer {
    config: Arc<Config>,
}

impl PolicyServer {
    /// Create a new policy server
    pub async fn new(config: Config) -> anyhow::Result<Self> {
        // TODO: Initialize OPA client or embedded OPA
        // TODO: Load policies from policy_dir
        
        Ok(Self {
            config: Arc::new(config),
        })
    }

    /// Get bind address
    pub fn bind_addr(&self) -> &str {
        &self.config.server.bind
    }

    /// Run the policy server
    pub async fn run(&self) -> anyhow::Result<()> {
        // TODO: Implement gRPC server with tonic
        // TODO: Register policy service handlers
        info!("Policy server ready (implementation in progress)");
        Ok(())
    }
}

