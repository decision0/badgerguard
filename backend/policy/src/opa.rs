//! OPA (Open Policy Agent) client

use anyhow::Result;
use serde_json::Value;
use badgerguard_shared::models::{PolicyRequest, PolicyResponse};

/// OPA client
pub struct OpaClient {
    base_url: String,
}

impl OpaClient {
    /// Create a new OPA client
    pub fn new(base_url: String) -> Self {
        Self { base_url }
    }

    /// Evaluate policy
    pub async fn evaluate(&self, _request: &PolicyRequest) -> Result<PolicyResponse> {
        // TODO: Implement OPA query
        // POST /v1/data/badgerguard/policy/allow
        anyhow::bail!("Not implemented yet")
    }

    /// Load policy from file
    pub async fn load_policy(&self, _policy_id: &str, _rego: &str) -> Result<()> {
        // TODO: Implement OPA policy upload
        // PUT /v1/policies/{policy_id}
        Ok(())
    }
}

