use anyhow::Result;

pub struct OidcClaims {
    pub sub: String,
    pub email: String,
    pub name: Option<String>,
    pub email_verified: bool,
}

pub struct AuthStart {
    pub auth_url: String,
    pub csrf: String,
    pub nonce: String,
    pub pkce_verifier: String,
}

pub trait OidcFlow: Send + Sync {
    fn start_auth(&self) -> AuthStart;
    fn exchange_and_verify(
        &self,
        code: &str,
        expected_nonce: &str,
        pkce_verifier: &str,
    ) -> Result<OidcClaims>;
}
