use redis::AsyncTypedCommands;
use redis::Client;
use tracing::error;
use std::env;

use crate::context::jamerr::AppErr;

pub async fn init_redis() -> RedisTemplate {
    // 从环境变量获取 Redis URL
    let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set");
    println!("Redis URL: {}", redis_url); // 这会打印出解码后的 URL

    let client = redis::Client::open(redis_url).expect("初始化redis连接出错");
    return RedisTemplate {
        client,
    };
}

pub struct RedisTemplate {
    client: Client,
}

impl RedisTemplate {
    pub async fn redis_set(&self, key: &str, value: &str) -> Result<(), AppErr> {
        let mut con = self.client.get_multiplexed_tokio_connection()
        .await.map_err(|e| {
            error!("redis连接出错:{}", e);
            AppErr::DbError(e.to_string())
        })?;
        let _ = con.set(key, value).await.map_err(|e| AppErr::DbError(e.to_string()))?;
        Ok(())
    }
}

