use std::sync::{Arc, atomic::AtomicUsize};

use diesel::r2d2::{ConnectionManager, Pool};
use diesel::MysqlConnection;


use crate::{dao::UserDao, redisconfig::RedisTemplate};

#[derive(Clone)]
pub struct AppState<T> where T: UserDao + Send + Sync {
    pub pool: Pool<ConnectionManager<MysqlConnection>>,
    pub user_dao: Arc<T>,
    pub save_count: Arc<AtomicUsize>,
    pub redis_client: Arc<RedisTemplate>,
}

impl<T> AppState<T> where T: UserDao + Send + Sync {
    pub fn new(pool: Pool<ConnectionManager<MysqlConnection>>, user_dao: T, redis_client: RedisTemplate) -> Self {
        Self {
            pool,
            user_dao: Arc::new(user_dao),
            save_count: Arc::new(AtomicUsize::new(0)),
            redis_client: Arc::new(redis_client),
        }
    }

    pub fn get_save_count(&self) -> usize {
        self.save_count.load(std::sync::atomic::Ordering::Relaxed)
    }

    pub fn inc_save_count(&self) {
        self.save_count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }
}
