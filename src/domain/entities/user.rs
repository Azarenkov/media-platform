use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use validator::Validate;

#[derive(Default, Serialize, Deserialize, FromRow, Validate)]
pub struct User {
    pub id: Option<u128>,
    #[validate(email)]
    pub email: String,
    pub username: String,
    pub password: String,
}
