use anyhow::anyhow;
use async_trait::async_trait;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use tracing::error;

use crate::entity::prelude::*;
pub mod org;
pub use org::OrgDao;
pub use org::OrgDaoImpl;

#[async_trait]
pub trait UserDao: Send + Sync {
    async fn query_user_by_id(&self, id: u32) -> anyhow::Result<Option<User>>;
    async fn get_all_users(&self) -> anyhow::Result<Vec<User>>;
    async fn add_user(&self, user: &CreateUser) -> anyhow::Result<u32>;
    async fn update_user(&self, user: &CreateUser) -> anyhow::Result<u32>;
    async fn delete_user(&self, id: u32) -> anyhow::Result<u32>;
    async fn get_user_detail(&self, id: u32) -> anyhow::Result<Option<(User, Org)>>;
    async fn find_user_by_username(&self, username: &str) -> anyhow::Result<Option<User>>;
    async fn update_last_login_time(&self, id: u32) -> anyhow::Result<bool>;
}

#[derive(Clone)]
pub struct UserDaoImpl {
    db: DatabaseConnection,
}

#[async_trait]
impl UserDao for UserDaoImpl {
    async fn query_user_by_id(&self, uid: u32) -> anyhow::Result<Option<User>> {
        let user = MoAppUser::find_by_id(uid)
            .one(&self.db)
            .await
            .map_err(|err| {
                error!("query_user_by_id database error: {:?}", err);
                anyhow!(err)
            })?;
        Ok(user)
    }

    async fn get_all_users(&self) -> anyhow::Result<Vec<User>> {
        let users = MoAppUser::find().all(&self.db).await.map_err(|err| {
            error!("get_all_users database error: {:?}", err);
            anyhow!(err)
        })?;
        Ok(users)
    }

    async fn add_user(&self, user: &CreateUser) -> anyhow::Result<u32> {
        let active_model = MoAppUserActiveModel {
            emp_id: Set(user.emp_id.clone()),
            user_name: Set(user.user_name.clone()),
            password: Set(user.password.clone()),
            email: Set(user.email.clone()),
            phone: Set(user.phone.clone()),
            org_id: Set(user.org_id),
            role: Set(user.role.clone()),
            status: Set(user.status),
            ..Default::default()
        };

        let result = active_model.insert(&self.db).await.map_err(|err| {
            error!("Database insert error: {:?}", err);
            anyhow!(err)
        })?;

        Ok(result.id)
    }

    async fn update_user(&self, user: &CreateUser) -> anyhow::Result<u32> {
        if user.id.is_none() {
            return Ok(0);
        }

        let uid = user.id.unwrap();

        let existing = MoAppUser::find_by_id(uid)
            .one(&self.db)
            .await
            .map_err(|err| {
                error!("Database query error: {:?}", err);
                anyhow!(err)
            })?;

        if existing.is_none() {
            return Ok(0);
        }

        let mut active_model: MoAppUserActiveModel = existing.unwrap().into();
        active_model.emp_id = Set(user.emp_id.clone());
        active_model.user_name = Set(user.user_name.clone());
        active_model.password = Set(user.password.clone());
        active_model.email = Set(user.email.clone());
        active_model.phone = Set(user.phone.clone());
        active_model.org_id = Set(user.org_id);
        active_model.role = Set(user.role.clone());
        active_model.status = Set(user.status);

        active_model.update(&self.db).await.map_err(|err| {
            error!("Database update error: {:?}", err);
            anyhow!(err)
        })?;

        Ok(1)
    }

    async fn delete_user(&self, uid: u32) -> anyhow::Result<u32> {
        let result = MoAppUser::delete_by_id(uid)
            .exec(&self.db)
            .await
            .map_err(|err| {
                error!("Database delete error: {:?}", err);
                anyhow!(err)
            })?;

        Ok(result.rows_affected as u32)
    }

    async fn get_user_detail(&self, id: u32) -> anyhow::Result<Option<(User, Org)>> {
        let result = MoAppUser::find()
            .filter(crate::entity::mo_app_user::Column::Id.eq(id))
            .find_also_related(MoGlOrg)
            .one(&self.db)
            .await
            .map_err(|err: sea_orm::DbErr| {
                error!("Database query error: {:?}", err);
                anyhow!(err)
            })?;
        match result {
            Some((user, Some(org))) => Ok(Some((user, org))),
            _ => Ok(None),
        }
    }

    async fn find_user_by_username(&self, username: &str) -> anyhow::Result<Option<User>> {
        let user = MoAppUser::find()
            .filter(crate::entity::mo_app_user::Column::UserName.eq(username))
            .one(&self.db)
            .await
            .map_err(|err| {
                error!("find_user_by_username database error: {:?}", err);
                anyhow!(err)
            })?;
        Ok(user)
    }

    async fn update_last_login_time(&self, id: u32) -> anyhow::Result<bool> {
        use sea_orm::Set;
        use chrono::Utc;
        
        let user = MoAppUser::find_by_id(id).one(&self.db).await.map_err(|err| {
            error!("update_last_login_time query error: {:?}", err);
            anyhow!(err)
        })?;
        
        if let Some(user) = user {
            let mut user_active_model: crate::entity::mo_app_user::ActiveModel = user.into();
            user_active_model.last_login_time = Set(Some(Utc::now().naive_utc()));
            user_active_model.update(&self.db).await.map_err(|err| {
                error!("update_last_login_time update error: {:?}", err);
                anyhow!(err)
            })?;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

impl UserDaoImpl {
    pub fn new(db: DatabaseConnection) -> Self {
        UserDaoImpl { db }
    }
}
