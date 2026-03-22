use std::sync::Arc;

use rust_chess::board::Board;

use crate::{
    configs::Config,
    repositories::{
        ephemeral_match::EphemeralMatchRepository, persistent_match::PersistentMatchRepository,
        user::UserRepository,
    },
    utils::pubsub::PubSub,
};

pub type PubSubFactory = dyn Fn() -> Box<dyn PubSub> + Send + Sync;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub pubsub_factory: Arc<PubSubFactory>,
    pub user_repo: Arc<dyn UserRepository>,
    pub ephemeral_match_repo: Arc<dyn EphemeralMatchRepository>,
    pub persistent_match_repo: Arc<dyn PersistentMatchRepository>,
    pub initial_board: Board,
}

impl AppState {
    pub async fn new(
        config: Arc<Config>,
        pubsub_factory: Arc<PubSubFactory>,
        user_repo: Arc<dyn UserRepository>,
        ephemeral_match_repo: Arc<dyn EphemeralMatchRepository>,
        persistent_match_repo: Arc<dyn PersistentMatchRepository>,
        initial_board: Board,
    ) -> Self {
        Self {
            config,
            pubsub_factory,
            user_repo,
            ephemeral_match_repo,
            persistent_match_repo,
            initial_board,
        }
    }
}
