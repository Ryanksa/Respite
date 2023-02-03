use std::fs;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub store_socket_addr: String,
}

impl Config {
    pub fn new() -> Self {
        let content = fs::read_to_string("./Config.toml").unwrap_or_default();
        let config: Config = toml::from_str(&content).unwrap_or_else(|_| {
            println!("Config.toml not found...");
            Config {
                store_socket_addr: "".to_owned(),
            }
        });
        return config;
    }
}
