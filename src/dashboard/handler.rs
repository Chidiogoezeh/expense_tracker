use crate::{error::AppError, middleware::AuthUser, state::AppState};
use axum::{
    Extension, Json,
    extract::{Query, State},
};
use chrono::NaiveDate;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct ReportQuery {
    from: NaiveDate,
    to: NaiveDate,
    currency: Option<String>,
}

#[utoipa::path(
    get,
    path = "/dashboard/summary",
    params(
        ("from" = NaiveDate, Query, description = "Inclusive start date, YYYY-MM-DD"),
        ("to" = NaiveDate, Query, description = "Inclusive end date, YYYY-MM-DD"),
        ("currency" = Option<String>, Query, description = "Currency code; defaults according to the service")
    ),
    responses(
        (status = 200, description = "Financial summary", body = super::service::DashboardSummary),
        (status = 400, description = "Invalid date range or currency"),
        (status = 401, description = "Authentication required"),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer_auth" = [])),
    tag = "Reports"
)]
pub async fn summary(
    State(state): State<AppState>,
    Extension(user): Extension<AuthUser>,
    Query(query): Query<ReportQuery>,
) -> Result<Json<super::service::DashboardSummary>, AppError> {
    let result = state
        .dashboard_service
        .summary(user.id, query.from, query.to, query.currency)
        .await?;

    Ok(Json(result))
}

#[utoipa::path(
    get,
    path = "/reports/categories",
    params(
        ("from" = NaiveDate, Query, description = "Inclusive start date, YYYY-MM-DD"),
        ("to" = NaiveDate, Query, description = "Inclusive end date, YYYY-MM-DD"),
        ("currency" = Option<String>, Query, description = "Currency code; defaults according to the service")
    ),
    responses(
        (status = 200, description = "Expense totals grouped by category", body = [super::service::CategoryReportItem]),
        (status = 400, description = "Invalid date range or currency"),
        (status = 401, description = "Authentication required"),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer_auth" = [])),
    tag = "Reports"
)]
pub async fn category_report(
    State(state): State<AppState>,
    Extension(user): Extension<AuthUser>,
    Query(query): Query<ReportQuery>,
) -> Result<Json<Vec<super::service::CategoryReportItem>>, AppError> {
    let result = state
        .dashboard_service
        .category_report(user.id, query.from, query.to, query.currency)
        .await?;

    Ok(Json(result))
}
