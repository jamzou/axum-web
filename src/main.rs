pub mod api;
pub mod context;
pub mod db;

use axum::{routing::post, Router};
use dotenvy::dotenv;
use tower_http::trace::TraceLayer;
use tracing::info;

use crate::api::query_user;

#[tokio::main]
async fn main() {
    dotenv().ok();
    // initialize tracing
    tracing_subscriber::fmt::init();

    let pool = db::establish_conn().await;

    // build our application with a route
    let app = Router::new()
        .route("/query_user", post(query_user))
        .layer(TraceLayer::new_for_http())
        .with_state(pool);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    let addr = listener.local_addr().unwrap();
    info!("server listening on {}", addr);
    axum::serve(listener, app).await.unwrap();
}
