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
        let end_with_utc = raw.ends_with("Z");
        let is_iso = NaiveDate::parse_from_str(raw, "%Y-%m-%d").is_ok();

        if (!end_with_utc || !is_iso) {
            return Err(DomainError::with_ctx(
                DATE_FORMAT_ISO,
                vec![("raw", raw.to_string())],
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn as_date_must_be_without_time() {
        let dt = Utc::now();
        let date = Date(dt);

        let only_date = date.as_date();

        assert_eq!(only_date, dt.date_naive());
    }

    #[test]
    fn must_be_x_days() {
        let mut date = Date::now();
        const DAYS: i64 = 3;
        date.add_days(DAYS);
        assert_eq!(
            date.as_date(),
            Utc::now().date_naive() + Duration::days(DAYS)
        );
    }
}
