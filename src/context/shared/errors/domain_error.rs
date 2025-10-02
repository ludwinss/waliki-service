use std::{
    collections::HashMap,
    error::Error,
    fmt::{self, Display},
};

use super::code::Code;

#[derive(Debug)]
pub struct DomainError {
    code: Code,
    context: Option<HashMap<&'static str, String>>,
}

impl DomainError {
    pub fn new(code: Code) -> Self {
        Self {
            code,
            context: None,
        }
    }

    pub fn with_ctx(code: Code, ctx: impl IntoIterator<Item = (&'static str, String)>) -> Self {
        Self {
            code,
            context: Some(ctx.into_iter().collect()),
        }
    }
}

impl Display for DomainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DomainError({}, {:?})", self.code, self.context)
    }
}

impl Error for DomainError {}
