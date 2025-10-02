use std::sync::Arc;

use crate::context::user::app::commands::login_with_google::LoginWithGoogleHandler;
use crate::context::user::app::services::oidc_flow::OidcFlow;
use crate::context::user::infrastructure::auth::google_oidc_flow::GoogleOidcFlow;
use crate::context::user::infrastructure::postgres::pool::init_pool;
use crate::context::user::infrastructure::postgres::users::repository::PgUserRepository;
use crate::platform::config::api::ApiConfig;

#[derive(Clone)]
pub struct AppState {
    pub oidc_flow: Arc<dyn OidcFlow + Send + Sync>,
    pub login_with_google: Arc<LoginWithGoogleHandler>,
}

impl AppState {
    pub fn from_cfg(cfg: &ApiConfig) -> Self {
        let flow = GoogleOidcFlow::discover(
            &cfg.common.oidc_google_issuer_uri,
            cfg.common.oidc_google_client_id.clone(),
            Some(cfg.common.oidc_google_client_secret.clone()),
            cfg.common.oidc_google_redirect_uri.clone(),
        )
        .expect("OIDC discovery failed");

        let pool = init_pool(cfg.common.postgres_uri.as_str());
        let user_repo = Arc::new(PgUserRepository { pool });
        let login_with_google = Arc::new(LoginWithGoogleHandler::new(user_repo));

        Self {
            oidc_flow: Arc::new(flow),
            login_with_google,
        }
    }
}
