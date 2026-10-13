use axum::{
    Json,
    extract::{Extension, Path, State},
    http::StatusCode,
};

use serde::Serialize;

use uuid::Uuid;

use validator::{Validate, ValidationError};

use tracing::{info, warn};

use crate::{error::AppError, middleware::AuthUser, state::AppState};

fn validate_amount(amount: f64) -> Result<(), ValidationError> {
    if amount <= 0.0 || !amount.is_finite() {
        return Err(ValidationError::new("invalid_amount"));
    }

    Ok(())
}

#[derive(serde::Deserialize, Validate)]
pub struct CreateExpense {
    #[validate(length(min = 1, max = 200))]
    pub description: String,

    #[validate(custom(function = "validate_amount"))]
    pub amount: f64,

    #[validate(length(min = 1, max = 50))]
    pub category: String,
}

#[derive(Serialize)]
pub struct ExpenseResponse {
    pub id: Uuid,
    pub description: String,
    pub amount: f64,
    pub category: String,
}

pub async fn create_expense(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Json(input): Json<CreateExpense>,
) -> Result<(StatusCode, Json<ExpenseResponse>), AppError> {
    input.validate().map_err(|_| {
        warn!(
            user_id = %auth_user.id,
            "Expense validation failed"
        );

        AppError::BadRequest
    })?;

    let expense = state
        .expense_service
        .create(
            auth_user.id,
            input.description,
            input.amount,
            input.category,
        )
        .await?;

    info!(
        user_id = %auth_user.id,
        expense_id = %expense.id,
        "Expense created"
    );

    Ok((
        StatusCode::CREATED,
        Json(ExpenseResponse {
            id: expense.id,
            description: expense.description,
            amount: expense.amount,
            category: expense.category,
        }),
    ))
}

pub async fn get_expenses(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
) -> Result<Json<Vec<ExpenseResponse>>, AppError> {
    let expenses = state.expense_service.get_all(auth_user.id).await?;

    let response = expenses
        .into_iter()
        .map(|expense| ExpenseResponse {
            id: expense.id,
            description: expense.description,
            amount: expense.amount,
            category: expense.category,
        })
        .collect();

    info!(
        user_id = %auth_user.id,
        "Expenses retrieved"
    );

    Ok(Json(response))
}

pub async fn delete_expense(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    state.expense_service.delete(id, auth_user.id).await?;

    info!(
        user_id = %auth_user.id,
        expense_id = %id,
        "Expense deleted"
    );

    Ok(StatusCode::NO_CONTENT)
}

#[derive(Serialize)]
pub struct TotalResponse {
    pub total: f64,
}

pub async fn get_total(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
) -> Result<Json<TotalResponse>, AppError> {
    let total = state.expense_service.get_total(auth_user.id).await?;

    info!(
        user_id = %auth_user.id,
        "Expense total retrieved"
    );

    Ok(Json(TotalResponse { total }))
}

#[derive(Serialize)]
pub struct CategoryTotalResponse {
    pub category: String,
    pub total: f64,
}

pub async fn get_category_totals(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
) -> Result<Json<Vec<CategoryTotalResponse>>, AppError> {
    let totals = state
        .expense_service
        .get_category_totals(auth_user.id)
        .await?;

    let response = totals
        .into_iter()
        .map(|item| CategoryTotalResponse {
            category: item.category,
            total: item.total,
        })
        .collect();

    info!(
        user_id = %auth_user.id,
        "Category totals retrieved"
    );

    Ok(Json(response))
}
