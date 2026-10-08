mod auth;
mod config;
mod error;
mod expense;
mod middleware;

use config::Config;

use axum::{
    Router, middleware as axum_middleware,
    routing::{delete, get, post},
};

use sqlx::{PgPool, postgres::PgPoolOptions};

use tower_http::trace::TraceLayer;

use tracing::info;

use tracing_subscriber::EnvFilter;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub jwt_secret: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::from_env()?;

    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await?;

    let state = AppState {
        pool,
        jwt_secret: config.jwt_secret.clone(),
    };

    // Protected routes
    let protected_routes = Router::new()
        .route("/profile", get(auth::profile))
        .route("/expenses", get(expense::get_expenses))
        .route("/expenses", post(expense::create_expense))
        .route("/expenses/{id}", delete(expense::delete_expense))
        .route("/expenses/total", get(expense::get_total))
        .route("/expenses/categories", get(expense::get_category_totals))
        .layer(axum_middleware::from_fn_with_state(
            state.clone(),
            middleware::auth_middleware,
        ));

    // Public routes
    let app = Router::new()
        .route("/register", post(auth::register))
        .route("/login", post(auth::login))
        .merge(protected_routes)
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    // Start server
    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", config.port)).await?;

    info!("Server running on http://127.0.0.1:{}", config.port);

    axum::serve(listener, app).await?;

    Ok(())
}
