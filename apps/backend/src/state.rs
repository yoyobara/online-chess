use std::sync::Arc;

use sqlx::{Pool, Postgres};
use tokio::sync::broadcast::Sender;

use crate::{configs::Config, internal_broadcast::InternalMessageWithMetadata};

#[derive(Clone, Debug)]
pub struct AppState {
    pub config: Arc<Config>,
    pub pool: Pool<Postgres>,
    pub internal_sender: Sender<InternalMessageWithMetadata>,
}

impl AppState {
    pub async fn new(
        config: Arc<Config>,
        pool: Pool<Postgres>,
        internal_sender: Sender<InternalMessageWithMetadata>,
    ) -> Self {
        Self {
            config,
            pool,
            internal_sender,
        }
    }
}
