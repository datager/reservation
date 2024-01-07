use std::sync::Condvar;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use tokio::fs;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Config {
    pub db: DbConfig,
    pub server: ServerConfig,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DbConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub dbname: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

impl Config {
    pub async fn load(filename: &str) -> Result<Self> {
        let config = fs::read_to_string(filename).await.unwrap();
        Ok(serde_yaml::from_str(&config)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn config_should_be_loaded() {
        let config = Config::load("fixtures/config.yml").await.unwrap();
        assert_eq!(
            config,
            Config {
                db: DbConfig {
                    host: "localhost".to_string(),
                    port: 5432,
                    user: "y".to_string(),
                    password: "postgres".to_string(),
                    dbname: "reservation".to_string(),
                },
                server: ServerConfig {
                    host: "localhost".to_string(),
                    port: 8080,
                },
            }
        )
    }
}
