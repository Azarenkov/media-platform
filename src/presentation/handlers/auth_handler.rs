use actix_web::{HttpResponse, Responder, get, post, web};

use crate::{
    domain::{entities::user::User, services::errors::AuthServiceError},
    middleware::auth::encode_token,
    presentation::shared::{app_state::AppState, dto::user_dto::UserAuthRequest},
};

pub fn auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1/auth")
            .service(register_user)
            .service(auth),
    );
}

#[post("/register")]
async fn register_user(
    mut user: web::Json<User>,
    app_state: web::Data<AppState>,
) -> Result<impl Responder, AuthServiceError> {
    app_state.auth_service.register_user(&mut user).await?;
    Ok(HttpResponse::Created().json("User registered successfully"))
}

#[get("/login")]
async fn auth(
    user: web::Json<UserAuthRequest>,
    app_state: web::Data<AppState>,
) -> Result<impl Responder, AuthServiceError> {
    let id = app_state
        .auth_service
        .auth_user(&user.email, &user.password)
        .await?;

    let token = encode_token(id as usize, &app_state.secret)
        .await
        .map_err(|_| AuthServiceError::Internal)?;

    Ok(HttpResponse::Accepted()
        .append_header(("Authorization", format!("Bearer {}", token)))
        .json("User auth successfully"))
}
