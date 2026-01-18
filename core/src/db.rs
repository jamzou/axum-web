use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::time::Duration;

use crate::config::Config;

pub async fn establish_conn(config: &Config) -> DatabaseConnection {
    let db_config = &config.db;
    let database_url = db_config.url.clone();

    let mut opt = ConnectOptions::new(database_url);
    opt.max_connections(db_config.max_connections)
        .min_connections(db_config.min_connections)
        .connect_timeout(Duration::from_secs(db_config.connect_timeout))
        .acquire_timeout(Duration::from_secs(db_config.acquire_timeout))
        .idle_timeout(Duration::from_secs(db_config.idle_timeout))
        .max_lifetime(Duration::from_secs(db_config.max_lifetime))
        .sqlx_logging(db_config.sql_log);

    Database::connect(opt)
        .await
        .expect("Failed to create database connection")
}
