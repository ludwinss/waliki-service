use axum::{Router, routing::get};

mod health;

fn main() -> anyhow::Result<()> {
    println!("Hello, world!");
    let app: Router<()> = Router::new().route("/health", get(health::handler_with_db));

    // let listener = tokio::net::TcpListener::
    // axum::serve
    Ok(())
}
