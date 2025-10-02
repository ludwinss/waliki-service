use actix_web::web;

use crate::adapters::http::actix::controllers::user;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api").service(
            web::scope("/auth")
                .route("/google", web::get().to(user::auth_google))
                .route("/callback", web::get().to(user::auth_callback)),
        ),
    );
}
