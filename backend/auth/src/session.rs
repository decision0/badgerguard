//! Session management

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// User session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: String,
    pub user_id: String,
    pub tenant_id: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub device_id: Option<String>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub mfa_verified: bool,
    pub mfa_method: Option<String>,
}

impl Session {
    /// Create a new session
    pub fn new(
        user_id: String,
        tenant_id: String,
        expires_in_seconds: i64,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            user_id,
            tenant_id,
            created_at: now,
            expires_at: now + chrono::Duration::seconds(expires_in_seconds),
            device_id: None,
            ip_address: None,
            user_agent: None,
            mfa_verified: false,
            mfa_method: None,
        }
    }

    /// Check if session is expired
    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }
}

