use std::{error, fmt};

use crate::DomainError;

pub struct Quantity {
    value: i32,
}

#[derive(Debug, PartialEq, Eq)]
pub enum QuantityError {
    Negative { quantity: i32 },
}

impl Quantity {
    pub fn new(value: i32) -> Result<Self, QuantityError> {
        Self::ensure_is_positive(value)?;

        Ok(Self { value })
    }

    pub fn value(&self) -> i32 {
        self.value
    }

    fn ensure_is_positive(value: i32) -> Result<(), QuantityError> {
        if value <= 0 {
            return Err(QuantityError::Negative { quantity: value });
        }
        Ok(())
    }
}

impl fmt::Display for QuantityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Negative { quantity } => write!(f, "Negative quantity: {}", quantity),
        }
    }
}

impl error::Error for QuantityError {}

impl DomainError for QuantityError {
    fn code(&self) -> &'static str {
        match self {
            Self::Negative { .. } => "QUANTITY::NEGATIVE",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quantity_not_allowed_zeros() {
        let result = Quantity::new(0);

        assert!(result.is_err());
    }

    #[test]
    fn quantity_not_allowed_negatives() {
        let result = Quantity::new(-1);

        assert!(result.is_err());
    }

    #[test]
    fn quantity_is_valid_value() {
        let result = Quantity::new(1);

        assert!(result.is_ok());
    }
}
