use std::str::FromStr;

use crate::context::{
    shared::errors::domain_error::DomainError,
    user::domain::errors::value_objects::INVALID_EXTERNAL_PROVIDER,
};

#[derive(PartialEq, Eq, Clone, Hash)]
pub enum ExternalProvider {
    Google,
}

impl ExternalProvider {
    pub fn parse(value: String) -> Result<ExternalProvider, DomainError> {
        match value.as_str() {
            "google" => Ok(ExternalProvider::Google),
            _ => Err(DomainError::new(INVALID_EXTERNAL_PROVIDER)),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            ExternalProvider::Google => "google",
        }
    }
}

impl FromStr for ExternalProvider {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "google" => Ok(ExternalProvider::Google),
            _ => Err(DomainError::new(INVALID_EXTERNAL_PROVIDER)),
        }
    }
}
