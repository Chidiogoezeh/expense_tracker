pub mod handler;
pub mod repository;
pub mod service;

use axum::{Router, routing::get};

use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/dashboard/summary", get(handler::summary))
        .route("/reports/categories", get(handler::category_report))
}
