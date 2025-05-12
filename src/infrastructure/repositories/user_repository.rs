use sqlx::{PgPool, types::Uuid};

use crate::domain::{
    entities::user::User,
    repositories::{errors::UserRepositoryError, user_repository_abstract::UserRepositoryAbstract},
};

impl From<sqlx::Error> for UserRepositoryError {
    fn from(value: sqlx::Error) -> Self {
        Self::Database(value.to_string())
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

        let result = sqlx::query(query)
            .bind(&user.email)
            .bind(&user.username)
            .bind(&user.password)
            .execute(&self.pool)
            .await;

        match result {
            Ok(_) => Ok(()),
            Err(err) => match err {
                sqlx::Error::Database(e) => {
                    if e.is_unique_violation() {
                        return Err(UserRepositoryError::AlreadyExist(user.email.to_owned()));
                    }
                    Err(UserRepositoryError::Database(e.to_string()))
                }

                _ => Err(err.into()),
            },
        }
    }

    async fn find_id_by_email(&self, email: &str) -> Result<u128, UserRepositoryError> {
        let result = sqlx::query_scalar!("SELECT id FROM users WHERE email = $1", email)
            .fetch_optional(&self.pool)
            .await?;

        match result {
            Some(id) => Ok(id.as_u128()),
            None => Err(UserRepositoryError::NotFound(email.to_owned())),
        }
    }

    async fn find_password_by_email(&self, email: &str) -> Result<String, UserRepositoryError> {
        let result = sqlx::query_scalar!("SELECT password FROM users WHERE email = $1", email)
            .fetch_optional(&self.pool)
            .await?;

        match result {
            Some(password) => Ok(password),
            None => Err(UserRepositoryError::NotFound(email.to_owned())),
        }
    }

    async fn delete_user_by_id(&self, id: u128) -> Result<(), UserRepositoryError> {
        println!("{}", id);
        let result = sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(Uuid::from_u128(id))
            .execute(&self.pool)
            .await;

        match result {
            Ok(value) => {
                if value.rows_affected() == 0 {
                    return Err(UserRepositoryError::NotFound(id.to_string()));
                }
            }
            Err(e) => return Err(UserRepositoryError::Database(e.to_string())),
        }
        Ok(())
    }
}
