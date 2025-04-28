use crate::{
    domain::services::user_service::UserService,
    infrastructure::repositories::user_repository::UserRepository,
};

pub struct AppState {
    pub user_service: UserService<UserRepository>,
}

impl AppState {
    pub fn new(user_service: UserService<UserRepository>) -> Self {
        Self { user_service }
    }
}
