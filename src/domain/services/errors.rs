use thiserror::Error;

use crate::domain::repositories::errors::UserRepositoryError;

#[derive(Debug, Error)]
pub enum UserServiceError {
    #[error("Cannot create user: email `{0}` is already registered")]
    EmailAlreadyRegistered(String),

    #[error("Internal error: `{0}`")]
    Internal(String),
}

impl From<UserRepositoryError> for UserServiceError {
    fn from(value: UserRepositoryError) -> Self {
        match value {
            UserRepositoryError::AlreadyExist(email) => Self::EmailAlreadyRegistered(email),
            UserRepositoryError::Database(err) => Self::Internal(err),
        }
    }
}
