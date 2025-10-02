use crate::context::user::{
    domain::value_objects::{
        external_provider::ExternalProvider, external_subject::ExternalSubject,
        identity_link::IdentityLink,
    },
    infraestructure::postgres::identities::models::NewIdentityRow,
};

use super::models::IdentityRow;

impl TryFrom<IdentityRow> for IdentityLink {
    type Error = anyhow::Error;

    fn try_from(value: IdentityRow) -> Result<Self, Self::Error> {
        let new_provider = ExternalProvider::parse(value.provider)?;
        let new_subject = ExternalSubject::parse(value.subject)?;

        Ok(IdentityLink::new(new_provider, new_subject))
    }
}

impl<'a> From<&'a IdentityLink> for NewIdentityRow<'a> {
    fn from(value: &'a IdentityLink) -> Self {
        NewIdentityRow {
            subject: value.subject().as_str(),
            provider: value.provider().as_str(),
        }
    }
}
