#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct OidcAuthStart {
    pub authorization_url: String,
    pub csrf_token: String,
    pub nonce: String,
    pub pkce_verifier: String,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct OidcClaims {
    pub subject: String,
    pub email: String,
    pub email_verified: bool,
    pub name: Option<String>,
}

#[allow(dead_code)]
#[derive(thiserror::Error, Debug)]
pub enum OidcError {
    #[error("oidc discovery error: {0}")]
    Discovery(String),
    #[error("oidc flow error: {0}")]
    Flow(String),
}

#[async_trait::async_trait]
#[allow(dead_code)]
pub trait OidcFlow: Send + Sync {
    async fn start(&self) -> Result<OidcAuthStart, OidcError>;
    async fn exchange(
        &self,
        code: &str,
        expected_nonce: &str,
        pkce_verifier: &str,
    ) -> Result<OidcClaims, OidcError>;
}
