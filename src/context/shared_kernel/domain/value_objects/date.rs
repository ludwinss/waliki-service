use chrono::{DateTime, Duration, NaiveDate, Utc};

use crate::{
    code,
    context::shared_kernel::errors::{code::Code, domain_error::DomainError},
};

const DATE_FORMAT_ISO: Code = code!("SHARED_KERNEL", "DATE", "VO", "001");

#[derive(PartialEq, Debug, Clone)]
pub struct Date(DateTime<Utc>);

impl Date {
    pub fn now() -> Self {
        Self(Utc::now())
    }

    pub fn from_iso_string(raw: &str) -> Result<Date, DomainError> {
        Self::ensure_is_string_iso_utc(raw)?;
        let date_utc: DateTime<Utc> = raw.parse().unwrap();
        Ok(Date(date_utc))
    }

    pub fn as_date(&self) -> NaiveDate {
        self.0.date_naive()
    }

    pub fn add_days(&mut self, days: i64) {
        self.0 += Duration::days(days);
    }

    fn ensure_is_string_iso_utc(raw: &str) -> Result<(), DomainError> {
        if !raw.ends_with("Z") {
            return Err(DomainError::with_ctx(DATE_FORMAT_ISO, vec![]));
        }

        match DateTime::parse_from_rfc3339(raw) {
            Ok(datetime) => {
                if datetime.offset().local_minus_utc() != 0 {
                    return Err(DomainError::new(DATE_FORMAT_ISO));
                }
                Ok(())
            }
            Err(_) => Err(DomainError::with_ctx(DATE_FORMAT_ISO, vec![])),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn must_instance_naive_date_without_time() {
        let dt = Utc::now();
        let date = Date(dt);

        let only_date = date.as_date();

        assert_eq!(only_date, dt.date_naive());
    }

    #[test]
    fn must_add_days_in_date() {
        let mut date = Date::now();
        const DAYS: i64 = 3;
        date.add_days(DAYS);
        assert_eq!(
            date.as_date(),
            Utc::now().date_naive() + Duration::days(DAYS)
        );
    }

    #[test]
    fn must_create_date_from_iso_string() {
        let date = Date::from_iso_string("2022-01-01T00:00:00Z").unwrap();

        assert_eq!(date.as_date(), NaiveDate::from_ymd_opt(2022, 1, 1).unwrap());
    }
}
