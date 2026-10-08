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
        .get("Authorization")
        .and_then(|value| value.to_str().ok())
        .ok_or(AppError::Unauthorized)?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(AppError::Unauthorized)?;

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(state.jwt_secret.as_bytes()),
        &Validation::new(Algorithm::HS256),
    )
    .map_err(|_| AppError::Unauthorized)?;

    let user_id = Uuid::parse_str(&token_data.claims.sub).map_err(|_| AppError::Unauthorized)?;

    request.extensions_mut().insert(AuthUser { id: user_id });

    Ok(next.run(request).await)
}
