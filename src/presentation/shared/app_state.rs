use crate::{
    domain::services::{auth_service::AuthService, user_service::UserService},
    infrastructure::{hasher::ArgonHasher, repositories::user_repository::UserRepository},
};

pub struct AppState {
    pub user_service: UserService<UserRepository>,
    pub auth_service: AuthService<UserRepository, ArgonHasher>,
}

impl AppState {
    pub fn new(
        user_service: UserService<UserRepository>,
        auth_service: AuthService<UserRepository, ArgonHasher>,
    ) -> Self {
        Self {
            user_service,
            auth_service,
        }
    }
}
