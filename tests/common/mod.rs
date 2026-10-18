use std::time::Duration;

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

    // The app uses IP-based rate limiting on authentication routes.
    builder = builder.extension(ConnectInfo(SocketAddr::from(([127, 0, 0, 1], 40000))));

    let request = builder
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
