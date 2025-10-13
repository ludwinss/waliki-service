use std::sync::Arc;

use crate::context::user::application::{
    errors::AppError,
    ports::{clock::Clock, id_generator::IdGenerator},
    usecases::login_with_google::{
        request::LoginWithGoogleRequest, response::LoginWithGoogleResponse,
    },
};
use crate::context::user::domain::{
    entities::user::User,
    repository::user_repository::UserRepository,
    value_objects::{
        email::Email, external_provider::ExternalProvider, external_subject::ExternalSubject,
        fullname::Fullname,
    },
};

pub struct LoginWithGoogleHandler<R, C, G>
where
    R: UserRepository,
    C: Clock,
    G: IdGenerator,
{
    repo: Arc<R>,
    clock: Arc<C>,
    id_gen: Arc<G>,
}

impl<R, C, G> LoginWithGoogleHandler<R, C, G>
where
    R: UserRepository,
    C: Clock,
    G: IdGenerator,
{
    pub fn new(repo: Arc<R>, clock: Arc<C>, id_gen: Arc<G>) -> Self {
        Self {
            repo,
            clock,
            id_gen,
        }
    }

    pub fn execute(
        &self,
        req: LoginWithGoogleRequest,
    ) -> Result<LoginWithGoogleResponse, AppError> {
        let email = Email::parse(&req.email)?;
        let fullname = match req.name.as_deref() {
            Some(s) => Some(Fullname::parse(s)?),
            None => None,
        };
        let sub = ExternalSubject::parse(req.sub)?;

        if let Some(mut user) = self.repo.find_by_email(&email)? {
            if user.email_verified_at().is_none() && req.email_verified {
                user.mark_email_verified_at(self.clock.now());
            }
            if let Some(new_name) = fullname {
                if user.fullname() != Some(&new_name) {
                    user.rename(new_name);
                }
            }
            user.link_or_update_identities(ExternalProvider::Google, sub);
            self.repo.save(&user)?;
            return Ok(LoginWithGoogleResponse {
                user_uuid: *user.uuid(),
            });
        }

        let uuid = self.id_gen.new_uuid();
        let mut new_user = User::new(fullname, email, uuid);
        new_user.link_or_update_identities(ExternalProvider::Google, sub);
        self.repo.save(&new_user)?;
        Ok(LoginWithGoogleResponse {
            user_uuid: *new_user.uuid(),
        })
    }
}
