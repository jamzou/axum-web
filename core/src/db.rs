use std::env;
use sea_orm::{Database, DatabaseConnection, ConnectOptions};
use std::time::Duration;

pub async fn establish_conn() -> DatabaseConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be setted ");

    let mut opt = ConnectOptions::new(database_url);
    opt.max_connections(20)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true);

    Database::connect(opt)
        .await
        .expect("Failed to create database connection")
}
