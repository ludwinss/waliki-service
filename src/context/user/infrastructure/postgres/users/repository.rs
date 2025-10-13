use chrono::Utc;
use diesel::prelude::*;

use crate::context::user::{
    application::errors::RepoError,
    domain::{
        entities::user::User, repository::user_repository::UserRepository,
        value_objects::email::Email,
    },
    infrastructure::postgres::{
        identities::{
            mapper::{to_new_identity, to_update_identity},
            models::UserIdentityRow,
        },
        pool::PgPool,
        schema::{
            users::{self as users_table, dsl as users_dsl},
            users_identities::{self as identities_table, dsl as identities_dsl},
        },
        users::{
            mapper,
            models::{NewUser, UpdateUser, UserRow},
        },
    },
};

const STATUS_ACTIVE: &str = "active";

pub struct PgUserRepository {
    pool: PgPool,
}

impl PgUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl UserRepository for PgUserRepository {
    fn find_by_email(&self, email_vo: &Email) -> Result<Option<User>, RepoError> {
        let mut conn = self
            .pool
            .get()
            .map_err(|e| RepoError::new(format!("failed to get database connection: {e}")))?;

        let user_row = users_dsl::users
            .filter(users_dsl::email.eq(email_vo.as_str()))
            .first::<UserRow>(&mut conn)
            .optional()
            .map_err(|e| RepoError::new(format!("failed to query user by email: {e}")))?;

        let Some(row) = user_row else {
            return Ok(None);
        };

        let identity_rows: Vec<UserIdentityRow> = identities_dsl::users_identities
            .filter(identities_dsl::user_id.eq(row.id))
            .load::<UserIdentityRow>(&mut conn)
            .map_err(|e| RepoError::new(format!("failed to query user identities: {e}")))?;

        mapper::to_domain(row, identity_rows).map(Some)
    }

    fn save(&self, user: &User) -> Result<(), RepoError> {
        let mut conn = self
            .pool
            .get()
            .map_err(|e| RepoError::new(format!("failed to get database connection: {e}")))?;

        conn.transaction::<(), RepoError, _>(|transaction_conn| {
            let now = Utc::now();
            let fullname = user.fullname().map(|name| name.as_str().to_string());
            let email_str = user.email().as_str().to_string();
            let verified_at = user.email_verified_at().cloned();
            let status = STATUS_ACTIVE.to_string();

            let new_user = NewUser {
                uuid: *user.uuid(),
                email: email_str.clone(),
                name: fullname.clone(),
                status: status.clone(),
                email_verified_at: verified_at,
            };

            let update_user = UpdateUser {
                email: email_str.clone(),
                name: fullname.clone(),
                status,
                email_verified_at: verified_at,
                updated_at: now,
            };

            let persisted: UserRow = diesel::insert_into(users_table::table)
                .values(new_user)
                .on_conflict(users_dsl::uuid)
                .do_update()
                .set(update_user)
                .get_result(transaction_conn)
                .map_err(|e| RepoError::new(format!("failed to upsert user: {e}")))?;

            for (provider, subject) in user.identities().iter() {
                let new_identity = to_new_identity(persisted.id, *provider, subject);
                let update_identity = to_update_identity(subject, now);
                diesel::insert_into(identities_table::table)
                    .values(new_identity)
                    .on_conflict((identities_dsl::user_id, identities_dsl::provider))
                    .do_update()
                    .set(update_identity)
                    .execute(transaction_conn)
                    .map_err(|e| RepoError::new(format!("failed to upsert user identity: {e}")))?;
            }

            Ok(())
        })
    }
}
