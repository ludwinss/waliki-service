pub struct CommonConfig {
    pub app_env: String,

    pub oidc_google_client_id: String,
    pub oidc_google_client_secret: String,
    pub oidc_google_redirect_uri: String,
    pub oidc_google_issuer_uri: String,

    pub secret_key: String,
}

impl CommonConfig {
    pub fn from_env() -> Self {
        use super::helpers::get_required;

        Self {
            app_env: get_required("APP_ENV"),

            oidc_google_client_id: get_required("OIDC_GOOGLE_CLIENT_ID"),
            oidc_google_client_secret: get_required("OIDC_GOOGLE_CLIENT_SECRET"),
            oidc_google_redirect_uri: get_required("OIDC_GOOGLE_REDIRECT_URI"),
            oidc_google_issuer_uri: get_required("OIDC_GOOGLE_ISSUER_URI"),

            secret_key: get_required("SECRET_KEY"),
        }
    }

    pub fn is_dev(&self) -> bool {
        self.app_env == "development"
    }
}
