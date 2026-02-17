use crate::errors::AppError;
use crate::models::User;
use sqlx::error::ErrorKind;
use sqlx::PgPool;

pub struct UserRepository {
    pub pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_user(
        &self,
        username: &str,
        email: &str,
        password_hash: &str,
    ) -> Result<User, AppError> {
        let user = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (username, email, password)
            VALUES ($1, $2, $3)
            RETURNING id, username, email, password, created_at, updated_at, role::text AS role, attempts
            "#,
        )
        .bind(username)
        .bind(email)
        .bind(password_hash)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            if let sqlx::Error::Database(db) = &e {
                if db.kind() == ErrorKind::UniqueViolation {
                    return AppError::DuplicateKeyError(
                        "User with this email or username already exists".to_string(),
                    );
                }
            }
            AppError::DatabaseError(e)
        })?;
        Ok(user)
    }

    pub async fn find_user(&self, email: &str, username: &str) -> Result<Option<User>, AppError> {
        let user =
            sqlx::query_as::<_, User>(
                "SELECT id, username, email, password, created_at, updated_at, role::text AS role, attempts FROM users WHERE email = $1 OR username = $2"
            )
                .bind(email)
                .bind(username)
                .fetch_optional(&self.pool)
                .await
                .map_err(|e| AppError::DatabaseError(e));
        user
    }

    pub async fn update_attempts(&self, email: &str) -> Result<(), AppError> {
        sqlx::query("UPDATE users SET attempts = attempts + 1 WHERE email = $1")
            .bind(email)
            .execute(&self.pool)
            .await
            .map_err(AppError::DatabaseError)?;
        Ok(())
    }

    pub async fn reset_attempts(&self, email: &str) -> Result<(), AppError> {
        sqlx::query("UPDATE users SET attempts = 0 WHERE email = $1")
            .bind(email)
            .execute(&self.pool)
            .await
            .map_err(AppError::DatabaseError)?;
        Ok(())
    }
}
