use actix_web::{HttpResponse, Responder, post, web};

use crate::{
    domain::{entities::user::User, services::errors::UserServiceError},
    presentation::shared::app_state::AppState,
};

pub fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api/v1/user").service(register_user));
}

#[post("/register")]
async fn register_user(
    user: web::Json<User>,
    app_state: web::Data<AppState>,
) -> Result<impl Responder, UserServiceError> {
    app_state.user_service.create(&user).await?;
    Ok(HttpResponse::Ok().json("User registered successfully"))
}
