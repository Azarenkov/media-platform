use serde::Deserialize;

#[derive(Deserialize)]
pub struct EmailRequest {
    pub email: String,
}
