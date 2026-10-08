use sqlx::PgPool;
use uuid::Uuid;

pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<UserRecord>, sqlx::Error> {
        sqlx::query_as!(
            UserRecord,
            r#"
            SELECT id, email, password_hash
            FROM users
            WHERE email = $1
            "#,
            email
        )
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<UserRecord>, sqlx::Error> {
        sqlx::query_as!(
            UserRecord,
            r#"
            SELECT id, email, password_hash
            FROM users
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn create(
        &self,
        id: Uuid,
        email: &str,
        password_hash: &str,
    ) -> Result<UserRecord, sqlx::Error> {
        sqlx::query_as!(
            UserRecord,
            r#"
            INSERT INTO users (id, email, password_hash)
            VALUES ($1, $2, $3)
            RETURNING id, email, password_hash
            "#,
            id,
            email,
            password_hash
        )
        .fetch_one(&self.pool)
        .await
    }
}

#[derive(Debug)]
pub struct UserRecord {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
}
