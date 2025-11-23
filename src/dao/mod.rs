use async_trait::async_trait;
use sqlx::{MySql, Pool};

use crate::{
    context::jamerr::AppErr,
    domain::{CreateUser, User},
};
use anyhow::anyhow;
use tracing::error;
#[async_trait]
pub trait UserDao: Send + Sync {
    async fn query_user_by_id(&self, id: u32) -> Result<Option<User>, AppErr>;
    async fn get_all_users(&self) -> Result<Vec<User>, AppErr>;
    // 添加用户,返回新增的id
    async fn add_user(&self, user: &CreateUser) -> Result<u32, AppErr>;
    // 修改用户,返回更新的行数
    async fn update_user(&self, user: &CreateUser) -> Result<u32, AppErr>;
    async fn delete_user(&self, id: u32) -> Result<u32, AppErr>;
}

pub struct UserDaoImpl {
    pool: Pool<MySql>, //克隆pool不会克隆连接池
}
#[async_trait]
impl UserDao for UserDaoImpl {
    async fn query_user_by_id(&self, id: u32) -> Result<Option<User>, AppErr> {
        let user = sqlx::query_as("SELECT * FROM mo_app_user where id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|err| {
                error!("query_user_by_id Database error: {:?}", err);
                anyhow!(err)
            })?;
        Ok(user)
    }

    async fn get_all_users(&self) -> Result<Vec<User>, AppErr> {
        let users = sqlx::query_as("SELECT * FROM mo_app_user")
            .fetch_all(&self.pool)
            .await
            .map_err(|err| {
                error!("get_all_users Database error: {:?}", err);
                anyhow!(err)
            })?;
        Ok(users)
    }

    async fn add_user(&self, user: &CreateUser) -> Result<u32, AppErr> {
        let r = sqlx::query("INSERT INTO mo_app_user (emp_id, user_name, age, birthday, create_time) VALUES (?, ?, ?, ?, NOW())")
        .bind(user.emp_id.clone())
        .bind(user.user_name.clone())
        .bind(user.age)
        .bind(user.birthday.clone())
        .execute(&self.pool)
        .await
        .map_err(|err| {
            error!("add_user Database error: {:?}", err);
            anyhow!(err)
        })?;
        Ok(r.last_insert_id() as u32)
    }

    async fn update_user(&self, user: &CreateUser) -> Result<u32, AppErr> {
        let id = user.id.ok_or(anyhow!("id is null"))?;
        let r = sqlx::query(
        "update mo_app_user set emp_id=?, user_name = ?, age=?, birthday=?,update_time=NOW() where id = ?",
        )
        .bind(user.emp_id.clone())
        .bind(user.user_name.clone())
        .bind(user.age)
        .bind(user.birthday.clone())
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(|err| {
            error!("update_user Database error: {}", err);
            anyhow!(err)
        })?;
        Ok(r.rows_affected() as u32)
    }

    async fn delete_user(&self, id: u32) -> Result<u32, AppErr> {
        let r = sqlx::query("DELETE FROM mo_app_user where id = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|err| {
                error!("delete_user Database error: {}", err);
                anyhow!(err)
            })?;
        Ok(r.rows_affected() as u32)
    }
}
impl UserDaoImpl {
    pub fn new(pool: Pool<MySql>) -> Self {
        UserDaoImpl { pool }
    }
}
