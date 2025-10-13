use std::collections::HashMap;

use chrono::{DateTime, Utc};

use crate::context::shared_kernel::value_objects::uuid::Uuid;
use crate::context::user::domain::value_objects::{
    email::Email, external_provider::ExternalProvider, external_subject::ExternalSubject,
    fullname::Fullname,
};

pub struct User {
    uuid: Uuid,
    email: Email,
    fullname: Option<Fullname>,
    email_verified_at: Option<DateTime<Utc>>,
    identities: HashMap<ExternalProvider, ExternalSubject>,
}

impl User {
    pub fn new(fullname: Option<Fullname>, email: Email, uuid: Uuid) -> Self {
        Self {
            uuid,
            email,
            fullname,
            email_verified_at: None,
            identities: HashMap::new(),
        }
    }

    pub fn restore(
        fullname: Option<Fullname>,
        email: Email,
        uuid: Uuid,
        email_verified_at: Option<DateTime<Utc>>,
        identities: HashMap<ExternalProvider, ExternalSubject>,
    ) -> Self {
        Self {
            uuid,
            email,
            fullname,
            email_verified_at,
            identities,
        }
    }

    pub fn uuid(&self) -> &Uuid {
        &self.uuid
    }

    pub fn email(&self) -> &Email {
        &self.email
    }

    pub fn fullname(&self) -> Option<&Fullname> {
        self.fullname.as_ref()
    }

    pub fn email_verified_at(&self) -> Option<&DateTime<Utc>> {
        self.email_verified_at.as_ref()
    }

    pub fn identities(&self) -> &HashMap<ExternalProvider, ExternalSubject> {
        &self.identities
    }

    pub fn rename(&mut self, name: Fullname) {
        self.fullname = Some(name);
    }

    pub fn mark_email_verified_at(&mut self, at: DateTime<Utc>) {
        self.email_verified_at = Some(at);
    }

    pub fn link_or_update_identities(&mut self, provider: ExternalProvider, sub: ExternalSubject) {
        self.identities.insert(provider, sub);
    }
}
