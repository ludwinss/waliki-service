use std::sync::Arc;

use actix_session::Session;
use actix_web::http::header::LOCATION;
use actix_web::{HttpResponse, Responder, get, post, web};
use serde::{Deserialize, Serialize};

use crate::adapters::http::actix::{
    error_mapper::to_http_error,
    guards::{access_token::AccessTokenGuard, refresh_token::RefreshTokenGuard},
    user::{dto::login_with_google_out::LoginWithGoogleOut, mapper},
};
use crate::context::shared_kernel::{
    application::{
        ports::token_issuer::Claims,
        services::session_tokens::{SessionTokenError, SessionTokenIssuer},
    },
    domain::value_objects::uuid::Uuid,
};
use crate::context::user::application::{
    ports::oidc_flow::OidcFlow,
    usecases::login_with_google::{
        LoginWithGoogleUseCase, request::LoginWithGoogleRequest, response::LoginWithGoogleResponse,
    },
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
    session_tokens: web::Data<Arc<dyn SessionTokenIssuer>>,
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
    let session_tokens = session_tokens.get_ref().clone();

    match web::block(move || handler_clone.execute(req)).await {
        Ok(Ok(resp)) => match issue_login_tokens(resp, session_tokens.as_ref()) {
            Ok(body) => HttpResponse::Ok().json(body),
            Err(resp) => resp,
        },
        Ok(Err(e)) => to_http_error(e),
        Err(e) => {
            tracing::error!(target = "waliki_service", error = %e, "blocking task error");
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[post("/auth/refresh")]
pub async fn refresh_tokens(
    guard: RefreshTokenGuard,
    session_tokens: web::Data<Arc<dyn SessionTokenIssuer>>,
) -> impl Responder {
    let subject = guard.claims().subject().to_string();
    let user_uuid = match Uuid::parse_str(&subject) {
        Ok(uuid) => uuid,
        Err(_) => {
            tracing::warn!(
                target = "waliki_service",
                subject = %subject,
                "refresh token subject is not a valid UUID"
            );
            return HttpResponse::BadRequest().body("invalid token subject");
        }
    };
    let response = LoginWithGoogleResponse { user_uuid };
    match issue_login_tokens(response, session_tokens.get_ref().as_ref()) {
        Ok(body) => HttpResponse::Ok().json(body),
        Err(err) => err,
    }
}

#[get("/auth/me")]
pub async fn current_session(guard: AccessTokenGuard) -> impl Responder {
    HttpResponse::Ok().json(ClaimsOut::from(guard.claims()))
}

#[derive(Serialize)]
struct ClaimsOut {
    iss: String,
    sub: String,
    aud: String,
    exp: u64,
    nbf: u64,
    iat: u64,
}

impl From<&Claims> for ClaimsOut {
    fn from(claims: &Claims) -> Self {
        Self {
            iss: claims.issuer().to_string(),
            sub: claims.subject().to_string(),
            aud: claims.audience().to_string(),
            exp: claims.expiration().as_secs(),
            nbf: claims.not_before().as_secs(),
            iat: claims.issued_at().as_secs(),
        }
    }
}

fn issue_login_tokens(
    resp: LoginWithGoogleResponse,
    issuer: &dyn SessionTokenIssuer,
) -> Result<LoginWithGoogleOut, HttpResponse> {
    let tokens = issuer
        .issue_for(&resp.user_uuid)
        .map_err(map_session_token_error)?;
    Ok(mapper::to_out(resp, &tokens.access, &tokens.refresh))
}

fn map_session_token_error(err: SessionTokenError) -> HttpResponse {
    match &err {
        SessionTokenError::Clock(e) => {
            tracing::error!(
                target = "waliki_service",
                error = ?e,
                "system clock is behind UNIX_EPOCH"
            );
        }
        SessionTokenError::EncodeAccess(e) => {
            tracing::error!(
                target = "waliki_service",
                error = %e,
                "failed to encode access token"
            );
        }
        SessionTokenError::EncodeRefresh(e) => {
            tracing::error!(
                target = "waliki_service",
                error = %e,
                "failed to encode refresh token"
            );
        }
    }
    HttpResponse::InternalServerError().finish()
}
