use diesel::prelude::*;

use crate::context::user::infraestructure::postgres::schema::user::users;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserRow {
    pub uuid: uuid::Uuid,
    pub name: Option<String>,
    pub email: String,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUserRow<'a> {
    pub status: &'a str,
    pub name: Option<&'a str>,
    pub email: &'a str,
}
