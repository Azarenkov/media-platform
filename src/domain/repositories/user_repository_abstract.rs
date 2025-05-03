use crate::domain::entities::user::User;

use super::errors::UserRepositoryError;

pub trait UserRepositoryAbstract {
    async fn save(&self, user: &User) -> Result<(), UserRepositoryError>;
    async fn find_id_by_email(&self, email: &str) -> Result<u128, UserRepositoryError>;
    async fn find_password_by_email(&self, email: &str) -> Result<String, UserRepositoryError>;
}
