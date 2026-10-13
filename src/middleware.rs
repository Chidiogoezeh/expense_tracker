use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode};
use uuid::Uuid;

use crate::{AppState, auth::Claims, error::AppError};

#[derive(Clone)]
pub struct AuthUser {
    pub id: Uuid,
}

pub async fn auth_middleware(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let auth_header = request
        .headers()
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .ok_or_else(|| {
            tracing::warn!("Authentication rejected: missing or invalid header");
            AppError::Unauthorized
        })?;

    let (scheme, token) = auth_header.split_once(' ').ok_or_else(|| {
        tracing::warn!("Authentication rejected: malformed authorization scheme");
        AppError::Unauthorized
    })?;

    if !scheme.eq_ignore_ascii_case("Bearer")
        || token.is_empty()
        || token.trim() != token
        || token.contains(' ')
    {
        tracing::warn!("Authentication rejected: invalid Bearer format");
        return Err(AppError::Unauthorized);
    }

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(state.jwt_secret.as_bytes()),
        &Validation::new(Algorithm::HS256),
    )
    .map_err(|_| {
        tracing::warn!("Authentication rejected: invalid or expired token");
        AppError::Unauthorized
    })?;

    let user_id = Uuid::parse_str(&token_data.claims.sub).map_err(|_| {
        tracing::warn!("Authentication rejected: invalid user identifier");
        AppError::Unauthorized
    })?;

    request.extensions_mut().insert(AuthUser { id: user_id });

    Ok(next.run(request).await)
}
