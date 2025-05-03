use thiserror::Error;

#[derive(Error, Debug)]
pub enum UserRepositoryError {
    #[error("User already exist with email: `{0}`")]
    AlreadyExist(String),

    #[error("User not found wtih email: `{0}`")]
    NotFound(String),

    #[error("Database error: {0}")]
    Database(String),
}
