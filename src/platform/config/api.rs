use crate::platform::config::{
    common::CommonConfig,
    helpers::{get_default, load_env_layers},
};

#[derive(Clone)]
pub struct ApiConfig {
    pub common: CommonConfig,
    pub host: String,
    pub cookie_secure: bool,
}

pub fn load_api_config() -> ApiConfig {
    load_env_layers("api");

    let common = CommonConfig::from_env();
    let cookie_secure = std::env::var("COOKIE_SECURE")
        .ok()
        .and_then(|s| s.parse::<bool>().ok())
        .unwrap_or(!common.is_dev());

    ApiConfig {
        common,
        host: get_default("API_HOST", "0.0.0.0:3000"),
        cookie_secure,
    }
}
