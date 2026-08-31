use axum::{Router, routing::get, serve};
use platform::Config;

mod health;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::from_env();

    let app: Router<()> = Router::new().route("/health", get(health::handler_with_db));

    let address = format!("0.0.0.0:{}", config.port());
    let listener = tokio::net::TcpListener::bind(&address).await?;

    serve(listener, Router::new().nest(config.subdomain(), app)).await?;

    Ok(())
}
