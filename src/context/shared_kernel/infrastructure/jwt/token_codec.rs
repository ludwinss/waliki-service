use std::time::Duration;

use anyhow::{Context, Result, anyhow};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

use crate::context::shared_kernel::application::ports::{
    token_issuer::{AccessToken, Claims, RefreshToken, TokenIssuer},
    token_verifier::TokenVerifier,
};

#[derive(Clone)]
pub struct JwtCfg {
    alg: Algorithm,
    iss: String,
    aud: String,
    access_ttl: Duration,
    refresh_ttl: Duration,
}

impl JwtCfg {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        alg: Algorithm,
        iss: impl Into<String>,
        aud: impl Into<String>,
        access_ttl: Duration,
        refresh_ttl: Duration,
    ) -> Self {
        Self {
            alg,
            iss: iss.into(),
            aud: aud.into(),
            access_ttl,
            refresh_ttl,
        }
    }

    pub fn algorithm(&self) -> Algorithm {
        self.alg
    }

    pub fn issuer(&self) -> &str {
        &self.iss
    }

    pub fn audience(&self) -> &str {
        &self.aud
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct JwtStdClaims {
    sub: String,
    aud: String,
    iss: String,
    nbf: u64,
    exp: u64,
    iat: u64,
}

impl JwtStdClaims {
    fn from_claims(c: &Claims, exp: Duration) -> Self {
        Self {
            sub: c.subject().to_owned(),
            aud: c.audience().to_owned(),
            iss: c.issuer().to_owned(),
            nbf: c.not_before().as_secs(),
            exp: exp.as_secs(),
            iat: c.issued_at().as_secs(),
        }
    }
}

impl TryFrom<JwtStdClaims> for Claims {
    type Error = anyhow::Error;

    fn try_from(value: JwtStdClaims) -> Result<Self> {
        Ok(Claims::new(
            value.iss,
            value.sub,
            value.aud,
            Duration::from_secs(value.exp),
            Duration::from_secs(value.nbf),
            Duration::from_secs(value.iat),
        ))
    }
}

pub struct JwtTokenCodec {
    enc: EncodingKey,
    dec: DecodingKey,
    cfg: JwtCfg,
}

impl JwtTokenCodec {
    pub fn hs256(secret: &[u8], cfg: JwtCfg) -> Self {
        Self::new(
            EncodingKey::from_secret(secret),
            DecodingKey::from_secret(secret),
            cfg,
        )
    }

    pub fn new(enc: EncodingKey, dec: DecodingKey, cfg: JwtCfg) -> Self {
        Self { enc, dec, cfg }
    }

    fn header(&self) -> Header {
        Header::new(self.cfg.algorithm())
    }

    fn validation(&self) -> Validation {
        let mut validation = Validation::new(self.cfg.algorithm());
        validation.set_audience(&[self.cfg.audience().to_string()]);
        validation.set_issuer(&[self.cfg.issuer().to_string()]);
        validation
    }

    fn encode_with_exp(&self, c: &Claims, exp: Duration) -> Result<String> {
        let claims = JwtStdClaims::from_claims(c, exp);
        encode(&self.header(), &claims, &self.enc).context("failed to encode JWT")
    }

    fn decode_token(&self, token: &str) -> Result<Claims> {
        let validation = self.validation();
        let data = decode::<JwtStdClaims>(token, &self.dec, &validation)
            .context("failed to decode JWT")?;
        Claims::try_from(data.claims)
    }

    fn apply_ttl(base: Duration, ttl: Duration) -> Result<Duration> {
        base.checked_add(ttl)
            .ok_or_else(|| anyhow!("failed to compute expiration with ttl"))
    }
}

impl TokenIssuer for JwtTokenCodec {
    fn encode_access(&self, c: &Claims) -> Result<AccessToken> {
        let exp = Self::apply_ttl(c.expiration(), self.cfg.access_ttl)?;
        let token = self.encode_with_exp(c, exp)?;
        Ok(AccessToken::new(token, self.cfg.access_ttl))
    }

    fn encode_refresh(&self, c: &Claims) -> Result<RefreshToken> {
        let exp = Self::apply_ttl(c.expiration(), self.cfg.refresh_ttl)?;
        let token = self.encode_with_exp(c, exp)?;
        Ok(RefreshToken::new(token, self.cfg.refresh_ttl))
    }
}

impl TokenVerifier for JwtTokenCodec {
    fn verify_access(&self, token: &str) -> Result<Claims> {
        self.decode_token(token)
    }

    fn verify_refresh(&self, token: &str) -> Result<Claims> {
        self.decode_token(token)
    }
}
