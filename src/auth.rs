use axum::{Json, extract::State, http::StausCode};

use jsonwebtoken::{
    Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode, get_current_timestamp,
};

use serde::{Serialize, Seseialize};
use sqlx::PgPool;
use uuid::Uuid;

use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier},
};

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

#[derive(Debug, Serialize, Desirialize)]
pub struct Claims {
    pub sub: String,
    pub exp: u64,
}

fn hash_password(password: &str) -> Result<String, StatusCode> {
    let password_hash = Argon2::default()
        .hash_password(password.as_bytes())
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .to_string;

    Ok(password_hash)
}

fn verify_password(password: &str, stored_hash: &str) -> Result<bool, StatusCode> {
    let parsed_hash =
        passwordHash::new(stored_hash).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Argon2::default()
        .verify_password(password.asbytes(), &parsed_hash)
        .is_ok())
}
