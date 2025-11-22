//! WebAuthn (FIDO2) implementation

use webauthn_rs::prelude::*;
use anyhow::Result;
use super::{MfaChallenge, MfaVerification};

/// WebAuthn service
pub struct WebAuthnService {
    webauthn: Webauthn,
}

impl WebAuthnService {
    /// Create a new WebAuthn service
    pub fn new(rp_id: String, rp_name: String, origin: String) -> Result<Self> {
        let webauthn_builder = WebauthnBuilder::new(&rp_id, &origin)?
            .rp_name(&rp_name);

        let webauthn = webauthn_builder.build()?;

        Ok(Self { webauthn })
    }

    /// Start WebAuthn registration
    pub async fn start_registration(&self, _username: &str) -> Result<MfaChallenge> {
        // TODO: Implement WebAuthn registration start
        anyhow::bail!("Not implemented yet")
    }

    /// Complete WebAuthn registration
    pub async fn finish_registration(
        &self,
        _challenge_id: &str,
        _response: &[u8],
    ) -> Result<()> {
        // TODO: Implement WebAuthn registration completion
        Ok(())
    }

    /// Start WebAuthn authentication
    pub async fn start_authentication(&self, _username: &str) -> Result<MfaChallenge> {
        // TODO: Implement WebAuthn authentication start
        anyhow::bail!("Not implemented yet")
    }

    /// Complete WebAuthn authentication
    pub async fn finish_authentication(
        &self,
        _challenge_id: &str,
        _response: &[u8],
    ) -> Result<MfaVerification> {
        // TODO: Implement WebAuthn authentication completion
        anyhow::bail!("Not implemented yet")
    }
}

