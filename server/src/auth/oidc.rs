//! OpenID Connect Integration

use openidconnect::{
    core::{CoreClient, CoreProviderMetadata},
    ClientId, ClientSecret, IssuerUrl, RedirectUrl,
};

/// OIDC client configuration.
pub struct OidcClient {
    client: CoreClient,
}

impl OidcClient {
    /// Create a new OIDC client.
    pub async fn new(
        _issuer_url: &str,
        _client_id: &str,
        _client_secret: &str,
        _redirect_url: &str,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        // TODO: Implement OIDC client setup
        todo!()
    }

    /// Generate authorization URL.
    pub fn authorization_url(&self) -> String {
        // TODO: Implement
        todo!()
    }

    /// Exchange authorization code for tokens.
    pub async fn exchange_code(&self, _code: &str) -> Result<(), Box<dyn std::error::Error>> {
        // TODO: Implement
        todo!()
    }
}
