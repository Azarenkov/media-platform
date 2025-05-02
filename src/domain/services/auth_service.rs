use std::sync::Arc;

use crate::{
    domain::{
        entities::user::User, hasher_abstract::HasherAbstract,
        repositories::user_repository_abstract::UserRepositoryAbstract,
    },
    infrastructure::hasher,
};

use super::errors::AuthServiceError;

pub struct AuthService<UserRepo, Hasher>
where
    UserRepo: UserRepositoryAbstract,
    Hasher: HasherAbstract,
{
    user_repository: Arc<UserRepo>,
    hasher: Hasher,
}

impl<UserRepo, Hasher> AuthService<UserRepo, Hasher>
where
    UserRepo: UserRepositoryAbstract,
    Hasher: HasherAbstract,
{
    pub fn new(user_repository: Arc<UserRepo>, hasher: Hasher) -> Self {
        Self {
            user_repository,
            hasher,
        }
    }

    pub async fn register_user(&self, user: &mut User) -> Result<(), AuthServiceError> {
        let password_hash = self
            .hasher
            .hash(user.password.to_owned())
            .await
            .map_err(|_| AuthServiceError::Internal)?;

        user.password = password_hash;
        self.user_repository.save(user).await?;
        Ok(())
    }
}
