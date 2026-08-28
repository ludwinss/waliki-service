use axum::http::StatusCode;

pub async fn handler_with_db() -> StatusCode {
    //TODO: implement conn check
    StatusCode::OK
}
