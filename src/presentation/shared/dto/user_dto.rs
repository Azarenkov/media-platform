use serde::Deserialize;

#[derive(Deserialize)]
pub struct UserAuthRequest {
    pub email: String,
    pub password: String,
}
