use argon2::{
    Argon2,
    password_hash::{PasswordHasher, PasswordVerifier, phc::PasswordHash},
};

use std::sync::OnceLock;

use jsonwebtoken::{Algorithm, EncodingKey, Header, encode, get_current_timestamp};

use uuid::Uuid;

use crate::{
    auth::{Claims, repository::UserRepository},
    error::AppError,
};

pub struct AuthService {
    repository: UserRepository,
    jwt_secret: String,
}

impl AuthService {
    pub fn new(repository: UserRepository, jwt_secret: String) -> Self {
        Self {
            repository,
            jwt_secret,
        }
    }

    pub async fn register(
        &self,
        email: &str,
        password: &str,
    ) -> Result<RegistrationResult, AppError> {
        let email = email.trim().to_lowercase();

        let existing = self
            .repository
            .find_by_email(&email)
            .await
            .map_err(|error| {
                tracing::error!(
                    error = %error,
                    operation = "register_find_existing_user",
                    "Database operation failed"
                );
                AppError::Database
            })?;

        if existing.is_some() {
            return Err(AppError::Conflict);
        }

        let password_hash = hash_password(password)?;

        let user = self
            .repository
            .create(Uuid::new_v4(), &email, &password_hash)
            .await
            .map_err(|error| {
                if let sqlx::Error::Database(db_error) = &error {
                    if db_error.is_unique_violation() {
                        return AppError::Conflict;
                    }
                }

                tracing::error!(
                    error = %error,
                    operation = "register_create_user",
                    "Database operation failed"
                );

                AppError::Database
            })?;
        tracing::info!(
            user_id = %user.id,
            "User registered"
        );

        Ok(RegistrationResult {
            id: user.id,
            email: user.email,
        })
    }

    pub async fn login(&self, email: &str, password: &str) -> Result<String, AppError> {
        let email = email.trim().to_lowercase();

        let user = self
            .repository
            .find_by_email(&email)
            .await
            .map_err(|error| {
                tracing::error!(
                    error = %error,
                    operation = "login_find_user",
                    "Database operation failed"
                );
                AppError::Database
            })?;

        let Some(user) = user else {
            let dummy_hash = dummy_password_hash()?;
            let _ = verify_password(password, dummy_hash)?;
            return Err(AppError::Unauthorized);
        };

        let valid = verify_password(password, &user.password_hash)?;

        if !valid {
            return Err(AppError::Unauthorized);
        }

        let token = create_token(user.id, &self.jwt_secret)?;

        tracing::info!(
            user_id = %user.id,
            "User authenticated"
        );

        Ok(token)
    }

    pub async fn profile(&self, user_id: Uuid) -> Result<ProfileResult, AppError> {
        let user = self.repository.find_by_id(user_id).await.map_err(|error| {
            tracing::error!(
                error = %error,
                operation = "profile_find_user",
                "Database operation failed"
            );
            AppError::Database
        })?;

        let Some(user) = user else {
            return Err(AppError::NotFound);
        };

        Ok(ProfileResult {
            id: user.id,
            email: user.email,
        })
    }
}

pub struct RegistrationResult {
    pub id: Uuid,
    pub email: String,
}

pub struct ProfileResult {
    pub id: Uuid,
    pub email: String,
}

fn hash_password(password: &str) -> Result<String, AppError> {
    Argon2::default()
        .hash_password(password.as_bytes())
        .map(|hash| hash.to_string())
        .map_err(|_| AppError::Internal)
}

fn verify_password(password: &str, stored_hash: &str) -> Result<bool, AppError> {
    let parsed_hash = PasswordHash::new(stored_hash).map_err(|_| AppError::Internal)?;

    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

fn dummy_password_hash() -> Result<&'static str, AppError> {
    static DUMMY_HASH: OnceLock<Result<String, ()>> = OnceLock::new();

    DUMMY_HASH
        .get_or_init(|| hash_password("dummy-password-not-used").map_err(|_| ()))
        .as_ref()
        .map(String::as_str)
        .map_err(|_| AppError::Internal)
}

fn create_token(user_id: Uuid, jwt_secret: &str) -> Result<String, AppError> {
    const TOKEN_LIFETIME_SECONDS: u64 = 3000;

    let claims = Claims {
        sub: user_id.to_string(),
        exp: get_current_timestamp() + TOKEN_LIFETIME_SECONDS,
    };

    encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    )
    .map_err(|_| AppError::Internal)
}
