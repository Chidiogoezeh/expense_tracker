use axum::{
    Extension, Json,
    extract::{Query, State},
};
use chrono::NaiveDate;
use serde::Deserialize;

use crate::{error::AppError, middleware::AuthUser, state::AppState};

#[derive(Deserialize)]
pub struct ReportQuery {
    from: NaiveDate,
    to: NaiveDate,
    currency: Option<String>,
}

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
