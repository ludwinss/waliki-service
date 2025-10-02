pub struct LoginWithGoogle {
    pub sub: String,
    pub email: String,
    pub name: Option<String>,
    pub email_verified: bool,
}

pub struct LoginWithGoogleResult {
    pub user_uuid: String,
}
