use std::sync::Arc;

use actix_session::Session;
use actix_web::http::header::LOCATION;
use actix_web::{HttpResponse, Responder, get, web};
use serde::Deserialize;

use crate::adapters::http::actix::{error_mapper::to_http_error, user::mapper};
use crate::context::user::application::{
    ports::oidc_flow::OidcFlow,
    usecases::login_with_google::{LoginWithGoogleUseCase, request::LoginWithGoogleRequest},
};

const SESSION_CSRF_KEY: &str = "oidc::csrf";
const SESSION_NONCE_KEY: &str = "oidc::nonce";
const SESSION_PKCE_KEY: &str = "oidc::pkce";

#[get("/auth/google")]
pub async fn google_auth_start(
    session: Session,
    flow: web::Data<Arc<dyn OidcFlow>>,
) -> impl Responder {
    match flow.start().await {
        Ok(start) => {
            let redirect_url = start.authorization_url.clone();
            if let Err(e) = session.insert(SESSION_CSRF_KEY, &start.csrf_token) {
                tracing::error!(
                    target = "waliki_service",
                    "failed to store csrf token in session: {e}"
                );
                return HttpResponse::InternalServerError().finish();
            }
            if let Err(e) = session.insert(SESSION_NONCE_KEY, &start.nonce) {
                tracing::error!(
                    target = "waliki_service",
                    "failed to store nonce in session: {e}"
                );
                return HttpResponse::InternalServerError().finish();
            }
            if let Err(e) = session.insert(SESSION_PKCE_KEY, &start.pkce_verifier) {
                tracing::error!(
                    target = "waliki_service",
                    "failed to store pkce verifier in session: {e}"
                );
                return HttpResponse::InternalServerError().finish();
            }
            HttpResponse::Found()
                .append_header((LOCATION, redirect_url))
                .finish()
        }
        Err(e) => {
            tracing::error!(target = "waliki_service", "failed to start oidc flow: {e}");
            HttpResponse::BadGateway().finish()
        }
    }
}

#[derive(Deserialize)]
struct GoogleCallbackQuery {
    state: Option<String>,
    code: Option<String>,
    error: Option<String>,
    #[serde(default)]
    error_description: Option<String>,
}

fn take_session_value(session: &Session, key: &str, label: &str) -> Result<String, HttpResponse> {
    match session.get::<String>(key) {
        Ok(Some(value)) => {
            let _ = session.remove(key);
            Ok(value)
        }
        Ok(None) => {
            tracing::warn!(
                target = "waliki_service",
                session_key = %label,
                "missing OIDC session value"
            );
            Err(HttpResponse::BadRequest().body(format!("missing {label} session value")))
        }
        Err(e) => {
            tracing::error!(
                target = "waliki_service",
                session_key = %label,
                error = %e,
                "failed to read OIDC session value"
            );
            Err(HttpResponse::InternalServerError().finish())
        }
    }
}

#[get("/auth/callback")]
pub async fn google_callback(
    session: Session,
    flow: web::Data<Arc<dyn OidcFlow>>,
    handler: web::Data<Arc<dyn LoginWithGoogleUseCase>>,
    query: web::Query<GoogleCallbackQuery>,
) -> impl Responder {
    let query = query.into_inner();

    if let Some(error) = query.error {
        tracing::warn!(
            target = "waliki_service",
            error = %error,
            description = ?query.error_description,
            "received error from Google authorization callback"
        );
        let _ = session.remove(SESSION_CSRF_KEY);
        let _ = session.remove(SESSION_NONCE_KEY);
        let _ = session.remove(SESSION_PKCE_KEY);
        return HttpResponse::BadRequest().body("authorization failed");
    }

    let returned_state = match query.state {
        Some(state) => state,
        None => {
            tracing::warn!(
                target = "waliki_service",
                "callback missing state parameter"
            );
            return HttpResponse::BadRequest().body("missing state parameter");
        }
    };

    let expected_state = match take_session_value(&session, SESSION_CSRF_KEY, "csrf state") {
        Ok(value) => value,
        Err(resp) => return resp,
    };

    if returned_state != expected_state {
        tracing::warn!(
            target = "waliki_service",
            expected = %expected_state,
            returned = %returned_state,
            "state mismatch in OIDC callback"
        );
        return HttpResponse::BadRequest().body("invalid state parameter");
    }

    let code = match query.code {
        Some(code) => code,
        None => {
            tracing::warn!(target = "waliki_service", "callback missing code parameter");
            return HttpResponse::BadRequest().body("missing authorization code");
        }
    };

    let nonce = match take_session_value(&session, SESSION_NONCE_KEY, "nonce") {
        Ok(value) => value,
        Err(resp) => return resp,
    };

    let pkce = match take_session_value(&session, SESSION_PKCE_KEY, "pkce verifier") {
        Ok(value) => value,
        Err(resp) => return resp,
    };

    let claims = match flow.exchange(&code, &nonce, &pkce).await {
        Ok(claims) => claims,
        Err(e) => {
            tracing::error!(
                target = "waliki_service",
                error = %e,
                "failed to exchange authorization code"
            );
            return HttpResponse::BadGateway().finish();
        }
    };

    let req = LoginWithGoogleRequest {
        sub: claims.subject,
        email: claims.email,
        name: claims.name,
        email_verified: claims.email_verified,
    };

    let handler_clone = handler.get_ref().clone();

    match web::block(move || handler_clone.execute(req)).await {
        Ok(Ok(resp)) => HttpResponse::Ok().json(mapper::to_out(resp)),
        Ok(Err(e)) => to_http_error(e),
        Err(e) => {
            tracing::error!(target = "waliki_service", error = %e, "blocking task error");
            HttpResponse::InternalServerError().finish()
        }
    }
}
