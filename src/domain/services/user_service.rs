use crate::domain::{
    entities::user::User, repositories::user_repository_abstract::UserRepositoryAbstract,
};

use super::errors::UserServiceError;

pub struct UserService<T>
where
    T: UserRepositoryAbstract,
{
    user_repository: T,
}

impl<T> UserService<T>
where
    T: UserRepositoryAbstract,
{
    pub fn new(user_repository: T) -> Self {
        Self { user_repository }
    }

    pub async fn create(&self, user: &User) -> Result<(), UserServiceError> {
        self.user_repository.save(user).await?;
        Ok(())
    }

    pub async fn get_by_email(&self, email: &str) -> Result<User, UserServiceError> {
        let user = self.user_repository.find_by_email(email).await?;
        Ok(user)
    }
}
