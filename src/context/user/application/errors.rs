use crate::context::shared_kernel::errors::domain_error::DomainError;

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("validation error: {0}")]
    Validation(#[from] DomainError),
    #[error("repository error: {0}")]
    Repository(#[from] RepoError),
}

#[derive(thiserror::Error, Debug)]
#[error("{message}")]
pub struct RepoError {
    pub message: String,
}

impl RepoError {
    pub fn new(msg: impl Into<String>) -> Self {
        Self {
            message: msg.into(),
        }
    }
}

impl From<diesel::result::Error> for RepoError {
    fn from(value: diesel::result::Error) -> Self {
        RepoError::new(format!("database error: {value}"))
    }
}
