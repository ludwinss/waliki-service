use chrono::{DateTime, Utc};
use diesel::prelude::*;
use uuid::Uuid;

use crate::context::user::infrastructure::postgres::schema::users_identities;
use crate::context::user::infrastructure::postgres::users::models::UserRow;

#[derive(Debug, Clone, Queryable, Identifiable, Associations)]
#[diesel(table_name = users_identities)]
#[diesel(belongs_to(UserRow, foreign_key = user_id))]
pub struct UserIdentityRow {
    pub id: i64,
    pub uuid: Uuid,
    pub subject: String,
    pub provider: String,
    pub user_id: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = users_identities)]
pub struct NewUserIdentity {
    pub uuid: Uuid,
    pub subject: String,
    pub provider: String,
    pub user_id: i64,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = users_identities)]
pub struct UpdateUserIdentity {
    pub subject: String,
    pub updated_at: DateTime<Utc>,
}
