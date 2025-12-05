use std::{
    future::{Ready, ready},
    sync::Arc,
};

use actix_web::{
    Error, FromRequest, HttpRequest,
    dev::Payload,
    error::{ErrorInternalServerError, ErrorUnauthorized},
    web,
};

use crate::adapters::http::actix::guards::bearer_token;
use crate::context::shared_kernel::application::ports::{
    token_issuer::Claims, token_verifier::TokenVerifier,
};

pub struct RefreshTokenGuard {
    claims: Claims,
}

impl RefreshTokenGuard {
    pub fn claims(&self) -> &Claims {
        &self.claims
    }
}

impl FromRequest for RefreshTokenGuard {
    type Error = Error;
    type Future = Ready<Result<Self, Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let token = match bearer_token(req) {
            Ok(token) => token,
            Err(err) => return ready(Err(err)),
        };

        let verifier = match req.app_data::<web::Data<Arc<dyn TokenVerifier>>>() {
            Some(v) => v.get_ref().clone(),
            None => {
                tracing::error!(
                    target = "waliki_service",
                    "token verifier missing from application data"
                );
                return ready(Err(ErrorInternalServerError("token verifier unavailable")));
            }
        };

        ready(match verifier.verify_refresh(&token) {
            Ok(claims) => Ok(Self { claims }),
            Err(e) => {
                tracing::warn!(
                    target = "waliki_service",
                    error = %e,
                    "invalid refresh token"
                );
                Err(ErrorUnauthorized("invalid refresh token"))
            }
        })
    }
}
