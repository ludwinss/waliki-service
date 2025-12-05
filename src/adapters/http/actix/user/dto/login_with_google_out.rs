#[derive(serde::Serialize)]
pub struct LoginWithGoogleOut {
    pub user_uuid: String,
    pub access_token: String,
    pub refresh_token: String,
    pub access_expires_in: u64,
    pub refresh_expires_in: u64,
}
