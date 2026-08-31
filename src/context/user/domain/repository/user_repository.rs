use crate::context::user::{
    application::errors::RepoError,
    domain::{entities::user::User, value_objects::email::Email},
};

pub trait UserRepository: Send + Sync {
    fn find_by_email(&self, email: &Email) -> Result<Option<User>, RepoError>;
    fn save(&self, user: &User) -> Result<(), RepoError>;
}
