use sqlx::mysql::MySqlPoolOptions;
use sqlx::{MySql, Pool};
use std::env;
use std::time::Duration;

pub async fn establish_conn() -> Pool<MySql> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be setted ");
    let max_conn = 10;
    let min_conn = 5;
    let timeout = 3;

    let error_string = format!("connot connect to mysql {}", database_url);
    let pool = MySqlPoolOptions::new()
        .max_connections(max_conn)
        .min_connections(min_conn)
        .acquire_timeout(Duration::from_secs(timeout))
        .connect(&database_url)
        .await
        .expect(error_string.as_str());
    pool
}
