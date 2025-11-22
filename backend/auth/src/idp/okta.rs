//! Okta OIDC integration

use super::oidc::OidcClient;
use anyhow::Result;

/// Okta identity provider
pub struct OktaIdp {
    client: OidcClient,
    base_url: String,
}

impl OktaIdp {
    /// Create a new Okta IdP client
    pub fn new(
        base_url: String,
        client_id: String,
        client_secret: String,
        redirect_url: String,
    ) -> Result<Self> {
        let client = OidcClient::new(
            client_id,
            client_secret,
            format!("{}/oauth2/v1/authorize", base_url),
            format!("{}/oauth2/v1/token", base_url),
            redirect_url,
            format!("{}/oauth2/v1/userinfo", base_url),
        )?;

        Ok(Self { client, base_url })
    }
}

