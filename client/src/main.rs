pub mod context;
pub mod controller;
pub mod redisconfig;

use std::env;

use axum::{routing::post, Router};
use dotenvy::dotenv;
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::prelude::*;
use tracing_subscriber::{fmt::Layer, EnvFilter};
use tonic::transport::Channel;
use reqwest::Client;

type GrpcClient = grpc_dsl::user::user_service_client::UserServiceClient<Channel>;

use crate::context::appstate::AppState;
use crate::controller::user::{add_user, delete_user, query_user, query_user_by_id, update_user};

#[derive(serde::Deserialize, Clone)]
struct ConsulService {
    #[serde(rename = "Address")]
    address: String,
    #[serde(rename = "Port")]
    port: u16,
}

#[derive(serde::Deserialize)]
struct ConsulServiceEntry {
    #[serde(rename = "Service")]
    service: ConsulService,
}

async fn discover_core_addr_from_consul() -> Option<String> {
    let consul_addr = env::var("CONSUL_HTTP_ADDR").unwrap_or_else(|_| "http://127.0.0.1:8500".to_string());
    let url = format!(
        "{}/v1/health/service/core-user?passing=true",
        consul_addr.trim_end_matches('/')
    );

    let resp = Client::new().get(url).send().await.ok()?;
    let entries: Vec<ConsulServiceEntry> = resp.json().await.ok()?;
    let service = entries.first()?.service.clone();
    let address = if service.address.is_empty() {
        "127.0.0.1".to_string()
    } else {
        service.address
    };
    Some(format!("http://{}:{}", address, service.port))
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let file_appender = RollingFileAppender::new(
        Rotation::DAILY,
        "./logs",
        "application.log",
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

    let grpc_addr = if let Ok(addr) = env::var("CORE_GRPC_ADDR") {
        addr
    } else if let Some(addr) = discover_core_addr_from_consul().await {
        addr
    } else {
        "http://127.0.0.1:50051".to_string()
    };
    let channel = Channel::from_shared(grpc_addr).unwrap().connect().await.unwrap();
    let grpc_client = GrpcClient::new(channel);

    let redis_client = redisconfig::init_redis().await;
    let appstate = AppState::new(grpc_client, redis_client);

    let user_routes = Router::new()
        .route("/query_user", post(query_user))
        .route("/add_user", post(add_user))
        .route("/delete_user", post(delete_user))
        .route("/update_user", post(update_user))
        .route("/query_user_by_id", post(query_user_by_id));

    let api_routes = Router::new().nest("/user", user_routes);

    let app = Router::new()
        .nest("/api", api_routes)
        .layer(TraceLayer::new_for_http())
        .with_state(appstate);

    let port = env::var("PORT").unwrap_or("8080".to_string());
    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap();
    let addr = listener.local_addr().unwrap();
    info!("client http server listening on {}", addr);
    axum::serve(listener, app).await.unwrap();
}
