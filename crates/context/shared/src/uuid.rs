use std::fmt;

use uuid::{Uuid as UuidValue, Version};

use crate::DomainError;

pub struct Uuid {
    val: UuidValue,
}

#[derive(Debug)]
pub enum UuidError {
    Invalid,
    IsNotV7,
}

impl Uuid {
    pub fn from(uuid: &str) -> Result<Self, UuidError> {
        let uuid_parsed = Self::ensure_is_valid(uuid)?;
        Self::ensure_is_v7(uuid)?;

        Ok(Self { val: uuid_parsed })
    }

    pub fn value(&self) -> UuidValue {
        self.val
    }

    fn ensure_is_valid(value: &str) -> Result<UuidValue, UuidError> {
        let uuid = UuidValue::parse_str(value).map_err(|_| UuidError::Invalid)?;

        Ok(uuid)
    }

    fn ensure_is_v7(value: &str) -> Result<(), UuidError> {
        let uuid = Self::ensure_is_valid(value)?;

        if uuid.get_version() != Some(Version::SortRand) {
            return Err(UuidError::IsNotV7);
        }

        Ok(())
    }
}

impl fmt::Display for UuidError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Invalid => write!(f, "Uuid is invalid"),
            Self::IsNotV7 => write!(f, "Uuid is not V7"),
        }
    }
}

impl std::error::Error for UuidError {}

impl DomainError for UuidError {
    fn code(&self) -> &'static str {
        match self {
            Self::IsNotV7 => "UUID::IS_NOT_V7",
            UuidError::Invalid => "UUID::INVALID",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uuid_is_not_valid() {
        let result = Uuid::from("7fc275ab-5a08-4582-a4d2-4836743");

        assert!(result.is_err());
    }

    #[test]
    fn uuid_is_not_v7() {
        let result = Uuid::from("7fc275ab-5a08-4582-a4d2-4ccaaf836743");

        assert!(result.is_err());
    }

    #[test]
    fn uuid_is_valid() {
        let result = Uuid::from("01a0688f-6fd1-74e7-a842-b880ff8cca39");

        assert!(result.is_ok());
    }
}
