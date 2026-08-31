use axum::{Router, routing::get, serve};
use platform::Config;

mod health;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::from_env();
    let db = platform::db::init_pool(config.database_url(), config.database_pool()).await?;

    let health: Router<()> = Router::new()
        .route("/health", get(health::handler_with_db))
        .with_state(db);

    let address = format!("0.0.0.0:{}", config.port());
    let listener = tokio::net::TcpListener::bind(&address).await?;

    serve(listener, health).await?;

    Ok(())
}
