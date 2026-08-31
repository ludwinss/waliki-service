use dotenvy::dotenv;
use std::{env, str::FromStr};

pub struct Config {
    port: u16,
    subdomain: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok();
        Self {
            port: Self::get_required("PORT"),
            subdomain: format!(
                "/{}",
                Self::get("SUBDOMAIN").unwrap_or_else(|| "api".to_owned())
            ),
        }
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn subdomain(&self) -> &String {
        &self.subdomain
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
