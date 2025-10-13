#[derive(serde::Deserialize)]
pub struct LoginWithGoogleIn {
    pub sub: String,
    pub email: String,
    pub name: Option<String>,
    #[serde(default)]
    pub email_verified: bool,
}
