mod common;

use axum::http::{Method, StatusCode};
use serde_json::json;

use common::{register_and_login, send_request, test_app};

#[tokio::test]
async fn creates_expense_transaction() {
    let app = test_app().await;
    let (router, token) = register_and_login(app.router).await;

    let response = send_request(
        router,
        Method::POST,
        "/transactions",
        Some(&token),
        Some(json!({
            "type": "EXPENSE",
            "description": "Groceries",
            "amount": "12500.50",
            "currency": "NGN",
            "category": "Food",
            "occurred_at": "2026-10-10T15:30:00Z"
        })),
    )
    .await;

    assert_eq!(
        response.status,
        StatusCode::CREATED,
        "Transaction creation failed: {}",
        response.body
    );
}

#[tokio::test]
async fn rejects_invalid_transaction_amounts() {
    let app = test_app().await;
    let (router, token) = register_and_login(app.router).await;

    for amount in ["0", "-10.00", "abc", "12.12345"] {
        let response = send_request(
            router.clone(),
            Method::POST,
            "/transactions",
            Some(&token),
            Some(json!({
                "type": "EXPENSE",
                "description": "Test transaction",
                "amount": amount,
                "currency": "NGN",
                "category": "Food",
                "occurred_at": "2026-10-10T15:30:00Z"
            })),
        )
        .await;

        assert_eq!(
            response.status,
            StatusCode::BAD_REQUEST,
            "Amount {amount:?} should be rejected. Response: {}",
            response.body
        );
    }
}

#[tokio::test]
async fn listing_transactions_requires_authentication() {
    let app = test_app().await;

    let response = send_request(app.router, Method::GET, "/transactions", None, None).await;

    assert_eq!(
        response.status,
        StatusCode::UNAUTHORIZED,
        "Unexpected response body: {}",
        response.body
    );
}
