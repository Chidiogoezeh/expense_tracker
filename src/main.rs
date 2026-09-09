use std::sync::Arc;

use axum::{
    Json, Router,
    extract::State,
    routing::{get, post},
};

use tokio::sync::Mutex;

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

#[tokio::main]
async fn main() {
    let state: AppState = Arc::new(Mutex::new(ExpenseTracker::new()));

    let app = Router::new()
        .route("/expenses", get(get_expenses))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Server running on http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}
