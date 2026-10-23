use serde_json::{Value, json};
use std::time::Duration;
use uuid::Uuid;

use axum::{
    Router,
    body::Body,
    extract::ConnectInfo,
    http::{Method, Request, StatusCode, header},
};
use http_body_util::BodyExt;
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use tower::ServiceExt;

use expense_tracker::{
    AppState,
    auth::{repository::UserRepository, service::AuthService},
    budget::{repository::BudgetRepository, service::BudgetService},
    build_app,
    dashboard::{repository::DashboardRepository, service::DashboardService},
    transaction::{repository::TransactionRepository, service::TransactionService},
};

pub struct TestApp {
    pub router: Router,
}

pub struct TestResponse {
    pub status: StatusCode,
    pub body: String,
}

#[allow(dead_code)]
pub async fn register_and_login(router: Router) -> (Router, String) {
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

pub async fn test_app() -> TestApp {
    let database_url = std::env::var("TEST_DATABASE_URL")
        .expect("Set TEST_DATABASE_URL to your dedicated test database");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&database_url)
        .await
        .expect("Could not connect to TEST_DATABASE_URL");

    // Verify the connected database before applying migrations.
    let database_name: String = sqlx::query_scalar("SELECT current_database()")
        .fetch_one(&pool)
        .await
        .expect("Could not identify the connected database");

    assert_eq!(
        database_name, "expense_tracker_test",
        "Refusing to run tests against a database other than expense_tracker_test"
    );

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Could not apply database migrations");

    let jwt_secret = "integration-test-secret-use-only-never-use-in-production".to_string();

    let auth_service = AuthService::new(UserRepository::new(pool.clone()), jwt_secret.clone());

    let transaction_service = TransactionService::new(TransactionRepository::new(pool.clone()));

    let dashboard_service = DashboardService::new(DashboardRepository::new(pool.clone()));

    let budget_service = BudgetService::new(BudgetRepository::new(pool.clone()));

    let state = AppState {
        db: pool.clone(),
        auth_service: std::sync::Arc::new(auth_service),
        transaction_service: std::sync::Arc::new(transaction_service),
        dashboard_service: std::sync::Arc::new(dashboard_service),
        budget_service: std::sync::Arc::new(budget_service),
        jwt_secret,
    };

    TestApp {
        router: build_app(state),
    }
}

pub async fn send_request(
    router: Router,
    method: Method,
    uri: &str,
    token: Option<&str>,
    body: Option<serde_json::Value>,
) -> TestResponse {
    let body = match body {
        Some(value) => Body::from(value.to_string()),
        None => Body::empty(),
    };

    let mut builder = Request::builder()
        .method(method)
        .uri(uri)
        .header(header::CONTENT_TYPE, "application/json");

    if let Some(token) = token {
        builder = builder.header(header::AUTHORIZATION, format!("Bearer {token}"));
    }

    let request = builder
        .extension(ConnectInfo(SocketAddr::from(([127, 0, 0, 1], 40000))))
        .body(body)
        .expect("Could not build HTTP test request");

    let response = router
        .oneshot(request)
        .await
        .expect("HTTP test request failed");

    let status = response.status();

    let bytes = response
        .into_body()
        .collect()
        .await
        .expect("Could not read HTTP response body")
        .to_bytes();

    let body = String::from_utf8(bytes.to_vec()).expect("HTTP response body was not valid UTF-8");

    TestResponse { status, body }
}
