use crate::context::{
    shared::errors::domain_error::DomainError, user::domain::errors::value_objects as Error,
};

#[derive(PartialEq, Eq, Clone)]
pub struct Fullname {
    value: String,
}

static MAX_LENGTH: usize = 64;

impl Fullname {
    pub fn parse(raw: &str) -> Result<Fullname, DomainError> {
        let value = raw.trim();

        Self::ensure_is_not_empty(value)?;
        Self::ensure_is_not_max_length(value)?;

        let normalized = Self::collapse_ws(value);

        Ok(Fullname { value: normalized })
    }

    pub fn as_str(&self) -> &str {
        &self.value
    }

    fn ensure_is_not_empty(value: &str) -> Result<(), DomainError> {
        if value.is_empty() {
            return Err(DomainError::new(Error::EMPTY_FULLNAME));
        }
        Ok(())
    }

    fn ensure_is_not_max_length(value: &str) -> Result<(), DomainError> {
        if value.len() > MAX_LENGTH {
            return Err(DomainError::with_ctx(
                Error::INVALID_FULLNAME_LENGTH,
                [("max_length", MAX_LENGTH.to_string())],
            ));
        }
        Ok(())
    }

    fn collapse_ws(value: &str) -> String {
        value.split_whitespace().collect::<Vec<_>>().join(" ")
    }
}
