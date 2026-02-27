use std::sync::Arc;

use rust_chess::board::Board;

use crate::{
    configs::Config,
    repositories::{r#match::MatchRepository, user::UserRepository},
    utils::pubsub::PubSub,
};

pub type PubSubFactory = dyn Fn() -> Box<dyn PubSub> + Send + Sync;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub pubsub_factory: Arc<PubSubFactory>,
    pub user_repo: Arc<dyn UserRepository>,
    pub match_repo: Arc<dyn MatchRepository>,
    pub initial_board: Board,
}

impl AppState {
    pub async fn new(
        config: Arc<Config>,
        pubsub_factory: Arc<PubSubFactory>,
        user_repo: Arc<dyn UserRepository>,
        match_repo: Arc<dyn MatchRepository>,
        initial_board: Board,
    ) -> Self {
        Self {
            config,
            pubsub_factory,
            user_repo,
            match_repo,
            initial_board,
        }
    }
}
