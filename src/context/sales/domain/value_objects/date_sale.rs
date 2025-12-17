use crate::context::{
    sales::domain::errors::value_objects as Errors,
    shared_kernel::{domain::value_objects::date::Date, errors::domain_error::DomainError},
};

#[derive(PartialEq, Debug)]
pub struct DateSale {
    value: Date,
}

static SALE_EXPIRES_IN_DAYS: i64 = 15;

impl DateSale {
    pub fn create(value: Date) -> Result<DateSale, DomainError> {
        Self::ensure_is_not_in_future(&value)?;
        Self::ensure_is_before(&value)?;
        Ok(Self { value })
    }

    fn ensure_is_not_in_future(value: &Date) -> Result<(), DomainError> {
        let today = Date::now();

        if value.as_date() > today.as_date() {
            return Err(DomainError::new(Errors::DATE_SALE_IN_FUTURE));
        };
        Ok(())
    }

    fn ensure_is_before(value: &Date) -> Result<(), DomainError> {
        let mut value = value.clone();
        let today = Date::now();
        value.add_days(SALE_EXPIRES_IN_DAYS);
        if value.as_date() < today.as_date() {
            return Err(DomainError::new(Errors::DATE_SALE_EXPIRED));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn must_return_error_when_date_is_in_future() {
        let date_in_future = Date::from_iso_string("2030-01-01T00:00:00Z");

        let result = DateSale::create(date_in_future.unwrap());
        assert!(
            result.is_err(),
            "expected error for future date but got: {:?}",
            result
        );
    }

    #[test]
    fn must_return_error_when_date_is_expired() {
        let date_expired = Date::from_iso_string("2022-01-01T00:00:00Z");

        let result = DateSale::create(date_expired.unwrap());
        assert!(
            result.is_err(),
            "expected error for expired date but got: {:?}",
            result
        );
    }

    #[test]
    fn must_create_date_sale() {
        let date = Date::now();
        let result = DateSale::create(date);
        assert!(result.is_ok(), "expected ok but got: {:?}", result);
    }
}
