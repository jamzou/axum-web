use std::fmt::Display;


use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use diesel::prelude::*;
use crate::schema;
use schema::mo_app_user;


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(non_snake_case)]
#[derive(Queryable, Selectable)]
#[diesel(table_name = mo_app_user)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct User {
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(non_snake_case)]
#[derive(Insertable)] //新增
#[derive(AsChangeset)] //更新
#[diesel(table_name = mo_app_user)]
pub struct CreateUser {
    pub id: Option<u32>,
    pub emp_id: String,
    pub user_name: String,
    pub age: u8,
    pub birthday: String,
}

const DATE_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

pub fn serialize_date_time<S>(
    date: &Option<DateTime<Utc>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match date {
        Some(dt) => serializer.serialize_str(&dt.format(DATE_FORMAT).to_string()),
        None => serializer.serialize_none(),
    }
}

pub fn deserialize_date_time<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
        Some(str) => {
            let naive_dt = NaiveDateTime::parse_from_str(&str, DATE_FORMAT)
                .map_err(serde::de::Error::custom)?;
            Ok(Some(DateTime::<Utc>::from_naive_utc_and_offset(
                naive_dt, Utc,
            )))
        }
        None => Ok(None),
    }
}

pub fn serialize_naive<S>(
    date: &Option<NaiveDateTime>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match date {
        Some(dt) => serializer.serialize_str(&dt.format(DATE_FORMAT).to_string()),
        None => serializer.serialize_none(),
    }
}

pub fn deserialize_naive<'de, D>(deserializer: D) -> Result<Option<NaiveDateTime>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
        Some(str) => {
            let naive_dt = NaiveDateTime::parse_from_str(&str, DATE_FORMAT)
                .map_err(serde::de::Error::custom)?;
            Ok(Some(naive_dt))
        }
        None => Ok(None),
    }
}

impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).unwrap())
    }
}

impl Display for CreateUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).unwrap())
    }
}


