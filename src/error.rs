use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug)]
pub enum AppError {
    BadRequest,
    Unauthorized,
    NotFound,
    Conflict,
    Database,
    Internal,
}

#[derive(Serialize, ToSchema)]
pub struct ErrorResponse {
    pub error: ErrorBody,
}

#[derive(Serialize, ToSchema)]
pub struct ErrorBody {
    pub code: &'static str,
    pub message: &'static str,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code, message) = match self {
            AppError::BadRequest => (
                StatusCode::BAD_REQUEST,
                "bad_request",
                "The request contains invalid data",
            ),
            AppError::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                "unauthorized",
                "Authentication is required or invalid",
            ),
            AppError::NotFound => (
                StatusCode::NOT_FOUND,
                "not_found",
                "The requested resource was not found",
            ),
            AppError::Conflict => (
                StatusCode::CONFLICT,
                "conflict",
                "The request conflicts with an existing resource",
            ),
            AppError::Database => {
                tracing::error!("Database operation failed");

                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "database_error",
                    "A database operation failed",
                )
            }
            AppError::Internal => {
                tracing::error!("Unexpected internal application error");

                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "internal_error",
                    "An unexpected error occurred",
                )
            }
        };

        (
            status,
            Json(ErrorResponse {
                error: ErrorBody { code, message },
            }),
        )
            .into_response()
    }
}
