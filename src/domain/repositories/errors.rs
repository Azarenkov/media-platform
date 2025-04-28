use thiserror::Error;

#[derive(Error, Debug)]
pub enum UserRepositoryError {
    #[error("User already exist with email: `{0}`")]
    AlreadyExist(String),

    #[error("Database error: {0}")]
    Database(String),
}
