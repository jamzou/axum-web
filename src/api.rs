use anyhow::anyhow;
use axum::extract::State;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{MySql, Pool};

use crate::context::{jamerr::AppErr, res_wrapper::ResWrapper};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
#[allow(non_snake_case)]
pub struct User {
    id: u32,
    emp_id: String,
    user_name: String,
    age: u8,
    birthday: String,
    create_time: Option<DateTime<Utc>>,
    creater_id: Option<String>,
    update_time: Option<DateTime<Utc>>,
    updater_id: Option<String>,
}

pub async fn add_user() {}

pub async fn query_user(State(pool): State<Pool<MySql>>) -> Result<ResWrapper<Vec<User>>, AppErr> {
    let users = sqlx::query_as("SELECT * FROM mo_app_user")
        .fetch_all(&pool)
        .await
        .map_err(|err| anyhow!(err))?;
    Ok(ResWrapper::success(users))
}

// pub async fn query_user(
//     State(pool): State<MySqlPool>,
// ) -> Result<Json<Vec<User>>, (StatusCode, String)> {
//     let string_query = "SELECT * FROM mo_app_user";
//     let result = sqlx::query_as(string_query)
//         .fetch_all(&pool)
//         .await
//         .map_err(|err| {
//             (
//                 StatusCode::INTERNAL_SERVER_ERROR,
//                 format!("Error is: {}", err),
//             )
//         })?;
//     Ok(Json(result))
// }

pub async fn delete_user() {}

pub async fn update_user() {}
