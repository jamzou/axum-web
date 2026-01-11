use std::sync::{Arc, atomic::AtomicUsize};

use tonic::transport::Channel;

use crate::redisconfig::RedisTemplate;
use grpc_dsl::user::user_service_client::UserServiceClient;

pub type GrpcClient = UserServiceClient<Channel>;

#[derive(Clone)]
pub struct AppState {
    pub grpc_client: GrpcClient,
    pub save_count: Arc<AtomicUsize>,
    pub redis_client: Arc<RedisTemplate>,
}

impl AppState {
    pub fn new(grpc_client: GrpcClient, redis_client: RedisTemplate) -> Self {
        Self {
            grpc_client,
            save_count: Arc::new(AtomicUsize::new(0)),
            redis_client: Arc::new(redis_client),
        }
    }

    pub fn get_save_count(&self) -> usize {
        self.save_count.load(std::sync::atomic::Ordering::Relaxed)
    }

    pub fn inc_save_count(&self) {
        self.save_count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }
}
