use std::sync::Arc;

use sqlx::{MySql, Pool};

use crate::dao::UserDao;

#[derive(Clone)]
pub struct AppState {
    pub pool: Pool<MySql>,
    pub user_dao: Arc<Box<dyn UserDao + Send + Sync>>,
}

impl AppState {
    pub fn new(pool: Pool<MySql>, user_dao: Box<dyn UserDao + Send + Sync>) -> Self {
        Self {
            pool,
            user_dao: Arc::new(user_dao),
        }
    }
}
