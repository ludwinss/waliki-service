use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::context::user::domain::value_objects::{
    external_provider::ExternalProvider, external_subject::ExternalSubject,
};

use super::models::{NewUserIdentity, UpdateUserIdentity};

pub fn to_new_identity(
    user_id: i64,
    provider: ExternalProvider,
    subject: &ExternalSubject,
) -> NewUserIdentity {
    NewUserIdentity {
        uuid: Uuid::new_v4(),
        subject: subject.as_str().to_string(),
        provider: provider.as_str().to_string(),
        user_id,
    }
}

pub fn to_update_identity(subject: &ExternalSubject, now: DateTime<Utc>) -> UpdateUserIdentity {
    UpdateUserIdentity {
        subject: subject.as_str().to_string(),
        updated_at: now,
    }
}
