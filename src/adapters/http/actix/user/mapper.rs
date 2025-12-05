use crate::{
    adapters::http::actix::user::dto::login_with_google_out::LoginWithGoogleOut,
    context::{
        shared_kernel::application::ports::token_issuer::{AccessToken, RefreshToken},
        user::application::usecases::login_with_google::response::LoginWithGoogleResponse,
    },
};

pub fn to_out(
    resp: LoginWithGoogleResponse,
    access: &AccessToken,
    refresh: &RefreshToken,
) -> LoginWithGoogleOut {
    LoginWithGoogleOut {
        user_uuid: resp.user_uuid.to_string(),
        access_token: access.value().to_string(),
        refresh_token: refresh.value().to_string(),
        access_expires_in: access.expires_in().as_secs(),
        refresh_expires_in: refresh.expires_in().as_secs(),
    }
}
