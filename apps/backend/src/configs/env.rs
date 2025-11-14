use once_cell::sync::Lazy;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub jwt_secret: String,
}

pub static ENV_CONFIG: Lazy<Config> = Lazy::new(|| envy::from_env().unwrap());
