//! Gateway configuration

use serde::{Deserialize, Serialize};
use std::path::Path;
use anyhow::{Context, Result};

/// Gateway configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Server bind address
    pub server: ServerConfig,
    /// TLS configuration
    pub tls: Option<TlsConfig>,
    /// Upstream applications
    pub upstreams: Vec<UpstreamConfig>,
    /// Auth service gRPC endpoint
    pub auth_service: String,
    /// Policy engine endpoint
    pub policy_engine: String,
    /// Redis connection for sessions
    pub redis: RedisConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    /// Bind address (e.g., "0.0.0.0:8080")
    pub bind: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TlsConfig {
    /// Certificate file path
    pub cert: String,
    /// Private key file path
    pub key: String,
    /// Enable ACME (Let's Encrypt)
    #[serde(default)]
    pub acme: bool,
    /// ACME domain
    pub acme_domain: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpstreamConfig {
    /// Application ID
    pub id: String,
    /// Application name
    pub name: String,
    /// Route pattern (e.g., "/app/*")
    pub route: String,
    /// Upstream URL
    pub upstream: String,
    /// Sensitivity level (low, medium, high)
    pub sensitivity: String,
    /// Required MFA methods
    pub required_mfa: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedisConfig {
    /// Redis connection URL
    pub url: String,
}

impl Config {
    /// Load configuration from file
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = std::fs::read_to_string(path.as_ref())
            .with_context(|| format!("Failed to read config file: {:?}", path.as_ref()))?;
        toml::from_str(&content)
            .with_context(|| "Failed to parse config file")
    }
}

