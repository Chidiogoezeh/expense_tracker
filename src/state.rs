use std::sync::Arc;

use crate::{auth::service::AuthService, expense::service::ExpenseService};

#[derive(Clone)]
pub struct AppState {
    pub auth_service: Arc<AuthService>,
    pub expense_service: Arc<ExpenseService>,
    pub jwt_secret: String,
}
