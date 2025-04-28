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
}
