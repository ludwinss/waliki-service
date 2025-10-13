use std::sync::Arc;

use crate::context::shared_kernel::infrastructure::adapters::{
    system_clock::SystemClock, uuid_v7_generator::UuidV7Generator,
};
use crate::context::user::{
    application::ports::oidc_flow::OidcFlow,
    application::usecases::login_with_google::{
        LoginWithGoogleUseCase, handler::LoginWithGoogleHandler,
    },
    infrastructure::{
        auth::google_oidc_flow::{GoogleOidcConfig, GoogleOidcFlow},
        postgres::{
            pool::{PgPool, init_pool},
            users::repository::PgUserRepository,
        },
    },
};
use crate::platform::config::api::ApiConfig;
use anyhow::Context;

pub struct AppState {
    pub login_with_google: Arc<dyn LoginWithGoogleUseCase>,
    pub oidc_flow: Arc<dyn OidcFlow>,
}

impl AppState {
    pub fn from_cfg(cfg: &ApiConfig) -> anyhow::Result<Self> {
        let pool: PgPool = init_pool(cfg.common.postgres_uri.as_str())
            .with_context(|| "failed to initialize PostgreSQL pool")?;
        let repo = Arc::new(PgUserRepository::new(pool));
        let clock = Arc::new(SystemClock);
        let id_gen = Arc::new(UuidV7Generator);

        let login_with_google: Arc<dyn LoginWithGoogleUseCase> =
            Arc::new(LoginWithGoogleHandler::new(repo, clock, id_gen));

        let oidc_config = GoogleOidcConfig {
            client_id: cfg.common.oidc_google_client_id.clone(),
            client_secret: Some(cfg.common.oidc_google_client_secret.clone()),
            redirect_uri: cfg.common.oidc_google_redirect_uri.clone(),
            issuer: cfg.common.oidc_google_issuer_uri.clone(),
        };
        let oidc_flow = GoogleOidcFlow::new(oidc_config).map_err(anyhow::Error::from)?;
        let oidc_flow: Arc<dyn OidcFlow> = Arc::new(oidc_flow);

        Ok(Self {
            login_with_google,
            oidc_flow,
        })
    }
}
