use serde::{Deserialize, Serialize};
use std::{env, fs};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub auth_uri: String,
    pub store_uri: String,
    pub menu_uri: String,
    pub waiter_uri: String,
    pub db_uri: String,
    pub db_pool_size: u32,
    pub jwt_secret: String,
}

impl Config {
    pub fn new() -> Self {
        let content = fs::read_to_string("./Config.toml").unwrap_or_default();
        let config: Config = toml::from_str(&content).unwrap_or_else(|_| Config {
            auth_uri: env::var("auth_uri").unwrap_or_default(),
            store_uri: env::var("store_uri").unwrap_or_default(),
            menu_uri: env::var("menu_uri").unwrap_or_default(),
            waiter_uri: env::var("waiter_uri").unwrap_or_default(),
            db_uri: env::var("db_uri").unwrap_or_default(),
            db_pool_size: env::var("db_pool_size")
                .unwrap_or_default()
                .parse()
                .unwrap_or_default(),
            jwt_secret: env::var("jwt_secret").unwrap_or_default(),
        });
        return config;
    }
}
