mod config;
mod dao;
mod db;
mod entity;
mod service;
use grpc_dsl::org::org_service_server::OrgServiceServer;
use reqwest::Client;
use sea_orm::DatabaseConnection;
use service::prelude::*;
use tonic::transport::Server;
use tracing::info;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::prelude::*;
use tracing_subscriber::{fmt::Layer, EnvFilter};

use crate::dao::{OrgDao, OrgDaoImpl, UserDao, UserDaoImpl};
//必须用这个来创建对应的 UserServiceServer
use grpc_dsl::user::user_service_server::UserServiceServer;
async fn register_with_consul(
    consul_addr: &str,
    host: &str,
    port: u16,
) -> Result<(), anyhow::Error> {
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config_path = config::get_config_path();
    let config = config::load_config(&config_path).expect("加载配置文件出错");
    let file_appender = RollingFileAppender::new(
        Rotation::DAILY,
        config.log.directory.clone(),
        config.log.file.clone(),
    );

    let file_layer = Layer::new()
        .with_writer(file_appender)
        .with_ansi(false)
        .with_line_number(true);

    let console_layer = Layer::new()
        .with_writer(std::io::stderr)
        .with_line_number(true);

    //日志输出级别
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(&config.log.level));

    tracing_subscriber::registry()
        .with(env_filter)
        .with(file_layer)
        .with(console_layer)
        .init();

    let db: DatabaseConnection = db::establish_conn(&config).await;
    let user_dao = UserDaoImpl::new(db.clone());
    let svc = UserServiceImpl::new(user_dao);

    let port = config.server.port;
    use local_ip_address::local_ip;
    let host = local_ip().expect("获取当前ip失败").to_string();
    println!("This is my local IP address: {}", host);
    if let Err(e) = register_with_consul(&config.consul.address, &host, port).await {
        tracing::warn!("failed to register core service in consul: {}", e);
    }

    let addr = format!("{}:{}", host, port).parse()?;
    info!("gRPC core listening on {}", addr);

    Server::builder()
        .add_service(UserServiceServer::new(svc))
        .add_service(OrgServiceServer::new(OrgServiceImpl::new(OrgDaoImpl::new(db))))
        .serve(addr)
        .await?;

    Ok(())
}
