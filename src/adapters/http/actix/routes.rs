use actix_web::web;

use crate::adapters::http::actix::user::controllers::auth_controller::{
    current_session, google_auth_start, google_callback, refresh_tokens,
};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(google_auth_start)
            .service(google_callback)
            .service(refresh_tokens)
            .service(current_session),
    );
}
