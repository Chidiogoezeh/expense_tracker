use axum::{
    Json,
    extract::{Extension, State},
    http::StatusCode,
};

use argon2::{
    Argon2,
    password_hash::{PasswordHasher, PasswordVerifier, phc::PasswordHash},
};

use jsonwebtoken::{Algorithm, EncodingKey, Header, encode, get_current_timestamp};

use serde::{Deserialize, Serialize};

use uuid::Uuid;

use validator::Validate;

use tracing::{info, warn};

use crate::{AppState, error::AppError, middleware::AuthUser};

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

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: u64,
}

fn hash_password(password: &str) -> Result<String, AppError> {
    Argon2::default()
        .hash_password(password.as_bytes())
        .map(|hash| hash.to_string())
        .map_err(|_| AppError::Internal)
}

fn verify_password(password: &str, stored_hash: &str) -> Result<bool, AppError> {
    let parsed_hash = PasswordHash::new(stored_hash).map_err(|_| AppError::Internal)?;

    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

fn create_token(user_id: Uuid, jwt_secret: &str) -> Result<String, AppError> {
    const TOKEN_LIFETIME_SECONDS: u64 = 3000;

    let claims = Claims {
        sub: user_id.to_string(),
        exp: get_current_timestamp() + TOKEN_LIFETIME_SECONDS,
    };

    encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    )
    .map_err(|_| AppError::Internal)
}

#[derive(Serialize)]
pub struct RegistrationResponse {
    pub id: Uuid,
    pub email: String,
}

pub async fn register(
    State(state): State<AppState>,
    Json(input): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<RegistrationResponse>), AppError> {
    info!("Registration request received");
    // Validate
    input.validate().map_err(|_| {
        warn!("Registration validation failed");
        AppError::BadRequest
    })?;

    // Normalize email
    let email = input.email.trim().to_lowercase();

    // Check whether email already exists
    let existing = sqlx::query!("SELECT id FROM users WHERE email = $1", email)
        .fetch_optional(&state.pool)
        .await
        .map_err(|_| AppError::Database)?;

    if existing.is_some() {
        return Err(AppError::Conflict);
    }

    // Hash password
    let password_hash = hash_password(&input.password)?;

    // Create user
    let user = sqlx::query!(
        r#"
        INSERT INTO users (id, email, password_hash)
        VALUES ($1, $2, $3)
        RETURNING id, email
        "#,
        Uuid::new_v4(),
        email,
        password_hash
    )
    .fetch_one(&state.pool)
    .await
    .map_err(|_| AppError::Database)?;

    info!(
        user_id = %user.id,
        "User registered successfully"
    );

    Ok((
        StatusCode::CREATED,
        Json(RegistrationResponse {
            id: user.id,
            email: user.email,
        }),
    ))
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}

pub async fn login(
    State(state): State<AppState>,
    Json(input): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, AppError> {
    // Validate
    input.validate().map_err(|_| AppError::BadRequest)?;
    let email = input.email.trim().to_lowercase();

    // Find user
    let user = sqlx::query!(
        r#"
        SELECT id, password_hash
        FROM users
        WHERE email = $1
        "#,
        email
    )
    .fetch_optional(&state.pool)
    .await
    .map_err(|_| AppError::Database)?;

    let Some(user) = user else {
        return Err(AppError::Unauthorized);
    };

    // Verify password
    let valid = verify_password(&input.password, &user.password_hash)?;

    if !valid {
        return Err(AppError::Unauthorized);
    }

    // Create JWT
    let token = create_token(user.id, &state.jwt_secret)?;

    Ok(Json(LoginResponse { token }))
}

pub async fn profile(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
) -> Result<Json<serde_json::Value>, AppError> {
    let user = sqlx::query!(
        r#"
        SELECT id, email
        FROM users
        WHERE id = $1
        "#,
        auth_user.id
    )
    .fetch_optional(&state.pool)
    .await
    .map_err(|_| AppError::Database)?;

    let Some(user) = user else {
        return Err(AppError::NotFound);
    };

    Ok(Json(serde_json::json!({
        "id": user.id,
        "email": user.email
    })))
}
