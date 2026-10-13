use std::sync::Arc;

use crate::{auth::service::AuthService, transaction::service::TransactionService};

#[derive(Clone)]
pub struct AppState {
    pub auth_service: Arc<AuthService>,
    pub transaction_service: Arc<TransactionService>,
    pub jwt_secret: String,
}
