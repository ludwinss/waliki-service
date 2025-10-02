use once_cell::sync::Lazy;
use regex::Regex;
use std::fmt;
use tracing::{debug, warn};

use crate::context::shared::errors::domain_error::DomainError;
use crate::context::user::domain::errors::value_objects as Errors;

static EMAIL_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?i)^[a-z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?(?:\.[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?)*$")
        .expect("InternalError:: failed to create regex")
});

#[derive(PartialEq, Eq, Clone)]
pub struct Email {
    value: String,
}

impl Email {
    const MAX_LENGTH: usize = 254;

    pub fn parse(raw: &str) -> Result<Email, DomainError> {
        debug!(target:"email", raw=?raw, "parse: raw input");
        let value = raw.trim();
        debug!(target:"email", trimmed=?value, len=value.len(), "parse: after trim");

        Self::ensure_is_not_empty(value)?;
        Self::ensure_format_is_valid(value)?;
        Self::ensure_length_is_valid(value)?;

        let normalized = Self::lowercase_domain(&Email {
            value: value.to_string(),
        });
        debug!(target:"email", normalized=?normalized, "parse: normalized (domain lowercased)");
        Ok(Email { value: normalized })
    }

    pub fn as_str(&self) -> &str {
        &self.value
    }

    fn ensure_is_not_empty(value: &str) -> Result<(), DomainError> {
        if value.is_empty() {
            warn!(target:"email", "empty email");
            return Err(DomainError::new(Errors::EMPTY_EMAIL));
        }
        Ok(())
    }

    fn ensure_format_is_valid(value: &str) -> Result<(), DomainError> {
        let ok = EMAIL_REGEX.is_match(value);
        debug!(target:"email.regex", value=?value, ok, "regex check");
        if !ok {
            debug!(
                target:"email.regex",
                has_at=?value.contains('@'),
                many_at=%(value.matches('@').count()),
                has_unicode=%(!value.is_ascii()),
                domain_has_underscore=%value.split('@').nth(1).map(|d| d.contains('_')).unwrap_or(false),
                "format hints"
            );
            return Err(DomainError::new(Errors::INVALID_EMAIL));
        }
        Ok(())
    }

    fn ensure_length_is_valid(value: &str) -> Result<(), DomainError> {
        if value.len() > Self::MAX_LENGTH {
            warn!(target:"email", len=value.len(), max=Self::MAX_LENGTH, "too long");
            return Err(DomainError::with_ctx(
                Errors::INVALID_EMAIL_LENGTH,
                [("max_length", Self::MAX_LENGTH.to_string())],
            ));
        }
        Ok(())
    }

    fn lowercase_domain(&self) -> String {
        match self.value.rsplit_once('@') {
            Some((local, domain)) => format!("{}@{}", local, domain.to_ascii_lowercase()),
            None => self.value.to_string(),
        }
    }
}

impl fmt::Display for Email {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}
