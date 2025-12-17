use crate::context::{
    sales::domain::errors::value_objects::QUANTITY_NOT_POSITIVE,
    shared_kernel::errors::domain_error::DomainError,
};

#[derive(PartialEq, Debug)]
pub struct Quantity {
    value: i32,
}

impl Quantity {
    pub fn new(value: i32) -> Result<Quantity, DomainError> {
        Self::ensure_is_positive(value)?;
        Ok(Self { value })
    }

    fn ensure_is_positive(value: i32) -> Result<(), DomainError> {
        if value <= 0 {
            return Err(DomainError::new(QUANTITY_NOT_POSITIVE));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn must_return_error_when_quantity_is_not_positive() {
        let result = Quantity::new(-1);

        assert!(
            result.is_err(),
            "expected error for negative quantity but got: {:?}",
            result
        );
    }
}
