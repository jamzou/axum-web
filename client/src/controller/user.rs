use axum::{extract::State, Form, Json};
use serde::Deserialize;
use tonic::Request;
use tracing::info;

use crate::context::appstate::GrpcClient;
use crate::context::{appstate::AppState, jamerr::AppErr, res_wrapper::ResWrapper};
use grpc_dsl::user::{AddUserRequest, LoginUserRequest, RegisterUserRequest};
use grpc_dsl::common::{IdRequest, Empty};

pub async fn add_user(
    State(appstate): State<AppState>,
    Json(payload): Json<HttpCreateUser>,
) -> Result<ResWrapper<u32>, AppErr> {
    info!(
        "add user via gRPC, payload: {}",
        serde_json::to_string(&payload).unwrap_or_default()
    );

    let mut client: GrpcClient = appstate.grpc_client.clone();
    let req = AddUserRequest {
        id: payload.id.unwrap_or(0),
        emp_id: payload.emp_id,
        user_name: payload.user_name,
        password: payload.password.clone(),
        email: payload.email.clone().unwrap_or_default(),
        phone: payload.phone.clone().unwrap_or_default(),
        org_id: payload.org_id.unwrap_or(0),
        role: payload.role.clone().unwrap_or("user".to_string()),
        status: payload.status.unwrap_or(1) as i32,
    };

    let response = client
        .add_user(Request::new(req))
        .await
        .map_err(|e| AppErr::Other(e.into()))?;
    let id = response.into_inner().id;
    appstate.inc_save_count();
    info!("save count: {}", appstate.get_save_count());

    Ok(ResWrapper::success(id))
}

pub async fn query_user(
    State(appstate): State<AppState>,
) -> Result<ResWrapper<Vec<grpc_dsl::user::UserData>>, AppErr> {
    let mut client: GrpcClient = appstate.grpc_client.clone();
    let response = client
        .get_all_users(Request::new(grpc_dsl::common::Empty {}))
        .await
        .map_err(|e| AppErr::Other(e.into()))?;
    let users = response.into_inner().users;
    Ok(ResWrapper::success(users))
}

#[derive(Deserialize)]
pub struct IdParam {
    pub id: u32,
}

#[derive(Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HttpCreateUser {
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

pub async fn query_user_by_id(
    State(appstate): State<AppState>,
    Form(id_param): Form<IdParam>,
) -> Result<ResWrapper<Option<grpc_dsl::user::UserData>>, AppErr> {
    let redis_cli = appstate.redis_client.clone();
    let key = format!("user:{}", id_param.id);

    if let Some(user_json) = redis_cli.redis_get(&key).await? {
        if let Ok(user) = serde_json::from_str::<grpc_dsl::user::UserData>(&user_json) {
            return Ok(ResWrapper::success(Some(user)));
        } else {
            info!("redis cache format changed, ignore key {}", key);
        }
    }

    let mut client: GrpcClient = appstate.grpc_client.clone();
    let response = client
        .get_user_by_id(Request::new(IdRequest { id: id_param.id }))
        .await
        .map_err(|e| AppErr::Other(e.into()))?;
    let res = response.into_inner();

    if res.found {
        if let Some(u) = res.user.clone() {
            let json = serde_json::to_string(&u)?;
            redis_cli.redis_set(&key, &json).await?;
            info!("set redis cache key:{} value:{}", key, json);
        }
        Ok(ResWrapper::success(res.user))
    } else {
        Ok(ResWrapper::success(None))
    }
}

pub async fn delete_user(
    State(appstate): State<AppState>,
    Form(id_param): Form<IdParam>,
) -> Result<ResWrapper<u32>, AppErr> {
    let mut client: GrpcClient = appstate.grpc_client.clone();
    let response = client
        .delete_user(Request::new(IdRequest { id: id_param.id }))
        .await
        .map_err(|e| AppErr::Other(e.into()))?;
    let count = response.into_inner().count;

    let redis_client = appstate.redis_client.clone();
    tokio::spawn(async move {
        let _ = redis_client
            .redis_del(&format!("user:{}", id_param.id))
            .await;
        info!("delete user:{}", id_param.id);
    });

    Ok(ResWrapper::success(count))
}

#[derive(Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginRequest {
    pub user_name: String,
    pub password: String,
}

pub async fn login(
    State(appstate): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<ResWrapper<grpc_dsl::user::LoginResponse>, AppErr> {
    info!(
        "login user via gRPC, username: {}",
        payload.user_name
    );

    let mut client: GrpcClient = appstate.grpc_client.clone();
    let req = LoginUserRequest {
        user_name: payload.user_name,
        password: payload.password,
    };

    let response = client
        .login(Request::new(req))
        .await
        .map_err(|e| AppErr::Other(e.into()))?;
    let login_result = response.into_inner();

    Ok(ResWrapper::success(login_result))
}

#[derive(Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterRequest {
    pub emp_id: String,
    pub user_name: String,
    pub password: String,
    pub email: String,
    pub phone: String,
    pub org_id: Option<u32>,
    pub role: String,
}

pub async fn register(
    State(appstate): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<ResWrapper<grpc_dsl::user::RegisterResponse>, AppErr> {
    info!(
        "register user via gRPC, username: {}",
        payload.user_name
    );

    let mut client: GrpcClient = appstate.grpc_client.clone();
    let req = RegisterUserRequest {
        emp_id: payload.emp_id,
        user_name: payload.user_name,
        password: payload.password,
        email: payload.email,
        phone: payload.phone,
        org_id: payload.org_id.unwrap_or(0),
        role: payload.role,
    };

    let response = client
        .register(Request::new(req))
        .await
        .map_err(|e| AppErr::Other(e.into()))?;
    let register_result = response.into_inner();

    Ok(ResWrapper::success(register_result))
}

pub async fn update_user(
    State(appstate): State<AppState>,
    Json(user): Json<HttpCreateUser>,
) -> Result<ResWrapper<u32>, AppErr> {
    if user.id.is_none() {
        return Err(AppErr::ParamError("id is required for update".to_string()));
    }

    let mut client: GrpcClient = appstate.grpc_client.clone();
    let req = AddUserRequest {
        id: user.id.unwrap(),
        emp_id: user.emp_id,
        user_name: user.user_name,
        password: user.password.clone(),
        email: user.email.clone().unwrap_or_default(),
        phone: user.phone.clone().unwrap_or_default(),
        org_id: user.org_id.unwrap_or(0),
        role: user.role.clone().unwrap_or("user".to_string()),
        status: user.status.unwrap_or(1) as i32,
    };

    let response = client
        .update_user(Request::new(req.clone()))
        .await
        .map_err(|e| AppErr::Other(e.into()))?;
    let count = response.into_inner().count;
    let user_id = req.id;

    let redis_client = appstate.redis_client.clone();
    tokio::spawn(async move {
        let key = format!("user:{}", user_id);
        let mut client_inner = client;
        let response = client_inner
            .get_user_by_id(Request::new(IdRequest { id: user_id }))
            .await;
        if let Ok(resp) = response {
            let res = resp.into_inner();
            if let Some(u) = res.user {
                if let Ok(value) = serde_json::to_string(&u) {
                    let _ = redis_client.redis_set(&key, &value).await;
                    info!("update redis cache:{}", key);
                }
            }
        }
    });

    Ok(ResWrapper::success(count))
}
