use std::sync::Arc;

use crate::{
    auth::service::AuthService, budget::service::BudgetService,
    dashboard::service::DashboardService, transaction::service::TransactionService,
};

#[derive(Clone)]
pub struct AppState {
    pub auth_service: Arc<AuthService>,
    pub transaction_service: Arc<TransactionService>,
    pub dashboard_service: Arc<DashboardService>,
    pub budget_service: Arc<BudgetService>,
    pub jwt_secret: String,
}
