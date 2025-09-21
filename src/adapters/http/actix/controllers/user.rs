use actix_session::Session;
use actix_web::{HttpResponse, Responder, web};

use crate::{
    adapters::http::actix::{dto::auth::GoogleCallbackQuery, state::AppState},
    platform::logger::log_warn,
};

const SESS_CSRF: &str = "oauth_csrf";
const SESS_NONCE: &str = "oauth_nonce";
const SESS_PKCE: &str = "oauth_pkce";

pub async fn auth_google(state: web::Data<AppState>, session: Session) -> impl Responder {
    let init = state.oidc_flow.start_auth();
    if session.insert(SESS_CSRF, &init.csrf).is_err()
        || session.insert(SESS_NONCE, &init.nonce).is_err()
        || session.insert(SESS_PKCE, &init.pkce_verifier).is_err()
    {
        log_warn("Failed to insert session data");
        return HttpResponse::InternalServerError().finish();
    }

    session.renew();
    HttpResponse::SeeOther()
        .append_header(("Location", init.auth_url.as_str()))
        .finish()
}

pub async fn auth_callback(
    state: web::Data<AppState>,
    session: Session,
    query: web::Query<GoogleCallbackQuery>,
) -> impl Responder {
    let expected_state: String = match session.get(SESS_CSRF).unwrap_or(None) {
        Some(v) => v,
        None => {
            log_warn("missing csrf");
            return HttpResponse::Unauthorized().finish();
        }
    };

    if expected_state != query.state {
        log_warn("invalid state");
        return HttpResponse::Unauthorized().body("invalid state");
    }
    let pkce: String = match session.get(SESS_PKCE).unwrap_or(None) {
        Some(v) => v,
        None => {
            log_warn("missing pkce");
            return HttpResponse::BadRequest().body("missing pkce");
        }
    };
    let nonce: String = match session.get(SESS_NONCE).unwrap_or(None) {
        Some(v) => v,
        None => {
            log_warn("missing nonce");
            return HttpResponse::BadRequest().body("missing nonce");
        }
    };

    match state
        .oidc_flow
        .exchange_and_verify(&query.code, &nonce, &pkce)
        .await
    {
        Ok(claims) => {
            session.remove(SESS_CSRF);
            session.remove(SESS_NONCE);
            session.remove(SESS_PKCE);
            session.renew();

            HttpResponse::Ok().json(serde_json::json!({
                "sub": claims.sub,
                "email": claims.email,
                "email_verified": claims.email_verified,
                "name": claims.name,
            }))
        }
        Err(e) => {
            log_warn(&e.to_string());
            HttpResponse::Unauthorized().body(e.to_string())
        }
    }
}
