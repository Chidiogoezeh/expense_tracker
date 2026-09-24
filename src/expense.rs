use axum::{
    Json,
    extract::{Extension, Path, State},
    http::StatusCode,
};

use uuid::Uuid;

use crate::{AppState, middleware::AuthUser};

#[derive(serde::Deserialize)]
pub struct CreateExpense {
    pub description: String,
    pub amount: f64,
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
) -> Result<(StatusCode, Json<ExpenseRow>), StatusCode> {
    if input.amount <= 0.0 {
        return Err(StatusCode::BAD_REQUEST);
    }

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
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::CREATED, Json(expense)))
}

pub async fn get_expenses(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
) -> Result<Json<Vec<ExpenseRow>>, StatusCode> {
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
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(expenses))
}

pub async fn delete_expense(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
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
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(StatusCode::NO_CONTENT)
}

#[derive(sqlx::FromRow)]
pub struct TotalResult {
    pub total: f64,
}

pub async fn get_total(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
) -> Result<Json<serde_json::Value>, StatusCode> {
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
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

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
) -> Result<Json<Vec<CategoryTotal>>, StatusCode> {
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
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(totals))
}
