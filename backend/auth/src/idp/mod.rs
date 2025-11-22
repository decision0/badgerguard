//! Identity provider integration

pub mod oidc;
pub mod google;
pub mod okta;
pub mod jumpcloud;
pub mod ad;

use anyhow::Result;
use serde::{Deserialize, Serialize};

/// Identity provider trait
pub trait IdentityProvider: Send + Sync {
    /// Authenticate user and return user info
    fn authenticate(&self, code: &str) -> Result<UserInfo>;
    
    /// Get authorization URL
    fn authorization_url(&self) -> String;
}

/// User information from IdP
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    pub id: String,
    pub email: String,
    pub name: Option<String>,
    pub groups: Vec<String>,
    pub attributes: serde_json::Value,
}

