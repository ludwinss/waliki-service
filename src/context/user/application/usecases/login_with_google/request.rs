pub struct LoginWithGoogleRequest {
    pub sub: String,
    pub email: String,
    pub name: Option<String>,
    pub email_verified: bool,
}
