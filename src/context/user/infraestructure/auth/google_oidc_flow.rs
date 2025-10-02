use anyhow::{Context, Result, bail};
use openidconnect::{
    AuthorizationCode, ClientId, ClientSecret, CsrfToken, IssuerUrl, Nonce, PkceCodeChallenge,
    PkceCodeVerifier, RedirectUrl, Scope,
    core::{CoreAuthenticationFlow, CoreClient, CoreProviderMetadata},
};

use crate::context::user::app::services::oidc_flow::{AuthStart, OidcClaims, OidcFlow};
use crate::platform::utils::hidden_sensible_data::last4;

use openidconnect::reqwest::blocking::{Client as OidcHttpClient, ClientBuilder};
use openidconnect::reqwest::redirect::Policy;

use tracing::{debug, error, info, warn};

pub struct GoogleOidcFlow {
    provider: CoreProviderMetadata,
    client_id: String,
    client_secret: Option<ClientSecret>,
    redirect_uri: RedirectUrl,
    http: OidcHttpClient,
}

impl GoogleOidcFlow {
    pub fn discover(
        issuer: &str,
        client_id: String,
        client_secret: Option<String>,
        redirect_uri: String,
    ) -> Result<Self> {
        info!(target: "oidc", issuer, redirect_uri, "OIDC discover");

        debug!(target: "oidc", "Building blocking HTTP client (redirect=none)...");
        let http = ClientBuilder::new()
            .redirect(Policy::none())
            .build()
            .context("http client build")?;

        let issuer = IssuerUrl::new(issuer.to_string()).context("bad issuer")?;

        debug!(target: "oidc", "Fetching provider metadata…");
        let provider = CoreProviderMetadata::discover(&issuer, &http).context("discover")?;

        info!(
            target: "oidc",
            auth_endpoint = %provider.authorization_endpoint(),
            token_endpoint = %provider.token_endpoint().map(|u| u.as_str()).unwrap_or("<none>"),
            "OIDC provider ok"
        );

        let redirect_uri = RedirectUrl::new(redirect_uri).context("bad redirect")?;
        debug!(target: "oidc", redirect = %redirect_uri, "Redirect URL parse ok");

        Ok(Self {
            provider,
            client_id: client_id.clone(),
            client_secret: client_secret.map(ClientSecret::new),
            redirect_uri,
            http,
        })
    }
}

impl OidcFlow for GoogleOidcFlow {
    fn start_auth(&self) -> AuthStart {
        info!(target: "oidc", "Starting OIDC authorization…");

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

        debug!(
            target: "oidc",
            csrf = %format!("***{}", last4(csrf.secret())),
            nonce = %format!("***{}", last4(nonce.secret())),
            pkce  = %format!("***{}", last4(pkce_verifier.secret())),
            redirect = %self.redirect_uri,
            "Auth URL generated"
        );

        AuthStart {
            auth_url: auth_url.to_string(),
            csrf: csrf.secret().to_string(),
            nonce: nonce.secret().to_string(),
            pkce_verifier: pkce_verifier.secret().to_string(),
        }
    }

    fn exchange_and_verify(
        &self,
        code: &str,
        expected_nonce: &str,
        pkce_verifier: &str,
    ) -> Result<OidcClaims> {
        info!(target: "oidc", "Exchanging authorization code for tokens…");
        debug!(
            target: "oidc",
            code = %format!("***{}", last4(code)),
            nonce = %format!("***{}", last4(expected_nonce)),
            pkce  = %format!("***{}", last4(pkce_verifier)),
            "Inputs"
        );

        let client = CoreClient::from_provider_metadata(
            self.provider.clone(),
            ClientId::new(self.client_id.clone()),
            self.client_secret.clone(),
        )
        .set_redirect_uri(self.redirect_uri.clone());

        let token_resp = client
            .exchange_code(AuthorizationCode::new(code.to_string()))?
            .set_pkce_verifier(PkceCodeVerifier::new(pkce_verifier.to_string()))
            .request(&self.http)
            .map_err(|e| {
                error!(target: "oidc", error = ?e, "Token request failed");
                e
            })
            .context("request token")?;

        let id_token = token_resp
            .extra_fields()
            .id_token()
            .cloned()
            .ok_or_else(|| {
                warn!(target: "oidc", "No ID token in token response");
                anyhow::anyhow!("get id token")
            })?;

        info!(target: "oidc", "Verifying ID token claims…");
        let nonce = Nonce::new(expected_nonce.to_string());
        let claims = id_token
            .claims(&client.id_token_verifier(), &nonce)
            .map_err(|e| {
                error!(target: "oidc", error = ?e, "ID token claims verification failed");
                e
            })
            .context("claims")?;

        if !claims
            .audiences()
            .iter()
            .any(|a| a.as_str() == self.client_id)
        {
            let got: Vec<_> = claims.audiences().iter().map(|a| a.as_str()).collect();
            warn!(
                target: "oidc",
                expected = %self.client_id,
                got = ?got,
                "Audience mismatch"
            );
            bail!("audience not found");
        }

        if claims.nonce().map(|n| n.secret()) != Some(nonce.secret()) {
            warn!(target: "oidc", "Nonce mismatch");
            bail!("nonce not ok");
        }

        let email = claims
            .email()
            .map(|e| e.as_str().to_string())
            .ok_or_else(|| {
                warn!(target: "oidc", "Email claim missing");
                anyhow::anyhow!("email")
            })?;

        info!(
            target: "oidc",
            sub_suffix = %last4(claims.subject().as_str()),
            email = %email,
            "OIDC login verified"
        );

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
