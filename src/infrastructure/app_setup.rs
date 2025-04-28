use std::error::Error;

use actix_web::{App, HttpServer, web};

use crate::{
    config::Config,
    domain::services::user_service::UserService,
    presentation::{handlers::user_handler::user_routes, shared::app_state::AppState},
};

use super::{db_connection::db_connect, repositories::user_repository::UserRepository};

pub struct AppDependencies {
    pub app_state: web::Data<AppState>,
}

impl AppDependencies {
    pub async fn init(config: &Config) -> Self {
        let pool = db_connect(&config.db_url).await;
        let user_repo = UserRepository::new(pool);
        let user_service = UserService::new(user_repo);

        let app_state = web::Data::new(AppState::new(user_service));

        Self { app_state }
    }
}

pub async fn server(app_state: web::Data<AppState>, port: &str) -> Result<(), Box<dyn Error>> {
    let address = format!("0.0.0.0:{}", port);
    println!("App address: {}", &address);

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .configure(user_routes)
    })
    .bind(address)?
    .run()
    .await?;
    Ok(())
}
