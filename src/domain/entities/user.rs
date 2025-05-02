use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Serialize, Deserialize, FromRow, Default)]
pub struct User {
    pub id: Option<i32>,
    pub email: String,
    pub username: String,
    pub password: String,
}

impl User {
    pub fn new(id: Option<i32>, email: String, username: String, password: String) -> Self {
        Self {
            id,
            email,
            username,
            password,
        }
    }
}
