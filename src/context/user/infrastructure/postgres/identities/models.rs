use diesel::prelude::*;

use crate::context::user::infrastructure::postgres::schema::user::users_identities;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = users_identities)]
pub struct IdentityRow {
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub id: String,
    pub uuid: uuid::Uuid,
    pub subject: String,
    pub provider: String,
    pub user_id: String,
}

#[derive(Insertable)]
#[diesel(table_name = users_identities)]
pub struct NewIdentityRow<'a> {
    pub subject: &'a str,
    pub provider: &'a str,
}
