use sqlx::{Pool, Postgres};

use crate::configs::{load_env, load_pool, Config};

#[derive(Clone)]
pub struct AppState {
    pub config: Config,
    pub pool: Pool<Postgres>,
}

impl AppState {
    pub async fn new() -> Self {
        let config = load_env();
        let pool = load_pool(&config.database_url).await;

        Self { config, pool }
    }
}
