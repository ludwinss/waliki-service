use std::{sync::Arc, time::Duration};

use jsonwebtoken::Algorithm;

use crate::context::shared_kernel::{
    application::{
        ports::{token_issuer::TokenIssuer, token_verifier::TokenVerifier},
        services::session_tokens::{JwtClaimsContext, JwtSessionTokenIssuer, SessionTokenIssuer},
    },
    infrastructure::{
        adapters::{system_clock::SystemClock, uuid_v7_generator::UuidV7Generator},
        jwt::token_codec::{JwtCfg, JwtTokenCodec},
    },
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
use crate::platform::config::{api::ApiConfig, helpers::parse_secret_key};
use anyhow::Context;

pub struct AppState {
    pub login_with_google: Arc<dyn LoginWithGoogleUseCase>,
    pub oidc_flow: Arc<dyn OidcFlow>,
    pub session_tokens: Arc<dyn SessionTokenIssuer>,
    pub token_verifier: Arc<dyn TokenVerifier>,
}

impl AppState {
    pub async fn from_cfg(cfg: &ApiConfig) -> anyhow::Result<Self> {
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
        let oidc_flow = GoogleOidcFlow::new(oidc_config)
            .await
            .map_err(anyhow::Error::from)?;
        let oidc_flow: Arc<dyn OidcFlow> = Arc::new(oidc_flow);

        let jwt_secret = parse_secret_key(&cfg.common.secret_key);
        let jwt_cfg = JwtCfg::new(
            Algorithm::HS256,
            cfg.common.jwt_issuer.clone(),
            cfg.common.jwt_audience.clone(),
            Duration::from_secs(cfg.common.jwt_access_ttl_secs),
            Duration::from_secs(cfg.common.jwt_refresh_ttl_secs),
        );
        let token_codec = Arc::new(JwtTokenCodec::hs256(&jwt_secret, jwt_cfg));
        let token_issuer: Arc<dyn TokenIssuer> = token_codec.clone();
        let token_verifier: Arc<dyn TokenVerifier> = token_codec.clone();

        let jwt_claims = JwtClaimsContext {
            issuer: cfg.common.jwt_issuer.clone(),
            audience: cfg.common.jwt_audience.clone(),
        };
        let session_tokens: Arc<dyn SessionTokenIssuer> =
            Arc::new(JwtSessionTokenIssuer::new(token_issuer.clone(), jwt_claims));

        Ok(Self {
            login_with_google,
            oidc_flow,
            session_tokens,
            token_verifier,
        })
    }
}
