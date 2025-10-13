use actix_web::web;

use crate::adapters::http::actix::user::controllers::auth_controller::google_callback;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(google_callback);
}
