use std::fmt;

use uuid::Uuid as UuidCrate;

#[derive(Clone)]
pub struct Uuid {
    value: UuidCrate,
}

impl Uuid {
    pub fn new() -> Self {
        Self {
            value: UuidCrate::new_v4(),
        }
    }

    fn hyphenated(&self) -> String {
        self.value.hyphenated().to_string()
    }
}

impl fmt::Display for Uuid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.hyphenated())
    }
}

impl From<UuidCrate> for Uuid {
    fn from(u: UuidCrate) -> Self {
        Self { value: u }
    }
}
