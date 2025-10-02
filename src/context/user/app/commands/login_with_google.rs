use std::sync::Arc;

use anyhow::Result;

use crate::{
    adapters::http::actix::dto::login_with_google::{LoginWithGoogle, LoginWithGoogleResult},
    context::{
        shared::domain::value_objects::uuid::Uuid,
        user::domain::{
            entities::user as Entities,
            repository::user_repository::UserRepository,
            value_objects::{
                email::Email, external_provider::ExternalProvider,
                external_subject::ExternalSubject, fullname::Fullname,
            },
        },
    },
};

pub struct LoginWithGoogleHandler {
    pub repo: Arc<dyn UserRepository>,
}

impl LoginWithGoogleHandler {
    pub fn new(repo: Arc<dyn UserRepository + Send + Sync>) -> Self {
        Self { repo }
    }
    pub fn execute(&self, params: LoginWithGoogle) -> Result<LoginWithGoogleResult> {
        let email = Email::parse(&params.email)?;
        let fullname: Option<Fullname> = params.name.as_deref().map(Fullname::parse).transpose()?;

        let user_founded = self.repo.find_by_email(&email)?;

        let sub_parsed = ExternalSubject::parse(params.sub)?;
        if let Some(mut user) = user_founded {
            if user.email_verified_at().is_none() && params.email_verified {
                user.mark_email_verified_now();
            }

            if let Some(new_name) = fullname {
                if user.fullname() != Some(&new_name) {
                    user.rename(new_name);
                }
            }

            user.link_or_update_identites(ExternalProvider::Google, sub_parsed);

            let _ = self.repo.save(&user);

            return Ok(LoginWithGoogleResult {
                user_uuid: user.uuid().to_string(),
            });
        }

        let uuid = Uuid::new();

        let mut new_user = Entities::User::new(fullname, email, uuid);

        new_user.link_or_update_identites(ExternalProvider::Google, sub_parsed);

        let _ = self.repo.save(&new_user);
        Ok({
            LoginWithGoogleResult {
                user_uuid: new_user.uuid().to_string(),
            }
        })
    }
}
