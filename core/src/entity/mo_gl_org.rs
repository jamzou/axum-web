use super::date_serde::{deserialize_naive, serialize_naive};
use chrono::NaiveDateTime;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "mo_gl_org")]
#[serde(rename_all = "camelCase")]
pub struct Model {
    /**
     * 主键id
     */
    #[sea_orm(primary_key)]
    pub id: u32,
    /**
     * 部门代码
     */
    pub code: String,
    /**
     * 部门名称
     */
    pub name: String,
    /**
     * 父部门id
     */
    pub parent_id: u32,
    /**
     * 是否删除
     */
    pub deleted: bool,
    /**
     * 删除时间
     */
    #[serde(
        serialize_with = "serialize_naive",
        deserialize_with = "deserialize_naive"
    )]
    pub deleted_at: Option<NaiveDateTime>,
    /**
     * 创建人
     */
    pub created_by: Option<String>,
    /**
     * 更新人
     */
    pub updated_by: Option<String>,
    
    /**
     * 创建时间
     */
    #[serde(
        serialize_with = "serialize_naive",
        deserialize_with = "deserialize_naive"
    )]
    pub created_at: Option<NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_naive",
        deserialize_with = "deserialize_naive"
    )]
    /**
     * 更新时间
     */
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::mo_app_user::Entity")]
    User,
}

impl Related<super::mo_app_user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
