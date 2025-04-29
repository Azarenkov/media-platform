use crate::domain::entities::user::User;

use super::errors::UserRepositoryError;

pub trait UserRepositoryAbstract {
    async fn save(&self, user: &User) -> Result<(), UserRepositoryError>;
    async fn find_by_email(&self, email: &str) -> Result<User, UserRepositoryError>;
}
