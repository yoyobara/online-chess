use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};

use crate::configs::env::{load_config, Config};

#[derive(Clone)]
pub struct AppState {
    pub config: Config,
    pub pool: Pool<Sqlite>,
}

impl AppState {
    pub async fn new() -> Self {
        let config = load_config();
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .min_connections(1)
            .connect(&config.database_url)
            .await
            .unwrap();

        Self { config, pool }
    }
}
