use actix_web::{HttpResponse, Responder};

pub async fn auth_google() -> impl Responder {
    HttpResponse::Ok()
}

pub async fn auth_callback() -> impl Responder {
    HttpResponse::Ok()
}
