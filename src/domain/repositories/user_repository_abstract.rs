use crate::domain::entities::user::User;

use super::errors::UserRepositoryError;

pub trait UserRepositoryAbstract {
    async fn save(&self, user: &User) -> Result<(), UserRepositoryError>;
    async fn find_email_and_password(&self, user: &User) -> Result<(), UserRepositoryError>;
    // async fn validate_email_and_password(&self, user: &User) -> Result<(), UserRepositoryError>;
}
