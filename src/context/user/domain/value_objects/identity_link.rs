use crate::context::user::domain::value_objects::{
    external_provider::ExternalProvider, external_subject::ExternalSubject,
};

#[derive(PartialEq, Eq, Clone, Hash)]
pub struct IdentityLink {
    subject: ExternalSubject,
    provider: ExternalProvider,
}

impl IdentityLink {
    pub fn new(provider: ExternalProvider, sub: ExternalSubject) -> Self {
        IdentityLink {
            subject: sub,
            provider,
        }
    }

    pub fn subject(&self) -> &ExternalSubject {
        &self.subject
    }

    pub fn provider(&self) -> &ExternalProvider {
        &self.provider
    }
}
