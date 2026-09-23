use axum::{
    Json,
    extract::{Extension, State},
    http::StatusCode,
};

use jsonwebtoken::{Algorithm, EncodingKey, Header, encode, get_current_timestamp};

use serde::{Deserialize, Serialize};

use uuid::Uuid;

use argon2::{
    Argon2,
    password_hash::{PasswordHasher, PasswordVerifier, phc::PasswordHash},
};

use crate::{AppState, middleware::AuthUser};

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: u64,
}

fn hash_password(password: &str) -> Result<String, StatusCode> {
    let password_hash = Argon2::default()
        .hash_password(password.as_bytes())
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .to_string();

    Ok(password_hash)
}

fn verify_password(password: &str, stored_hash: &str) -> Result<bool, StatusCode> {
    let parsed_hash =
        PasswordHash::new(stored_hash).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}
fn create_token(user_id: Uuid, jwt_secret: &str) -> Result<String, StatusCode> {
    let claims = Claims {
        sub: user_id.to_string(),
        exp: get_current_timestamp() + 3000,
    };

    encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

#[derive(Serialize)]
pub struct RegistrationResponse {
    pub id: Uuid,
    pub email: String,
}

pub async fn register(
    State(state): State<AppState>,
    Json(input): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<RegistrationResponse>), StatusCode> {
    if input.email.trim().is_empty() || input.password.len() < 8 {
        return Err(StatusCode::BAD_REQUEST);
    }

    let email = input.email.trim().to_lowercase();

    let existing = sqlx::query!("SELECT id FROM users WHERE email = $1", email)
        .fetch_optional(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if existing.is_some() {
        return Err(StatusCode::CONFLICT);
    }

    let password_hash = hash_password(&input.password)?;

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
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

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
) -> Result<Json<LoginResponse>, StatusCode> {
    let email = input.email.trim().to_lowercase();

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
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let Some(user) = user else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    let valid = verify_password(&input.password, &user.password_hash)?;

    if !valid {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let token = create_token(user.id, &state.jwt_secret)?;

    Ok(Json(LoginResponse { token }))
}

pub async fn profile(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
) -> Result<Json<serde_json::Value>, StatusCode> {
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
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let Some(user) = user else {
        return Err(StatusCode::NOT_FOUND);
    };

    Ok(Json(serde_json::json!({
        "id": user.id,
        "email": user.email
    })))
}
