use std::{error::Error, sync::Arc};

use actix_web::{App, HttpServer, web};

use crate::{
    config::Config,
    domain::services::{account_service::AccountService, auth_service::AuthService},
    presentation::{
        handlers::{account_handler::account_routes, auth_handler::auth_routes},
        shared::app_state::AppState,
    },
};

use super::{
    db::db_connection::db_connect, hasher::ArgonHasher,
    repositories::user_repository::UserRepository,
};

pub struct AppDependencies {
    pub app_state: web::Data<AppState>,
}

impl AppDependencies {
    pub async fn init(config: &Config) -> Self {
        let pool = db_connect(&config.db_url).await;
        let hasher = ArgonHasher;
        let user_repo = Arc::new(UserRepository::new(pool));
        let account_service = AccountService::new(Arc::clone(&user_repo));
        let auth_service = AuthService::new(Arc::clone(&user_repo), hasher);

        let app_state = web::Data::new(AppState::new(
            account_service,
            auth_service,
            config.secret.to_owned(),
        ));

        Self { app_state }
    }
}

pub async fn server(app_state: web::Data<AppState>, port: &str) -> Result<(), Box<dyn Error>> {
    let address = format!("0.0.0.0:{}", port);
    println!("App address: {}", &address);

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            // .configure(user_routes)
            .configure(auth_routes)
            .configure(account_routes)
    })
    .bind(address)?
    .run()
    .await?;
    Ok(())
}
