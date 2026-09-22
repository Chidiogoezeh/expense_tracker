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
