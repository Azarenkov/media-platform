use std::sync::Arc;

use crate::domain::repositories::user_repository_abstract::UserRepositoryAbstract;

use super::errors::AccountServiceError;

pub struct AccountService<UserRepo>
where
    UserRepo: UserRepositoryAbstract,
{
    user_repository: Arc<UserRepo>,
}

impl<UserRepo> AccountService<UserRepo>
where
    UserRepo: UserRepositoryAbstract,
{
    pub fn new(user_repository: Arc<UserRepo>) -> Self {
        Self { user_repository }
    }

    pub async fn delete_account(&self, id: u128) -> Result<(), AccountServiceError> {
        self.user_repository.delete_user_by_id(id).await?;
        Ok(())
    }

    // pub async fn get_by_email(&self, email: &str) -> Result<User, UserServiceError> {
    //     let user = self.user_repository.find_by_email(email).await?;
    //     Ok(user)
    // }
}
