use crate::domain::entities::user::User;

use super::errors::UserRepositoryError;

pub trait UserRepositoryAbstract {
    async fn save(&self, user: &User) -> Result<(), UserRepositoryError>;
}
