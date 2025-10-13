use std::collections::HashMap;

use crate::context::user::application::errors::RepoError;
use crate::context::user::domain::entities::user::User;
use crate::context::user::domain::value_objects::{
    email::Email, external_provider::ExternalProvider, external_subject::ExternalSubject,
    fullname::Fullname,
};

use super::models::UserRow;
use crate::context::user::infrastructure::postgres::identities::models::UserIdentityRow;

pub fn to_domain(row: UserRow, identities: Vec<UserIdentityRow>) -> Result<User, RepoError> {
    let email = Email::parse(&row.email).map_err(|e| {
        RepoError::new(format!(
            "invalid email stored for user {} (uuid={}): {e}",
            row.id, row.uuid
        ))
    })?;

    let fullname = match row.name.as_deref() {
        Some(value) => Some(Fullname::parse(value).map_err(|e| {
            RepoError::new(format!(
                "invalid fullname stored for user {} (uuid={}): {e}",
                row.id, row.uuid
            ))
        })?),
        None => None,
    };

    let mut identity_map: HashMap<ExternalProvider, ExternalSubject> = HashMap::new();
    for identity in identities {
        let provider_text = identity.provider;
        let subject_text = identity.subject;

        let provider = ExternalProvider::parse(provider_text.as_str()).ok_or_else(|| {
            RepoError::new(format!(
                "unknown identity provider '{provider_text}' for user {} (uuid={})",
                row.id, row.uuid
            ))
        })?;
        let subject = ExternalSubject::parse(subject_text).map_err(|e| {
            RepoError::new(format!(
                "invalid identity subject stored for user {} provider {}: {e}",
                row.id,
                provider.as_str()
            ))
        })?;

        identity_map.insert(provider, subject);
    }

    Ok(User::restore(
        fullname,
        email,
        row.uuid,
        row.email_verified_at,
        identity_map,
    ))
}
