use std::sync::Arc;

use redis::aio::MultiplexedConnection;

use crate::{
    configs::Config,
    repositories::{r#match::MatchRepository, user::UserRepository},
    utils::matchmaking::RegistryMap,
};

#[derive(Clone, Debug)]
pub struct AppState {
    pub config: Arc<Config>,
    pub user_repo: Arc<dyn UserRepository>,
    pub match_repo: Arc<dyn MatchRepository>,

    pub redis_connection: MultiplexedConnection,

    pub matchmaking_registry_map: RegistryMap,
}

impl AppState {
    pub async fn new(
        config: Arc<Config>,
        user_repo: Arc<dyn UserRepository>,
        match_repo: Arc<dyn MatchRepository>,
        redis_connection: MultiplexedConnection,
        matchmaking_registry_map: RegistryMap,
    ) -> Self {
        Self {
            config,
            user_repo,
            match_repo,
            redis_connection,
            matchmaking_registry_map,
        }
    }
}
