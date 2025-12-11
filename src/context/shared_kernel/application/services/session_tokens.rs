use std::{
    sync::Arc,
    time::{Duration, SystemTime, SystemTimeError, UNIX_EPOCH},
};

use anyhow::Error;

use crate::context::shared_kernel::{
    application::ports::token_issuer::{AccessToken, Claims, RefreshToken, TokenIssuer},
    domain::value_objects::uuid::Uuid,
};

#[derive(Clone)]
pub struct JwtClaimsContext {
    pub issuer: String,
    pub audience: String,
}

#[derive(Clone)]
pub struct SessionTokens {
    pub access: AccessToken,
    pub refresh: RefreshToken,
}

#[derive(thiserror::Error, Debug)]
pub enum SessionTokenError {
    #[error("system clock error: {0}")]
    Clock(#[from] SystemTimeError),
    #[error("failed to encode access token")]
    EncodeAccess(#[source] Error),
    #[error("failed to encode refresh token")]
    EncodeRefresh(#[source] Error),
}

pub trait SessionTokenIssuer: Send + Sync {
    fn issue_for(&self, subject: &Uuid) -> Result<SessionTokens, SessionTokenError>;
}

#[derive(Clone)]
pub struct JwtSessionTokenIssuer {
    issuer: Arc<dyn TokenIssuer>,
    claims_ctx: JwtClaimsContext,
}

impl JwtSessionTokenIssuer {
    pub fn new(issuer: Arc<dyn TokenIssuer>, claims_ctx: JwtClaimsContext) -> Self {
        Self { issuer, claims_ctx }
    }

    fn build_claims(&self, subject: &Uuid) -> Result<Claims, SessionTokenError> {
        let now = current_epoch()?;
        Ok(Claims::new(
            self.claims_ctx.issuer.clone(),
            subject.to_string(),
            self.claims_ctx.audience.clone(),
            now,
            now,
            now,
        ))
    }
}

impl SessionTokenIssuer for JwtSessionTokenIssuer {
    fn issue_for(&self, subject: &Uuid) -> Result<SessionTokens, SessionTokenError> {
        let claims = self.build_claims(subject)?;
        let access = self
            .issuer
            .encode_access(&claims)
            .map_err(SessionTokenError::EncodeAccess)?;
        let refresh = self
            .issuer
            .encode_refresh(&claims)
            .map_err(SessionTokenError::EncodeRefresh)?;
        Ok(SessionTokens { access, refresh })
    }
}

fn current_epoch() -> Result<Duration, SessionTokenError> {
    Ok(SystemTime::now().duration_since(UNIX_EPOCH)?)
}
