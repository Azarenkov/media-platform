use std::sync::Arc;

use crate::domain::repositories::user_repository_abstract::UserRepositoryAbstract;

pub struct UserService<UserRepo>
where
    UserRepo: UserRepositoryAbstract,
{
    user_repository: Arc<UserRepo>,
}

impl<UserRepo> UserService<UserRepo>
where
    UserRepo: UserRepositoryAbstract,
{
    pub fn new(user_repository: Arc<UserRepo>) -> Self {
        Self { user_repository }
    }

    // pub async fn get_by_email(&self, email: &str) -> Result<User, UserServiceError> {
    //     let user = self.user_repository.find_by_email(email).await?;
    //     Ok(user)
    // }
}
