use actix_web::HttpResponse;

use crate::context::user::application::errors::AppError;

pub fn to_http_error(err: AppError) -> HttpResponse {
    match err {
        AppError::Validation(e) => {
            let msg = e.to_string();
            tracing::warn!(
                target = "waliki_service",
                error = %msg,
                "http request failed validation"
            );
            HttpResponse::BadRequest().json(msg)
        }
        AppError::Repository(e) => {
            let msg = e.to_string();
            tracing::error!(
                target = "waliki_service",
                error = %msg,
                "http request failed due to repository error"
            );
            HttpResponse::InternalServerError().json(msg)
        }
    }
}
