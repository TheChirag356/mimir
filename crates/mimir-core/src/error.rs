use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

// This enum represents every "kind" of failure a handler can produce.
#[derive(Debug)]
pub enum AppError {
    NotFound(String),
    BadRequest(String),
    Unauthorized(String),
    Database(sqlx::Error),
}

// `From<sqlx::Error> for AppError` is what makes `?` work inside handlers:
// when a db function returns `Err(sqlx::Error)`, `?` automatically converts
// it into `AppError::Database(...)` for you.
impl From<sqlx::Error> for AppError {
    fn from(e: sqlx::Error) -> Self {
        AppError::Database(e)
    }
}

// `IntoResponse` is the trait Axum uses to turn any return type into an
// actual HTTP response. Implementing it for AppError means you can return
// `Result<Json<T>, AppError>` from a handler and Axum knows what to do
// with the error case.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg),
            AppError::Database(e) => {
                eprintln!("DB error: {e}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "internal server error".to_string(),
                )
            }
        };
        (status, Json(json!({ "error": message }))).into_response()
    }
}
