pub mod error_mapper;
pub mod guards;
pub mod routes;
pub mod state;
pub mod user;

use actix_session::{SessionMiddleware, storage::CookieSessionStore};
use actix_web::cookie::{Key, SameSite};
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer, web};

use crate::adapters::http::actix;
use crate::adapters::http::actix::state::AppState;
use crate::platform::config::helpers::parse_secret_key;
use crate::platform::logger::LogOptions;
use crate::platform::{self, logger};

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    let cfg = platform::config::api::load_api_config();

    logger::init(LogOptions::default());

    let state = match AppState::from_cfg(&cfg).await {
        Ok(state) => state,
        Err(e) => {
            eprintln!("failed to initialize app state: {e:?}");
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "failed to initialize app state",
            ));
        }
    };
    let AppState {
        login_with_google,
        oidc_flow,
        session_tokens,
        token_verifier,
    } = state;
    let login_with_google = web::Data::new(login_with_google);
    let oidc_flow = web::Data::new(oidc_flow);
    let session_tokens = web::Data::new(session_tokens);
    let token_verifier = web::Data::new(token_verifier);

    let secret_bytes = parse_secret_key(&cfg.common.secret_key);
    let secret_key = Key::from(&secret_bytes);

    tracing::info!(target = "waliki_service", host = %cfg.host, "listening");
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a \"%r\" %s %b %T"))
            .app_data(login_with_google.clone())
            .app_data(oidc_flow.clone())
            .app_data(session_tokens.clone())
            .app_data(token_verifier.clone())
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                    .cookie_secure(cfg.cookie_secure)
                    .cookie_same_site(SameSite::Lax)
                    .build(),
            )
            .configure(actix::routes::configure)
    })
    .workers(1)
    .bind(&cfg.host)?
    .run()
    .await
}
