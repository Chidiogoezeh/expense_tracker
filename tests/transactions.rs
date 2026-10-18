mod common;

use axum::http::{Method, StatusCode};
use common::{send_request, test_app};
use serde_json::{Value, json};
use uuid::Uuid;

async fn register_and_login(router: axum::Router) -> (axum::Router, String) {
    let email = format!("user-{}@example.com", Uuid::new_v4());

    let registration = send_request(
        router.clone(),
        Method::POST,
        "/register",
        None,
        Some(json!({
            "email": email,
            "password": "StrongPassword123"
        })),
    )
    .await;

    assert_eq!(
        registration.status,
        StatusCode::CREATED,
        "Registration failed: {}",
        registration.body
    );

    let login = send_request(
        router.clone(),
        Method::POST,
        "/login",
        None,
        Some(json!({
            "email": email,
            "password": "StrongPassword123"
        })),
    )
    .await;

    assert_eq!(login.status, StatusCode::OK, "Login failed: {}", login.body);

    let response: Value =
        serde_json::from_str(&login.body).expect("Login response must be valid JSON");

    let token = response["token"]
        .as_str()
        .expect("Login response must contain a string field named token")
        .to_string();

    (router, token)
}

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
