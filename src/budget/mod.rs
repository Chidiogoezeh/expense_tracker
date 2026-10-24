pub mod handler;
pub mod repository;
pub mod service;

use crate::state::AppState;
use axum::{Router, routing::get};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/budgets", get(handler::list).post(handler::create))
        .route(
            "/budgets/{id}",
            axum::routing::patch(handler::update).delete(handler::delete),
        )
}
