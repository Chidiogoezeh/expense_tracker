pub mod auth;
pub mod budget;
pub mod config;
pub mod dashboard;
pub mod error;
pub mod middleware;
pub mod state;
pub mod transaction;

use axum::{
    Router, middleware as axum_middleware,
    routing::{get, post},
};
use tower_governor::{GovernorLayer, governor::GovernorConfigBuilder};
use tower_http::trace::TraceLayer;
use tracing::Level;

pub use state::AppState;

/// Builds the same application router used by the production server
/// and integration tests.
pub fn build_app(state: AppState) -> Router {
    let register_config = GovernorConfigBuilder::default()
        .per_second(1)
        .burst_size(3)
        .finish()
        .expect("valid registration rate-limit configuration");

    let register_route = Router::new()
        .route("/register", post(auth::handler::register))
        .layer(GovernorLayer::new(register_config));

    let login_config = GovernorConfigBuilder::default()
        .per_second(1)
        .burst_size(3)
        .finish()
        .expect("valid login rate-limit configuration");

    let login_route = Router::new()
        .route("/login", post(auth::handler::login))
        .layer(GovernorLayer::new(login_config));

    let protected_routes = Router::new()
        .route("/profile", get(auth::handler::profile))
        .merge(transaction::handler::routes())
        .merge(dashboard::routes())
        .merge(budget::routes())
        .layer(axum_middleware::from_fn_with_state(
            state.clone(),
            middleware::auth_middleware,
        ));

    register_route
        .merge(login_route)
        .merge(protected_routes)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &axum::http::Request<_>| {
                    tracing::info_span!(
                        "http_request",
                        method = %request.method(),
                        path = %request.uri().path(),
                    )
                })
                .on_response(tower_http::trace::DefaultOnResponse::new().level(Level::INFO)),
        )
        .with_state(state)
}
