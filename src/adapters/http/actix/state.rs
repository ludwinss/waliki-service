use std::sync::Arc;

use crate::context::user::domain::services::oidc_flow::OidcFlow;
use crate::context::user::infraestructure::auth::google_oidc_flow::GoogleOidcFlow;
use crate::platform::config::api::ApiConfig;

pub struct AppState {
    pub oidc_flow: Arc<dyn OidcFlow>,
}

impl AppState {
    pub async fn from_cfg(cfg: &ApiConfig) -> Self {
        let flow = GoogleOidcFlow::discover(
            &cfg.common.oidc_google_issuer_uri,
            cfg.common.oidc_google_client_id.clone(),
            Some(cfg.common.oidc_google_client_secret.clone()),
            cfg.common.oidc_google_redirect_uri.clone(),
        )
        .await
        .expect("OIDC discovery failed");

        Self {
            oidc_flow: Arc::new(flow),
        }
    }
}
