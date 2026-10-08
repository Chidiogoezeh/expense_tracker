use axum::{
    Json,
    extract::{Extension, State},
    http::StatusCode,
};

use serde::{Deserialize, Serialize};

use uuid::Uuid;

use validator::Validate;

use tracing::{info, warn};

use crate::{error::AppError, middleware::AuthUser, state::AppState};

#[derive(Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(email)]
    pub email: String,

    #[validate(length(min = 8))]
    pub password: String,
}

#[derive(Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email)]
    pub email: String,

    #[validate(length(min = 8))]
    pub password: String,
}

#[derive(Serialize)]
pub struct RegistrationResponse {
    pub id: Uuid,
    pub email: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}

pub async fn register(
    State(state): State<AppState>,
    Json(input): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<RegistrationResponse>), AppError> {
    info!("Registration request received");

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

pub async fn login(
    State(state): State<AppState>,
    Json(input): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, AppError> {
    info!("Login request received");

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

#[derive(Serialize)]
pub struct ProfileResponse {
    pub id: Uuid,
    pub email: String,
}

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
