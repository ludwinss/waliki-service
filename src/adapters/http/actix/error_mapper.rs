use actix_web::HttpResponse;

use crate::context::user::application::errors::AppError;

pub fn to_http_error(err: AppError) -> HttpResponse {
    match err {
        AppError::Validation(e) => HttpResponse::BadRequest().json(format!("{e}")),
        AppError::Repository(e) => HttpResponse::InternalServerError().json(format!("{e}")),
    }
}
