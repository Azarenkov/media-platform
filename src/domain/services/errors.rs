use thiserror::Error;

use crate::domain::repositories::errors::UserRepositoryError;

// Implementation UserService errors from other layers
#[derive(Debug, Error)]
pub enum UserServiceError {
    #[error("User not found")]
    NotFound,

    #[error("Internal error")]
    Internal,

    #[error("Default error")]
    Default,
}

impl From<UserRepositoryError> for UserServiceError {
    fn from(value: UserRepositoryError) -> Self {
        match value {
            UserRepositoryError::Database(e) => {
                eprintln!("{}", e);
                Self::Internal
            }
            UserRepositoryError::NotFound => Self::NotFound,
            UserRepositoryError::AlreadyExist(e) => {
                eprintln!("{}", e);
                Self::Default
            }
        }
    }
}

// Implementation AuthService errors from other layers
#[derive(Debug, Error)]
pub enum AuthServiceError {
    #[error("User not found")]
    NotFound,

    #[error("User already exist")]
    AlreadyRegistered,

    #[error("Internal error")]
    Internal,

    #[error("Default error")]
    Default,
}

impl From<UserRepositoryError> for AuthServiceError {
    fn from(value: UserRepositoryError) -> Self {
        match value {
            UserRepositoryError::AlreadyExist(e) => {
                eprintln!("{}", e);
                Self::AlreadyRegistered
            }
            UserRepositoryError::NotFound => Self::NotFound,
            UserRepositoryError::Database(e) => {
                eprintln!("{}", e);
                Self::Internal
            }
        }
    }
}
