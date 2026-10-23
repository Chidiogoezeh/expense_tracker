use axum::{extract::State, http::StatusCode};
use sqlx::query_scalar;
use tracing::warn;

use crate::AppState;

pub async fn health(State(state): State<AppState>) -> StatusCode {
    match query_scalar::<_, i32>("SELECT 1")
        .fetch_one(&state.db)
        .await
    {
        Ok(1) => StatusCode::OK,
        Ok(_) => {
            warn!("Database health check returned an unexpected result");
            StatusCode::SERVICE_UNAVAILABLE
        }
        Err(_) => {
            warn!("Database health check failed");
            StatusCode::SERVICE_UNAVAILABLE
        }
    }
}
