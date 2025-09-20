use thiserror::Error;

#[derive(Error, Debug)]
pub enum EmailError {
    #[error("Email inválido: {0}")]
    InvalidFormat(String),
    #[error("Email vacío")]
    Empty,
    #[error("Email demasiado largo: Máximo {0} caracteres")]
    InvalidLength(usize),
}

#[derive(Error, Debug)]
pub enum IdentityError {
    #[error("Email inválido: {0}")]
    InvalidFormat(String),
    #[error("Email vacío")]
    Empty,
    #[error("Email demasiado largo: Máximo {0} caracteres")]
    InvalidLength(usize),
}
