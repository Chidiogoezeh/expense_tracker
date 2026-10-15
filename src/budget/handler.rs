use axum::{
    Extension, Json,
    extract::{Path, State},
    http::StatusCode,
};
use chrono::NaiveDate;
use serde::Deserialize;
use uuid::Uuid;

use crate::{error::AppError, middleware::AuthUser, state::AppState};

use super::service::{BudgetInput, BudgetPatch};

#[derive(Deserialize)]
pub struct CreateBudgetRequest {
    category: String,
    amount_limit: String,
    currency: Option<String>,
    month: NaiveDate,
}

#[derive(Deserialize)]
pub struct UpdateBudgetRequest {
    category: Option<String>,
    amount_limit: Option<String>,
    currency: Option<String>,
    month: Option<NaiveDate>,
}

pub async fn list(
    State(state): State<AppState>,
    Extension(user): Extension<AuthUser>,
) -> Result<Json<Vec<super::service::BudgetResponse>>, AppError> {
    Ok(Json(state.budget_service.list(user.id).await?))
}

pub async fn create(
    State(state): State<AppState>,
    Extension(user): Extension<AuthUser>,
    Json(request): Json<CreateBudgetRequest>,
) -> Result<(StatusCode, Json<super::service::BudgetResponse>), AppError> {
    let budget = state
        .budget_service
        .create(
            user.id,
            BudgetInput {
                category: request.category,
                amount_limit: request.amount_limit,
                currency: request.currency.unwrap_or_else(|| "NGN".to_string()),
                month: request.month,
            },
        )
        .await?;

    Ok((StatusCode::CREATED, Json(budget)))
}

pub async fn update(
    State(state): State<AppState>,
    Extension(user): Extension<AuthUser>,
    Path(id): Path<Uuid>,
    Json(request): Json<UpdateBudgetRequest>,
) -> Result<Json<super::service::BudgetResponse>, AppError> {
    let budget = state
        .budget_service
        .update(
            user.id,
            id,
            BudgetPatch {
                category: request.category,
                amount_limit: request.amount_limit,
                currency: request.currency,
                month: request.month,
            },
        )
        .await?;

    Ok(Json(budget))
}

pub async fn delete(
    State(state): State<AppState>,
    Extension(user): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    state.budget_service.delete(user.id, id).await?;
    Ok(StatusCode::NO_CONTENT)
}
