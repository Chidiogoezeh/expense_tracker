use bigdecimal::{BigDecimal, ToPrimitive};
use chrono::{Datelike, NaiveDate};
use serde::Serialize;
use std::str::FromStr;
use uuid::Uuid;

use crate::error::AppError;

use super::repository::{BudgetRepository, BudgetWithSpendingRow};

pub struct BudgetService {
    repository: BudgetRepository,
}

#[derive(Serialize)]
pub struct BudgetResponse {
    pub id: Uuid,
    pub category: String,
    pub currency: String,
    pub month: NaiveDate,
    pub amount_limit: String,
    pub spent: String,
    pub remaining: String,
    pub usage_percentage: f64,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

pub struct BudgetInput {
    pub category: String,
    pub amount_limit: String,
    pub currency: String,
    pub month: NaiveDate,
}

impl BudgetService {
    pub fn new(repository: BudgetRepository) -> Self {
        Self { repository }
    }

    pub async fn list(&self, user_id: Uuid) -> Result<Vec<BudgetResponse>, AppError> {
        let rows = self.repository.find_all_by_user(user_id).await?;
        rows.into_iter().map(to_response).collect()
    }

    pub async fn create(
        &self,
        user_id: Uuid,
        input: BudgetInput,
    ) -> Result<BudgetResponse, AppError> {
        let input = validate_input(input)?;
        let row = self
            .repository
            .create(
                user_id,
                &input.category,
                &input.amount_limit,
                &input.currency,
                input.month,
            )
            .await?;

        let spending = self
            .repository
            .find_by_id_and_user(row.id, user_id)
            .await?
            .ok_or(AppError::Internal)?;

        to_response(spending)
    }

    pub async fn get(&self, user_id: Uuid, id: Uuid) -> Result<BudgetResponse, AppError> {
        let row = self
            .repository
            .find_by_id_and_user(id, user_id)
            .await?
            .ok_or(AppError::NotFound)?;

        to_response(row)
    }

    pub async fn update(
        &self,
        user_id: Uuid,
        id: Uuid,
        patch: BudgetPatch,
    ) -> Result<BudgetResponse, AppError> {
        if patch.category.is_none()
            && patch.amount_limit.is_none()
            && patch.currency.is_none()
            && patch.month.is_none()
        {
            return Err(AppError::BadRequest);
        }

        let current = self
            .repository
            .find_by_id_and_user(id, user_id)
            .await?
            .ok_or(AppError::NotFound)?;

        let input = validate_input(BudgetInput {
            category: patch.category.unwrap_or(current.category),
            amount_limit: patch
                .amount_limit
                .unwrap_or_else(|| current.amount_limit.to_string()),
            currency: patch.currency.unwrap_or(current.currency),
            month: patch.month.unwrap_or(current.month),
        })?;

        self.repository
            .update_by_id_and_user(
                id,
                user_id,
                &input.category,
                &input.amount_limit,
                &input.currency,
                input.month,
            )
            .await?
            .ok_or(AppError::NotFound)?;

        self.get(user_id, id).await
    }

    pub async fn delete(&self, user_id: Uuid, id: Uuid) -> Result<(), AppError> {
        if self.repository.delete_by_id_and_user(id, user_id).await? {
            Ok(())
        } else {
            Err(AppError::NotFound)
        }
    }
}

pub struct BudgetPatch {
    pub category: Option<String>,
    pub amount_limit: Option<String>,
    pub currency: Option<String>,
    pub month: Option<NaiveDate>,
}

fn validate_input(input: BudgetInput) -> Result<ValidatedBudget, AppError> {
    let category = input.category.trim().to_string();

    if category.is_empty() || category.chars().count() > 50 {
        return Err(AppError::BadRequest);
    }

    if input.currency.len() != 3 || !input.currency.bytes().all(|byte| byte.is_ascii_uppercase()) {
        return Err(AppError::BadRequest);
    }

    if input.month.day() != 1 {
        return Err(AppError::BadRequest);
    }

    let amount_limit =
        BigDecimal::from_str(input.amount_limit.trim()).map_err(|_| AppError::BadRequest)?;

    if amount_limit <= 0 {
        return Err(AppError::BadRequest);
    }

    // NUMERIC(19,4) permits at most 15 integer digits and 4 fractional digits.
    let normalized = amount_limit.normalized();
    let fixed = normalized.to_string();
    let (integer_part, fractional_part) = fixed.split_once('.').unwrap_or((&fixed, ""));

    let integer_digits = integer_part.trim_start_matches('-').len();
    let fractional_digits = fractional_part.len();

    if integer_digits > 15 || fractional_digits > 4 {
        return Err(AppError::BadRequest);
    }

    Ok(ValidatedBudget {
        category,
        amount_limit,
        currency: input.currency,
        month: input.month,
    })
}

struct ValidatedBudget {
    category: String,
    amount_limit: BigDecimal,
    currency: String,
    month: NaiveDate,
}

fn to_response(row: BudgetWithSpendingRow) -> Result<BudgetResponse, AppError> {
    let remaining = row.amount_limit.clone() - row.spent.clone();

    let percentage = (&row.spent / &row.amount_limit * BigDecimal::from(100))
        .to_f64()
        .ok_or(AppError::Internal)?;

    Ok(BudgetResponse {
        id: row.id,
        category: row.category,
        currency: row.currency.trim().to_string(),
        month: row.month,
        amount_limit: format_money(row.amount_limit),
        spent: format_money(row.spent),
        remaining: format_money(remaining),
        usage_percentage: percentage,
        created_at: row.created_at,
    })
}

fn format_money(value: BigDecimal) -> String {
    let mut value = value.to_string();

    if !value.contains('.') {
        value.push_str(".00");
        return value;
    }

    while value.ends_with('0') && value.len() - value.rfind('.').unwrap_or(value.len()) - 1 > 2 {
        value.pop();
    }

    if value.ends_with('.') {
        value.push_str("00");
    } else {
        let decimals = value.len() - value.rfind('.').unwrap() - 1;
        if decimals == 1 {
            value.push('0');
        }
    }

    value
}
