//! Google Workspace OIDC integration

use super::oidc::OidcClient;
use anyhow::Result;

/// Google Workspace identity provider
pub struct GoogleIdp {
    client: OidcClient,
}

impl GoogleIdp {
    /// Create a new Google IdP client
    pub fn new(
        client_id: String,
        client_secret: String,
        redirect_url: String,
    ) -> Result<Self> {
        let client = OidcClient::new(
            client_id,
            client_secret,
            "https://accounts.google.com/o/oauth2/v2/auth".to_string(),
            "https://oauth2.googleapis.com/token".to_string(),
            redirect_url,
            "https://openidconnect.googleapis.com/v1/userinfo".to_string(),
        )?;

        Ok(Self { client })
    }
}

