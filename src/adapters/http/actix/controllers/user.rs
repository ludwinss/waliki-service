use actix_session::Session;
use actix_web::{
    HttpResponse, Responder,
    error::{ErrorBadRequest, ErrorInternalServerError, ErrorUnauthorized},
    http::header,
    web,
};
use tracing::{debug, error, info};

use crate::{
    adapters::http::actix::{
        dto::{auth::GoogleCallbackQuery, login_with_google::LoginWithGoogle},
        state::AppState,
    },
    platform::utils::hidden_sensible_data::last4,
};

const SESS_CSRF: &str = "oauth_csrf";
const SESS_NONCE: &str = "oauth_nonce";
const SESS_PKCE: &str = "oauth_pkce";

pub async fn auth_google(
    state: web::Data<AppState>,
    session: Session,
) -> actix_web::Result<impl Responder> {
    info!("HTTP /auth/google");
    let init = state.oidc_flow.start_auth();

    session
        .insert(SESS_CSRF, &init.csrf)
        .map_err(ErrorInternalServerError)?;
    session
        .insert(SESS_NONCE, &init.nonce)
        .map_err(ErrorInternalServerError)?;
    session
        .insert(SESS_PKCE, &init.pkce_verifier)
        .map_err(ErrorInternalServerError)?;
    session.renew();

    debug!(
        "redirecting to provider: csrf=***{}, nonce=***{}, pkce=***{}",
        last4(&init.csrf),
        last4(&init.nonce),
        last4(&init.pkce_verifier),
    );

    Ok(HttpResponse::SeeOther()
        .append_header((header::LOCATION, init.auth_url.as_str()))
        .finish())
}

pub async fn auth_callback(
    state: web::Data<AppState>,
    session: Session,
    query: web::Query<GoogleCallbackQuery>,
) -> actix_web::Result<impl Responder> {
    info!("HTTP /auth/callback");
    debug!(
        "state=***{}, code=***{}",
        last4(&query.state),
        last4(&query.code)
    );

    let expected_state: String = session
        .get(SESS_CSRF)
        .map_err(|e| {
            error!("session.get(csrf) error: {e:?}");
            ErrorInternalServerError(e)
        })?
        .ok_or_else(|| {
            error!("missing csrf");
            ErrorUnauthorized("missing csrf")
        })?;
    if expected_state != query.state {
        error!("csrf mismatch");
        return Err(ErrorUnauthorized("invalid state"));
    }
    let pkce: String = session
        .get(SESS_PKCE)
        .map_err(|e| {
            error!("session.get(pkce) error: {e:?}");
            ErrorInternalServerError(e)
        })?
        .ok_or_else(|| {
            error!("missing pkce");
            ErrorBadRequest("missing pkce")
        })?;
    let nonce: String = session
        .get(SESS_NONCE)
        .map_err(|e| {
            error!("session.get(nonce) error: {e:?}");
            ErrorInternalServerError(e)
        })?
        .ok_or_else(|| {
            error!("missing nonce");
            ErrorBadRequest("missing nonce")
        })?;

    let flow = state.oidc_flow.clone();
    let code = query.code.clone();
    let claims = web::block(move || flow.exchange_and_verify(&code, &nonce, &pkce))
        .await
        .map_err(|e| {
            error!("web::block join (oidc): {e:?}");
            ErrorInternalServerError("blocking error")
        })?
        .map_err(|e| {
            error!("OIDC exchange failed: {:#}", e);
            ErrorUnauthorized(e.to_string())
        })?;

    info!(
        "OIDC ok: email={}, sub=***{}",
        claims.email,
        last4(&claims.sub)
    );

    let params = LoginWithGoogle {
        email: claims.email.clone(),
        email_verified: claims.email_verified,
        name: claims.name.clone(),
        sub: claims.sub.clone(),
    };

    let usecase = state.login_with_google.clone();
    let result = web::block(move || usecase.execute(params))
        .await
        .map_err(|e| {
            error!("web::block join (usecase): {e:?}");
            ErrorInternalServerError("blocking error")
        })?
        .map_err(|e| {
            error!("LoginWithGoogle failed: {:#}", e);
            ErrorInternalServerError(e.to_string())
        })?;

    session.remove(SESS_CSRF);
    session.remove(SESS_NONCE);
    session.remove(SESS_PKCE);
    session.renew();

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "user_uuid": result.user_uuid,
        "email_verified": claims.email_verified,
        "name": claims.name,
    })))
}
