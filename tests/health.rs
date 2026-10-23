mod common;

use axum::http::{Method, StatusCode};
use common::{send_request, test_app};

#[tokio::test]
async fn health_returns_ok_when_database_is_available() {
    let app = test_app().await;

    let response = send_request(app.router, Method::GET, "/health", None, None).await;

    assert_eq!(
        response.status,
        StatusCode::OK,
        "Health check failed: {}",
        response.body
    );
}
