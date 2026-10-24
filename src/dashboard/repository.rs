use crate::error::AppError;
use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

#[derive(Clone)]
pub struct DashboardRepository {
    pool: PgPool,
}

#[derive(FromRow)]
pub struct SummaryRow {
    pub income: BigDecimal,
    pub expenses: BigDecimal,
}

#[derive(FromRow)]
pub struct CategoryTotalRow {
    pub category: String,
    pub total: BigDecimal,
}

impl DashboardRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn summary(
        &self,
        user_id: Uuid,
        currency: &str,
        from: DateTime<Utc>,
        to_exclusive: DateTime<Utc>,
    ) -> Result<SummaryRow, AppError> {
        sqlx::query_as::<_, SummaryRow>(
            r#"
            SELECT
                COALESCE(
                    SUM(amount) FILTER (WHERE type = 'INCOME'),
                    0::NUMERIC
                ) AS income,
                COALESCE(
                    SUM(amount) FILTER (WHERE type = 'EXPENSE'),
                    0::NUMERIC
                ) AS expenses
            FROM transactions
            WHERE user_id = $1
              AND currency = $2
              AND occurred_at >= $3
              AND occurred_at < $4
            "#,
        )
        .bind(user_id)
        .bind(currency)
        .bind(from)
        .bind(to_exclusive)
        .fetch_one(&self.pool)
        .await
        .map_err(|error| {
            tracing::error!(error = %error, "Dashboard summary query failed");
            AppError::Database
        })
    }

    pub async fn category_totals(
        &self,
        user_id: Uuid,
        currency: &str,
        from: DateTime<Utc>,
        to_exclusive: DateTime<Utc>,
    ) -> Result<Vec<CategoryTotalRow>, AppError> {
        sqlx::query_as::<_, CategoryTotalRow>(
            r#"
            SELECT
                category,
                SUM(amount) AS total
            FROM transactions
            WHERE user_id = $1
              AND type = 'EXPENSE'
              AND currency = $2
              AND occurred_at >= $3
              AND occurred_at < $4
            GROUP BY category
            ORDER BY SUM(amount) DESC, category ASC
            "#,
        )
        .bind(user_id)
        .bind(currency)
        .bind(from)
        .bind(to_exclusive)
        .fetch_all(&self.pool)
        .await
        .map_err(|error| {
            tracing::error!(error = %error, "Category report query failed");
            AppError::Database
        })
    }
}
