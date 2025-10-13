use chrono::{DateTime, Utc};
use diesel::prelude::*;
use uuid::Uuid;

use crate::context::user::infrastructure::postgres::schema::users;

#[derive(Debug, Clone, Queryable, Identifiable)]
#[diesel(table_name = users)]
pub struct UserRow {
    pub id: i64,
    pub uuid: Uuid,
    pub email: String,
    pub name: Option<String>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub email_verified_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub uuid: Uuid,
    pub email: String,
    pub name: Option<String>,
    pub status: String,
    pub email_verified_at: Option<DateTime<Utc>>,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = users)]
pub struct UpdateUser {
    pub email: String,
    pub name: Option<String>,
    pub status: String,
    pub email_verified_at: Option<DateTime<Utc>>,
    pub updated_at: DateTime<Utc>,
}
