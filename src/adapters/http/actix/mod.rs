pub mod controllers;
pub mod dto;
pub mod routes;
pub mod state;

use actix_session::{SessionMiddleware, storage::CookieSessionStore};
use actix_web::cookie::{Key, SameSite};
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer, web};

use crate::adapters::http::actix;
use crate::adapters::http::actix::state::AppState;
use crate::platform;
use crate::platform::config::helpers::parse_secret_key;

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    let cfg = platform::config::api::load_api_config();

    let state = AppState::from_cfg(&cfg).await;
    let state = web::Data::new(state);

    let secret_bytes = parse_secret_key(&cfg.common.secret_key);
    let secret_key = Key::from(&secret_bytes);

    println!("Listening on {}", cfg.host);
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
