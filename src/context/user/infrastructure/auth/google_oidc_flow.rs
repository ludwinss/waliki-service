use std::ops::Deref;

use openidconnect::core::{CoreClient, CoreProviderMetadata, CoreResponseType};
use openidconnect::{
    AccessTokenHash, AuthenticationFlow, AuthorizationCode, ClientId, ClientSecret, CsrfToken,
    IssuerUrl, Nonce, OAuth2TokenResponse, PkceCodeChallenge, PkceCodeVerifier, RedirectUrl, Scope,
    TokenResponse,
};

use crate::context::user::application::ports::oidc_flow::{
    OidcAuthStart, OidcClaims, OidcError, OidcFlow,
};

#[derive(Clone)]
pub struct GoogleOidcConfig {
    pub client_id: String,
    pub client_secret: Option<String>,
    pub redirect_uri: String,
    pub issuer: String,
}

#[derive(Clone)]
pub struct GoogleOidcFlow {
    provider_metadata: CoreProviderMetadata,
    client_id: ClientId,
    client_secret: Option<ClientSecret>,
    redirect_uri: RedirectUrl,
    http_client: reqwest::Client,
}

impl GoogleOidcFlow {
    pub async fn new(config: GoogleOidcConfig) -> Result<Self, OidcError> {
        let http_client = reqwest::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .map_err(|e| OidcError::Discovery(format!("failed to build HTTP client: {e}")))?;

        let issuer = IssuerUrl::new(config.issuer.clone())
            .map_err(|e| OidcError::Discovery(format!("invalid issuer url: {e}")))?;

        let provider_metadata = CoreProviderMetadata::discover_async(issuer, &http_client)
            .await
            .map_err(|e| OidcError::Discovery(format!("failed to discover provider: {e}")))?;

        let client_id = ClientId::new(config.client_id);
        let client_secret = config.client_secret.map(ClientSecret::new);
        let redirect_uri = RedirectUrl::new(config.redirect_uri)
            .map_err(|e| OidcError::Discovery(format!("invalid redirect uri: {e}")))?;

        Ok(Self {
            provider_metadata,
            client_id,
            client_secret,
            redirect_uri,
            http_client,
        })
    }
}

#[async_trait::async_trait]
impl OidcFlow for GoogleOidcFlow {
    async fn start(&self) -> Result<OidcAuthStart, OidcError> {
        let client = CoreClient::from_provider_metadata(
            self.provider_metadata.clone(),
            self.client_id.clone(),
            self.client_secret.clone(),
        )
        .set_redirect_uri(self.redirect_uri.clone());

        let (pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();
        let (authorize_url, csrf_token, nonce) = client
            .authorize_url(
                AuthenticationFlow::<CoreResponseType>::AuthorizationCode,
                CsrfToken::new_random,
                Nonce::new_random,
            )
            .set_pkce_challenge(pkce_code_challenge)
            .add_scope(Scope::new("openid".to_string()))
            .add_scope(Scope::new("email".to_string()))
            .add_scope(Scope::new("profile".to_string()))
            .url();

        Ok(OidcAuthStart {
            authorization_url: authorize_url.to_string(),
            csrf_token: csrf_token.secret().to_string(),
            nonce: nonce.secret().to_string(),
            pkce_verifier: pkce_code_verifier.secret().to_string(),
        })
    }

    async fn exchange(
        &self,
        code: &str,
        expected_nonce: &str,
        pkce_verifier: &str,
    ) -> Result<OidcClaims, OidcError> {
        let client = CoreClient::from_provider_metadata(
            self.provider_metadata.clone(),
            self.client_id.clone(),
            self.client_secret.clone(),
        )
        .set_redirect_uri(self.redirect_uri.clone());

        let token_request = client
            .exchange_code(AuthorizationCode::new(code.to_string()))
            .map_err(|e| OidcError::Flow(format!("failed to prepare token exchange: {e}")))?;
        let token_response = token_request
            .set_pkce_verifier(PkceCodeVerifier::new(pkce_verifier.to_string()))
            .request_async(&self.http_client)
            .await
            .map_err(|e| OidcError::Flow(format!("failed to exchange authorization code: {e}")))?;

        let id_token = token_response
            .id_token()
            .ok_or_else(|| OidcError::Flow("provider did not return an id_token".to_string()))?;
        let id_token_verifier = client.id_token_verifier();
        let claims = id_token
            .claims(&id_token_verifier, &Nonce::new(expected_nonce.to_string()))
            .map_err(|e| OidcError::Flow(format!("invalid id_token claims: {e}")))?;

        if let Some(expected_hash) = claims.access_token_hash() {
            let signing_alg = id_token.signing_alg().map_err(|e| {
                OidcError::Flow(format!("failed to determine signing algorithm: {e}"))
            })?;
            let signing_key = id_token
                .signing_key(&id_token_verifier)
                .map_err(|e| OidcError::Flow(format!("failed to obtain signing key: {e}")))?;
            let actual_hash = AccessTokenHash::from_token(
                token_response.access_token(),
                signing_alg,
                signing_key,
            )
            .map_err(|e| OidcError::Flow(format!("failed to compute access token hash: {e}")))?;

            if actual_hash != *expected_hash {
                return Err(OidcError::Flow(
                    "access token hash mismatch; possible token substitution".to_string(),
                ));
            }
        }

        let email = claims
            .email()
            .map(|email| email.deref().clone())
            .ok_or_else(|| OidcError::Flow("email claim not provided".to_string()))?;
        let email_verified = claims.email_verified().unwrap_or(false);
        let name = claims
            .name()
            .and_then(|name| name.get(None))
            .map(|name| name.deref().clone());

        Ok(OidcClaims {
            subject: claims.subject().as_str().to_string(),
            email,
            email_verified,
            name,
        })
    }
}
