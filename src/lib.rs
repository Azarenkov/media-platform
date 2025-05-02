use std::error::Error;

use config::Config;
use infrastructure::app_setup::{AppDependencies, server};

pub mod config;
mod domain;
mod infrastructure;
mod middleware;
mod presentation;

pub async fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let deps = AppDependencies::init(config).await;
    server(deps.app_state, &config.port).await?;

    Ok(())
}
