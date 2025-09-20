use crate::context::user::domain::{entities::user::User, errors::IdentityError};

trait UserRepository {
    fn save(&mut self, user: &User) -> Result<(), IdentityError>;
}
