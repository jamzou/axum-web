use super::date_serde::{deserialize_naive, serialize_naive};
use chrono::NaiveDateTime;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "mo_app_user")]
#[serde(rename_all = "camelCase")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u32,
    /**
     * 员工编号
     */
    pub emp_id: String,
    /**
     * 用户名
     */
    pub user_name: String,
    /**
     * 加密后的密码
     */
    pub password: String,
    /**
     * 邮箱
     */
    pub email: Option<String>,
    /**
     * 手机号
     */
    pub phone: Option<String>,
    /**
     * 组织id
     */
    pub org_id: Option<u32>,
    /**
     * 用户角色
     */
    pub role: Option<String>,
    /**
     * 用户状态：1-激活，0-禁用
     */
    pub status: Option<i8>,
    /**
     * 最后登录时间
     */
    #[serde(
        serialize_with = "serialize_naive",
        deserialize_with = "deserialize_naive"
    )]
    pub last_login_time: Option<NaiveDateTime>,
    /**
     * 创建时间
     */
    #[serde(
        serialize_with = "serialize_naive",
        deserialize_with = "deserialize_naive"
    )]
    pub created_at: Option<NaiveDateTime>,
    pub created_by: Option<String>,
    #[serde(
        serialize_with = "serialize_naive",
        deserialize_with = "deserialize_naive"
    )]
    pub updated_at: Option<NaiveDateTime>,
    pub updated_by: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::mo_gl_org::Entity",
        from = "Column::OrgId",
        to = "super::mo_gl_org::Column::Id"
    )]
    Org,
}

impl Related<super::mo_gl_org::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Org.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateUser {
    pub id: Option<u32>,
    pub emp_id: String,
    pub user_name: String,
    pub password: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub org_id: Option<u32>,
    pub role: Option<String>,
    pub status: Option<i8>,
}
#[allow(unused)]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LoginUser {
    pub user_name: String,
    pub password: String,
}

#[allow(unused)]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserInfo {
    pub id: u32,
    pub emp_id: String,
    pub user_name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub org_id: Option<u32>,
    pub role: Option<String>,
}

impl std::fmt::Display for Model {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).unwrap())
    }
}

impl std::fmt::Display for CreateUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).unwrap())
    }
}

impl std::fmt::Display for LoginUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).unwrap())
    }
}

impl std::fmt::Display for UserInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).unwrap())
    }
}
