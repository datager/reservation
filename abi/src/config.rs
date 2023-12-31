use serde::{Deserialize, Serialize};
use std::fs;

use crate::Error;

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
    #[serde(default = "default_pool_size")]
    pub max_connections: u32,
}

fn default_pool_size() -> u32 {
    5
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

impl Config {
    pub fn load(filename: &str) -> Result<Self, Error> {
        let config = fs::read_to_string(filename).map_err(|_| Error::ConfigReadError)?;
        Ok(serde_yaml::from_str(&config).map_err(|_| Error::ConfigParseError)?)
    }
}

impl DbConfig {
    pub fn url(&self) -> String {
        if self.password.is_empty() {
            format!(
                "postgres://{}@{}:{}/{}",
                self.user, self.host, self.port, self.dbname
            )
        } else {
            format!(
                "postgres://{}:{}@{}:{}/{}",
                self.user, self.password, self.host, self.port, self.dbname
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_should_be_loaded() {
        let config = Config::load("fixtures/config.yml").unwrap();
        assert_eq!(
            config,
            Config {
                db: DbConfig {
                    host: "localhost".to_string(),
                    port: 5432,
                    user: "y".to_string(),
                    password: "postgres".to_string(),
                    dbname: "reservation".to_string(),
                    max_connections: 5,
                },
                server: ServerConfig {
                    host: "localhost".to_string(),
                    port: 50051,
                },
            }
        )
    }
}
