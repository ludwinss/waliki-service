use std::collections::HashSet;

use chrono::{DateTime, Utc};

use crate::context::{
    shared::domain::value_objects::{status::Status, uuid::Uuid},
    user::domain::value_objects::{
        email::Email, external_provider::ExternalProvider, external_subject::ExternalSubject,
        fullname::Fullname, identity_link::IdentityLink,
    },
};

#[derive(Clone)]
pub struct User {
    uuid: Uuid,
    fullname: Option<Fullname>,
    email: Email,
    identities: HashSet<IdentityLink>,
    email_verified_at: Option<DateTime<chrono::Utc>>,
    status: Status,
}

impl User {
    pub fn new(fullname: Option<Fullname>, email: Email, uuid: Uuid) -> Self {
        Self {
            uuid,
            fullname,
            email,
            email_verified_at: None,
            identities: HashSet::new(),
            status: Status::Active,
        }
    }

    pub fn rename(&mut self, fullname: Fullname) {
        self.fullname = Some(fullname);
    }

    pub fn mark_email_verified_now(&mut self) {
        if self.email_verified_at.is_none() {
            self.email_verified_at = Some(Utc::now());
            // TODO: add event
        }
    }

    pub fn email(&self) -> &Email {
        &self.email
    }
    pub fn uuid(&self) -> &Uuid {
        &self.uuid
    }
    pub fn email_verified_at(&self) -> Option<&DateTime<Utc>> {
        self.email_verified_at.as_ref()
    }
    pub fn fullname(&self) -> Option<&Fullname> {
        self.fullname.as_ref()
    }
    pub fn status(&self) -> &Status {
        &self.status
    }
    pub fn link_or_update_identites(
        &mut self,
        provider: ExternalProvider,
        sub: ExternalSubject,
    ) -> bool {
        self.identities.insert(IdentityLink::new(provider, sub))
    }
}
