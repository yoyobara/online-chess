use std::sync::Arc;

use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub port: u16,
    pub jwt_secret: String,
    pub database_url: String,
    pub redis_url: String,
}

pub fn load_env() -> anyhow::Result<Arc<Config>> {
    envy::from_env().map(Arc::new).map_err(Into::into)
}
