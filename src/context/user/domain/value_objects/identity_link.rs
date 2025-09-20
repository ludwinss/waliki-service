use crate::context::user::domain::value_objects::{
    external_provider::ExternalProvider, external_subject::ExternalSubject,
};

pub struct IdentityLink {
    subject: ExternalSubject,
    provider: ExternalProvider,
}
