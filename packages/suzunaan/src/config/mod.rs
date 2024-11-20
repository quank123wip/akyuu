use std::env;

pub struct Config {
    pub db_url: String,
    pub host: String,
    pub port: String,
    pub server_url: String,
    pub greet_message: String,
}

pub fn get_config() {
    dotenvy::dotenv().ok();
}
