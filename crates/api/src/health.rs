use axum::{extract::State, http::StatusCode};
use platform::Db;

pub async fn handler_with_db(State(db): State<Db>) -> StatusCode {
    match platform::db::ping(&db).await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::SERVICE_UNAVAILABLE,
    }
}
