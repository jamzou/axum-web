use redis::AsyncTypedCommands;
use redis::Client;
use redis::aio::MultiplexedConnection;
use tracing::error;
use tracing::info;
use std::env;

use crate::context::jamerr::AppErr;

pub async fn init_redis() -> RedisTemplate {
    let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set");
    info!("Redis URL: {}", redis_url);

    let client = redis::Client::open(redis_url)
        .expect("初始化redis连接出错");
    client.get_multiplexed_async_connection()
        .await
        .expect("初始化redis连接出错");
    RedisTemplate { client }
}

pub struct RedisTemplate {
    client: Client,
}

impl RedisTemplate {
    async fn get_connection(&self) -> Result<MultiplexedConnection, AppErr> {
        self.client.get_multiplexed_tokio_connection()
            .await
            .map_err(|e| {
                error!("redis连接出错:{}", e);
                AppErr::DbError(e.to_string())
            })
    }

    pub async fn redis_set(&self, key: &str, value: &str) -> Result<(), AppErr> {
        let mut con = self.get_connection().await?;
        let _ = con.set(key, value).await.map_err(|e| AppErr::DbError(e.to_string()))?;
        Ok(())
    }

    pub async fn redis_get(&self, key: &str) -> Result<Option<String>, AppErr> {
        let mut con = self.get_connection().await?;
        let value = con.get(key).await.map_err(|e| AppErr::DbError(e.to_string()));
        value
    }

    pub async fn redis_exists(&self, key: &str) -> Result<bool, AppErr> {
        let mut con = self.get_connection().await?;
        let exists = con.exists(key).await.map_err(|e| AppErr::DbError(e.to_string()));
        exists
    }

    pub async fn redis_del(&self, key: &str) -> Result<usize, AppErr> {
        let mut con = self.get_connection().await?;
        let del = con.del(key).await.map_err(|e| AppErr::DbError(e.to_string()));
        del
    }

    pub async fn redis_unlink(&self, key: &str) -> Result<usize, AppErr> {
        let mut con = self.get_connection().await?;
        let unlink = con.unlink(key).await.map_err(|e| AppErr::DbError(e.to_string()));
        unlink
    }
}
