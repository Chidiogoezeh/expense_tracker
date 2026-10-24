use super::service::{BudgetInput, BudgetPatch};
use crate::{error::AppError, middleware::AuthUser, state::AppState};
use axum::{
    Extension, Json,
    extract::{Path, State},
    http::StatusCode,
};
use chrono::NaiveDate;
use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Deserialize, ToSchema)]
pub struct CreateBudgetRequest {
    category: String,
    amount_limit: String,
    currency: Option<String>,
    month: NaiveDate,
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateBudgetRequest {
    category: Option<String>,
    amount_limit: Option<String>,
    currency: Option<String>,
    month: Option<NaiveDate>,
}

#[utoipa::path(
    get,
    path = "/budgets",
    responses(
        (status = 200, description = "User's budgets", body = [super::service::BudgetResponse]),
        (status = 401, description = "Authentication required", body = crate::error::ErrorResponse),
        (status = 500, description = "Internal server error", body = crate::error::ErrorResponse)
    ),
    security(("bearer_auth" = [])),
    tag = "Budgets"
)]
pub async fn list(
    State(state): State<AppState>,
    Extension(user): Extension<AuthUser>,
) -> Result<Json<Vec<super::service::BudgetResponse>>, AppError> {
    Ok(Json(state.budget_service.list(user.id).await?))
}

#[utoipa::path(
    post,
    path = "/budgets",
    request_body = CreateBudgetRequest,
    responses(
        (status = 201, description = "Budget created", body = super::service::BudgetResponse),
        (status = 400, description = "Invalid budget data", body = crate::error::ErrorResponse),
        (status = 401, description = "Authentication required", body = crate::error::ErrorResponse),
        (status = 409, description = "Budget conflicts with an existing budget", body = crate::error::ErrorResponse),
        (status = 500, description = "Internal server error", body = crate::error::ErrorResponse)
    ),
    security(("bearer_auth" = [])),
    tag = "Budgets"
)]
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

#[utoipa::path(
    patch,
    path = "/budgets/{id}",
    params(
        ("id" = Uuid, Path, description = "Budget ID")
    ),
    request_body = UpdateBudgetRequest,
    responses(
        (status = 200, description = "Budget updated", body = super::service::BudgetResponse),
        (status = 400, description = "Invalid budget data", body = crate::error::ErrorResponse),
        (status = 401, description = "Authentication required", body = crate::error::ErrorResponse),
        (status = 404, description = "Budget not found", body = crate::error::ErrorResponse),
        (status = 500, description = "Internal server error", body = crate::error::ErrorResponse)
    ),
    security(("bearer_auth" = [])),
    tag = "Budgets"
)]
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

#[utoipa::path(
    delete,
    path = "/budgets/{id}",
    params(
        ("id" = Uuid, Path, description = "Budget ID")
    ),
    responses(
        (status = 204, description = "Budget deleted"),
        (status = 401, description = "Authentication required", body = crate::error::ErrorResponse),
        (status = 404, description = "Budget not found", body = crate::error::ErrorResponse),
        (status = 500, description = "Internal server error", body = crate::error::ErrorResponse)
    ),
    security(("bearer_auth" = [])),
    tag = "Budgets"
)]
pub async fn delete(
    State(state): State<AppState>,
    Extension(user): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    state.budget_service.delete(user.id, id).await?;
    Ok(StatusCode::NO_CONTENT)
}
