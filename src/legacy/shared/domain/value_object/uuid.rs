use core::fmt;

use uuid::Uuid as UuidCrate;

#[derive(Clone)]
pub struct Uuid(pub UuidCrate);

impl Default for Uuid {
    fn default() -> Self {
        Self::new()
    }
}

impl Uuid {
    pub fn new() -> Self {
        Self(UuidCrate::new_v4())
    }

    fn hyphenated(&self) -> String {
        self.0.hyphenated().to_string()
    }
}

impl fmt::Display for Uuid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.hyphenated())
    }
}
