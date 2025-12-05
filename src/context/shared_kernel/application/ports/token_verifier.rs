use anyhow::Result;

use super::token_issuer::Claims;

pub trait TokenVerifier: Send + Sync {
    fn verify_access(&self, token: &str) -> Result<Claims>;
    fn verify_refresh(&self, token: &str) -> Result<Claims>;
}
