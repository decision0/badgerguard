//! Active Directory / Entra ID integration

use super::oidc::OidcClient;
use anyhow::Result;

/// Active Directory / Entra ID identity provider
pub struct ActiveDirectoryIdp {
    client: OidcClient,
    tenant_id: String,
}

impl ActiveDirectoryIdp {
    /// Create a new AD/Entra IdP client
    pub fn new(
        tenant_id: String,
        client_id: String,
        client_secret: String,
        redirect_url: String,
    ) -> Result<Self> {
        let client = OidcClient::new(
            client_id,
            client_secret,
            format!("https://login.microsoftonline.com/{}/oauth2/v2.0/authorize", tenant_id),
            format!("https://login.microsoftonline.com/{}/oauth2/v2.0/token", tenant_id),
            redirect_url,
            "https://graph.microsoft.com/oidc/userinfo".to_string(),
        )?;

        Ok(Self { client, tenant_id })
    }
}

