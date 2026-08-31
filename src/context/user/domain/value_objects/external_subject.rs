use crate::context::{
    shared_kernel::errors::domain_error::DomainError,
    user::domain::errors::value_objects::{
        EMPTY_EXTERNAL_SUBJECT, INVALID_EXTERNAL_SUBJECT, INVALID_EXTERNAL_SUBJECT_LENGTH,
    },
};

#[derive(PartialEq, Eq, Clone, Hash)]
pub struct ExternalSubject(String);

impl ExternalSubject {
    pub fn parse(raw: String) -> Result<ExternalSubject, DomainError> {
        let value = raw.trim();
        Self::ensure_is_not_empty(value)?;
        Self::ensure_length_is_valid(value)?;
        Self::ensure_is_valid(value)?;
        Ok(ExternalSubject(value.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    fn ensure_is_not_empty(value: &str) -> Result<(), DomainError> {
        if value.is_empty() {
            return Err(DomainError::new(EMPTY_EXTERNAL_SUBJECT));
        }
        Ok(())
    }

    fn ensure_length_is_valid(value: &str) -> Result<(), DomainError> {
        if value.len() > 255 {
            return Err(DomainError::with_ctx(
                INVALID_EXTERNAL_SUBJECT_LENGTH,
                [("value", value.to_string())],
            ));
        }
        Ok(())
    }

    fn ensure_is_valid(value: &str) -> Result<(), DomainError> {
        if value.chars().any(|c| c.is_control()) {
            return Err(DomainError::new(INVALID_EXTERNAL_SUBJECT));
        }
        Ok(())
    }
}
