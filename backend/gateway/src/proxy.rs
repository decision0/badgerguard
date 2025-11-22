//! Reverse proxy implementation

use crate::config::Config;
use hyper::body::Incoming;
use hyper::Request;
use hyper::Response;
use http_body_util::Full;
use bytes::Bytes;
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing::{info, error};

use crate::middleware::AuthMiddleware;
use crate::health::health_check;

/// Proxy server
pub struct ProxyServer {
    config: Arc<Config>,
}

impl ProxyServer {
    /// Create a new proxy server
    pub async fn new(config: Config) -> anyhow::Result<Self> {
        Ok(Self {
            config: Arc::new(config),
        })
    }

    /// Get bind address
    pub fn bind_addr(&self) -> &str {
        &self.config.server.bind
    }

    /// Run the proxy server
    pub async fn run(&self) -> anyhow::Result<()> {
        // TODO: Implement HTTP server with hyper
        // TODO: Add routing for /healthz
        // TODO: Add auth middleware
        // TODO: Add upstream proxying
        info!("Proxy server ready (implementation in progress)");
        Ok(())
    }
}

