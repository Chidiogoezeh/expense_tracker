use crate::{
    auth::service::AuthService, budget::service::BudgetService,
    dashboard::service::DashboardService, transaction::service::TransactionService,
};
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub auth_service: Arc<AuthService>,
    pub transaction_service: Arc<TransactionService>,
    pub dashboard_service: Arc<DashboardService>,
    pub budget_service: Arc<BudgetService>,
    pub jwt_secret: String,
}
