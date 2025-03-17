use thiserror::Error;

use crate::core::domain::tienda::{
    entities::country::Country, repository::country_repository::CountryRepository,
};

#[derive(Debug, Error)]
pub enum CellphoneError {
    #[error("Formato inválido")]
    InvalidFormat,
    #[error("Se esperaban {expected} dígitos, pero se encontraron {found} para {country_iso}")]
    InvalidPhoneLength {
        expected: usize,
        found: usize,
        country_iso: String,
    },
    #[error("País no soportado")]
    CountryCodeNotSupported,
}

#[derive(Clone)]
pub struct Cellphone {
    country: Country,
    value: String,
}

impl Cellphone {
    pub fn new(
        raw_phone: &str,
        country_repo: &dyn CountryRepository,
    ) -> Result<Self, CellphoneError> {
        if !raw_phone.starts_with("+") {
            return Err(CellphoneError::InvalidFormat);
        }

        let country_founded = match country_repo.find_by_phone_prefix(&raw_phone) {
            Some(country) => country,
            None => return Err(CellphoneError::CountryCodeNotSupported),
        };

        let prefix_len = country_founded.phone_prefix.len();
        let local_part = raw_phone[prefix_len..].trim();
        let local_len = local_part.len();

        if !local_part.chars().all(|c| c.is_ascii_digit()) {
            return Err(CellphoneError::InvalidPhoneLength {
                expected: country_founded.phone_digits,
                found: local_len,
                country_iso: country_founded.iso_code.to_string(),
            });
        }

        Ok(Self {
            country: country_founded,
            value: raw_phone.to_string(),
        })
    }

    pub fn country(&self) -> &Country {
        &self.country
    }

    pub fn local_number(&self) -> &str {
        &self.value
    }

    pub fn full_number(&self) -> String {
        format!("{}{}", self.country.phone_prefix, self.value)
    }
}
