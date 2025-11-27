pub mod context;
pub mod controller;
pub mod dao;
pub mod db;
pub mod redisconfig;
pub mod domain;

use std::env;

use axum::{routing::post, Router};
use dotenvy::dotenv;
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::prelude::*;
use tracing_subscriber::{fmt::Layer, EnvFilter};

use crate::context::appstate::AppState;
use crate::controller::user::{add_user, delete_user, query_user, query_user_by_id, update_user};
use crate::dao::UserDaoImpl;

#[tokio::main]
async fn main() {
    dotenv().ok();
    // initialize tracing
    // 创建每日轮转的日志文件
    let file_appender = RollingFileAppender::new(
        Rotation::DAILY,
        "./logs",          // 日志文件目录
        "application.log", // 日志文件前缀
    );

    // 创建文件层
    let file_layer = Layer::new()
        .with_writer(file_appender)
        .with_ansi(false)
        .with_line_number(true);

    // 创建控制台层
    let console_layer = Layer::new()
        .with_writer(std::io::stderr)
        .with_line_number(true);

    // 初始化订阅器
    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        .with(file_layer)
        .with(console_layer)
        .init();

    let pool = db::establish_conn().await;
    let redis_client = redisconfig::init_redis().await;
    let user_dao = UserDaoImpl::new(pool.clone());
    let appstate = AppState::new(pool, user_dao, redis_client);

    // 创建用户相关路由组
    let user_routes = Router::new()
        .route("/query_user", post(query_user))
        .route("/add_user", post(add_user))
        .route("/delete_user", post(delete_user))
        .route("/update_user", post(update_user))
        .route("/query_user_by_id", post(query_user_by_id));

    // 创建 API 路由组
    let api_routes = Router::new().nest("/user", user_routes);

    // 主应用路由
    let app = Router::new()
        .nest("/api", api_routes)
        .layer(TraceLayer::new_for_http())
        .with_state(appstate);

    // run our app with hyper, default 8080 listening globally on port 3000
    let port = env::var("PORT").unwrap_or("8080".to_string());
    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap();
    let addr = listener.local_addr().unwrap();
    info!("server listening on {}", addr);
    axum::serve(listener, app).await.unwrap();
}
