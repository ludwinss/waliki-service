use crate::context::{
    sales::domain::{
        errors::value_objects::QUANTITY_NOT_POSITIVE, value_objects::currency::Currency,
    },
    shared_kernel::errors::domain_error::DomainError,
};

pub struct Money {
    amount_minor: i64,
    currency: Currency,
}

impl Money {
    pub fn new(amount_minor: i64, currency: Currency) -> Result<Money, DomainError> {
        Self::ensure_is_positive(amount_minor)?;
        Ok(Self {
            amount_minor,
            currency,
        })
    }

    fn ensure_is_positive(value: i64) -> Result<(), DomainError> {
        if value <= 0 {
            return Err(DomainError::new(QUANTITY_NOT_POSITIVE));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn must_create_money() {
        let money = Money::new(10, Currency::BoB).unwrap();
        assert_eq!(money.amount_minor, 10);
    }
    #[test]
    fn must_return_error_create_money_negative() {
        let money = Money::new(0, Currency::BoB);
        assert!(money.is_err());
    }
}
