use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::{delete, get, post},
};

use sqlx::{PgPool, postgres::PgPoolOptions};
use uuid::Uuid;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub jwt_secret: String,
}

#[derive(serde::Deserialize)]
struct CreateExpense {
    description: String,
    amount: f64,
    category: String,
}

#[derive(Debug, serde::Serialize, sqlx::FromRow)]
struct ExpenseRow {
    id: Uuid,
    description: String,
    amount: f64,
    category: String,
}

#[derive(sqlx::FromRow)]
struct TotalResult {
    total: f64,
}

#[derive(serde::Serialize, sqlx::FromRow)]
struct CategoryTotal {
    category: String,
    total: f64,
}

async fn get_expenses(State(pool): State<PgPool>) -> Result<Json<Vec<ExpenseRow>>, StatusCode> {
    let expenses =
        sqlx::query_as::<_, ExpenseRow>("SELECT id, description, amount, category FROM expenses")
            .fetch_all(&pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(expenses))
}

async fn create_expense(
    State(pool): State<PgPool>,
    Json(input): Json<CreateExpense>,
) -> Result<(StatusCode, Json<ExpenseRow>), StatusCode> {
    if input.amount <= 0.0 {
        return Err(StatusCode::BAD_REQUEST);
    }

    let id = Uuid::new_v4();

    let expense = sqlx::query_as::<_, ExpenseRow>(
        r#"
        INSERT INTO expenses (id, description, amount, category)
        VALUES ($1, $2, $3, $4)
        RETURNING id, description, amount, category
        "#,
    )
    .bind(id)
    .bind(input.description)
    .bind(input.amount)
    .bind(input.category)
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::CREATED, Json(expense)))
}

async fn delete_expense(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    let result = sqlx::query("DELETE FROM expenses WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(StatusCode::NO_CONTENT)
}

async fn get_total(State(pool): State<PgPool>) -> Result<Json<serde_json::Value>, StatusCode> {
    let result = sqlx::query_as::<_, TotalResult>(
        "SELECT COALESCE(SUM(amount), 0.0) AS total FROM expenses",
    )
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(serde_json::json!({
        "total": result.total
    })))
}

async fn get_category_totals(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<CategoryTotal>>, StatusCode> {
    let totals = sqlx::query_as::<_, CategoryTotal>(
        r#"
        SELECT category, SUM(amount) AS total
        FROM expenses
        GROUP BY category
        "#,
    )
    .fetch_all(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(totals))
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .unwrap();

    let state = AppState { pool, jwt_secret };

    // Routes -> Handlers
    let app = Router::new()
        .route("/expenses", get(get_expenses))
        .route("/expenses", post(create_expense))
        .route("/expenses/{id}", delete(delete_expense))
        .route("/expenses/total", get(get_total))
        .route("/expenses/categories", get(get_category_totals))
        .with_state(pool);

    // Start server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Server running on http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}
