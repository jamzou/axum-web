mod db;
mod dao;
mod entity;

use std::env;

use dotenvy::dotenv;
use sea_orm::DatabaseConnection;
use tonic::{transport::Server, Request, Response, Status};
use tracing::info;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::prelude::*;
use tracing_subscriber::{fmt::Layer, EnvFilter};
use reqwest::Client;

use crate::dao::{UserDao, UserDaoImpl};
use crate::entity::prelude::{User as DbUser, CreateUser as DbCreateUser};
use grpc_dsl::user::user_service_server::{UserService, UserServiceServer};
use grpc_dsl::user::{CreateUserRequest, IdRequest, IdResponse, RowsAffected, UserData, UserListResponse, UserResponse, Empty};

struct UserServiceImpl<D: UserDao + Send + Sync + 'static> {
    dao: D,
}

impl From<DbUser> for UserData {
    fn from(u: DbUser) -> Self {
        UserData {
            id: u.id,
            emp_id: u.emp_id,
            user_name: u.user_name,
            age: u.age.unwrap_or_default() as u32,
            birthday: u.birthday.unwrap_or_default(),
        }
    }
}

impl From<CreateUserRequest> for DbCreateUser {
    fn from(req: CreateUserRequest) -> Self {
        DbCreateUser {
            id: if req.id == 0 { None } else { Some(req.id) },
            emp_id: req.emp_id,
            user_name: req.user_name,
            age: req.age as u8,
            birthday: req.birthday,
        }
    }
}

async fn register_with_consul(host: &str, port: u16) -> Result<(), anyhow::Error> {
    let consul_addr = env::var("CONSUL_HTTP_ADDR").unwrap_or_else(|_| "http://127.0.0.1:8500".to_string());
    let register_url = format!(
        "{}/v1/agent/service/register",
        consul_addr.trim_end_matches('/')
    );

    let payload = serde_json::json!({
        "ID": format!("core-user-{}-{}", host, port),
        "Name": "core-user",
        "Address": host,
        "Port": port,
        "Check": {
            "TCP": format!("{}:{}", host, port),
            "Interval": "10s"
        }
    });

    let client = Client::new();
    client
        .put(register_url)
        .json(&payload)
        .send()
        .await?
        .error_for_status()?;

    Ok(())
}

#[tonic::async_trait]
impl<D> UserService for UserServiceImpl<D>
where
    D: UserDao + Send + Sync + 'static,
{
    async fn add_user(&self, request: Request<CreateUserRequest>) -> Result<Response<IdResponse>, Status> {
        let req = request.into_inner();
        let create: DbCreateUser = req.into();
        let id = self.dao.add_user(&create).await.map_err(|e| Status::internal(e.to_string()))?;
        Ok(Response::new(IdResponse { id }))
    }

    async fn get_all_users(&self, _request: Request<Empty>) -> Result<Response<UserListResponse>, Status> {
        let users = self.dao.get_all_users().await.map_err(|e| Status::internal(e.to_string()))?;
        let users = users.into_iter().map(UserData::from).collect();
        Ok(Response::new(UserListResponse { users }))
    }

    async fn get_user_by_id(&self, request: Request<IdRequest>) -> Result<Response<UserResponse>, Status> {
        let id = request.into_inner().id;
        let user = self.dao.query_user_by_id(id).await.map_err(|e| Status::internal(e.to_string()))?;
        match user {
            Some(u) => Ok(Response::new(UserResponse { found: true, user: Some(UserData::from(u)) })),
            None => Ok(Response::new(UserResponse { found: false, user: None })),
        }
    }

    async fn delete_user(&self, request: Request<IdRequest>) -> Result<Response<RowsAffected>, Status> {
        let id = request.into_inner().id;
        let count = self.dao.delete_user(id).await.map_err(|e| Status::internal(e.to_string()))?;
        Ok(Response::new(RowsAffected { count }))
    }

    async fn update_user(&self, request: Request<CreateUserRequest>) -> Result<Response<RowsAffected>, Status> {
        let req = request.into_inner();
        let create: DbCreateUser = req.into();
        let count = self.dao.update_user(&create).await.map_err(|e| Status::internal(e.to_string()))?;
        Ok(Response::new(RowsAffected { count }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let file_appender = RollingFileAppender::new(
        Rotation::DAILY,
        "./logs",
        "core.log",
    );

    let file_layer = Layer::new()
        .with_writer(file_appender)
        .with_ansi(false)
        .with_line_number(true);

    let console_layer = Layer::new()
        .with_writer(std::io::stderr)
        .with_line_number(true);

    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        .with(file_layer)
        .with(console_layer)
        .init();

    let db: DatabaseConnection = db::establish_conn().await;
    let user_dao = UserDaoImpl::new(db.clone());
    let svc = UserServiceImpl { dao: user_dao };

    let host = env::var("CORE_SERVICE_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port: u16 = 50051;
    if let Err(e) = register_with_consul(&host, port).await {
        tracing::warn!("failed to register core service in consul: {}", e);
    }

    let addr = format!("0.0.0.0:{}", port).parse()?;
    info!("gRPC core listening on {}", addr);

    Server::builder()
        .add_service(UserServiceServer::new(svc))
        .serve(addr)
        .await?;

    Ok(())
}
