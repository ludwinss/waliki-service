use crate::adapters::http::actix::user::dto::{
    login_with_google_in::LoginWithGoogleIn, login_with_google_out::LoginWithGoogleOut,
};
use crate::context::user::application::usecases::login_with_google::{
    request::LoginWithGoogleRequest, response::LoginWithGoogleResponse,
};

pub fn to_app(in_dto: LoginWithGoogleIn) -> LoginWithGoogleRequest {
    LoginWithGoogleRequest {
        sub: in_dto.sub,
        email: in_dto.email,
        name: in_dto.name,
        email_verified: in_dto.email_verified,
    }
}

pub fn to_out(resp: LoginWithGoogleResponse) -> LoginWithGoogleOut {
    LoginWithGoogleOut {
        user_uuid: resp.user_uuid.to_string(),
    }
}
