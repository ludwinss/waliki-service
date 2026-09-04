#![allow(dead_code)]
use std::fmt;

use shared::DomainError;

pub struct Name {
    value: String,
}

#[derive(Debug)]
enum NameError {
    Empty,
    HasSpaces,
    MaxLength {
        max_length: usize,
        current_length: usize,
    },
}

const MAX_LENGTH: usize = 255;

impl Name {
    pub fn new(value: String) -> Self {
        Self { value }
    }

    fn ensure_not_empty(value: &str) -> Result<(), NameError> {
        if value.is_empty() {
            return Err(NameError::Empty);
        }
        Ok(())
    }

    fn ensure_has_no_spaces(value: &str) -> Result<(), NameError> {
        if value != value.trim() {
            return Err(NameError::HasSpaces);
        }

        Ok(())
    }

    fn ensure_max_length(value: &str) -> Result<(), NameError> {
        if value.len() > MAX_LENGTH {
            return Err(NameError::MaxLength {
                max_length: MAX_LENGTH,
                current_length: value.len(),
            });
        }
        Ok(())
    }
}

impl fmt::Display for NameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NameError::Empty => write!(f, "Name cannot be empty"),
            NameError::HasSpaces => write!(f, "Name cannot have spaces"),
            NameError::MaxLength {
                max_length,
                current_length,
            } => write!(
                f,
                "Name cannot be longer than {} characters ({} provided)",
                max_length, current_length
            ),
        }
    }
}

impl std::error::Error for NameError {}

impl DomainError for NameError {
    fn code(&self) -> &'static str {
        match self {
            NameError::Empty => "NAME::EMPTY",
            NameError::HasSpaces => "NAME::HAS_SPACES",
            NameError::MaxLength { .. } => "NAME::MAX_LENGTH",
        }
    }
}
