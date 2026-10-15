use bigdecimal::BigDecimal;
use chrono::{NaiveDate, Utc};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

use crate::error::AppError;

#[derive(Clone)]
pub struct BudgetRepository {
    pool: PgPool,
}

#[derive(Debug, FromRow)]
pub struct BudgetRow {
    pub id: Uuid,
}

#[derive(Debug, FromRow)]
pub struct BudgetWithSpendingRow {
    pub id: Uuid,
    pub category: String,
    pub amount_limit: BigDecimal,
    pub currency: String,
    pub month: NaiveDate,
    pub created_at: chrono::DateTime<Utc>,
    pub spent: BigDecimal,
}

impl BudgetRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(
        &self,
        user_id: Uuid,
        category: &str,
        amount_limit: &BigDecimal,
        currency: &str,
        month: NaiveDate,
    ) -> Result<BudgetRow, AppError> {
        let id = Uuid::new_v4();

        sqlx::query_as::<_, BudgetRow>(
            r#"
            INSERT INTO budgets
                (id, user_id, category, amount_limit, currency, month)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING
                id, user_id, category, amount_limit,
                currency, month, created_at
            "#,
        )
        .bind(id)
        .bind(user_id)
        .bind(category)
        .bind(amount_limit)
        .bind(currency)
        .bind(month)
        .fetch_one(&self.pool)
        .await
        .map_err(map_write_error)
    }

    pub async fn find_all_by_user(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<BudgetWithSpendingRow>, AppError> {
        sqlx::query_as::<_, BudgetWithSpendingRow>(
            r#"
            SELECT
                b.id,
                b.category,
                b.amount_limit,
                b.currency,
                b.month,
                b.created_at,
                COALESCE(SUM(t.amount), 0::NUMERIC) AS spent
            FROM budgets b
            LEFT JOIN transactions t
                ON t.user_id = b.user_id
               AND t.type = 'EXPENSE'
               AND t.category = b.category
               AND t.currency = b.currency
               AND t.occurred_at >= b.month::timestamp AT TIME ZONE 'UTC'
               AND t.occurred_at < (b.month + INTERVAL '1 month') AT TIME ZONE 'UTC'
            WHERE b.user_id = $1
            GROUP BY
                b.id, b.category, b.amount_limit, b.currency,
                b.month, b.created_at
            ORDER BY b.month DESC, b.category ASC, b.id ASC
            "#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
        .map_err(map_read_error)
    }

    pub async fn find_by_id_and_user(
        &self,
        id: Uuid,
        user_id: Uuid,
    ) -> Result<Option<BudgetWithSpendingRow>, AppError> {
        sqlx::query_as::<_, BudgetWithSpendingRow>(
            r#"
            SELECT
                b.id,
                b.category,
                b.amount_limit,
                b.currency,
                b.month,
                b.created_at,
                COALESCE(SUM(t.amount), 0::NUMERIC) AS spent
            FROM budgets b
            LEFT JOIN transactions t
                ON t.user_id = b.user_id
               AND t.type = 'EXPENSE'
               AND t.category = b.category
               AND t.currency = b.currency
               AND t.occurred_at >= b.month::timestamp AT TIME ZONE 'UTC'
               AND t.occurred_at < (b.month + INTERVAL '1 month') AT TIME ZONE 'UTC'
            WHERE b.id = $1 AND b.user_id = $2
            GROUP BY
                b.id, b.category, b.amount_limit, b.currency,
                b.month, b.created_at
            "#,
        )
        .bind(id)
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(map_read_error)
    }

    pub async fn update_by_id_and_user(
        &self,
        id: Uuid,
        user_id: Uuid,
        category: &str,
        amount_limit: &BigDecimal,
        currency: &str,
        month: NaiveDate,
    ) -> Result<Option<BudgetRow>, AppError> {
        sqlx::query_as::<_, BudgetRow>(
            r#"
            UPDATE budgets
            SET category = $3,
                amount_limit = $4,
                currency = $5,
                month = $6
            WHERE id = $1 AND user_id = $2
            RETURNING
                id, user_id, category, amount_limit,
                currency, month, created_at
            "#,
        )
        .bind(id)
        .bind(user_id)
        .bind(category)
        .bind(amount_limit)
        .bind(currency)
        .bind(month)
        .fetch_optional(&self.pool)
        .await
        .map_err(map_write_error)
    }

    pub async fn delete_by_id_and_user(&self, id: Uuid, user_id: Uuid) -> Result<bool, AppError> {
        let result = sqlx::query("DELETE FROM budgets WHERE id = $1 AND user_id = $2")
            .bind(id)
            .bind(user_id)
            .execute(&self.pool)
            .await
            .map_err(map_write_error)?;

        Ok(result.rows_affected() == 1)
    }
}

fn map_read_error(error: sqlx::Error) -> AppError {
    tracing::error!(error = %error, "Budget database query failed");
    AppError::Database
}

fn map_write_error(error: sqlx::Error) -> AppError {
    if let sqlx::Error::Database(db_error) = &error {
        if db_error.is_unique_violation() {
            return AppError::Conflict;
        }
    }

    tracing::error!(error = %error, "Budget database write failed");
    AppError::Database
}
