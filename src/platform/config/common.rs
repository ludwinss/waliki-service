#[derive(Clone)]
pub struct CommonConfig {
    pub app_env: String,
    pub oidc_google_client_id: String,
    pub oidc_google_client_secret: String,
    pub oidc_google_redirect_uri: String,
    pub oidc_google_issuer_uri: String,
    pub secret_key: String,
    pub jwt_issuer: String,
    pub jwt_audience: String,
    pub jwt_access_ttl_secs: u64,
    pub jwt_refresh_ttl_secs: u64,

    pub postgres_uri: String,
}

impl CommonConfig {
    pub fn from_env() -> Self {
        use super::helpers::{get_default, get_required};

        let jwt_access_ttl_secs = parse_duration_env("JWT_ACCESS_TTL_SECONDS", 900);
        let jwt_refresh_ttl_secs = parse_duration_env("JWT_REFRESH_TTL_SECONDS", 1_209_600);

        Self {
            app_env: get_required("APP_ENV"),
            oidc_google_client_id: get_required("OIDC_GOOGLE_CLIENT_ID"),
            oidc_google_client_secret: get_required("OIDC_GOOGLE_CLIENT_SECRET"),
            oidc_google_redirect_uri: get_required("OIDC_GOOGLE_REDIRECT_URI"),
            oidc_google_issuer_uri: get_required("OIDC_GOOGLE_ISSUER_URI"),
            secret_key: get_required("SECRET_KEY"),
            jwt_issuer: get_default("JWT_ISSUER", "waliki-service"),
            jwt_audience: get_default("JWT_AUDIENCE", "waliki-clients"),
            jwt_access_ttl_secs,
            jwt_refresh_ttl_secs,

            postgres_uri: get_required("DATABASE_URL"),
        }
    }

    pub fn is_dev(&self) -> bool {
        self.app_env == "development"
    }
}

fn parse_duration_env(var: &str, default: u64) -> u64 {
    use super::helpers::get_default;

    let value = get_default(var, &default.to_string());
    value
        .parse::<u64>()
        .unwrap_or_else(|_| panic!("{var} must be a positive integer"))
}
