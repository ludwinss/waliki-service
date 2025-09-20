pub fn load_env_layers(bin: &str) {
    let _ = dotenv::from_filename(".env");
    let app_env = std::env::var("APP_ENV").unwrap_or_else(|_| "development".into());
    let _ = dotenv::from_filename(format!(".env.{app_env}"));
    let _ = dotenv::from_filename(format!(".env.{bin}"));
    let _ = dotenv::from_filename(format!(".env.{bin}.{app_env}"));
}

pub fn get_required(name: &str) -> String {
    std::env::var(name).unwrap_or_else(|_| panic!("{} is required", name))
}

pub fn get_default(name: &str, default: &str) -> String {
    std::env::var(name).unwrap_or_else(|_| default.to_string())
}

pub fn parse_secret_key(raw: &str) -> Vec<u8> {
    use base64::Engine;
    if let Some(b64) = raw.strip_prefix("base64:") {
        base64::engine::general_purpose::STANDARD
            .decode(b64)
            .unwrap()
    } else {
        raw.as_bytes().to_vec()
    }
}
