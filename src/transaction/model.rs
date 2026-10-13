use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::{Validate, ValidationError};

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
pub enum TransactionType {
    #[serde(rename = "INCOME")]
    Income,
    #[serde(rename = "EXPENSE")]
    Expense,
}

impl TransactionType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Income => "INCOME",
            Self::Expense => "EXPENSE",
        }
    }
}

fn validate_amount_string(value: &str) -> Result<(), ValidationError> {
    let value = value.trim();

    if value.is_empty() || !value.is_ascii() {
        return Err(ValidationError::new("invalid_amount"));
    }

    let (whole, fractional) = match value.split_once('.') {
        Some((whole, fractional)) => (whole, fractional),
        None => (value, ""),
    };

    if whole.is_empty()
        || !whole.chars().all(|c| c.is_ascii_digit())
        || !fractional.chars().all(|c| c.is_ascii_digit())
        || value.matches('.').count() > 1
        || fractional.len() > 4
    {
        return Err(ValidationError::new("invalid_amount"));
    }

    let significant_whole = whole.trim_start_matches('0');

    if significant_whole.len() > 15 {
        return Err(ValidationError::new("amount_precision"));
    }

    let amount = value
        .parse::<BigDecimal>()
        .map_err(|_| ValidationError::new("invalid_amount"))?;

    if amount <= BigDecimal::from(0) {
        return Err(ValidationError::new("invalid_amount"));
    }

    Ok(())
}

fn validate_currency(value: &str) -> Result<(), ValidationError> {
    if value != "NGN" {
        return Err(ValidationError::new("unsupported_currency"));
    }

    Ok(())
}

fn validate_description(value: &str) -> Result<(), ValidationError> {
    let value = value.trim();

    if !(1..=200).contains(&value.chars().count()) {
        return Err(ValidationError::new("invalid_description"));
    }

    Ok(())
}

fn validate_category(value: &str) -> Result<(), ValidationError> {
    let value = value.trim();

    if !(1..=50).contains(&value.chars().count()) {
        return Err(ValidationError::new("invalid_category"));
    }

    Ok(())
}

fn default_currency() -> String {
    "NGN".to_owned()
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateTransactionRequest {
    #[validate(custom(function = "validate_description"))]
    pub description: String,

    #[validate(custom(function = "validate_amount_string"))]
    pub amount: String,

    #[validate(custom(function = "validate_category"))]
    pub category: String,

    #[serde(default = "default_currency")]
    #[validate(custom(function = "validate_currency"))]
    pub currency: String,

    #[serde(rename = "type")]
    pub transaction_type: TransactionType,

    pub occurred_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateTransactionRequest {
    #[validate(custom(function = "validate_description"))]
    pub description: Option<String>,

    #[validate(custom(function = "validate_amount_string"))]
    pub amount: Option<String>,

    #[validate(custom(function = "validate_category"))]
    pub category: Option<String>,

    #[validate(custom(function = "validate_currency"))]
    pub currency: Option<String>,

    #[serde(rename = "type")]
    pub transaction_type: Option<TransactionType>,

    pub occurred_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct TransactionResponse {
    pub id: Uuid,

    #[serde(rename = "type")]
    pub transaction_type: TransactionType,

    pub description: String,
    pub amount: String,
    pub currency: String,
    pub category: String,
    pub occurred_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct TransactionListResponse {
    pub items: Vec<TransactionResponse>,
    pub page: u32,
    pub limit: u32,
}

#[derive(Debug, sqlx::FromRow)]
pub struct TransactionRecord {
    pub id: Uuid,
    pub user_id: Uuid,
    pub transaction_type: String,
    pub description: String,
    pub amount: BigDecimal,
    pub currency: String,
    pub category: String,
    pub occurred_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

impl TryFrom<TransactionRecord> for TransactionResponse {
    type Error = crate::error::AppError;

    fn try_from(record: TransactionRecord) -> Result<Self, Self::Error> {
        let transaction_type = match record.transaction_type.as_str() {
            "INCOME" => TransactionType::Income,
            "EXPENSE" => TransactionType::Expense,
            _ => return Err(crate::error::AppError::Internal),
        };

        Ok(Self {
            id: record.id,
            transaction_type,
            description: record.description,
            amount: record.amount.to_string(),
            currency: record.currency,
            category: record.category,
            occurred_at: record.occurred_at,
            created_at: record.created_at,
        })
    }
}
