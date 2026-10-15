mod auth;
mod budget;
mod config;
mod dashboard;
mod error;
mod middleware;
mod state;
mod transaction;

use std::net::SocketAddr;
use std::sync::Arc;

use axum::{
    Router, middleware as axum_middleware,
    routing::{get, post},
};
use sqlx::postgres::PgPoolOptions;
use tower_governor::{GovernorLayer, governor::GovernorConfigBuilder};
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;

use config::Config;
use state::AppState;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let config = Config::from_env().expect("invalid application configuration");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await
        .map_err(|error| {
            tracing::error!(
                error = %error,
                "Failed to connect to PostgreSQL"
            );
            error
        })?;

    tracing::info!("Database connection pool established");

    let user_repository = auth::repository::UserRepository::new(pool.clone());

    let transaction_repository = transaction::repository::TransactionRepository::new(pool.clone());

    let dashboard_repository = dashboard::repository::DashboardRepository::new(pool.clone());

    let dashboard_service = dashboard::service::DashboardService::new(dashboard_repository);

    let budget_repository = budget::repository::BudgetRepository::new(pool.clone());

    let budget_service = budget::service::BudgetService::new(budget_repository);

    let auth_service = auth::service::AuthService::new(user_repository, config.jwt_secret.clone());

    let transaction_service = transaction::service::TransactionService::new(transaction_repository);

    let state = AppState {
        auth_service: Arc::new(auth_service),
        transaction_service: Arc::new(transaction_service),
        dashboard_service: Arc::new(dashboard_service),
        budget_service: Arc::new(budget_service),
        jwt_secret: config.jwt_secret,
    };

    let register_config = GovernorConfigBuilder::default()
        .per_second(1)
        .burst_size(5)
        .finish()
        .expect("valid registration rate-limit configuration");

    let register_route = Router::new()
        .route("/register", post(auth::handler::register))
        .layer(GovernorLayer::new(register_config));

    let login_config = GovernorConfigBuilder::default()
        .per_second(1)
        .burst_size(5)
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

    let app = register_route
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
                .on_response(
                    tower_http::trace::DefaultOnResponse::new().level(tracing::Level::INFO),
                ),
        )
        .with_state(state);

    let address = format!("{}:{}", config.bind_address, config.port);

    let listener = tokio::net::TcpListener::bind(&address).await?;

    tracing::info!(%address, "HTTP server listening");

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .with_graceful_shutdown(async {
        if tokio::signal::ctrl_c().await.is_ok() {
            tracing::info!("Shutdown signal received");
        }
    })
    .await?;

    tracing::info!("HTTP server stopped");

    Ok(())
}
