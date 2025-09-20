use chrono::{DateTime, Utc};

use crate::context::{
    shared::domain::value_objects::status::Status,
    user::domain::value_objects::{
        cellphone::Phone, email::Email, fullname::Fullname, identity_link::IdentityLink,
    },
};

pub struct User {
    fullname: Fullname,
    email: Email,
    phone: Option<Phone>,
    status: Status,
    identities: Vec<IdentityLink>,
    created_at: DateTime<chrono::Utc>,
    email_verified_at: Option<DateTime<chrono::Utc>>,
}

impl User {
    pub fn new(fullname: Fullname, email: Email) -> Self {
        Self {
            fullname,
            email,
            phone: None,
            email_verified_at: None,
            created_at: Utc::now(),
            status: Status::ACTIVE,
            identities: Vec::new(),
        }
    }

    pub fn verify_email() {}
}
