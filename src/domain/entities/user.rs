use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub email: String,
    pub username: String,
    pub password: String,
}

impl User {
    pub fn new(email: String, username: String, password: String) -> Self {
        Self {
            email,
            username,
            password,
        }
    }
}
