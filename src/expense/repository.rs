use sqlx::PgPool;
use uuid::Uuid;

pub struct ExpenseRepository {
    pool: PgPool,
}

impl ExpenseRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(
        &self,
        user_id: Uuid,
        description: String,
        amount: f64,
        category: String,
    ) -> Result<ExpenseRecord, sqlx::Error> {
        sqlx::query_as!(
            ExpenseRecord,
            r#"
            INSERT INTO expenses
                (id, user_id, description, amount, category)
            VALUES
                ($1, $2, $3, $4, $5)
            RETURNING id, description, amount, category
            "#,
            Uuid::new_v4(),
            user_id,
            description,
            amount,
            category
        )
        .fetch_one(&self.pool)
        .await
    }

    pub async fn find_all_by_user(&self, user_id: Uuid) -> Result<Vec<ExpenseRecord>, sqlx::Error> {
        sqlx::query_as!(
            ExpenseRecord,
            r#"
            SELECT id, description, amount, category
            FROM expenses
            WHERE user_id = $1
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await
    }

    pub async fn delete(&self, id: Uuid, user_id: Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            DELETE FROM expenses
            WHERE id = $1
            AND user_id = $2
            "#,
            id,
            user_id
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn get_total(&self, user_id: Uuid) -> Result<f64, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            SELECT COALESCE(SUM(amount), 0.0) AS total
            FROM expenses
            WHERE user_id = $1
            "#,
            user_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result.total.unwrap_or(0.0))
    }

    pub async fn get_category_totals(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<CategoryTotalRecord>, sqlx::Error> {
        sqlx::query_as!(
            CategoryTotalRecord,
            r#"
            SELECT category, SUM(amount) AS total
            FROM expenses
            WHERE user_id = $1
            GROUP BY category
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await
    }
}

#[derive(Debug)]
pub struct ExpenseRecord {
    pub id: Uuid,
    pub description: String,
    pub amount: f64,
    pub category: String,
}

#[derive(Debug)]
pub struct CategoryTotalRecord {
    pub category: String,
    pub total: Option<f64>,
}
