use std::sync::Arc;

use crate::domain::{
    entities::user::User, repositories::user_repository_abstract::UserRepositoryAbstract,
};

use super::errors::AuthServiceError;

pub struct AuthService<UserRepo>
where
    UserRepo: UserRepositoryAbstract,
{
    user_repository: Arc<UserRepo>,
}

impl<UserRepo> AuthService<UserRepo>
where
    UserRepo: UserRepositoryAbstract,
{
    pub fn new(user_repository: Arc<UserRepo>) -> Self {
        Self { user_repository }
    }

    pub async fn register_user(&self, user: &User) -> Result<(), AuthServiceError> {
        self.user_repository.save(user).await?;
        Ok(())
    }
}
