mod common;

use axum::http::{Method, StatusCode};
use common::{register_and_login, send_request, test_app};
use serde_json::json;

#[tokio::test]
async fn creates_monthly_budget() {
    let app = test_app().await;
    let (router, token) = register_and_login(app.router).await;

    let response = send_request(
        router,
        Method::POST,
        "/budgets",
        Some(&token),
        Some(json!({
            "category": "Food",
            "amount_limit": "10000.00",
            "currency": "NGN",
            "month": "2026-10-01"
        })),
    )
    .await;

    assert_eq!(
        response.status,
        StatusCode::CREATED,
        "Budget creation failed: {}",
        response.body
    );
}

#[tokio::test]
async fn listing_budgets_requires_authentication() {
    let app = test_app().await;

    let response = send_request(app.router, Method::GET, "/budgets", None, None).await;

    assert_eq!(
        response.status,
        StatusCode::UNAUTHORIZED,
        "Unexpected response body: {}",
        response.body
    );
}
