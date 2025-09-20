use regex::Regex;

use crate::context::user::domain::errors::EmailError;

pub struct Email {
    value: String,
}

impl Email {
    const EMAIL_PATTERN: &str = r"/[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*@(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?/g";

    const MAX_LENGTH: usize = 255;

    fn ensure_is_not_empty(value: &str) -> Result<(), EmailError> {
        if value.is_empty() {
            return Err(EmailError::Empty);
        }

        Ok(())
    }

    fn ensure_format_is_valid(value: &str) -> Result<(), EmailError> {
        let reg_exp_email = Regex::new(Self::EMAIL_PATTERN).unwrap();

        if !reg_exp_email.is_match(value) {
            return Err(EmailError::InvalidFormat(value.to_string()));
        }

        Ok(())
    }

    fn ensure_length_is_valid(value: &str) -> Result<(), EmailError> {
        if value.len() > Self::MAX_LENGTH {
            return Err(EmailError::InvalidLength(Self::MAX_LENGTH));
        }

        Ok(())
    }
}
