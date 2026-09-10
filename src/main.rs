use std::sync::Arc;

use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    routing::{get, post},
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

#[tokio::main]
async fn main() {
    // Create shared application state
    let state: AppState = Arc::new(Mutex::new(ExpenseTracker::new()));

    // Route -> Handler
    let app = Router::new()
        .route("/expenses", get(get_expenses))
        .with_state(state);

    // Start server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Server running on http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}
