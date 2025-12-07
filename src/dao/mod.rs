use async_trait::async_trait;
use diesel::query_dsl::methods::{FilterDsl, FindDsl, SelectDsl};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::{ExpressionMethods, MysqlConnection, OptionalExtension, RunQueryDsl, SelectableHelper};
use crate::schema;
use schema::mo_app_user::dsl::*;

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
#[derive(Clone)]
pub struct UserDaoImpl {
    pool: Pool<ConnectionManager<MysqlConnection>>, //克隆pool不会克隆连接池
}
#[async_trait]
impl UserDao for UserDaoImpl {
    async fn query_user_by_id(&self, uid: u32) -> Result<Option<User>, AppErr> {
        let mut conn = self.pool.get().map_err(|e| {
            error!("Failed to get connection from pool: {:?}", e);
            anyhow!(e)
        })?;
        //使用diesel根据id查询
        let user = mo_app_user
        .filter(id.eq(uid))
        .first::<User>(&mut conn)
        .optional() // 使用 optional 来处理查不到的情况
        .map_err(|err| {
            error!("query_user_by_id database error: {:?}", err);
            anyhow!(err)
        })?;
        Ok(user)
    }

    async fn get_all_users(&self) -> Result<Vec<User>, AppErr> {
        let mut conn = self.pool.get().map_err(|e| {
            error!("Failed to get connection from pool: {:?}", e);
            anyhow!(e)
        })?;
        let users = mo_app_user
        .select(User::as_select())
        .load::<User>(&mut conn)
        .map_err(|err| {
            error!("get_all_users database error: {:?}", err);
            anyhow!(err)
        })?;
        Ok(users)
    }

    async fn add_user(&self, user: &CreateUser) -> Result<u32, AppErr> {
        let mut conn = self.pool.get().map_err(|e| {
            error!("Failed to get connection from pool: {:?}", e);
            anyhow!(e)
        })?;

        diesel::insert_into(mo_app_user)
            .values(user)
            .execute(&mut conn)
            .map_err(|err| {
                error!("Database insert error: {:?}", err);
                anyhow!(err)
            })?;
        #[derive(diesel::prelude::QueryableByName)]
        struct IdResult {
            #[diesel(sql_type = diesel::sql_types::Unsigned<diesel::sql_types::Integer>)]
            uid: u32,
        }
        let last_id: u32 = diesel::sql_query("SELECT LAST_INSERT_ID() AS uid")
        .load::<IdResult>(&mut conn)
        .map_err(|err| {
            error!("Failed to fetch last insert id: {:?}", err);
            anyhow!(err)
        })?
        .pop()
        .expect("Expected one result from LAST_INSERT_ID()")
        .uid;
        Ok(last_id)
    }

    async fn update_user(&self, user: &CreateUser) -> Result<u32, AppErr> {
        let mut conn = self.pool.get().map_err(|e| {
            error!("Failed to get connection from pool: {:?}", e);
            anyhow!(e)
        })?;
        if let None = user.id {
            return Err(AppErr::ParamError("id is required".to_owned()));
        }
        let updated_rows = diesel::update(mo_app_user.find(user.id.expect("unkown err"))) // 假设 CreateUser 包含 id 字段
            .set(user)
            .execute(&mut conn)
            .map_err(|err| {
                error!("Database update error: {:?}", err);
                anyhow!(err)
            })?;

        Ok(updated_rows as u32)
    }

    async fn delete_user(&self, uid: u32) -> Result<u32, AppErr> {
        let mut conn = self.pool.get().map_err(|e| {
            error!("Failed to get connection from pool: {:?}", e);
            anyhow!(e)
        })?;

        let deleted_rows = diesel::delete(mo_app_user.filter(id.eq(uid)))
            .execute(&mut conn)
            .map_err(|err| {
                error!("Database delete error: {:?}", err);
                anyhow!(err)
            })?;

        Ok(deleted_rows as u32)
    }
}
impl UserDaoImpl {
    pub fn new(pool: Pool<ConnectionManager<MysqlConnection>>) -> Self {
        UserDaoImpl { pool }
    }
}
