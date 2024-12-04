use std::{env, fmt::Error};

#[derive(Debug, Clone)]
pub struct Config {
    pub db_url: String,
    pub host: String,
    pub port: String,
    pub server_url: String,
    pub greet_message: String,
}

pub fn get_config() -> Result<Config, Error> {
    dotenvy::dotenv().ok();
    let config = Config {
        db_url: env::var("DB_URL").expect("DB_URL must be set"),
        host: env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
        port: env::var("PORT").unwrap_or_else(|_| "3000".to_string()),
        server_url: env::var("SERVER_URL").unwrap_or_else(|_| "127.0.0.1:3000".to_string()),
        greet_message: env::var("GREET_MESSAGE").unwrap_or_else(|_| "Akyuu web api endpoint 「鈴奈庵」 0.0.1 is running.".to_string()),
    };
    Ok(config)
}
