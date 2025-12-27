use std::sync::Arc;

use sqlx::{Pool, Postgres};
use tokio::sync::broadcast::Sender;

use crate::{
    configs::Config, internal_broadcast::InternalMessageWithMetadata,
    repositories::user::UserRepository,
};

#[derive(Clone, Debug)]
pub struct AppState {
    pub config: Arc<Config>,
    pub internal_sender: Sender<InternalMessageWithMetadata>,
    pub pool: Pool<Postgres>,
    pub user_repo: Arc<dyn UserRepository>,
}

impl AppState {
    pub async fn new(
        config: Arc<Config>,
        internal_sender: Sender<InternalMessageWithMetadata>,
        pool: Pool<Postgres>,
        user_repo: Arc<dyn UserRepository>,
    ) -> Self {
        Self {
            config,
            internal_sender,
            pool,
            user_repo,
        }
    }
}
