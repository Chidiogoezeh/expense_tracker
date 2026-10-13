use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::transaction::model::TransactionRecord;

pub struct TransactionRepository {
    pool: PgPool,
}

pub struct TransactionFilters {
    pub transaction_type: Option<String>,
    pub category: Option<String>,
    pub from: Option<DateTime<Utc>>,
    pub until: Option<DateTime<Utc>>,
    pub limit: i64,
    pub offset: i64,
}

impl TransactionRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(
        &self,
        user_id: Uuid,
        transaction_type: &str,
        description: &str,
        amount: &BigDecimal,
        currency: &str,
        category: &str,
        occurred_at: DateTime<Utc>,
    ) -> Result<TransactionRecord, sqlx::Error> {
        sqlx::query_as::<_, TransactionRecord>(
            r#"
            INSERT INTO transactions
                (id, user_id, "type", description, amount,
                 currency, category, occurred_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING id, user_id, "type" AS transaction_type,
                      description, amount, currency, category,
                      occurred_at, created_at
            "#,
        )
        .bind(Uuid::new_v4())
        .bind(user_id)
        .bind(transaction_type)
        .bind(description)
        .bind(amount)
        .bind(currency)
        .bind(category)
        .bind(occurred_at)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn find_all_by_user(
        &self,
        user_id: Uuid,
        filters: &TransactionFilters,
    ) -> Result<Vec<TransactionRecord>, sqlx::Error> {
        sqlx::query_as::<_, TransactionRecord>(
            r#"
            SELECT id, user_id, "type" AS transaction_type,
                   description, amount, currency, category,
                   occurred_at, created_at
            FROM transactions
            WHERE user_id = $1
              AND ($2::text IS NULL OR "type" = $2)
              AND ($3::text IS NULL OR category = $3)
              AND ($4::timestamptz IS NULL OR occurred_at >= $4)
              AND ($5::timestamptz IS NULL OR occurred_at < $5)
            ORDER BY occurred_at DESC, id DESC
            LIMIT $6 OFFSET $7
            "#,
        )
        .bind(user_id)
        .bind(&filters.transaction_type)
        .bind(&filters.category)
        .bind(filters.from)
        .bind(filters.until)
        .bind(filters.limit)
        .bind(filters.offset)
        .fetch_all(&self.pool)
        .await
    }

    pub async fn find_by_id_and_user(
        &self,
        id: Uuid,
        user_id: Uuid,
    ) -> Result<Option<TransactionRecord>, sqlx::Error> {
        sqlx::query_as::<_, TransactionRecord>(
            r#"
            SELECT id, user_id, "type" AS transaction_type,
                   description, amount, currency, category,
                   occurred_at, created_at
            FROM transactions
            WHERE id = $1 AND user_id = $2
            "#,
        )
        .bind(id)
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn update_by_id_and_user(
        &self,
        id: Uuid,
        user_id: Uuid,
        transaction_type: Option<&str>,
        description: Option<&str>,
        amount: Option<&BigDecimal>,
        currency: Option<&str>,
        category: Option<&str>,
        occurred_at: Option<DateTime<Utc>>,
    ) -> Result<Option<TransactionRecord>, sqlx::Error> {
        sqlx::query_as::<_, TransactionRecord>(
            r#"
            UPDATE transactions
            SET "type" = COALESCE($3, "type"),
                description = COALESCE($4, description),
                amount = COALESCE($5, amount),
                currency = COALESCE($6, currency),
                category = COALESCE($7, category),
                occurred_at = COALESCE($8, occurred_at)
            WHERE id = $1 AND user_id = $2
            RETURNING id, user_id, "type" AS transaction_type,
                      description, amount, currency, category,
                      occurred_at, created_at
            "#,
        )
        .bind(id)
        .bind(user_id)
        .bind(transaction_type)
        .bind(description)
        .bind(amount)
        .bind(currency)
        .bind(category)
        .bind(occurred_at)
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn delete_by_id_and_user(
        &self,
        id: Uuid,
        user_id: Uuid,
    ) -> Result<bool, sqlx::Error> {
        let result = sqlx::query(
            r#"
            DELETE FROM transactions
            WHERE id = $1 AND user_id = $2
            "#,
        )
        .bind(id)
        .bind(user_id)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }
}
