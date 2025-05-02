use actix_web::{HttpResponse, Responder, get, post, web};

use crate::{
    domain::{entities::user::User, services::errors::AuthServiceError},
    presentation::shared::app_state::AppState,
};

pub fn auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api/v1/auth").service(register_user));
}

#[post("/register")]
async fn register_user(
    mut user: web::Json<User>,
    app_state: web::Data<AppState>,
) -> Result<impl Responder, AuthServiceError> {
    app_state.auth_service.register_user(&mut user).await?;
    Ok(HttpResponse::Created().json("User registered successfully"))
}

// #[get("/auth")]
// async fn auth(
//     user: web::Json<User>,
//     app_state: web::Data<AppState>,
// ) -> Result<impl Responder, AuthServiceError> {
//     // app_state.auth_service
// }
