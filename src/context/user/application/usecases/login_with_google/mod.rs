pub mod handler;
pub mod request;
pub mod response;

use handler::LoginWithGoogleHandler;
use request::LoginWithGoogleRequest;
use response::LoginWithGoogleResponse;

use crate::context::user::application::errors::AppError;
use crate::context::user::application::ports::{clock::Clock, id_generator::IdGenerator};
use crate::context::user::domain::repository::user_repository::UserRepository;

pub trait LoginWithGoogleUseCase: Send + Sync {
    fn execute(&self, req: LoginWithGoogleRequest) -> Result<LoginWithGoogleResponse, AppError>;
}

impl<R, C, G> LoginWithGoogleUseCase for LoginWithGoogleHandler<R, C, G>
where
    R: UserRepository,
    C: Clock,
    G: IdGenerator,
{
    fn execute(&self, req: LoginWithGoogleRequest) -> Result<LoginWithGoogleResponse, AppError> {
        LoginWithGoogleHandler::execute(self, req)
    }
}
