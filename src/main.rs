use std::{collections::HashMap, sync::Arc};

use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::{delete, get, post},
};

use tokio::sync::Mutex;

use expense_tracker::error::ExpenseError;
use expense_tracker::expense::Expense;
use expense_tracker::tracker::ExpenseTracker;

#[derive(serde::Deserialize)]
struct CreateExpense {
    description: String,
    amount: f64,
    category: String,
}

type AppState = Arc<Mutex<ExpenseTracker>>;

async fn get_expenses(State(state): State<AppState>) -> Json<Vec<Expense>> {
    let tracker = state.lock().await;

    Json(tracker.get_expenses().clone())
}

async fn create_expense(
    State(state): State<AppState>,
    Json(input): Json<CreateExpense>,
) -> Result<(StatusCode, Json<Expense>), (StatusCode, Json<serde_json::Value>)> {
    let mut tracker = state.lock().await;

    match tracker.add_expense(input.description, input.amount, input.category) {
        Ok(()) => {
            let expense = tracker.get_expenses().last().unwrap().clone();

            Ok((StatusCode::CREATED, Json(expense)))
        }

        Err(ExpenseError::InvalidAmount) => Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "Amount must be greater than zero"
            })),
        )),

        Err(ExpenseError::ExpenseNotFound) => Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "Expense not found"
            })),
        )),
    }
}

async fn delete_expense(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<StatusCode, (StatusCode, Json<serde_json::Value>)> {
    let mut tracker = state.lock().await;

    match tracker.delete_expense(&id) {
        Ok(()) => Ok(StatusCode::NO_CONTENT),

        Err(ExpenseError::ExpenseNotFound) => Err((
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "error": "Expense not found"
            })),
        )),

        Err(ExpenseError::InvalidAmount) => Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "Invalid amount"
            })),
        )),
    }
}

async fn get_total(State(state): State<AppState>) -> Json<serde_json::Value> {
    let tracker = state.lock().await;

    Json(serde_json::json!({
        "total": tracker.calculate_total()
    }))
}

async fn get_category_totals(State(state): State<AppState>) -> Json<HashMap<String, f64>> {
    let tracker = state.lock().await;

    Json(tracker.get_category_totals().clone())
}

#[tokio::main]
async fn main() {
    // Create shared application state
    let state: AppState = Arc::new(Mutex::new(ExpenseTracker::new()));

    // Routes -> Handlers
    let app = Router::new()
        .route("/expenses", get(get_expenses))
        .route("/expenses", post(create_expense))
        .route("/expenses/{id}", delete(delete_expense))
        .route("/expenses/total", get(get_total))
        .route("/expenses/categories", get(get_category_totals))
        .with_state(state);

    // Start server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Server running on http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}
