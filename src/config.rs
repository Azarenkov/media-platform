use std::env;

pub struct Config {
    pub db_url: String,
    pub port: String,
}

impl Default for Config {
    fn default() -> Self {
        let db_url = env::var("DB_URL").expect("DB_URL must be set");
        let port = env::var("PORT").expect("PORT must be set");

        Self { db_url, port }
    }
}
