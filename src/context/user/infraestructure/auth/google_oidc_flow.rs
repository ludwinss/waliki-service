use openidconnect::{
    AuthorizationCode, ClientId, ClientSecret, CsrfToken, IssuerUrl, Nonce, PkceCodeChallenge,
    PkceCodeVerifier, RedirectUrl, Scope,
    core::{CoreAuthenticationFlow, CoreClient, CoreProviderMetadata},
};

use crate::context::user::domain::{
    errors::IdentityError,
    services::oidc_flow::{AuthStart, OidcClaims, OidcFlow},
};
use openidconnect::reqwest::{Client as OidcHttpClient, redirect::Policy};

pub struct GoogleOidcFlow {
    provider: CoreProviderMetadata,
    client_id: String,
    client_secret: Option<ClientSecret>,
    redirect_uri: RedirectUrl,
    http: OidcHttpClient,
}

impl GoogleOidcFlow {
    pub async fn discover(
        issuer: &str,
        client_id: String,
        client_secret: Option<String>,
        redirect_uri: String,
    ) -> Result<Self, IdentityError> {
        let http = OidcHttpClient::builder()
            .redirect(Policy::none())
            .build()
            .map_err(|e| IdentityError::OidcOther(format!("http client build: {e}")))?;

        let issuer = IssuerUrl::new(issuer.to_string())
            .map_err(|_| IdentityError::InvalidIssuerOrAudience)?;

        let provider = CoreProviderMetadata::discover_async(issuer, &http)
            .await
            .map_err(|_| IdentityError::OidcNetwork)?;

        let redirect_uri = RedirectUrl::new(redirect_uri)
            .map_err(|_| IdentityError::OidcOther("bad redirect".into()))?;

        Ok(Self {
            provider,
            client_id: client_id.clone(),
            client_secret: client_secret.map(ClientSecret::new),
            redirect_uri,
            http,
        })
    }
}

#[async_trait::async_trait]
impl OidcFlow for GoogleOidcFlow {
    fn start_auth(&self) -> AuthStart {
        let client = CoreClient::from_provider_metadata(
            self.provider.clone(),
            ClientId::new(self.client_id.clone()),
            self.client_secret.clone(),
        )
        .set_redirect_uri(self.redirect_uri.clone());

        let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
        let (auth_url, csrf, nonce) = client
            .authorize_url(
                CoreAuthenticationFlow::AuthorizationCode,
                CsrfToken::new_random,
                Nonce::new_random,
            )
            .add_scope(Scope::new("openid".into()))
            .add_scope(Scope::new("email".into()))
            .add_scope(Scope::new("profile".into()))
            .set_pkce_challenge(pkce_challenge)
            .url();

        AuthStart {
            auth_url: auth_url.to_string(),
            csrf: csrf.secret().to_string(),
            nonce: nonce.secret().to_string(),
            pkce_verifier: pkce_verifier.secret().to_string(),
        }
    }

    async fn exchange_and_verify(
        &self,
        code: &str,
        expected_nonce: &str,
        pkce_verifier: &str,
    ) -> Result<OidcClaims, IdentityError> {
        let client = CoreClient::from_provider_metadata(
            self.provider.clone(),
            ClientId::new(self.client_id.clone()),
            self.client_secret.clone(),
        )
        .set_redirect_uri(self.redirect_uri.clone());

        let code_req = client
            .exchange_code(AuthorizationCode::new(code.to_string()))
            .map_err(|_| IdentityError::OidcNetwork)?;

        let token_resp = code_req
            .set_pkce_verifier(PkceCodeVerifier::new(pkce_verifier.to_string()))
            .request_async(&self.http)
            .await
            .map_err(|_| IdentityError::OidcNetwork)?;

        let id_token = token_resp
            .extra_fields()
            .id_token()
            .cloned()
            .ok_or(IdentityError::InvalidIdToken)?;

        let nonce = Nonce::new(expected_nonce.to_string());
        let claims = id_token
            .claims(&client.id_token_verifier(), &nonce)
            .map_err(|e| IdentityError::OidcOther(e.to_string()))?;

        if !claims
            .audiences()
            .iter()
            .any(|a| a.as_str() == self.client_id)
        {
            return Err(IdentityError::InvalidIssuerOrAudience);
        }

        let nonce_ok = claims.nonce().map(|n| n.secret()) == Some(nonce.secret());
        if !nonce_ok {
            return Err(IdentityError::InvalidTimestamps);
        }

        let email = claims
            .email()
            .map(|e| e.as_str().to_string())
            .ok_or(IdentityError::OidcNotFoundEmail)?;

        Ok(OidcClaims {
            sub: claims.subject().as_str().to_string(),
            email,
            email_verified: claims.email_verified().unwrap_or(false),
            name: claims
                .name()
                .and_then(|n| n.get(None))
                .map(|s| s.to_string()),
        })
    }
}
