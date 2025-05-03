use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Serialize, Deserialize, FromRow, Default)]
pub struct User {
    pub id: Option<u128>,
    pub email: String,
    pub username: String,
    pub password: String,
}
