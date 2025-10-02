use crate::context::{
    shared::domain::value_objects::uuid::Uuid,
    user::domain::{
        entities::user::User,
        value_objects::{email::Email, fullname::Fullname},
    },
};
use anyhow::Result;

use super::models::UserRow;

impl TryFrom<UserRow> for User {
    type Error = anyhow::Error;

    fn try_from(value: UserRow) -> Result<User> {
        let fullname: Option<Fullname> = value.name.as_deref().map(Fullname::parse).transpose()?;
        let email = Email::parse(value.email.as_str())?;
        let uuid = Uuid::from(value.uuid);

        Ok(User::new(fullname, email, uuid))
    }
}
