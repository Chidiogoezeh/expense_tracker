use bigdecimal::BigDecimal;
use chrono::{DateTime, NaiveDate, TimeZone, Utc};
use uuid::Uuid;

use crate::error::AppError;
use serde::Serialize;

use super::repository::{CategoryTotalRow, DashboardRepository, SummaryRow};

const DEFAULT_CURRENCY: &str = "NGN";

pub struct DashboardService {
    repository: DashboardRepository,
}

#[derive(Serialize)]
pub struct DashboardSummary {
    pub currency: String,
    pub income: String,
    pub expenses: String,
    pub net_cash_flow: String,
}

#[derive(Serialize)]
pub struct CategoryReportItem {
    pub category: String,
    pub total: String,
}

impl DashboardService {
    pub fn new(repository: DashboardRepository) -> Self {
        Self { repository }
    }

    pub async fn summary(
        &self,
        user_id: Uuid,
        from: NaiveDate,
        to: NaiveDate,
        currency: Option<String>,
    ) -> Result<DashboardSummary, AppError> {
        let currency = validate_currency(currency)?;
        let (from, to_exclusive) = date_range(from, to)?;

        let row = self
            .repository
            .summary(user_id, &currency, from, to_exclusive)
            .await?;

        Ok(build_summary(row, currency))
    }

    pub async fn category_report(
        &self,
        user_id: Uuid,
        from: NaiveDate,
        to: NaiveDate,
        currency: Option<String>,
    ) -> Result<Vec<CategoryReportItem>, AppError> {
        let currency = validate_currency(currency)?;
        let (from, to_exclusive) = date_range(from, to)?;

        let rows = self
            .repository
            .category_totals(user_id, &currency, from, to_exclusive)
            .await?;

        Ok(rows.into_iter().map(build_category_item).collect())
    }
}

fn validate_currency(currency: Option<String>) -> Result<String, AppError> {
    let currency = currency.unwrap_or_else(|| DEFAULT_CURRENCY.to_string());

    if currency.len() != 3 || !currency.bytes().all(|byte| byte.is_ascii_uppercase()) {
        return Err(AppError::BadRequest);
    }

    Ok(currency)
}

fn date_range(from: NaiveDate, to: NaiveDate) -> Result<(DateTime<Utc>, DateTime<Utc>), AppError> {
    if from > to {
        return Err(AppError::BadRequest);
    }

    let start = Utc.from_utc_datetime(&from.and_hms_opt(0, 0, 0).ok_or(AppError::BadRequest)?);

    // The exclusive upper bound includes every time on `to`.
    let day_after_to = to.succ_opt().ok_or(AppError::BadRequest)?;

    let end = Utc.from_utc_datetime(
        &day_after_to
            .and_hms_opt(0, 0, 0)
            .ok_or(AppError::BadRequest)?,
    );

    Ok((start, end))
}

fn build_summary(row: SummaryRow, currency: String) -> DashboardSummary {
    let net_cash_flow = row.income.clone() - row.expenses.clone();

    DashboardSummary {
        currency,
        income: format_money(row.income),
        expenses: format_money(row.expenses),
        net_cash_flow: format_money(net_cash_flow),
    }
}

fn build_category_item(row: CategoryTotalRow) -> CategoryReportItem {
    CategoryReportItem {
        category: row.category,
        total: format_money(row.total),
    }
}

// Preserve the database's decimal value. Trim unnecessary trailing zeroes,
// but always return at least two fractional digits.
fn format_money(value: BigDecimal) -> String {
    let mut value = value.to_string();

    if !value.contains('.') {
        value.push_str(".00");
        return value;
    }

    while value.ends_with('0') && value.len() - value.rfind('.').unwrap_or(value.len()) - 1 > 2 {
        value.pop();
    }

    value
}
