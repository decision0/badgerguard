//! Shared data models

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// User model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub tenant_id: String,
    pub email: String,
    pub name: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Tenant model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tenant {
    pub id: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
}

/// Upstream application
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpstreamApp {
    pub id: String,
    pub tenant_id: String,
    pub name: String,
    pub route: String,
    pub upstream_url: String,
    pub sensitivity: String,
}

/// Policy decision request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyRequest {
    pub user_id: String,
    pub tenant_id: String,
    pub app_id: String,
    pub device_id: Option<String>,
    pub ip_address: String,
    pub country: Option<String>,
    pub time_of_day: String,
    pub day_of_week: String,
}

/// Policy decision response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyResponse {
    pub allowed: bool,
    pub reason: Option<String>,
    pub required_mfa: Vec<String>,
}

