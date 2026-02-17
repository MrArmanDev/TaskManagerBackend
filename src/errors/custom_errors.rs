
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json
};
use serde_json::json;

#[derive(Debug)]
pub enum AppError {
    DatabaseError(sqlx::Error),
    ValidationError(String),
    #[allow(dead_code)]
    AuthenticationError(String),
    #[allow(dead_code)]
    NotFound(String),
    #[allow(dead_code)]
    InternalServerError(String),
    PasswordError,
    DuplicateKeyError(String),
    #[allow(dead_code)]
    Unauthorized,
    JwtErrors,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::DatabaseError(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            AppError::ValidationError(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::AuthenticationError(msg) => (StatusCode::UNAUTHORIZED, msg),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::InternalServerError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::PasswordError => (StatusCode::BAD_REQUEST, "Password error".to_string()),
            AppError::DuplicateKeyError(msg) => (StatusCode::CONFLICT, msg),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized".to_string()),
            AppError::JwtErrors => (StatusCode::INTERNAL_SERVER_ERROR, "JWT error".to_string())
        };

        let body = Json(json!({ "error": error_message }));
        (status, body).into_response()
    }
}


