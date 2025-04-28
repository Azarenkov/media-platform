use std::error::Error;

use dotenv::dotenv;
use media_platform::{config::Config, run};

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let config = Config::new();

    run(&config).await?;
    Ok(())
}
