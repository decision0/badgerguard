//! Multi-factor authentication

pub mod webauthn;
pub mod totp;
pub mod push;

use anyhow::Result;
use serde::{Deserialize, Serialize};

/// MFA method type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MfaMethod {
    WebAuthn,
    Totp,
    Push,
}

/// MFA challenge
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MfaChallenge {
    pub id: String,
    pub method: MfaMethod,
    pub user_id: String,
    pub expires_at: chrono::DateTime<chrono::Utc>,
    pub data: serde_json::Value,
}

/// MFA verification result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MfaVerification {
    pub challenge_id: String,
    pub success: bool,
    pub device_id: Option<String>,
}

