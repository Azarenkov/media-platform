use actix_web::{HttpResponse, Responder, get, web};

use crate::{
    domain::services::errors::UserServiceError,
    presentation::shared::{app_state::AppState, dto::user_dto::EmailRequest},
};

pub fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api/v1/user"));
    // .service(get_user));
}

// #[get("/get_by_email")]
// async fn get_user(
//     email_data: web::Json<EmailRequest>,
//     app_state: web::Data<AppState>,
// ) -> Result<impl Responder, UserServiceError> {
//     let user = app_state
//         .user_service
//         .get_by_email(&email_data.email)
//         .await?;
//     Ok(HttpResponse::Ok().json(user))
// }
