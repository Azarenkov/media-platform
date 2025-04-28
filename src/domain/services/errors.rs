use thiserror::Error;

use crate::domain::repositories::errors::UserRepositoryError;

#[derive(Debug, Error)]
pub enum UserServiceError {
    #[error("Email already registered")]
    EmailAlreadyRegistered,

    #[error("Internal error")]
    Internal,
}

impl From<UserRepositoryError> for UserServiceError {
    fn from(value: UserRepositoryError) -> Self {
        match value {
            UserRepositoryError::AlreadyExist(_) => Self::EmailAlreadyRegistered,
            UserRepositoryError::Database(_) => Self::Internal,
        }
    }
}
