use crate::context::{
    sales::domain::errors::value_objects as Errors,
    shared_kernel::{domain::value_objects::date::Date, errors::domain_error::DomainError},
};

#[derive(PartialEq, Debug)]
struct DateSale {
    value: Date,
}

static SALE_EXPIRES_IN_DAYS: i64 = 15;

impl DateSale {
    pub fn create(value: Date) -> Result<DateSale, DomainError> {
        let value = Date::now();
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
        if value.as_date() > today.as_date() {
            return Err(DomainError::new(Errors::DATE_SALE_EXPIRED));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ensure_is_not_in_future() {
        let value = Date::now();
        let result = DateSale::ensure_is_not_in_future(&value);
        assert!(result.is_err());
    }
}
