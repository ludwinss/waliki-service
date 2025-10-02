use crate::context::user::{
    domain::{
        entities::user::User, repository::user_repository::UserRepository,
        value_objects::email::Email,
    },
    infraestructure::postgres::{schema::user::users::dsl::*, users::models::NewUserRow},
};
use anyhow::Result;

use crate::context::user::infraestructure::postgres::pool::PgPool;
use diesel::{OptionalExtension, SelectableHelper, prelude::*};

use super::models::UserRow;

pub struct PgUserRepository {
    pub pool: PgPool,
}

impl UserRepository for PgUserRepository {
    fn find_by_email(&self, e: &Email) -> Result<Option<User>> {
        let mut conn = self.pool.get()?;

        let res = users
            .filter(email.eq(e.as_str()))
            .select(UserRow::as_select())
            .first::<UserRow>(&mut *conn)
            .optional()?;

        let user_transpose = res.map(User::try_from).transpose()?;

        Ok(user_transpose)
    }

    fn save(&self, user: &User) -> Result<()> {
        let mut conn = self.pool.get()?;

        let name_opt: Option<&str> = user.fullname().map(|f| f.as_str());

        let new_user = NewUserRow {
            status: user.status().as_str(),
            email: user.email().as_str(),
            name: name_opt,
        };

        diesel::insert_into(users)
            .values(&new_user)
            .execute(&mut *conn)?;

        Ok(())
    }
}
