use openidconnect::core::{CoreClient, CoreProviderMetadata};
use openidconnect::reqwest::async_http_client;
use openidconnect::{ClientId, ClientSecret, IssuerUrl, RedirectUrl};

use crate::platform::config::api::ApiConfig;

#[derive(Clone, Debug)]
pub struct AppState {
    #[allow(dead_code)]
    pub oidc: CoreClient,
    #[allow(dead_code)]
    pub client_id: String,
}

impl AppState {
    pub async fn from_cfg(cfg: &ApiConfig) -> Self {
        let issuer =
            IssuerUrl::new(cfg.common.oidc_google_issuer_uri.clone()).expect("invalid issuer url");

        let metadata = CoreProviderMetadata::discover_async(issuer, async_http_client)
            .await
            .expect("OIDC discovery failed");

        let client = CoreClient::from_provider_metadata(
            metadata,
            ClientId::new(cfg.common.oidc_google_client_id.clone()),
            Some(ClientSecret::new(
                cfg.common.oidc_google_client_secret.clone(),
            )),
        )
        .set_redirect_uri(
            RedirectUrl::new(cfg.common.oidc_google_redirect_uri.clone())
                .expect("invalid redirect URI"),
        );

        Self {
            oidc: client,
            client_id: cfg.common.oidc_google_client_id.clone(),
        }
    }
}
