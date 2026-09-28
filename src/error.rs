use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(Debug)]
pub enum AppError {
    BadRequest,
    Unauthorized,
    NotFound,
    Conflict,
    Database,
    Internal,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::BadRequest => (StatusCode::BAD_REQUEST, "Bad request"),

            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized"),

            AppError::NotFound => (StatusCode::NOT_FOUND, "Not found"),

            AppError::Conflict => (StatusCode::CONFLICT, "Resource already exists"),

            AppError::Database => (StatusCode::INTERNAL_SERVER_ERROR, "Database error"),

            AppError::Internal => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"),
        };

        (status, message).into_response()
    }
}
