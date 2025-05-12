use crate::{
    domain::services::{account_service::AccountService, auth_service::AuthService},
    infrastructure::{hasher::ArgonHasher, repositories::user_repository::UserRepository},
};

pub struct AppState {
    pub account_service: AccountService<UserRepository>,
    pub auth_service: AuthService<UserRepository, ArgonHasher>,
    pub secret: String,
}

impl AppState {
    pub fn new(
        account_service: AccountService<UserRepository>,
        auth_service: AuthService<UserRepository, ArgonHasher>,
        secret: String,
    ) -> Self {
        Self {
            account_service,
            auth_service,
            secret,
        }
    }
}
