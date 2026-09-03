use std::fmt::{Debug, Display};

pub trait DomainError: Display + Debug + Send + Sync + 'static {
    fn code(&self) -> &'static str;
}
