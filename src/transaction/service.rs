use crate::{
    error::AppError,
    transaction::{
        model::{
            CreateTransactionRequest, TransactionRecord, TransactionResponse,
            UpdateTransactionRequest,
        },
        repository::{
            NewTransaction, TransactionChanges, TransactionFilters, TransactionRepository,
        },
    },
};
use bigdecimal::BigDecimal;
use chrono::Utc;
use uuid::Uuid;

pub struct TransactionService {
    repository: TransactionRepository,
}

impl TransactionService {
    pub fn new(repository: TransactionRepository) -> Self {
        Self { repository }
    }

    fn database_error(error: sqlx::Error, user_id: Uuid, operation: &'static str) -> AppError {
        tracing::error!(
            user_id = %user_id,
            operation,
            error = %error,
            "Transaction database operation failed"
        );
        AppError::Database
    }

    fn to_response(record: TransactionRecord) -> Result<TransactionResponse, AppError> {
        record.try_into()
    }

    pub async fn create(
        &self,
        user_id: Uuid,
        input: CreateTransactionRequest,
    ) -> Result<TransactionResponse, AppError> {
        let amount = input
            .amount
            .parse::<BigDecimal>()
            .map_err(|_| AppError::BadRequest)?;

        let occurred_at = input.occurred_at.unwrap_or_else(Utc::now);

        let record = self
            .repository
            .create(NewTransaction {
                user_id,
                transaction_type: input.transaction_type.as_str(),
                description: input.description.trim(),
                amount: &amount,
                currency: &input.currency,
                category: input.category.trim(),
                occurred_at,
            })
            .await
            .map_err(|error| Self::database_error(error, user_id, "create"))?;

        Self::to_response(record)
    }

    pub async fn get(&self, id: Uuid, user_id: Uuid) -> Result<TransactionResponse, AppError> {
        let record = self
            .repository
            .find_by_id_and_user(id, user_id)
            .await
            .map_err(|error| Self::database_error(error, user_id, "get"))?
            .ok_or(AppError::NotFound)?;

        Self::to_response(record)
    }

    pub async fn list(
        &self,
        user_id: Uuid,
        filters: &TransactionFilters,
    ) -> Result<Vec<TransactionResponse>, AppError> {
        let records = self
            .repository
            .find_all_by_user(user_id, filters)
            .await
            .map_err(|error| Self::database_error(error, user_id, "list"))?;

        records.into_iter().map(Self::to_response).collect()
    }

    pub async fn update(
        &self,
        id: Uuid,
        user_id: Uuid,
        input: UpdateTransactionRequest,
    ) -> Result<TransactionResponse, AppError> {
        if input.description.is_none()
            && input.amount.is_none()
            && input.category.is_none()
            && input.currency.is_none()
            && input.transaction_type.is_none()
            && input.occurred_at.is_none()
        {
            return Err(AppError::BadRequest);
        }

        let amount = input
            .amount
            .as_deref()
            .map(str::parse::<BigDecimal>)
            .transpose()
            .map_err(|_| AppError::BadRequest)?;

        let transaction_type = input.transaction_type.map(|value| value.as_str());

        let description = input.description.as_deref().map(str::trim);

        let category = input.category.as_deref().map(str::trim);

        let record = self
            .repository
            .update_by_id_and_user(
                id,
                user_id,
                TransactionChanges {
                    transaction_type,
                    description,
                    amount: amount.as_ref(),
                    currency: input.currency.as_deref(),
                    category,
                    occurred_at: input.occurred_at,
                },
            )
            .await
            .map_err(|error| Self::database_error(error, user_id, "update"))?
            .ok_or(AppError::NotFound)?;

        Self::to_response(record)
    }

    pub async fn delete(&self, id: Uuid, user_id: Uuid) -> Result<(), AppError> {
        let deleted = self
            .repository
            .delete_by_id_and_user(id, user_id)
            .await
            .map_err(|error| Self::database_error(error, user_id, "delete"))?;

        if !deleted {
            return Err(AppError::NotFound);
        }

        Ok(())
    }
}
