use dotenvy::dotenv;
use std::{env, str::FromStr};

pub struct Config {
    server_port: u16,
    path_subdomain: String,
    database_url: String,
    database_pool: u32,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok();
        Self {
            server_port: Self::get_required("SERVER_PORT"),
            path_subdomain: format!(
                "/{}",
                Self::get("PATH_SUBDOMAIN").unwrap_or_else(|| "api".to_owned())
            ),
            database_url: Self::get_required("DATABASE_URL"),
            database_pool: Self::get_required("DATABASE_POOL"),
        }
    }

    pub fn port(&self) -> u16 {
        self.server_port
    }

    pub fn subdomain(&self) -> &str {
        &self.path_subdomain
    }

    pub fn database_pool(&self) -> u32 {
        self.database_pool
    }

    pub fn database_url(&self) -> &str {
        &self.database_url
    }

    fn get_required<T>(name: &str) -> T
    where
        T: FromStr,
    {
        env::var(name)
            .ok()
            .and_then(|s| s.parse::<T>().ok())
            .unwrap_or_else(|| panic!("{name} is required"))
    }

    fn get<T>(name: &str) -> Option<T>
    where
        T: FromStr,
    {
        env::var(name).ok().and_then(|s| s.parse::<T>().ok())
    }
}
