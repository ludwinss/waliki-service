use actix_web::web;

use crate::adapters::http::actix::controllers;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api").service(
            web::scope("/auth")
                .route("/google", web::get().to(controllers::user::auth_google))
                .route("/callback", web::get().to(controllers::user::auth_callback)),
        ),
    );
}
