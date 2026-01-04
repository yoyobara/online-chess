use std::sync::Arc;

use redis::aio::MultiplexedConnection;

use crate::{configs::Config, repositories::user::UserRepository, utils::matchmaking::RegistryMap};

#[derive(Clone, Debug)]
pub struct AppState {
    pub config: Arc<Config>,
    pub user_repo: Arc<dyn UserRepository>,
    pub redis_connection: MultiplexedConnection,

    pub matchmaking_registry_map: RegistryMap,
}

impl AppState {
    pub async fn new(
        config: Arc<Config>,
        user_repo: Arc<dyn UserRepository>,
        matchmaking_registry_map: RegistryMap,
        redis_connection: MultiplexedConnection,
    ) -> Self {
        Self {
            config,
            user_repo,
            matchmaking_registry_map,
            redis_connection,
        }
    }
}
