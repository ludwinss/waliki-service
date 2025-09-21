use thiserror::Error;

// #[derive(Error, Debug)]
// pub enum EmailError {
//     #[error("Email inválido: {0}")]
//     InvalidFormat(String),
//     #[error("Email vacío")]
//     Empty,
//     #[error("Email demasiado largo: Máximo {0} caracteres")]
//     InvalidLength(usize),
// }

#[derive(Error, Debug)]
pub enum IdentityError {
    #[error("invalid id token")]
    InvalidIdToken,
    #[error("invalid issuer or audience")]
    InvalidIssuerOrAudience,
    #[error("id token expired or invalid timestamps")]
    InvalidTimestamps,
    #[error("oidc network error")]
    OidcNetwork,
    #[error("unexpected oidc error: {0}")]
    OidcOther(String),
    #[error("not found email")]
    OidcNotFoundEmail,
}
