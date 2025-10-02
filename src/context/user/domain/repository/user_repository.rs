use crate::context::user::domain::{entities::user::User, value_objects::email::Email};

use anyhow::Result;
pub trait UserRepository: Send + Sync {
    // TODO: chambiar anyhow por una manejador de erroes en el dominio
    fn find_by_email(&self, email: &Email) -> Result<Option<User>>;
    fn save(&self, user: &User) -> Result<()>;
}
