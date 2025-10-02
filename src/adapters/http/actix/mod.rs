pub mod controllers;
pub mod dto;
pub mod routes;
pub mod state;

use actix_session::{SessionMiddleware, storage::CookieSessionStore};
use actix_web::cookie::{Key, SameSite};
use actix_web::middleware::Logger;
use actix_web::rt::task::spawn_blocking;
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

    let state = {
        let cfg_clone = cfg.clone();
        spawn_blocking(move || AppState::from_cfg(&cfg_clone))
            .await
            .map_err(|e| {
                eprintln!("spawn_blocking(AppState::from_cfg) panicked: {e}");
                std::io::Error::new(std::io::ErrorKind::Other, "OIDC init task panicked")
            })?
    };
    let state = web::Data::new(state);

    let secret_bytes = parse_secret_key(&cfg.common.secret_key);
    let secret_key = Key::from(&secret_bytes);

    tracing::info!(target = "waliki_service", host = %cfg.host, "listening");
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a \"%r\" %s %b %T"))
            .app_data(state.clone())
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                    .cookie_secure(cfg.cookie_secure)
                    .cookie_same_site(SameSite::Lax)
                    .build(),
            )
            .configure(actix::routes::config)
    })
    .workers(1)
    .bind(&cfg.host)?
    .run()
    .await
}
