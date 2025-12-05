use anyhow::Result;
use std::time::Duration;

#[derive(Clone, Debug)]
pub struct AccessToken {
    token: String,
    expires_in: Duration,
}

impl AccessToken {
    pub fn new(token: impl Into<String>, expires_in: Duration) -> Self {
        Self {
            token: token.into(),
            expires_in,
        }
    }

    pub fn value(&self) -> &str {
        &self.token
    }

    pub fn expires_in(&self) -> Duration {
        self.expires_in
    }
}

#[derive(Clone, Debug)]
pub struct RefreshToken {
    token: String,
    expires_in: Duration,
}

impl RefreshToken {
    pub fn new(token: impl Into<String>, expires_in: Duration) -> Self {
        Self {
            token: token.into(),
            expires_in,
        }
    }

    pub fn value(&self) -> &str {
        &self.token
    }

    pub fn expires_in(&self) -> Duration {
        self.expires_in
    }
}

#[derive(Clone, Debug)]
pub struct Claims {
    iss: String,
    sub: String,
    aud: String,
    exp: Duration,
    nbf: Duration,
    iat: Duration,
}

impl Claims {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        iss: impl Into<String>,
        sub: impl Into<String>,
        aud: impl Into<String>,
        exp: Duration,
        nbf: Duration,
        iat: Duration,
    ) -> Self {
        Self {
            iss: iss.into(),
            sub: sub.into(),
            aud: aud.into(),
            exp,
            nbf,
            iat,
        }
    }

    pub fn issuer(&self) -> &str {
        &self.iss
    }

    pub fn subject(&self) -> &str {
        &self.sub
    }

    pub fn audience(&self) -> &str {
        &self.aud
    }

    pub fn expiration(&self) -> Duration {
        self.exp
    }

    pub fn not_before(&self) -> Duration {
        self.nbf
    }

    pub fn issued_at(&self) -> Duration {
        self.iat
    }
}

pub trait TokenIssuer: Send + Sync {
    fn encode_access(&self, c: &Claims) -> Result<AccessToken>;
    fn encode_refresh(&self, c: &Claims) -> Result<RefreshToken>;
}
