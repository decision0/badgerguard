//! JumpCloud OIDC integration

use super::oidc::OidcClient;
use anyhow::Result;

/// JumpCloud identity provider
pub struct JumpCloudIdp {
    client: OidcClient,
    base_url: String,
}

impl JumpCloudIdp {
    /// Create a new JumpCloud IdP client
    pub fn new(
        base_url: String,
        client_id: String,
        client_secret: String,
        redirect_url: String,
    ) -> Result<Self> {
        let client = OidcClient::new(
            client_id,
            client_secret,
            format!("{}/oauth2/auth", base_url),
            format!("{}/oauth2/token", base_url),
            redirect_url,
            format!("{}/userinfo", base_url),
        )?;

        Ok(Self { client, base_url })
    }
}

