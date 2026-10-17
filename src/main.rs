use std::{net::SocketAddr, sync::Arc};

use expense_tracker::{AppState, auth, budget, build_app, config::Config, dashboard, transaction};
use sqlx::postgres::PgPoolOptions;
use tracing_subscriber::EnvFilter;

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

    let app = build_app(state);

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
