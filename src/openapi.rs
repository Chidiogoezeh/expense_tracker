use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::{Modify, OpenApi};

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer_auth",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            );
        }
    }
}

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::health::health,
        crate::auth::handler::register,
        crate::auth::handler::login,
        crate::auth::handler::profile,
        crate::transaction::handler::create_transaction,
        crate::transaction::handler::list_transactions,
        crate::transaction::handler::get_transaction,
        crate::transaction::handler::update_transaction,
        crate::transaction::handler::delete_transaction,
        crate::dashboard::handler::summary,
        crate::dashboard::handler::category_report,
        crate::budget::handler::list,
        crate::budget::handler::create,
        crate::budget::handler::update,
        crate::budget::handler::delete
    ),
    components(
        schemas(
            crate::auth::handler::RegisterRequest,
            crate::auth::handler::LoginRequest,
            crate::auth::handler::RegistrationResponse,
            crate::auth::handler::LoginResponse,
            crate::auth::handler::ProfileResponse,
            crate::transaction::model::CreateTransactionRequest,
            crate::transaction::model::UpdateTransactionRequest,
            crate::transaction::model::TransactionResponse,
            crate::transaction::model::TransactionListResponse,
            crate::transaction::model::TransactionType,
            crate::budget::handler::CreateBudgetRequest,
            crate::budget::handler::UpdateBudgetRequest,
            crate::budget::service::BudgetResponse,
            crate::dashboard::service::DashboardSummary,
            crate::dashboard::service::CategoryReportItem
        )
    ),
    modifiers(&SecurityAddon),
    tags(
        (name = "Health", description = "API health"),
        (name = "Authentication", description = "Registration, login, and profile"),
        (name = "Transactions", description = "Income and expense transactions"),
        (name = "Budgets", description = "Monthly budgets"),
        (name = "Reports", description = "Financial summaries and spending reports")
    ),
    info(
        title = "Personal Finance API",
        version = "1.0.0",
        description = "API for managing personal income, expenses, budgets, and financial reports."
    )
)]
pub struct ApiDoc;
