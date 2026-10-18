mod common;

use axum::http::{Method, StatusCode};
use common::{send_request, test_app};

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
