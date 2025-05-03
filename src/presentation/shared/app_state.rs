use crate::{
    domain::services::{auth_service::AuthService, user_service::UserService},
    infrastructure::{hasher::ArgonHasher, repositories::user_repository::UserRepository},
};

pub struct AppState {
    pub user_service: UserService<UserRepository>,
    pub auth_service: AuthService<UserRepository, ArgonHasher>,
    pub secret: String,
}

impl AppState {
    pub fn new(
        user_service: UserService<UserRepository>,
        auth_service: AuthService<UserRepository, ArgonHasher>,
        secret: String,
    ) -> Self {
        Self {
            user_service,
            auth_service,
            secret,
        }
    }
}
