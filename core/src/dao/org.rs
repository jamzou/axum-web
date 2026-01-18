use crate::entity::prelude::*;
use anyhow::anyhow;
use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
use tracing::error;

#[async_trait]
pub trait OrgDao: Send + Sync {
    async fn query_org_by_id(&self, id: u32) -> anyhow::Result<Option<Org>>;
    async fn get_all_orgs(&self) -> anyhow::Result<Vec<Org>>;
    async fn add_org(&self, org: &Org) -> anyhow::Result<u32>;
    async fn update_org(&self, org: &Org) -> anyhow::Result<u32>;
    async fn delete_org(&self, id: u32) -> anyhow::Result<u32>;
}

#[derive(Clone)]
pub struct OrgDaoImpl {
    db: DatabaseConnection,
}

#[async_trait]
impl OrgDao for OrgDaoImpl {
    async fn query_org_by_id(&self, uid: u32) -> anyhow::Result<Option<Org>> {
        let user = MoGlOrg::find_by_id(uid)
            .one(&self.db)
            .await
            .map_err(|err| {
                error!("query_user_by_id database error: {:?}", err);
                anyhow!(err)
            })?;
        Ok(user)
    }

    async fn get_all_orgs(&self) -> anyhow::Result<Vec<Org>> {
        let users = MoGlOrg::find().all(&self.db).await.map_err(|err| {
            error!("get_all_users database error: {:?}", err);
            anyhow!(err)
        })?;
        Ok(users)
    }

    async fn add_org(&self, org: &Org) -> anyhow::Result<u32> {
        let active_model = MoGlOrgActiveModel {
            org_name: Set(org.org_name.clone()),
            org_code: Set(org.org_code.clone()),
            update_time: Set(Some(Utc::now().naive_utc())),
            ..Default::default()
        };

        let result = active_model.insert(&self.db).await.map_err(|err| {
            error!("Database insert error: {:?}", err);
            anyhow!(err)
        })?;

        Ok(result.id)
    }

    async fn update_org(&self, org: &Org) -> anyhow::Result<u32> {
        let existing = MoGlOrg::find_by_id(org.id)
            .one(&self.db)
            .await
            .map_err(|err| {
                error!("Database query error: {:?}", err);
                anyhow!(err)
            })?;

        if existing.is_none() {
            return Ok(0);
        }

        let mut active_model: MoGlOrgActiveModel = existing.unwrap().into();
        active_model.org_code = Set(org.org_code.clone());
        active_model.org_name = Set(org.org_name.clone());
        active_model.update_time = Set(Some(Utc::now().naive_utc()));

        active_model.update(&self.db).await.map_err(|err| {
            error!("Database update error: {:?}", err);
            anyhow!(err)
        })?;

        Ok(1)
    }

    async fn delete_org(&self, uid: u32) -> anyhow::Result<u32> {
        let result = MoGlOrg::delete_by_id(uid)
            .exec(&self.db)
            .await
            .map_err(|err| {
                error!("Database delete error: {:?}", err);
                anyhow!(err)
            })?;

        Ok(result.rows_affected as u32)
    }
}

impl OrgDaoImpl {
    pub fn new(db: DatabaseConnection) -> Self {
        OrgDaoImpl { db }
    }
}
