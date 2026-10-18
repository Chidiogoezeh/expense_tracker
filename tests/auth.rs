mod common;

use axum::http::{Method, StatusCode};
use common::{send_request, test_app};

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
