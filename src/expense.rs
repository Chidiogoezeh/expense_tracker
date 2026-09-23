use axum::{
    Json,
    extract::{Extension, Path, State},
    http::StatusCode,
};

use sqlx::PgPool;
use uuid::Uuid;

use crate::{AppState, middleware::AuthUser};

#[derive(Clone, serde::Serialize)]
pub struct ExpenseId(Uuid);

impl ExpenseId {
    pub fn new() -> ExpenseId {
        ExpenseId(Uuid::new_v4())
    }

    pub fn as_string(&self) -> String {
        self.0.to_string()
    }

    pub fn as_uuid(&self) -> Uuid {
        self.0
    }
}

impl Default for ExpenseId {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, serde::Serialize)]
pub struct Expense {
    pub id: ExpenseId,
    pub description: String,
    pub amount: f64,
    pub category: String,
}

impl Expense {
    pub fn new(description: String, amount: f64, category: String) -> Expense {
        Expense {
            id: ExpenseId::new(),
            description,
            amount,
            category,
        }
    }
}

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

pub trait DisplayExpense {
    fn display(&self);
}

impl DisplayExpense for Expense {
    fn display(&self) {
        println!(
            "ID: {} | {} | ₦{} | {}",
            self.id.0, self.description, self.amount, self.category
        );
    }
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
