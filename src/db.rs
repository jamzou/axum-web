use std::env;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::MysqlConnection;
type DbPool = Pool<ConnectionManager<MysqlConnection>>;

pub async fn establish_conn() -> DbPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be setted ");
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    
    let pool = Pool::builder()
        .max_size(20) // 最大连接数
        .min_idle(Some(5)) // 最小空闲连接数
        .build(manager)
        .expect("Failed to create pool");
    pool
}
