//! Auth service configuration

use serde::{Deserialize, Serialize};
use std::path::Path;
use anyhow::{Context, Result};

/// Auth service configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// gRPC server bind address
    pub server: ServerConfig,
    /// PostgreSQL database URL
    pub database: DatabaseConfig,
    /// Redis connection for sessions
    pub redis: RedisConfig,
    /// Identity provider configurations
    pub idp: Vec<IdpConfig>,
    /// MFA configuration
    pub mfa: MfaConfig,
    /// JWT signing key
    pub jwt_secret: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    /// Bind address (e.g., "0.0.0.0:50051")
    pub bind: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    /// PostgreSQL connection URL
    pub url: String,
    /// Maximum number of connections
    #[serde(default = "default_max_connections")]
    pub max_connections: u32,
}

fn default_max_connections() -> u32 {
    10
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedisConfig {
    /// Redis connection URL
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdpConfig {
    /// IdP ID
    pub id: String,
    /// IdP name
    pub name: String,
    /// IdP type (google, okta, jumpcloud, ad)
    pub r#type: String,
    /// OIDC client ID
    pub client_id: String,
    /// OIDC client secret
    pub client_secret: String,
    /// OIDC discovery URL
    pub discovery_url: String,
    /// OIDC redirect URL
    pub redirect_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MfaConfig {
    /// Enable WebAuthn
    #[serde(default = "default_true")]
    pub webauthn_enabled: bool,
    /// WebAuthn RP ID
    pub webauthn_rp_id: Option<String>,
    /// Enable TOTP
    #[serde(default = "default_true")]
    pub totp_enabled: bool,
    /// Enable push notifications
    #[serde(default = "default_true")]
    pub push_enabled: bool,
    /// FCM server key (for push)
    pub fcm_server_key: Option<String>,
    /// APNs key ID (for push)
    pub apns_key_id: Option<String>,
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

