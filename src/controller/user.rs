use axum::{extract::State, Form, Json};
use serde::Deserialize;
use tracing::info;

use crate::{
    context::{appstate::AppState, jamerr::AppErr, res_wrapper::ResWrapper}, dao::UserDao, domain::{CreateUser, User}
};

pub async fn add_user<T:UserDao>(
    State(appstate): State<AppState<T>>,
    Json(payload): Json<CreateUser>,
) -> Result<ResWrapper<u32>, AppErr> {
    info!("add user, payload: {}", payload);
    let r = appstate.user_dao.add_user(&payload).await?;
    appstate.inc_save_count();
    info!("save count: {}", appstate.get_save_count());
    Ok(ResWrapper::success(r))
}

// pub async fn query_user(State(pool): State<Pool<MySql>>) -> Result<ResWrapper<Vec<User>>, AppErr> {
//     let users = sqlx::query_as("SELECT * FROM mo_app_user")
//         .fetch_all(&pool)
//         .await
//         .map_err(|err| anyhow!(err))?;
//     Ok(ResWrapper::success(users))
// }

pub async fn query_user<T:UserDao>(State(appstate): State<AppState<T>>) -> Result<ResWrapper<Vec<User>>, AppErr> {
    let users = appstate.user_dao.get_all_users().await?;
    Ok(ResWrapper::success(users))
}

#[derive(Deserialize)]
pub struct IdParam {
    id: u32,
}
/// form urlencoded type param
pub async fn query_user_by_id<T:UserDao>(
    State(appstate): State<AppState<T>>,
    Form(id_param): Form<IdParam>,
) -> Result<ResWrapper<Option<User>>, AppErr> {
    let user = appstate.user_dao.query_user_by_id(id_param.id).await?;
    Ok(ResWrapper::success(user))
}

pub async fn delete_user<T:UserDao>(
    State(appstate): State<AppState<T>>,
    Form(id_param): Form<IdParam>,
) -> Result<ResWrapper<u32>, AppErr> {
    let r = appstate.user_dao.delete_user(id_param.id).await?;
    Ok(ResWrapper::success(r))
}

pub async fn update_user<T:UserDao>(
    State(appstate): State<AppState<T>>,
    Json(user): Json<CreateUser>,
) -> Result<ResWrapper<u32>, AppErr> {
    let r = appstate.user_dao.update_user(&user).await?;
    Ok(ResWrapper::success(r))
}
