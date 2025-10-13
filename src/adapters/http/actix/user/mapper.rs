use crate::adapters::http::actix::user::dto::login_with_google_out::LoginWithGoogleOut;
use crate::context::user::application::usecases::login_with_google::response::LoginWithGoogleResponse;

pub fn to_out(resp: LoginWithGoogleResponse) -> LoginWithGoogleOut {
    LoginWithGoogleOut {
        user_uuid: resp.user_uuid.to_string(),
    }
}
