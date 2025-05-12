use actix_web::{HttpResponse, Responder, delete, web};

use crate::{
    domain::services::errors::AccountServiceError, middleware::auth::AuthToken,
    presentation::shared::app_state::AppState,
};

pub fn account_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api/v1/account").service(delete_account));
}

#[delete("/delete")]
async fn delete_account(
    auth_token: AuthToken,
    app_state: web::Data<AppState>,
) -> Result<impl Responder, AccountServiceError> {
    app_state
        .account_service
        .delete_account(auth_token.id as u128)
        .await?;
    Ok(HttpResponse::Ok().json(format!(
        "Account was deleted successfully for id: {}",
        auth_token.id as u128
    )))
}
