mod auth;
mod error;
mod expense;
mod middleware;

use axum::{
    Router, middleware as axum_middleware,
    routing::{delete, get, post},
};

use sqlx::{PgPool, postgres::PgPoolOptions};

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub jwt_secret: String,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    let state = AppState { pool, jwt_secret };

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
        .with_state(state);

    // Start server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Server running on http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}
