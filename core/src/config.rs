use serde::Deserialize;
use std::env;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub db: DbConfig,
    pub server: ServerConfig,
    pub log: LogConfig,
    pub consul: ConsulConfig,
}

#[derive(Debug, Deserialize)]
pub struct DbConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connect_timeout: u64,
    pub acquire_timeout: u64,
    pub idle_timeout: u64,
    pub max_lifetime: u64,
    pub sql_log: bool,
}

#[derive(Debug, Deserialize)]
pub struct LogConfig {
    pub level: String,
    pub directory: String,
    pub file: String,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    // pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct ConsulConfig {
    pub address: String,
}

pub fn load_config(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(path)?;
    let config: Config = serde_yml::from_str(&contents)?;
    Ok(config)
}

pub fn get_config_path() -> String {
    env::var("CONFIG_PATH").unwrap_or_else(|_| "config/config.yaml".to_string())
}
