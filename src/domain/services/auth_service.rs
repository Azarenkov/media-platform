use std::sync::Arc;

use crate::domain::{
    entities::user::User, hasher_abstract::HasherAbstract,
    repositories::user_repository_abstract::UserRepositoryAbstract,
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

    pub async fn auth_user(&self, email: &str, password: &str) -> Result<u128, AuthServiceError> {
        let id = self.user_repository.find_id_by_email(email).await?;
        let password_hash = self.user_repository.find_password_by_email(email).await?;

        if !self
            .hasher
            .verify(password.to_owned(), password_hash)
            .await
            .map_err(|_| AuthServiceError::Internal)?
        {
            return Err(AuthServiceError::InvalidCredentials);
        }

        Ok(id)
    }
}
