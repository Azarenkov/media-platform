use std::env;

pub struct Config {
    pub db_url: String,
    pub port: String,
    pub secret: String,
}

impl Default for Config {
    fn default() -> Self {
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
        let secret = env::var("SECRET").expect("SECRET must be set");

        Self {
            db_url,
            port,
            secret,
        }
    }
}
