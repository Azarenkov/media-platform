use sqlx::PgPool;

use crate::domain::{
    entities::user::User,
    repositories::{errors::UserRepositoryError, user_repository_abstract::UserRepositoryAbstract},
};

impl From<sqlx::Error> for UserRepositoryError {
    fn from(value: sqlx::Error) -> Self {
        match value {
            sqlx::Error::Database(e) => {
                if e.is_unique_violation() {
                    return Self::AlreadyExist(e.to_string());
                }
                Self::Database(e.to_string())
            }
            sqlx::Error::RowNotFound => Self::NotFound,
            e => Self::Database(e.to_string()),
        }
    }
}

pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl UserRepositoryAbstract for UserRepository {
    async fn save(&self, user: &User) -> Result<(), UserRepositoryError> {
        let query = "INSERT INTO users (email, username, password) VALUES ($1, $2, $3)";

        sqlx::query(query)
            .bind(&user.email)
            .bind(&user.username)
            .bind(&user.password)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn find_id_by_email(&self, email: &str) -> Result<u128, UserRepositoryError> {
        let result = sqlx::query_scalar!("SELECT id FROM users WHERE email = $1", email)
            .fetch_optional(&self.pool)
            .await?;

        match result {
            Some(id) => Ok(id.as_u128()),
            None => Err(UserRepositoryError::NotFound),
        }
    }

    async fn find_password_by_email(&self, email: &str) -> Result<String, UserRepositoryError> {
        let result = sqlx::query_scalar!("SELECT password FROM users WHERE email = $1", email)
            .fetch_optional(&self.pool)
            .await?;

        match result {
            Some(password) => Ok(password),
            None => Err(UserRepositoryError::NotFound),
        }
    }
}
