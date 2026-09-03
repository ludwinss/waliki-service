use std::{error, fmt};

use rust_decimal::Decimal;

use crate::DomainError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Currency {
    Bob,
    Usd,
    Sol,
    Brl,
}

#[derive(Debug, PartialEq, Eq)]
pub enum MoneyError {
    Negative { value: Decimal },
    CurrencyMismatch { left: Currency, right: Currency },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Money {
    value: Decimal,
    currency: Currency,
}

impl Money {
    pub fn new(value: Decimal, currency: Currency) -> Result<Self, MoneyError> {
        Self::ensure_not_negative(value)?;

        Ok(Self { value, currency })
    }

    pub fn signed(value: Decimal, currency: Currency) -> Self {
        Self { value, currency }
    }

    pub fn value(&self) -> Decimal {
        self.value
    }

    pub fn currency(&self) -> Currency {
        self.currency
    }

    pub fn is_negative(&self) -> bool {
        self.value.is_sign_negative()
    }

    pub fn add(&self, other: &Money) -> Result<Money, MoneyError> {
        self.ensure_same_currency(other)?;

        Ok(Money::signed(self.value + other.value, self.currency))
    }

    pub fn sub(&self, other: &Money) -> Result<Money, MoneyError> {
        self.ensure_same_currency(other)?;

        Ok(Money::signed(self.value - other.value, self.currency))
    }

    fn ensure_not_negative(value: Decimal) -> Result<(), MoneyError> {
        if value.is_sign_negative() {
            return Err(MoneyError::Negative { value });
        }
        Ok(())
    }

    fn ensure_same_currency(&self, other: &Money) -> Result<(), MoneyError> {
        if self.currency != other.currency {
            return Err(MoneyError::CurrencyMismatch {
                left: self.currency,
                right: other.currency,
            });
        }
        Ok(())
    }
}

impl fmt::Display for MoneyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Negative { value } => write!(f, "Money must not be negative: {}", value),
            Self::CurrencyMismatch { left, right } => {
                write!(f, "Currency mismatch: {:?} vs {:?}", left, right)
            }
        }
    }
}

impl error::Error for MoneyError {}

impl DomainError for MoneyError {
    fn code(&self) -> &'static str {
        match self {
            Self::Negative { .. } => "money is negative",
            Self::CurrencyMismatch { .. } => "money currency mismatch",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dec(raw: &str) -> Decimal {
        raw.parse().unwrap()
    }

    #[test]
    fn new_rejects_negative() {
        let result = Money::new(dec("-1"), Currency::Bob);

        assert_eq!(result, Err(MoneyError::Negative { value: dec("-1") }));
    }

    #[test]
    fn new_accepts_zero_and_positive() {
        assert!(Money::new(dec("0"), Currency::Bob).is_ok());
        assert!(Money::new(dec("10.50"), Currency::Bob).is_ok());
    }

    #[test]
    fn signed_allows_negative() {
        let diff = Money::signed(dec("-5"), Currency::Bob);

        assert!(diff.is_negative());
    }

    #[test]
    fn decimal_sum_is_exact() {
        let a = Money::new(dec("0.1"), Currency::Bob).unwrap();
        let b = Money::new(dec("0.2"), Currency::Bob).unwrap();

        assert_eq!(a.add(&b).unwrap().value(), dec("0.3"));
    }

    #[test]
    fn add_rejects_currency_mismatch() {
        let a = Money::new(dec("1"), Currency::Bob).unwrap();
        let b = Money::new(dec("1"), Currency::Usd).unwrap();

        assert!(a.add(&b).is_err());
    }
}
