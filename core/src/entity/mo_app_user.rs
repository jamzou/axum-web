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
    pub emp_id: String,
    pub user_name: String,
    pub org_id: Option<i64>,
    pub age: Option<u8>,
    pub birthday: Option<String>,
    #[serde(
        serialize_with = "serialize_naive",
        deserialize_with = "deserialize_naive"
    )]
    pub create_time: Option<NaiveDateTime>,
    pub creater_id: Option<String>,
    #[serde(
        serialize_with = "serialize_naive",
        deserialize_with = "deserialize_naive"
    )]
    pub update_time: Option<NaiveDateTime>,
    pub updater_id: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateUser {
    pub id: Option<u32>,
    pub emp_id: String,
    pub user_name: String,
    pub age: u8,
    pub birthday: String,
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
