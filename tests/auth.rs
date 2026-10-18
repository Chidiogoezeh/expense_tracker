mod common;

use axum::http::{Method, StatusCode};
use common::{send_request, test_app};
use serde_json::json;
use uuid::Uuid;

fn unique_email() -> String {
    format!("user-{}@example.com", Uuid::new_v4())
}

#[tokio::test]
async fn registration_succeeds_with_valid_data() {
    let app = test_app().await;

    let response = send_request(
        app.router,
        Method::POST,
        "/register",
        None,
        Some(json!({
            "email": unique_email(),
            "password": "StrongPassword123"
        })),
    )
    .await;

    assert_eq!(
        response.status,
        StatusCode::CREATED,
        "Unexpected response body: {}",
        response.body
    );
}

#[tokio::test]
async fn duplicate_email_returns_conflict() {
    let app = test_app().await;
    let email = unique_email();

    let first = send_request(
        app.router.clone(),
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
        first.status,
        StatusCode::CREATED,
        "Initial registration failed: {}",
        first.body
    );

    let second = send_request(
        app.router,
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
        second.status,
        StatusCode::CONFLICT,
        "Duplicate registration should fail: {}",
        second.body
    );
}

#[tokio::test]
async fn invalid_registration_returns_bad_request() {
    let app = test_app().await;

    let response = send_request(
        app.router,
        Method::POST,
        "/register",
        None,
        Some(json!({
            "email": "not-an-email",
            "password": "short"
        })),
    )
    .await;

    assert_eq!(
        response.status,
        StatusCode::BAD_REQUEST,
        "Unexpected response body: {}",
        response.body
    );
}

#[tokio::test]
async fn wrong_password_returns_unauthorized() {
    let app = test_app().await;
    let email = unique_email();

    let registration = send_request(
        app.router.clone(),
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
        app.router,
        Method::POST,
        "/login",
        None,
        Some(json!({
            "email": email,
            "password": "WrongPassword123"
        })),
    )
    .await;

    assert_eq!(
        login.status,
        StatusCode::UNAUTHORIZED,
        "Wrong password should be rejected: {}",
        login.body
    );
}

#[tokio::test]
async fn profile_requires_authentication() {
    let app = test_app().await;

    let response = send_request(app.router, Method::GET, "/profile", None, None).await;

    assert_eq!(
        response.status,
        StatusCode::UNAUTHORIZED,
        "Unexpected response body: {}",
        response.body
    );
}

#[tokio::test]
async fn invalid_jwt_returns_unauthorized() {
    let app = test_app().await;

    let response = send_request(
        app.router,
        Method::GET,
        "/profile",
        Some("not-a-valid-jwt"),
        None,
    )
    .await;

    assert_eq!(
        response.status,
        StatusCode::UNAUTHORIZED,
        "Unexpected response body: {}",
        response.body
    );
}
