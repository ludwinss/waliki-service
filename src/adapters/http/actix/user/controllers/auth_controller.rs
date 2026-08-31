use std::sync::Arc;

use actix_web::{HttpResponse, Responder, post, web};

use crate::adapters::http::actix::{
    error_mapper::to_http_error,
    user::{dto::login_with_google_in::LoginWithGoogleIn, mapper},
};
use crate::context::user::application::usecases::login_with_google::LoginWithGoogleUseCase;

#[post("/auth/google/callback")]
pub async fn google_callback(
    handler: web::Data<Arc<dyn LoginWithGoogleUseCase>>,
    payload: web::Json<LoginWithGoogleIn>,
) -> impl Responder {
    let req = mapper::to_app(payload.into_inner());
    match handler.execute(req) {
        Ok(resp) => HttpResponse::Ok().json(mapper::to_out(resp)),
        Err(e) => to_http_error(e),
    }
}
