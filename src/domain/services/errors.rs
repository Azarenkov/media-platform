use thiserror::Error;

use crate::domain::repositories::errors::UserRepositoryError;

// Implementation UserService errors from other layers
#[derive(Debug, Error)]
pub enum UserServiceError {
    #[error("User not found with email: `{0}`")]
    NotFound(String),

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
            UserRepositoryError::NotFound(email) => Self::NotFound(email),
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
    #[error("InvalidCredentials")]
    InvalidCredentials,

    #[error("User already exist with email: `{0}`")]
    AlreadyRegistered(String),

    #[error("Internal error")]
    Internal,
}

impl From<UserRepositoryError> for AuthServiceError {
    fn from(value: UserRepositoryError) -> Self {
        match value {
            UserRepositoryError::AlreadyExist(email) => {
                eprintln!("{}", email);
                Self::AlreadyRegistered(email)
            }
            UserRepositoryError::NotFound(_email) => Self::InvalidCredentials,
            UserRepositoryError::Database(e) => {
                eprintln!("{}", e);
                Self::Internal
            }
        }
    }
}
