use std::sync::Arc;

use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub jwt_secret: String,
    pub database_url: String,
}

pub fn load_env() -> Arc<Config> {
    Arc::new(envy::from_env().unwrap())
}
