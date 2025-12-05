use actix_web::{Error, HttpRequest, error::ErrorUnauthorized, http::header};

pub mod access_token;
pub mod refresh_token;

pub(crate) fn bearer_token(req: &HttpRequest) -> Result<String, Error> {
    let header_value = req
        .headers()
        .get(header::AUTHORIZATION)
        .ok_or_else(|| ErrorUnauthorized("missing Authorization header"))?;
    let header_value = header_value
        .to_str()
        .map_err(|_| ErrorUnauthorized("invalid Authorization header"))?;

    let mut parts = header_value.split_whitespace();
    match (parts.next(), parts.next()) {
        (Some(scheme), Some(token)) if scheme.eq_ignore_ascii_case("bearer") => {
            Ok(token.to_string())
        }
        _ => Err(ErrorUnauthorized("invalid Authorization header")),
    }
}
