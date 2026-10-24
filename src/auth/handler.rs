use crate::{error::AppError, middleware::AuthUser, state::AppState};
use axum::{
    Json,
    extract::{Extension, State},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use tracing::{info, warn};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Validate, ToSchema)]
pub struct RegisterRequest {
    #[schema(example = "user@example.com")]
    #[validate(email)]
    pub email: String,

    #[schema(example = "StrongPassword123")]
    #[validate(length(min = 8, max = 128))]
    pub password: String,
}

#[derive(Deserialize, Validate, ToSchema)]
pub struct LoginRequest {
    #[schema(example = "user@example.com")]
    #[validate(email)]
    pub email: String,

    #[schema(example = "StrongPassword123")]
    #[validate(length(min = 8, max = 128))]
    pub password: String,
}

#[derive(Serialize, ToSchema)]
pub struct RegistrationResponse {
    pub id: Uuid,
    pub email: String,
}

#[derive(Serialize, ToSchema)]
pub struct LoginResponse {
    pub token: String,
}

#[derive(Serialize, ToSchema)]
pub struct ProfileResponse {
    pub id: Uuid,
    pub email: String,
}

#[utoipa::path(
    post,
    path = "/register",
    request_body = RegisterRequest,
    responses(
        (status = 201, description = "User registered", body = RegistrationResponse),
        (status = 400, description = "Invalid registration data"),
        (status = 409, description = "Email already exists"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Authentication"
)]

pub async fn register(
    State(state): State<AppState>,
    Json(input): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<RegistrationResponse>), AppError> {
    input.validate().map_err(|_| {
        warn!("Registration validation failed");
        AppError::BadRequest
    })?;

    let result = state
        .auth_service
        .register(&input.email, &input.password)
        .await?;

    info!(
        user_id = %result.id,
        "User registered successfully"
    );

    Ok((
        StatusCode::CREATED,
        Json(RegistrationResponse {
            id: result.id,
            email: result.email,
        }),
    ))
}

#[utoipa::path(
    post,
    path = "/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = LoginResponse),
        (status = 400, description = "Invalid request data"),
        (status = 401, description = "Invalid credentials"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Authentication"
)]

pub async fn login(
    State(state): State<AppState>,
    Json(input): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, AppError> {
    input.validate().map_err(|_| {
        warn!("Login validation failed");
        AppError::BadRequest
    })?;

    let token = state
        .auth_service
        .login(&input.email, &input.password)
        .await?;

    Ok(Json(LoginResponse { token }))
}

#[utoipa::path(
    get,
    path = "/profile",
    responses(
        (status = 200, description = "Authenticated user profile", body = ProfileResponse),
        (status = 401, description = "Authentication required"),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer_auth" = [])),
    tag = "Authentication"
)]

pub async fn profile(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
) -> Result<Json<ProfileResponse>, AppError> {
    let user = state.auth_service.profile(auth_user.id).await?;

    Ok(Json(ProfileResponse {
        id: user.id,
        email: user.email,
    }))
}
