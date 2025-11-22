//! Policy engine configuration

use serde::{Deserialize, Serialize};
use std::path::Path;
use anyhow::{Context, Result};

/// Policy engine configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// gRPC server bind address
    pub server: ServerConfig,
    /// OPA configuration
    pub opa: OpaConfig,
    /// Policy directory
    pub policy_dir: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    /// Bind address (e.g., "0.0.0.0:50052")
    pub bind: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpaConfig {
    /// OPA server URL (if external) or "embedded" for embedded OPA
    pub url: String,
    /// Enable policy hot-reload
    #[serde(default = "default_true")]
    pub hot_reload: bool,
}

fn default_true() -> bool {
    true
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

