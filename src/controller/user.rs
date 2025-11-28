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
    let redis_cli = appstate.redis_client;
    //拼接key字符串
    let key = format!("user:{}", id_param.id);
    let res = redis_cli.redis_get(&key).await?;
    if let Some(user_json) = res {
        //将user字符串反序列化为User对象
        let user: User = serde_json::from_str(&user_json)?;
        return Ok(ResWrapper::success(Some(user)));
    }
    let user = appstate.user_dao.query_user_by_id(id_param.id).await?;
    if let Some(u) = &user {
        //将u转成json字符串
        let json = serde_json::to_string(&u)?;
        redis_cli.redis_set(&key, &json).await?;
        info!("set redis cache key:{} value:{}", key, json);
    }
    
    Ok(ResWrapper::success(user))
}

pub async fn delete_user<T:UserDao>(
    State(appstate): State<AppState<T>>,
    Form(id_param): Form<IdParam>,
) -> Result<ResWrapper<u32>, AppErr> {
    let r = appstate.user_dao.delete_user(id_param.id).await?;
    tokio::spawn(async move {
        let _ = appstate.redis_client.redis_del(&format!("user:{}", id_param.id)).await;
        info!("delete user:{}", id_param.id);
    });
    Ok(ResWrapper::success(r))
}

pub async fn update_user<T:UserDao + 'static>(
    State(appstate): State<AppState<T>>,
    Json(user): Json<CreateUser>,
) -> Result<ResWrapper<u32>, AppErr> {
    let r = appstate.user_dao.update_user(&user).await?;
    let user_id = user.id.unwrap();
    info!("update user:{}", &user.id.unwrap());
    //查询并更新redis缓存
    tokio::spawn(async move {
        let key = format!("user:{}", user_id);
        let value = appstate.user_dao.query_user_by_id(user_id).await;
        match value {
            Ok(u) => {
                if let Some(u) = u {
                    let value = serde_json::to_string(&u);
                    if let Ok(value) = value {
                        let _ = appstate.redis_client.redis_set(&key, &value).await;
                        info!("update redis cache:{}", key);
                    }
                }
            },
            Err(_) => {},
        }
    });
    
    Ok(ResWrapper::success(r))
}
