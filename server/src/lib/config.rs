use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub store_uri: String,
    pub auth_uri: String,
    pub db_uri: String,
    pub db_pool_size: u32,
    pub jwt_secret: String,
}

impl Config {
    pub fn new() -> Self {
        let content = fs::read_to_string("./Config.toml").unwrap_or_default();
        let config: Config = toml::from_str(&content).unwrap_or_else(|_| {
            println!("Config.toml not found...");
            Config {
                store_uri: "".to_owned(),
                auth_uri: "".to_owned(),
                db_uri: "".to_owned(),
                db_pool_size: 0,
                jwt_secret: "".to_owned(),
            }
        });
        return config;
    }
}
