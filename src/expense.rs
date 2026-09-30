use axum::{
    Json,
    extract::{Extension, Path, State},
    http::StatusCode,
};

use uuid::Uuid;

use validator::{Validate, ValidationError};

use tracing::{info, warn};

use crate::{AppState, error::AppError, middleware::AuthUser};

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

#[derive(Debug, serde::Serialize, sqlx::FromRow)]
pub struct ExpenseRow {
    pub id: Uuid,
    pub description: String,
    pub amount: f64,
    pub category: String,
}

pub async fn create_expense(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Json(input): Json<CreateExpense>,
) -> Result<(StatusCode, Json<ExpenseRow>), AppError> {
    input.validate().map_err(|_| {
        warn!(
            user_id = %auth_user.id,
            "Expense validation failed"
        );

        AppError::BadRequest
    })?;

    let expense = sqlx::query_as::<_, ExpenseRow>(
        r#"
    INSERT INTO expenses
        (id, user_id, description, amount, category)
    VALUES
        ($1, $2, $3, $4, $5)
    RETURNING id, description, amount, category
    "#,
    )
    .bind(Uuid::new_v4())
    .bind(auth_user.id)
    .bind(input.description)
    .bind(input.amount)
    .bind(input.category)
    .fetch_one(&state.pool)
    .await
    .map_err(|_| AppError::Database)?;

    info!(
        user_id = %auth_user.id,
        expense_id = %expense.id,
        "Expense created"
    );

    Ok((StatusCode::CREATED, Json(expense)))
}

pub async fn get_expenses(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
) -> Result<Json<Vec<ExpenseRow>>, AppError> {
    let expenses = sqlx::query_as::<_, ExpenseRow>(
        r#"
        SELECT id, description, amount, category
        FROM expenses
        WHERE user_id = $1
        "#,
    )
    .bind(auth_user.id)
    .fetch_all(&state.pool)
    .await
    .map_err(|_| AppError::Database)?;

    Ok(Json(expenses))
}

pub async fn delete_expense(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    let result = sqlx::query(
        r#"
        DELETE FROM expenses
        WHERE id = $1
        AND user_id = $2
        "#,
    )
    .bind(id)
    .bind(auth_user.id)
    .execute(&state.pool)
    .await
    .map_err(|_| AppError::Database)?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }

    info!(
        user_id = %auth_user.id,
        expense_id = %id,
        "Expense deleted"
    );

    Ok(StatusCode::NO_CONTENT)
}

#[derive(sqlx::FromRow)]
pub struct TotalResult {
    pub total: f64,
}

pub async fn get_total(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
) -> Result<Json<serde_json::Value>, AppError> {
    let result = sqlx::query_as::<_, TotalResult>(
        r#"
        SELECT COALESCE(SUM(amount), 0.0) AS total
        FROM expenses
        WHERE user_id = $1
        "#,
    )
    .bind(auth_user.id)
    .fetch_one(&state.pool)
    .await
    .map_err(|_| AppError::Database)?;

    Ok(Json(serde_json::json!({
        "total": result.total
    })))
}

#[derive(serde::Serialize, sqlx::FromRow)]
pub struct CategoryTotal {
    pub category: String,
    pub total: f64,
}

pub async fn get_category_totals(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
) -> Result<Json<Vec<CategoryTotal>>, AppError> {
    let totals = sqlx::query_as::<_, CategoryTotal>(
        r#"
        SELECT category, SUM(amount) AS total
        FROM expenses
        WHERE user_id = $1
        GROUP BY category
        "#,
    )
    .bind(auth_user.id)
    .fetch_all(&state.pool)
    .await
    .map_err(|_| AppError::Database)?;

    Ok(Json(totals))
}
