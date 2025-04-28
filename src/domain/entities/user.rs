use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    pub email: String,
    pub username: String,
    pub password: String,
}

// impl User {
//     pub fn new(email: String, username: String, password: String) -> Self {
//         Self {
//             email,
//             username,
//             password,
//         }
//     }
// }
